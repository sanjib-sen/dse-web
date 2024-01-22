use scraper::{Html, Selector};

pub async fn get_stock(company_name: &str) -> Result<f32, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "https://dsebd.org/displayCompany.php";
    let body = client.get(url).query(&[("name",company_name)]).fetch_mode_no_cors().send().await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("#company > tbody > tr:nth-child(1) > td:nth-child(2)").unwrap();
    let element_of_trading_price = document.select(&selector).nth(0);
    let binding = element_of_trading_price.unwrap().text().collect::<Vec<_>>();
    let trading_price = binding.first().unwrap().to_string().parse::<f32>().unwrap();
    return Ok(trading_price);
}
