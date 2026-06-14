use crate::entity::Player;
use crate::entity::player::PlayerInputs;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerInputEvent {
    player: *mut Player,
    old_inputs: PlayerInputs,
}

impl PlayerInputEvent {
    pub fn new(player: *mut Player, old_inputs: PlayerInputs) -> Self {
        Self {
            player,
            old_inputs,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn is_holding_forward_key(&self) -> bool {
        self.inputs().forward
    }

    pub fn has_pressed_forward_key(&self) -> bool {
        !self.old_inputs.forward && self.inputs().forward
    }

    pub fn has_released_forward_key(&self) -> bool {
        self.old_inputs.forward && !self.inputs().forward
    }

    pub fn is_holding_backward_key(&self) -> bool {
        self.inputs().backward
    }

    pub fn has_pressed_backward_key(&self) -> bool {
        !self.old_inputs.backward && self.inputs().backward
    }

    pub fn has_released_backward_key(&self) -> bool {
        self.old_inputs.backward && !self.inputs().backward
    }

    pub fn is_holding_left_key(&self) -> bool {
        self.inputs().left
    }

    pub fn has_pressed_left_key(&self) -> bool {
        !self.old_inputs.left && self.inputs().left
    }

    pub fn has_released_left_key(&self) -> bool {
        self.old_inputs.left && !self.inputs().left
    }

    pub fn is_holding_right_key(&self) -> bool {
        self.inputs().right
    }

    pub fn has_pressed_right_key(&self) -> bool {
        !self.old_inputs.right && self.inputs().right
    }

    pub fn has_released_right_key(&self) -> bool {
        self.old_inputs.right && !self.inputs().right
    }

    pub fn is_holding_jump_key(&self) -> bool {
        self.inputs().jump
    }

    pub fn has_pressed_jump_key(&self) -> bool {
        !self.old_inputs.jump && self.inputs().jump
    }

    pub fn has_released_jump_key(&self) -> bool {
        self.old_inputs.jump && !self.inputs().jump
    }

    pub fn is_holding_shift_key(&self) -> bool {
        self.inputs().shift
    }

    pub fn has_pressed_shift_key(&self) -> bool {
        !self.old_inputs.shift && self.inputs().shift
    }

    pub fn has_released_shift_key(&self) -> bool {
        self.old_inputs.shift && !self.inputs().shift
    }

    pub fn is_holding_sprint_key(&self) -> bool {
        self.inputs().sprint
    }

    pub fn has_pressed_sprint_key(&self) -> bool {
        !self.old_inputs.sprint && self.inputs().sprint
    }

    pub fn has_released_sprint_key(&self) -> bool {
        self.old_inputs.sprint && !self.inputs().sprint
    }

    fn inputs(&self) -> PlayerInputs {
        unsafe { &*self.player }.inputs()
    }
}
