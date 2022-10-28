use std::error::Error;

use serde::Deserialize;
use colour::{blue, yellow};

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String
}


fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        blue!("> {}\n", a.title);
        yellow!("> {}\n", a.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=br&apiKey=9b95c40e64f24acea565aa175b6efdce&pageSize=4";
    let articles = get_articles(url)?;
    render_articles(&articles);


    Ok(())

}

