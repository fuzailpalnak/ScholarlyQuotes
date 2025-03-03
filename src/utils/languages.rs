#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    RomanUrdu,
    Arabic,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::RomanUrdu => "ur-Latn",
            Language::Arabic => "ar",
        }
    }

    pub const fn variants() -> &'static [Language] {
        &[Language::English, Language::RomanUrdu]
    }
}
