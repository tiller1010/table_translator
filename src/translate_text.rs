use text_translator::*;

use crate::read_lines;

pub fn translate_text(input: &str) -> String {
    let mut yandex_api_key = String::new();

    let lines = read_lines::read_lines(".env").unwrap();
    for line in lines {
        if line.contains("YANDEX_API_KEY") {
            yandex_api_key = line.split("=").collect::<Vec<&str>>()[1].to_string();
            yandex_api_key = yandex_api_key.replace("\"", "");
            break;
        }
    }

    let translator: Yandex = Yandex::with_key(&yandex_api_key);
    let translated_text: String = match translator.translate(input.to_string(), InputLanguage::Automatic, Language::Spanish) {
        Ok(result) => result,
        Err(err) => panic!("API error, could not translate text : {:#?}", err)
    };
    println!("Translated text: {}", translated_text);

    translated_text
}

