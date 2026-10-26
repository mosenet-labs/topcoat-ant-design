use topcoat::{
    context::Cx,
    l10n::{Locale, Translation, locale, try_locale},
};

topcoat::l10n::fluent::messages!(pub(crate) "./l10n/{locale}.ftl");

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
    /// Read the host's Topcoat locale, falling back to English.
    /// Explicit component `language` props continue to override this choice.
    pub fn current(cx: &Cx) -> Self {
        if try_locale(cx).is_some_and(|locale| locale.id().language().as_str() == "zh") {
            Self::ChineseSimplified
        } else {
            Self::English
        }
    }

    /// Locale used for Fluent messages and number/date formatting.
    pub fn locale(self) -> Locale {
        match self {
            Self::English => locale!("en"),
            Self::ChineseSimplified => locale!("zh"),
        }
    }

    pub(crate) fn resolve(self, translation: Translation<'_>) -> String {
        translation.resolve_in(&self.locale()).into_owned()
    }

    pub(crate) const fn select(self, english: &'static str, chinese: &'static str) -> &'static str {
        match self {
            Self::English => english,
            Self::ChineseSimplified => chinese,
        }
    }
}
