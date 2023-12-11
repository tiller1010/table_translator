use mysql::*;
use mysql::prelude::*;

use crate::read_lines;

pub fn insert_translations(locale: &str) -> Result<()> {
    let url = "mysql://root:example@localhost:3306/SS_sitedb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let filename = format!("{}_output.csv", &locale);
    let lines = read_lines::read_lines(&filename).unwrap();
    let mut data: Vec<Vec<String>> = Vec::new();
    for line in lines {
        data.push(read_lines::get_vector_from_csv_line(&line));
        //println!("{:?}", read_lines::get_vector_from_csv_line(&line));
    }

    let base_tables = [
        "ContentLayout_Localised",
        "ContentLayout_Localised_Live",
    ];

    for table in base_tables {
        let query = format!("INSERT INTO {} (Title, RecordID, Locale) VALUES (:title, :record_id, :locale)", table);
        conn.exec_batch(&query, data.iter().map(|vec| params! {
            "title" => vec.get(0).unwrap(),
            "record_id" => vec.get(2).unwrap(),
            "locale" => &locale
        }))?;
    }

    conn.exec_batch(r"INSERT INTO ContentLayout_Localised_Versions (Title, RecordID, Version, Locale) VALUES (:title, :record_id, :version, :locale)", data.iter().map(|vec| params! {
        "title" => vec.get(0).unwrap(),
        "record_id" => vec.get(2).unwrap(),
        "version" => 1,
        "locale" => &locale
    }))?;

    let html_tables = [
        "ContentLayoutHtml_Localised",
        "ContentLayoutHtml_Localised_Live",
    ];

    for table in html_tables {
        let query = format!("INSERT INTO {} (Content, RecordID, Locale) VALUES (:content, :record_id, :locale)", table);
        conn.exec_batch(&query, data.iter().map(|vec| params! {
            "content" => vec.get(1).unwrap(),
            "record_id" => vec.get(2).unwrap(),
            "locale" => &locale
        }))?;
    }

    conn.exec_batch(r"INSERT INTO ContentLayoutHtml_Localised_Versions (Content, RecordID, Version, Locale) VALUES (:content, :record_id, :version, :locale)", data.iter().map(|vec| params! {
        "content" => vec.get(1).unwrap(),
        "record_id" => vec.get(2).unwrap(),
        "version" => 1,
        "locale" => &locale
    }))?;

    Ok(())
}

