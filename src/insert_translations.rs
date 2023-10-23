use mysql::*;
use mysql::prelude::*;

use crate::read_lines;

pub fn insert_translations() -> Result<()> {
    let url = "mysql://root:example@localhost:3306/SS_sitedb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let lines = read_lines::read_lines("spanish_output.csv").unwrap();
    let mut data: Vec<Vec<String>> = Vec::new();
    for line in lines {
        data.push(read_lines::get_vector_from_csv_line(&line));
        //println!("{:?}", read_lines::get_vector_from_csv_line(&line));
    }

    conn.exec_batch(r"INSERT INTO ContentLayout_Localised (Title, RecordID, Locale) VALUES (:title, :record_id, :locale)", data.iter().map(|vec| params! {
        "title" => vec.get(0).unwrap(),
        "record_id" => vec.get(2).unwrap(),
        "locale" => "es_ES"
    }))?;

    conn.exec_batch(r"INSERT INTO ContentLayout_Localised_Live (Title, RecordID, Locale) VALUES (:title, :record_id, :locale)", data.iter().map(|vec| params! {
        "title" => vec.get(0).unwrap(),
        "record_id" => vec.get(2).unwrap(),
        "locale" => "es_ES"
    }))?;

    conn.exec_batch(r"INSERT INTO ContentLayout_Localised_Versions (Title, RecordID, Version, Locale) VALUES (:title, :record_id, :version, :locale)", data.iter().map(|vec| params! {
        "title" => vec.get(0).unwrap(),
        "record_id" => vec.get(2).unwrap(),
        "version" => 1,
        "locale" => "es_ES"
    }))?;

    conn.exec_batch(r"INSERT INTO ContentLayoutHtml_Localised (Content, RecordID, Locale) VALUES (:content, :record_id, :locale)", data.iter().map(|vec| params! {
        "content" => vec.get(1).unwrap(),
        "record_id" => vec.get(2).unwrap(),
        "locale" => "es_ES"
    }))?;

    conn.exec_batch(r"INSERT INTO ContentLayoutHtml_Localised_Live (Content, RecordID, Locale) VALUES (:content, :record_id, :locale)", data.iter().map(|vec| params! {
        "content" => vec.get(1).unwrap(),
        "record_id" => vec.get(2).unwrap(),
        "locale" => "es_ES"
    }))?;

    conn.exec_batch(r"INSERT INTO ContentLayoutHtml_Localised_Versions (Content, RecordID, Version, Locale) VALUES (:content, :record_id, :version, :locale)", data.iter().map(|vec| params! {
        "content" => vec.get(1).unwrap(),
        "record_id" => vec.get(2).unwrap(),
        "version" => 1,
        "locale" => "es_ES"
    }))?;

    Ok(())
}

