use crate::entity::ai::goal::{
    CombinedAttackGoal, DoNothingGoal, FollowTargetGoal, MeleeAttackGoal, RandomLookAroundGoal,
    RandomStrollGoal, RangedAttackGoal,
};
use crate::entity::ai::target::{ClosestEntityTarget, LastEntityDamagerTarget};
use crate::entity::ai::{CreatureAiAction, EntityAiGroupBuilder, GoalSelector, TargetSelector};
use crate::entity::{CreatureEntity, Damage, Entity, EntityId, EntityPosition};
use crate::world::{Block, BlockPosition, ChunkPosition, World, WorldSnapshot};
use spinel_network::types::Identifier;
use spinel_registry::EntityType;
use spinel_registry::damage_type::DamageType;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

#[test]
fn creature_kill_uses_minestom_removal_animation_delay() {
    let mut animated_creature = CreatureEntity::new(EntityType::ZOMBIE);

    assert_eq!(animated_creature.removal_animation_delay_millis(), 1000);
    assert!(animated_creature.kill());
    assert!(!animated_creature.is_removed());

    (0..19).for_each(|_| animated_creature.entity_mut().tick());

    assert!(!animated_creature.is_removed());
    animated_creature.entity_mut().tick();
    assert!(animated_creature.is_removed());

    let mut immediately_removed_creature = CreatureEntity::new(EntityType::ZOMBIE);
    immediately_removed_creature.set_removal_animation_delay_millis(0);

    assert!(immediately_removed_creature.kill());
    assert!(immediately_removed_creature.is_removed());
}

#[test]
fn ai_group_preempts_lower_priority_goal_in_minestom_order() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let high_should_start = Arc::new(AtomicBool::new(false));
    let high_should_end = Arc::new(AtomicBool::new(false));
    let low_should_start = Arc::new(AtomicBool::new(true));
    let low_should_end = Arc::new(AtomicBool::new(false));
    let high = RecordingGoal::new(
        "high",
        Arc::clone(&events),
        Arc::clone(&high_should_start),
        Arc::clone(&high_should_end),
    );
    let low = RecordingGoal::new(
        "low",
        Arc::clone(&events),
        Arc::clone(&low_should_start),
        Arc::clone(&low_should_end),
    );
    let group = EntityAiGroupBuilder::default()
        .add_goal_selector(high)
        .add_goal_selector(low)
        .build();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.add_ai_group(group);
    let world = ai_world();
    let snapshot = world.update_snapshot();

    creature.ai_tick(&snapshot, 1);
    assert_eq!(*events.lock().unwrap(), vec!["low:start", "low:tick"]);

    high_should_start.store(true, Ordering::SeqCst);
    creature.ai_tick(&snapshot, 2);
    assert_eq!(
        *events.lock().unwrap(),
        vec![
            "low:start",
            "low:tick",
            "low:end",
            "high:start",
            "high:tick"
        ]
    );

    high_should_start.store(false, Ordering::SeqCst);
    high_should_end.store(true, Ordering::SeqCst);
    low_should_start.store(false, Ordering::SeqCst);
    creature.ai_tick(&snapshot, 3);
    assert_eq!(
        *events.lock().unwrap(),
        vec![
            "low:start",
            "low:tick",
            "low:end",
            "high:start",
            "high:tick",
            "high:end"
        ]
    );
    assert!(creature.ai_groups()[0].current_goal_selector().is_none());
}

#[test]
fn active_goal_identity_survives_goal_selector_reordering() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let high = RecordingGoal::new(
        "high",
        Arc::clone(&events),
        Arc::new(AtomicBool::new(false)),
        Arc::new(AtomicBool::new(false)),
    );
    let low = RecordingGoal::new(
        "low",
        Arc::clone(&events),
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(false)),
    );
    let group = EntityAiGroupBuilder::default()
        .add_goal_selector(high)
        .add_goal_selector(low)
        .build();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.add_ai_group(group);
    let world = ai_world();
    let snapshot = world.update_snapshot();

    creature.ai_tick(&snapshot, 1);
    creature.ai_groups_mut()[0].goal_selectors_mut().swap(0, 1);
    creature.ai_tick(&snapshot, 2);

    assert_eq!(
        *events.lock().unwrap(),
        vec!["low:start", "low:tick", "low:tick"]
    );
}

