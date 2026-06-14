use spinel::{
    nbt::{Nbt, NbtCompound},
    server::world::{Block, BlockInstance, BlockPosition, ChunkPosition, World},
};
use std::io;

#[derive(Clone, Copy)]
pub enum ShowcaseSignCommand {
    Player,
    Entity,
    Inventory,
    World,
}

pub struct ShowcaseSigns;

impl ShowcaseSigns {
    pub fn install(world: &mut World) -> io::Result<()> {
        Self::entries()
            .into_iter()
            .try_for_each(|entry| Self::install_entry(world, entry))
    }

    pub fn command_at_position(position: BlockPosition) -> Option<ShowcaseSignCommand> {
        Self::entries()
            .into_iter()
            .find(|entry| entry.position == position)
            .map(|entry| entry.command)
    }

    fn install_entry(world: &mut World, entry: ShowcaseSignEntry) -> io::Result<()> {
        let chunk_position = ChunkPosition::new(
            entry.position.x.div_euclid(16),
            entry.position.z.div_euclid(16),
        );
        let _ = world.load_chunk(chunk_position)?;
        world.set_block_instance(
            entry.position,
            BlockInstance::from(Block::OAK_SIGN).with_nbt(Some(Self::sign_nbt(entry))),
        )?;
        Ok(())
    }

    fn entries() -> [ShowcaseSignEntry; 4] {
        [
            ShowcaseSignEntry::new(
                BlockPosition::new(1, 4, 5),
                "Player",
                ShowcaseSignCommand::Player,
            ),
            ShowcaseSignEntry::new(
                BlockPosition::new(3, 4, 5),
                "Entity",
                ShowcaseSignCommand::Entity,
            ),
            ShowcaseSignEntry::new(
                BlockPosition::new(5, 4, 5),
                "Inventory",
                ShowcaseSignCommand::Inventory,
            ),
            ShowcaseSignEntry::new(
                BlockPosition::new(7, 4, 5),
                "World",
                ShowcaseSignCommand::World,
            ),
        ]
    }

    fn sign_nbt(entry: ShowcaseSignEntry) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert(
            "front_text".to_string(),
            Nbt::Compound(Self::blank_text_compound()),
        );
        compound.insert(
            "back_text".to_string(),
            Nbt::Compound(Self::label_text_compound(entry.label)),
        );
        compound.insert("is_waxed".to_string(), Nbt::Byte(1));
        compound
    }

    fn label_text_compound(label: &str) -> NbtCompound {
        Self::text_compound(Self::label_messages(label), true)
    }

    fn blank_text_compound() -> NbtCompound {
        Self::text_compound(Self::blank_messages(), false)
    }

    fn text_compound(messages: [Nbt; 4], has_glowing_text: bool) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("color".to_string(), Nbt::String("black".to_string()));
        compound.insert(
            "has_glowing_text".to_string(),
            Nbt::Byte(if has_glowing_text { 1 } else { 0 }),
        );
        compound.insert(
            "messages".to_string(),
            Nbt::List(Box::<[Nbt]>::from(messages)),
        );
        compound
    }

    fn label_messages(label: &str) -> [Nbt; 4] {
        [
            Nbt::String(label.to_string()),
            Nbt::String("Showcase".to_string()),
            Nbt::String("Right click".to_string()),
            Nbt::String(String::new()),
        ]
    }

    fn blank_messages() -> [Nbt; 4] {
        [
            Nbt::String(String::new()),
            Nbt::String(String::new()),
            Nbt::String(String::new()),
            Nbt::String(String::new()),
        ]
    }
}

#[derive(Clone, Copy)]
struct ShowcaseSignEntry {
    position: BlockPosition,
    label: &'static str,
    command: ShowcaseSignCommand,
}

impl ShowcaseSignEntry {
    const fn new(
        position: BlockPosition,
        label: &'static str,
        command: ShowcaseSignCommand,
    ) -> Self {
        Self {
            position,
            label,
            command,
        }
    }
}
