use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ArticleDataHeadline {
    main: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArticleDataResponse {
    web_url: String,
    source: String,
    pub_date: String,
    uri: String,
    headline: ArticleDataHeadline,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData<T> {
    docs: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    response: ResponseData<ArticleDataResponse>,
}

fn print_articles(articles: Vec<&ArticleDataResponse>) {
    for article in articles {
        println!("Web URL: {}", article.web_url);
        println!("Source: {}", article.source);
        println!("Published on: {}", article.pub_date);
        println!("URL: {}", article.uri);
        println!("Headline: {}", article.headline.main);
        println!("------------------------------");
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let api_key = &args[2];
    let url = format!(
        "https://api.nytimes.com/svc/search/v2/articlesearch.json?q={query}&api-key={key}",
        query = search_query,
        key = api_key
    );
    fetch_articles(url).await;
}

async fn fetch_articles(url: String) {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_articles(parsed.response.docs.iter().collect()),
                Err(_) => println!("The response didn't match the expected format."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("You need a valid api key to make this request");
        }
        other => {
            panic!("Sorry, an error occured {:?}", other);
        }
    };
}
