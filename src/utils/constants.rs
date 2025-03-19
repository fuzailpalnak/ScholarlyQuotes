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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum APILimit {
    TotalRequest,
    RefillRate,
    RefillInterval,
}

impl APILimit {
    pub fn as_usize(&self) -> usize {
        match self {
            APILimit::TotalRequest => 4,
            APILimit::RefillRate => 4,
            APILimit::RefillInterval => 86400000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestLimit {
    RPS,
    BurstSize,
}

impl RequestLimit {
    pub fn as_u64(&self) -> u64 {
        match self {
            RequestLimit::RPS => 2,
            RequestLimit::BurstSize => 3,
        }
    }
}
