#[allow(clippy::struct_excessive_bools)] // Does not apply
#[derive(Clone, Copy, Debug)]
pub(crate) struct FixerState {
    pub(crate) medium: bool,
    pub(crate) reddit: bool,
    pub(crate) tiktok: bool,
    pub(crate) twitter: bool,
    pub(crate) youtube: bool,
    pub(crate) threads: bool,
}

impl Default for FixerState {
    fn default() -> Self {
        Self {
            medium: true,
            reddit: true,
            tiktok: true,
            twitter: true,
            youtube: true,
            threads: true,
        }
    }
}

impl FixerState {
    pub(crate) fn medium(&mut self, value: bool) {
        self.medium = value;
    }

    pub(crate) fn tiktok(&mut self, value: bool) {
        self.tiktok = value;
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

    pub(crate) fn threads(&mut self, value: bool) {
        self.threads = value;
    }
}
