//use tokio;

mod get_source;
mod read_lines;
mod translate_text;

//#[tokio::main]
//async fn main() {
fn main() {
    get_source::get_source();

    // Put csv content into vector
    let lines = read_lines::read_lines("data.csv").unwrap();
    let mut data: Vec<Vec<String>> = Vec::new();
    for line in lines {
        data.push(read_lines::get_vector_from_csv_line(&line));
    }
    println!("{:?}", data);

    // Translate vector
//    let mut translated_data: Vec<Vec<String>> = Vec::new();
//    for row in data.iter_mut() {
//        let mut translated_row: Vec<String> = Vec::new();
//        for value in row.iter_mut() {
//            let translated_value = translate_text::translate_text(&value).await;
//            translated_row.push(translated_value);
//        }
//        *row = translated_row;
//        translated_data.push(row.to_vec());
//    }
//    println!("{:?}", translated_data);
//}
//
}

