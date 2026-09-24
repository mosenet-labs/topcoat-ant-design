/// Language for labels supplied by a component itself.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiLanguage {
    /// English is the default for component labels.
    #[default]
    English,
    /// Simplified Chinese labels.
    ChineseSimplified,
}

impl UiLanguage {
    pub(crate) const fn select(self, english: &'static str, chinese: &'static str) -> &'static str {
        match self {
            Self::English => english,
            Self::ChineseSimplified => chinese,
        }
    }
}
