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

    // select cl.Title, html.Content from ContentLayout as cl join ContentLayoutHtml as html on cl.ID = html.ID into outfile '/var/lib/mysql-files/out-new.csv' fields terminated by "," enclosed by '"' lines terminated by "\n";

    let selected_content: Vec<(_, _)> = conn.query_map("SELECT cl.Title, html.Content FROM ContentLayout AS cl JOIN ContentLayoutHtml AS html ON cl.ID = html.ID", |(title, content)| -> (Option<String>, Option<String>) {
        (
            title,
            content,
        )
    }).unwrap();
    println!("{:?}", selected_content);

//    let test: Vec<(_, _)> = conn.query_map("SELECT cl.ID, html.ID as HTMLID FROM ContentLayout as cl join ContentLayoutHtml as html on cl.ID = html.ID", |(id, htmlid)| -> (i32, i32) {(
//        id,
//        htmlid,
//    )}).unwrap();
//    println!("{:?}", test);
}