#[test]
fn removed_active_goal_keeps_running_until_it_ends() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let goal = RecordingGoal::new(
        "low",
        Arc::clone(&events),
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(false)),
    );
    let group = EntityAiGroupBuilder::default()
        .add_goal_selector(goal)
        .build();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.add_ai_group(group);
    let world = ai_world();
    let snapshot = world.update_snapshot();

    creature.ai_tick(&snapshot, 1);
    creature.ai_groups_mut()[0].goal_selectors_mut().clear();
    creature.ai_tick(&snapshot, 2);

    assert_eq!(
        *events.lock().unwrap(),
        vec!["low:start", "low:tick", "low:tick"]
    );
    assert!(creature.ai_groups()[0].current_goal_selector().is_some());
}

#[test]
fn current_goal_selector_rejects_a_selector_attached_to_another_group() {
    let first_group = EntityAiGroupBuilder::default()
        .add_goal_selector(RecordingGoal::new(
            "first",
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicBool::new(false)),
        ))
        .build();
    let selector = first_group.goal_selectors()[0].clone();
    let mut second_group = EntityAiGroupBuilder::default().build();

    assert!(!second_group.set_current_goal_selector(Some(selector)));
    assert!(second_group.current_goal_selector().is_none());
}

#[test]
fn goal_target_lookup_uses_target_selector_priority() {
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    let expected = creature.entity_id();
    let world = ai_world();
    let snapshot = world.update_snapshot();
    let mut goal = RecordingGoal::new(
        "goal",
        Arc::new(Mutex::new(Vec::new())),
        Arc::new(AtomicBool::new(false)),
        Arc::new(AtomicBool::new(false)),
    );
    let mut targets: Vec<Box<dyn TargetSelector>> = vec![
        Box::new(FixedTarget(None)),
        Box::new(FixedTarget(Some(expected))),
    ];

    assert_eq!(
        goal.find_target(&creature, &snapshot, &mut targets),
        Some(expected)
    );

    let group = EntityAiGroupBuilder::default()
        .add_target_selector(FixedTarget(None))
        .build();
    creature.add_ai_group(group);
    assert_eq!(creature.ai_groups()[0].target_selectors().len(), 1);
}

#[test]
fn closest_and_last_damager_targets_match_minestom_selection_rules() {
    let mut world = ai_world();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(1.5, 65.0, 1.5, 0.0, 0.0));
    let creature_id = creature.entity_id();
    let mut near_target = CreatureEntity::new(EntityType::COW);
    near_target.set_position(EntityPosition::new(3.5, 65.0, 1.5, 0.0, 0.0));
    let near_target_id = near_target.entity_id();
    let mut far_target = CreatureEntity::new(EntityType::COW);
    far_target.set_position(EntityPosition::new(8.5, 65.0, 1.5, 0.0, 0.0));
    world.add_entity(Entity::Creature(creature));
    world.add_entity(Entity::Creature(near_target));
    world.add_entity(Entity::Creature(far_target));
    let snapshot = world.update_snapshot();
    let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    let mut closest =
        ClosestEntityTarget::new(10.0, |entity| entity.entity_type() == EntityType::COW);

    assert_eq!(
        closest.find_target(creature, &snapshot),
        Some(near_target_id)
    );

    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    creature
        .entity_mut()
        .apply_damage(Damage::new(DamageType::GENERIC, 1.0).with_source(near_target_id));
    let mut last_damager = LastEntityDamagerTarget::new(3.0);
    assert_eq!(
        last_damager.find_target(creature, &snapshot),
        Some(near_target_id)
    );
    let mut out_of_range_last_damager = LastEntityDamagerTarget::new(1.0);
    assert_eq!(
        out_of_range_last_damager.find_target(creature, &snapshot),
        None
    );
}

