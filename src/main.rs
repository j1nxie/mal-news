use std::error::Error;
use tabled::{
    MaxWidth,
    Modify,
    Row,
    Tabled,
    Table,
};
use rss::Channel;

#[derive(Tabled)]
struct News {
    #[header("Date")]
    date: String,
    #[header("Title")]
    title: String,
    #[header("Link")]
    link: String,
}

impl News {
    fn new() -> Self {
        News {
            date: String::new(),
            title: String::new(),
            link: String::new(),
        }
    }
}

async fn fetch_rss(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

async fn parse_rss(rss: Channel) -> Result<Vec<News>, Box<dyn Error>> {
    let mut news_vector: Vec<News> = vec![];
    for item in rss.items() {
        let mut news = News::new();
        if let Some(date) = item.pub_date() {
            news.date = truncate(date, 16).to_string();
        }
        if let Some(title) = item.title() {
            news.title = title.to_string();
        }
        if let Some(link) = item.link() {
            news.link = link.strip_suffix("?_location=rss").unwrap().to_string();
        }
        news_vector.push(news);
    }
    Ok(news_vector)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let channel = fetch_rss("https://myanimelist.net/rss/news.xml").await?;
    let news = parse_rss(channel).await?;
    let table = Table::new(news)
        .with(Modify::new(Row(1..))
            .with(MaxWidth::wrapping(60)))
        .to_string();
    println!("{}", table);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*; 
    
    #[tokio::test]
    async fn test_fetch_rss() {
        if let Ok(rss_channel) = fetch_rss("http://lorem-rss.herokuapp.com/feed").await {
            assert_eq!(rss_channel.title(),
            "Lorem ipsum feed for an interval of 1 minutes with 10 item(s)");
            assert_eq!(rss_channel.link(),
            "http://example.com/");
        }
    }
}
