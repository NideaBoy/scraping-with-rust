extern crate dotenv;

use reqwest::Client;
use dotenv::dotenv;
use std::{env, fs::File, io::Write};

async fn get_data() -> Vec<String> {
    let mut img: Vec<String> = vec![];
    let client = Client::new();
    let url =  env::var("WEB_SCRAPPER_URL").unwrap();
    let response = client.get(url).send().await.unwrap();
    let html_content = response.text().await.unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_image_selector = scraper::Selector::parse("picture").unwrap();
    let html_product = document.select(&html_image_selector);
    for product in html_product {
        let image = product.select(&scraper::Selector::parse("img").unwrap()).next().and_then(|a| a.value().attr("src")).map(str::to_owned);
        img.push(image.unwrap())
    }
    img
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut file = File::create("src/files/imagenes.json")?;
    let res = get_data().await;
    let formated_string =  format!(r#"{{"image": {:?}}}"#, res);
    file.write_all(formated_string.as_bytes()).unwrap();
    Ok(())
}