#[test]
fn follow_target_owns_creature_target_and_starts_navigation() {
    let mut world = ai_world();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(1.5, 65.0, 1.5, 0.0, 0.0));
    let creature_id = creature.entity_id();
    let mut target = CreatureEntity::new(EntityType::COW);
    target.set_position(EntityPosition::new(8.5, 65.0, 1.5, 0.0, 0.0));
    let target_id = target.entity_id();
    let group = EntityAiGroupBuilder::default()
        .add_goal_selector(FollowTargetGoal::new(Duration::from_millis(100)))
        .add_target_selector(ClosestEntityTarget::new(16.0, |entity| {
            entity.entity_type() == EntityType::COW
        }))
        .build();
    creature.add_ai_group(group);
    world.add_entity(Entity::Creature(creature));
    world.add_entity(Entity::Creature(target));
    let snapshot = world.update_snapshot();
    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };

    creature.ai_tick(&snapshot, 1);

    assert_eq!(creature.target(), Some(target_id));
    assert_eq!(
        creature
            .navigator()
            .path_position()
            .map(|position| position.x()),
        Some(8.5)
    );
}

#[test]
fn random_look_and_stroll_goal_public_configuration_matches_minestom_surface() {
    let world = ai_world();
    let snapshot = world.update_snapshot();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(1.5, 65.0, 1.5, 0.0, 0.0));
    let mut look = RandomLookAroundGoal::with_generators(1, || 1, |_| (1.0, 0.0, 0.0));
    let mut targets = Vec::new();

    assert!(look.should_start(&creature, &snapshot, &mut targets));
    look.start(&mut creature, &snapshot, &mut targets);
    look.tick(&mut creature, &snapshot, &mut targets, 1);
    assert_eq!(creature.position().yaw(), -90.0);
    assert!(!look.should_end(&creature, &snapshot, &mut targets));
    look.tick(&mut creature, &snapshot, &mut targets, 2);
    assert!(look.should_end(&creature, &snapshot, &mut targets));

    let stroll = RandomStrollGoal::new(4);
    assert_eq!(stroll.radius(), 4);
}

#[test]
fn do_nothing_goal_clamps_chance_and_ends_after_its_duration() {
    let world = ai_world();
    let snapshot = world.update_snapshot();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    let mut targets = Vec::new();
    let mut goal = DoNothingGoal::new(0, 2.0);

    assert!(goal.should_start(&creature, &snapshot, &mut targets));
    goal.start(&mut creature, &snapshot, &mut targets);
    assert!(goal.should_end(&creature, &snapshot, &mut targets));
    goal.end(&mut creature, &snapshot, &mut targets);
}

#[test]
fn melee_and_combined_goals_queue_minestom_attack_branches() {
    let mut world = ai_world();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(1.5, 65.0, 1.5, 0.0, 0.0));
    let creature_id = creature.entity_id();
    let mut target = CreatureEntity::new(EntityType::COW);
    target.set_position(EntityPosition::new(2.5, 65.0, 1.5, 0.0, 0.0));
    let target_id = target.entity_id();
    world.add_entity(Entity::Creature(creature));
    world.add_entity(Entity::Creature(target));
    let snapshot = world.update_snapshot();
    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    let mut target_selectors: Vec<Box<dyn TargetSelector>> =
        vec![Box::new(FixedTarget(Some(target_id)))];
    let mut melee = MeleeAttackGoal::new(2.0, Duration::ZERO);

    assert!(melee.should_start(creature, &snapshot, &mut target_selectors));
    melee.start(creature, &snapshot, &mut target_selectors);
    melee.tick(creature, &snapshot, &mut target_selectors, 1);
    assert!(matches!(
        creature.take_ai_actions().as_slice(),
        [CreatureAiAction::Attack {
            source,
            target,
            should_swing_main_hand: true,
        }] if *source == creature_id && *target == target_id
    ));

    let mut combined =
        CombinedAttackGoal::new(2, Duration::ZERO, 8, 1.0, 0.0, Duration::ZERO, 4, false).unwrap();
    assert!(combined.should_start(creature, &snapshot, &mut target_selectors));
    combined.start(creature, &snapshot, &mut target_selectors);
    combined.tick(creature, &snapshot, &mut target_selectors, 2);
    assert!(matches!(
        creature.take_ai_actions().as_slice(),
        [CreatureAiAction::Attack {
            source,
            target,
            should_swing_main_hand: true,
        }] if *source == creature_id && *target == target_id
    ));
}

