//use tokio;
use clap::{arg, Command};

mod get_source;
mod read_lines;
mod translate_text;
mod insert_translations;

fn cli() -> Command {
    Command::new("translate_table")
        .subcommand(Command::new("generate_source"))
        .subcommand(Command::new("translate")
                    .arg(arg!(<locale> "Locale to translate to").default_value("es_ES").required(false))
                   )
        .subcommand(Command::new("insert_translations")
                    .arg(arg!(<locale> "Locale to insert into database").default_value("es_ES").required(false))
                   )
        .subcommand(Command::new("read_english_source"))
        .subcommand(Command::new("all")
                    .arg(arg!(<locale> "Locale to translate to").default_value("es_ES").required(false))
                   )
}

fn read_english_source() -> Vec<Vec<String>> {
    let lines = read_lines::read_lines("english_original.csv").unwrap();
    let mut data: Vec<Vec<String>> = Vec::new();
    for line in lines {
        data.push(read_lines::get_vector_from_csv_line(&line));
    }
    data
}

fn translate(language: &str) {
    // Put csv content into vector
    let mut data = read_english_source();

    // Translate vector
    let mut translated_data: Vec<Vec<String>> = Vec::new();
    for row in data.iter_mut() {
        let mut translated_row: Vec<String> = Vec::new();
        for value in row.iter_mut() {
            let translated_value = translate_text::translate_text(&value, &language);
            translated_row.push(translated_value);
        }
        *row = translated_row;
        translated_data.push(row.to_vec());
    }

    // output to csv
    let mut wtr = csv::Writer::from_path(format!("{}_output.csv", language)).unwrap();
    for vec in translated_data {
        if vec.get(0).is_none() || vec.get(1).is_none() || vec.get(2).is_none() {
            continue;
        }
        wtr.write_record(&[vec.get(0).unwrap(), vec.get(1).unwrap(), vec.get(2).unwrap()]).unwrap();
    }
}

//#[tokio::main]
//async fn main() {
fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("generate_source", sub_matches)) => {
            println!("{:?}", sub_matches);
            println!("Generating source...");
            // Generate english_original.csv from MySQL
            get_source::get_source();
        }
        Some(("translate", sub_matches)) => {
            println!("Translating...");
            let locale = sub_matches.get_one::<String>("locale").unwrap();
            translate(&locale);
        }
        Some(("insert_translations", sub_matches)) => {
            println!("Inserting translations...");
            let locale = sub_matches.get_one::<String>("locale").unwrap();
            let _ = insert_translations::insert_translations(locale);
        }
        Some(("read_english_source", sub_matches)) => {
            println!("{:?}", sub_matches);
            println!("Reading english source...");
            let english_source_vector = read_english_source();
            println!("{:?}", english_source_vector);
        }
        Some(("all", sub_matches)) => {
            println!("Generating source...");
            // Generate english_original.csv from MySQL
            get_source::get_source();
            println!("Translating...");
            let locale = sub_matches.get_one::<String>("locale").unwrap();
            translate(&locale);
            println!("Inserting translations...");
            let _ = insert_translations::insert_translations(locale);
        }
        _ => {
            println!("No command specified");
        }
    };

}

