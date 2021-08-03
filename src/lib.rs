use chrono::NaiveDate;
use log::trace;
use scraper::{Html, Selector};
use tabled::Tabled;
use tokio::runtime::Runtime;

pub const ITU_WEB: &str = "https://en.itu.dk/about-itu/vacancies";

#[derive(Debug, Clone, Hash, Tabled)]
pub struct Vacancy {
    pub deadline: NaiveDate,
    pub title: String,
    pub link: String,
}

#[derive(Debug, Clone)]
pub enum Error {
    Unimplemented,
    ParseError(String),
}

async fn get_html() -> Result<String, Error> {
    let body = reqwest::get(ITU_WEB).await.unwrap().text().await.unwrap();

    let dom = Html::parse_fragment(&body);
    let row = Selector::parse("#phmain_0_PnlMainContent > div.col > div > table").unwrap();
    dom.select(&row)
        .next()
        .map(|e| e.html())
        .ok_or_else(|| Error::ParseError("Failed to find table".to_owned()))
}

fn parse_html(html: impl AsRef<str>) -> Result<Vec<Vacancy>, Error> {
    let dom = Html::parse_fragment(html.as_ref());
    let row = Selector::parse("tr").unwrap();
    let cell = Selector::parse("td").unwrap();
    let pos_sel = Selector::parse("a").unwrap();

    let mut vacancies: Vec<Vacancy> = Vec::new();

    for tr in dom.select(&row) {
        trace!("tr: {:?}", tr.inner_html());
        let mut m = tr.select(&cell);
        let position = m.next();
        let deadline = m.next();
        if let (Some(pos), Some(dead)) = (position, deadline) {
            let pos = pos.select(&pos_sel).next().unwrap();
            let deadline = dead.text().collect::<String>();
            vacancies.push(Vacancy {
                title: pos.text().collect(),
                link: pos.value().attr("href").unwrap().to_string(),
                deadline: NaiveDate::parse_from_str(deadline.as_str(), "%d %B %Y")
                    .expect("Failed to parse date"),
            })
        }
    }

    Ok(vacancies)
}

pub fn get_vacancies() -> Result<Vec<Vacancy>, Error> {
    let html = Runtime::new()
        .unwrap()
        .block_on(async { get_html().await })?;
    parse_html(html)
}

#[cfg(test)]
mod tests {}