#[test]
fn ranged_goal_world_tick_spawns_and_shoots_default_arrow() {
    let mut world = ai_world();
    let mut creature = CreatureEntity::new(EntityType::SKELETON);
    creature.set_position(EntityPosition::new(1.5, 65.0, 1.5, 0.0, 0.0));
    let creature_id = creature.entity_id();
    let mut target = CreatureEntity::new(EntityType::COW);
    target.set_position(EntityPosition::new(6.5, 65.0, 1.5, 0.0, 0.0));
    let target_id = target.entity_id();
    let ranged = RangedAttackGoal::new(Duration::ZERO, 10, 8, false, 1.0, 0.0).unwrap();
    let group = EntityAiGroupBuilder::default()
        .add_goal_selector(ranged)
        .add_target_selector(FixedTarget(Some(target_id)))
        .build();
    creature.add_ai_group(group);
    world.add_entity(Entity::Creature(creature));
    world.add_entity(Entity::Creature(target));

    world.tick();

    let projectile = world.entities().find_map(|entity| match entity {
        Entity::Projectile(projectile) => Some(projectile),
        _ => None,
    });
    assert!(projectile.is_some_and(|projectile| {
        projectile.shooter() == Some(creature_id)
            && projectile.entity_type() == EntityType::ARROW
            && projectile.velocity().0.x > 0.0
    }));
}

struct RecordingGoal {
    name: &'static str,
    events: Arc<Mutex<Vec<&'static str>>>,
    should_start: Arc<AtomicBool>,
    should_end: Arc<AtomicBool>,
}

impl RecordingGoal {
    fn new(
        name: &'static str,
        events: Arc<Mutex<Vec<&'static str>>>,
        should_start: Arc<AtomicBool>,
        should_end: Arc<AtomicBool>,
    ) -> Self {
        Self {
            name,
            events,
            should_start,
            should_end,
        }
    }

    fn record(&self, action: &'static str) {
        let event = match (self.name, action) {
            ("high", "start") => "high:start",
            ("high", "tick") => "high:tick",
            ("high", "end") => "high:end",
            ("low", "start") => "low:start",
            ("low", "tick") => "low:tick",
            ("low", "end") => "low:end",
            _ => "goal:event",
        };
        self.events.lock().unwrap().push(event);
    }
}

impl GoalSelector for RecordingGoal {
    fn should_start(
        &mut self,
        _creature: &CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.should_start.load(Ordering::SeqCst)
    }

    fn start(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.record("start");
    }

    fn tick(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
        _time: u64,
    ) {
        self.record("tick");
    }

    fn should_end(
        &mut self,
        _creature: &CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.should_end.load(Ordering::SeqCst)
    }

    fn end(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.record("end");
    }
}

struct FixedTarget(Option<EntityId>);

impl TargetSelector for FixedTarget {
    fn find_target(
        &mut self,
        _creature: &CreatureEntity,
        _world: &WorldSnapshot,
    ) -> Option<EntityId> {
        self.0
    }
}

fn ai_world() -> World {
    let mut world = World::new(Identifier::minecraft("ai"));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    for block_x in 0..16 {
        for block_z in 0..16 {
            world
                .set_block(BlockPosition::new(block_x, 64, block_z), Block::STONE)
                .unwrap();
        }
    }
    world
}
