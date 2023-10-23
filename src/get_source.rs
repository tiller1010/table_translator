use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Content {
    title: String,
    content: String,
    record_id: String,
}

pub fn get_source() {
    let url = "mysql://root:example@localhost:3306/SS_sitedb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Check if "ContentLayout_Localised" has records
    let has_localised_records: bool = conn.query_first("SELECT EXISTS(SELECT * FROM ContentLayout_Localised WHERE Locale = 'en_US')").unwrap().unwrap();

    let mut query = "SELECT cl.Title, html.Content, cl.ID as RecordID FROM ContentLayout AS cl JOIN ContentLayoutHtml AS html ON cl.ID = html.ID WHERE cl.Locale = 'en_US'".to_string();

    if has_localised_records {
        // Query localised tables
        query = "SELECT cl.Title, html.Content, cl.RecordID as RecordID FROM ContentLayout_Localised AS cl JOIN ContentLayoutHtml_Localised AS html ON cl.RecordID = html.RecordID WHERE cl.Locale = 'en_US'".to_string();
    }

    let selected_content: Vec<(_, _, _)> = conn.query_map(query, |(title, content, record_id)| -> (Option<String>, Option<String>, Option<String>) {
        (
            title,
            content,
            record_id,
        )
    }).unwrap();

    let mut wtr = csv::Writer::from_path("english_original.csv").unwrap();
    for (title, content, record_id) in selected_content {
        let mut title = title;
        let mut content = content;
        if title.is_none() {
            title = Some("".to_string());
        }

        if content.is_none() {
            content = Some("".to_string());
        } else {
            content = Some(content.unwrap().replace("\n", " "));
        }

        if record_id.is_none() {
            continue;
        }
        wtr.write_record(&[title.unwrap(), content.unwrap(), record_id.unwrap()]).unwrap();
    }

    wtr.flush().unwrap();

    // Replace all `""` with `"`
    let file = std::fs::read_to_string("english_original.csv").unwrap();
    let file = file.replace("\"\"", "\"");
    std::fs::write("english_original.csv", file).unwrap();

}

