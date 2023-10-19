use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Content {
    title: String,
    content: String,
}

pub fn get_source() {
    let url = "mysql://root:example@localhost:3306/SS_sitedb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let selected_content: Vec<(_, _)> = conn.query_map("SELECT cl.Title, html.Content FROM ContentLayout AS cl JOIN ContentLayoutHtml AS html ON cl.ID = html.ID", |(title, content)| -> (Option<String>, Option<String>) {
        (
            title,
            content,
        )
    }).unwrap();
    println!("{:?}", selected_content);

    let mut wtr = csv::Writer::from_path("english_original.csv").unwrap();
    for (title, content) in selected_content {
        if title.is_none() || content.is_none() {
            continue;
        }
        wtr.write_record(&[title.unwrap(), content.unwrap()]).unwrap();
    }

}

