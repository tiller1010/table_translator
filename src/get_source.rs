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

    let selected_content: Vec<(_, _, _)> = conn.query_map("SELECT cl.Title, html.Content, cl.ID as RecordID FROM ContentLayout AS cl JOIN ContentLayoutHtml AS html ON cl.ID = html.ID", |(title, content, record_id)| -> (Option<String>, Option<String>, Option<String>) {
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

}

