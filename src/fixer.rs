#[allow(clippy::struct_excessive_bools)] // Does not apply
#[derive(Clone, Copy, Debug)]
pub(crate) struct FixerState {
    pub(crate) instagram: bool,
    pub(crate) medium: bool,
    pub(crate) twitter: bool,
    pub(crate) youtube: bool,
}

impl Default for FixerState {
    fn default() -> Self {
        Self {
            instagram: true,
            medium: true,
            twitter: true,
            youtube: true,
        }
    }
}

impl FixerState {
    pub(crate) fn instagram(&mut self, value: bool) -> &mut FixerState {
        self.instagram = value;
        self
    }

    pub(crate) fn medium(&mut self, value: bool) -> &mut FixerState {
        self.medium = value;
        self
    }

    pub(crate) fn twitter(&mut self, value: bool) -> &mut FixerState {
        self.twitter = value;
        self
    }

    pub(crate) fn youtube(&mut self, value: bool) -> &mut FixerState {
        self.youtube = value;
        self
    }
}
