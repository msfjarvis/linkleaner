#[allow(clippy::struct_excessive_bools)] // Does not apply
#[derive(Clone, Copy, Debug)]
pub(crate) struct FixerState {
    pub(crate) instagram: bool,
    pub(crate) medium: bool,
    pub(crate) reddit: bool,
    pub(crate) twitter: bool,
    pub(crate) youtube: bool,
}

impl Default for FixerState {
    fn default() -> Self {
        Self {
            instagram: true,
            medium: true,
            reddit: true,
            twitter: true,
            youtube: true,
        }
    }
}

impl FixerState {
    pub(crate) fn instagram(&mut self, value: bool) {
        self.instagram = value;
    }

    pub(crate) fn medium(&mut self, value: bool) {
        self.medium = value;
    }

    pub(crate) fn twitter(&mut self, value: bool) {
        self.twitter = value;
    }

    pub(crate) fn youtube(&mut self, value: bool) {
        self.youtube = value;
    }

    pub(crate) fn reddit(&mut self, value: bool) {
        self.reddit = value;
    }
}
