#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct PlayerInputs {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub shift: bool,
    pub sprint: bool,
}

impl PlayerInputs {
    pub(crate) fn refresh(
        &mut self,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        jump: bool,
        shift: bool,
        sprint: bool,
    ) -> PlayerInputs {
        let previous_inputs = *self;
        self.forward = forward;
        self.backward = backward;
        self.left = left;
        self.right = right;
        self.jump = jump;
        self.shift = shift;
        self.sprint = sprint;
        previous_inputs
    }
}
