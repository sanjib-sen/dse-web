use scraper::{Html, Selector};
struct Stock {
    name: String,
    trading_price: Option<f32>,
}

pub(crate) async fn get_data() -> Result<f32, reqwest::Error> {
    let stock_name = "ARAMIT".to_string();
    let mut stock = Stock {
        name: stock_name.clone(),
        trading_price: None,
    };
    let url = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        println!("No CLI URL provided, using default.");
        format!("https://dsebd.org/displayCompany.php?name={}", stock.name).into()
    };
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("#company > tbody > tr:nth-child(1) > td:nth-child(2)").unwrap();
    let element_of_trading_price = document.select(&selector).nth(0);
    let binding = element_of_trading_price.unwrap().text().collect::<Vec<_>>();
    let trading_price = binding.first().unwrap().to_string().parse::<f32>().unwrap();
    stock.trading_price = Some(trading_price);

    match stock.trading_price {
        Some(price) => println!("{price}"),
        None => println!("None"),
    }
    match stock.trading_price {
        Some(price) => return Ok(price),
        None => panic!("No data found"),
    }
}
