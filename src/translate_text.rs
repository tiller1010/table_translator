use text_translator::*;

use crate::read_lines;

pub fn translate_text(input: &str) -> String {

    // Remove surrounding quotes if they exist
    let mut input = input.to_string();
    if input.starts_with("\"") && input.ends_with("\"") {
        input = input.chars().skip(1).collect::<String>();
        input = input.chars().rev().skip(1).collect::<String>();
    }

    if input.len() == 0 {
        return String::new();
    }

    let mut yandex_api_key = String::new();

    let lines = read_lines::read_lines(".env").unwrap();
    for line in lines {
        if line.contains("YANDEX_API_KEY") {
            yandex_api_key = line.split("=").collect::<Vec<&str>>()[1].to_string();
            yandex_api_key = yandex_api_key.replace("\"", "");
            break;
        }
    }

    // If input string is html, break it up by tags and translate the text in between
    // then reassemble the html string
    if input.contains("<") {
        let mut translated_html = String::new();
        let html_tags = input.split("<").collect::<Vec<&str>>();
        for tag in html_tags {
            if tag.contains(">") {
                let mut html_tag = tag.split(">").collect::<Vec<&str>>();
                if html_tag.len() > 1 {
                    let translated_text = translate_text(html_tag[1]);
                    html_tag[1] = &translated_text;
                    translated_html.push_str("<");
                    translated_html.push_str(&html_tag.join(">"));
                } else {
                    translated_html.push_str(&html_tag[0]);
                }
            } else {
                translated_html.push_str(tag);
            }
        }
        return translated_html;
    }

    let translator: Yandex = Yandex::with_key(&yandex_api_key);
    let translated_text: String = match translator.translate(input.to_string(), InputLanguage::Automatic, Language::Spanish) {
        Ok(result) => result,
        Err(err) => panic!("API error, could not translate text : {:#?}", err)
    };
    println!("Translated text: {}", translated_text);

    translated_text
}

