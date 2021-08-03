use rss::{ChannelBuilder, GuidBuilder, ItemBuilder};

fn main() {
    let mut channel = ChannelBuilder::default()
        .title("ITU Vacancies")
        .link("https://blog.mtib.dev")
        .description("RSS feed for ITU vacancies")
        .build()
        .unwrap();
    let mut items = Vec::new();

    for v in itu_openrss::get_vacancies().unwrap() {
        let i = ItemBuilder::default()
            .author(Some("ITU".to_owned()))
            .link(Some(v.link.clone()))
            .title(Some(format!("{} - {}", v.deadline, v.title)))
            .guid(Some(GuidBuilder::default().value(v.link).build().unwrap()))
            .build()
            .unwrap();
        items.push(i);
    }

    channel.set_items(items);
    println!("{}", channel.to_string())
}
