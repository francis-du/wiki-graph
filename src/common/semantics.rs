use whatlang::{detect, Lang};

/// Identify word language
pub fn identify(words: String) -> &'static str {
    match detect(words.as_str()) {
        None => "en",
        Some(lang) => match lang.lang() {
            Lang::Eng => "en",
            Lang::Cmn => "zh",
            Lang::Spa => "es",
            Lang::Ita => "it",
            Lang::Fra => "fr",
            Lang::Jpn => "ja",
            Lang::Deu => "de",
            Lang::Kor => "ko",
            Lang::Rus => "ru",
            Lang::Vie => "vi",
            Lang::Tha => "th",
            Lang::Ukr => "uk",
            Lang::Por => "pt",
            Lang::Ara => "ar",
            Lang::Hin => "hi",
            Lang::Tur => "tr",
            Lang::Ind => "id",
            _ => "en",
        },
    }
}
