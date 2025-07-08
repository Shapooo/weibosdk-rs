use reqwest::{
    Client,
    header::{self, HeaderMap, HeaderValue},
};
use simple_logger;
use std::io::{self, Write};
use weibosdk_rs::{
    favorites::FavoritesAPI,
    login::{Login, SendCode},
    session::Session,
    weibo_api::WeiboAPI,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let session_file = "session.json";
    let favorites_file = "favorites.json";
    let headers = HeaderMap::from_iter([
        (
            header::USER_AGENT,
            HeaderValue::from_static("HONOR-PGT-AN10_9_WeiboIntlAndroid_6710"),
        ),
        (header::ACCEPT_ENCODING, HeaderValue::from_static("gzip")),
        (
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
        ),
        (header::HOST, HeaderValue::from_static("api.weibo.cn")),
        (header::CONNECTION, HeaderValue::from_static("Keep-Alive")),
    ]);
    let client = Client::builder().default_headers(headers).build().unwrap();

    let session = if let Ok(session) = Session::load(session_file) {
        println!("Loaded session from {}", session_file);
        let login_client = Login::new(client.clone());
        login_client.login_with_session(session).await?
    } else {
        println!("No session file found. Starting new login.");
        let send_code_client = SendCode::new(client.clone());

        print!("Please enter your phone number: ");
        io::stdout().flush()?;
        let mut phone_number = String::new();
        io::stdin().read_line(&mut phone_number)?;

        let waiting_login = send_code_client
            .get_send_code(phone_number.trim().to_string())
            .await?;

        print!("Please enter the SMS code: ");
        io::stdout().flush()?;
        let mut sms_code = String::new();
        io::stdin().read_line(&mut sms_code)?;

        let session = waiting_login.login(sms_code.trim()).await?;
        session
    };

    session.save(session_file)?;
    println!("Session saved to {}", session_file);
    let weibo_api = WeiboAPI::new(client, session);
    let favorites = weibo_api.favorites(1).await?;
    let favorites_json = favorites.json::<serde_json::Value>().await?;

    let favorites_content = serde_json::to_string_pretty(&favorites_json)?;
    std::fs::write(favorites_file, favorites_content)?;
    println!("Favorites saved to {}", favorites_file);

    Ok(())
}
