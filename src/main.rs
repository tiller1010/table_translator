//use std::io::{BufReader, BufRead, Error};
//use std::fs::File;
//use libretranslate::{Language, translate};
//use tokio;

mod get_source;

//async fn translate_text(input: &str) -> Future<Output = String> { 
//    let translated_text = translate(Language::English, Language::Spanish, &input, None).await;
//    translated_text.unwrap().target.as_pretty()
//}

//#[tokio::main]
//async fn main() {
fn main() {
    get_source::get_source();
//    // Put csv content into vector
//    let lines = read_lines("data.csv").unwrap();
//    let mut data: Vec<Vec<String>> = Vec::new();
//    for line in lines {
//        data.push(get_vector_from_csv_line(&line));
//    }
//    println!("{:?}", data);
//
//    // Translate vector
//    let mut translated_data: Vec<Vec<String>> = Vec::new();
//    for row in data.iter_mut() {
//        let mut translated_row: Vec<String> = Vec::new();
//        for value in row.iter_mut() {
//            let translated_value = translate_text(&value).await;
//            translated_row.push(translated_value);
//        }
//        *row = translated_row;
//        translated_data.push(row.to_vec());
//    }
//    println!("{:?}", translated_data);
//}
//
//fn read_lines(filename: &str) -> Result<Vec<String>, Error> {
//    let file = File::open(filename)?;
//    let reader = BufReader::new(file);
//    reader.lines().collect()
//}
//
//fn get_vector_from_csv_line(line: &str) -> Vec<String> {
//    let mut vector: Vec<String> = Vec::new();
//    for value in line.split(",") {
//        vector.push(value.parse::<String>().unwrap());
//    }
//    vector
}

