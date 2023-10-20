//use tokio;
use clap::Command;

mod get_source;
mod read_lines;
mod translate_text;

fn cli() -> Command {
    Command::new("translate_table")
        .subcommand(Command::new("generate_source"))
        .subcommand(Command::new("translate"))
        .subcommand(Command::new("all"))
}

fn translate() {
    // Put csv content into vector
    let lines = read_lines::read_lines("english_original.csv").unwrap();
    let mut data: Vec<Vec<String>> = Vec::new();
    for line in lines {
        data.push(read_lines::get_vector_from_csv_line(&line));
        println!("{:?}", read_lines::get_vector_from_csv_line(&line));
    }
//    println!("{:?}", data);

    // Translate vector
    let mut translated_data: Vec<Vec<String>> = Vec::new();
    for row in data.iter_mut() {
        let mut translated_row: Vec<String> = Vec::new();
        for value in row.iter_mut() {
            let translated_value = translate_text::translate_text(&value);
            translated_row.push(translated_value);
        }
        *row = translated_row;
        translated_data.push(row.to_vec());
    }
//    println!("{:?}", translated_data);

    // output to csv
    let mut wtr = csv::Writer::from_path("spanish_output.csv").unwrap();
    for vec in translated_data {
        if vec.get(0).is_none() || vec.get(1).is_none() {
            continue;
        }
        wtr.write_record(&[vec.get(0).unwrap(), vec.get(1).unwrap()]).unwrap();
    }
}

//#[tokio::main]
//async fn main() {
fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("generate_source", sub_matches)) => {
            println!("Generating source...");
            // Generate english_original.csv from MySQL
            get_source::get_source();
        }
        Some(("translate", sub_matches)) => {
            println!("Translating...");
            translate();
        }
        Some(("all", sub_matches)) => {
            println!("Generating source...");
            // Generate english_original.csv from MySQL
            get_source::get_source();
            println!("Translating...");
            translate();
        }
        _ => {
            println!("No command specified");
        }
    }

}

