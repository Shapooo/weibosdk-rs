use simple_logger;
use std::io::{self, Write};
use weibosdk_rs::{ApiClient, session::Session};

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let session_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("session.json");
    let client = weibosdk_rs::http_client::Client::new().unwrap();
    let weibo_api = ApiClient::new(client, Default::default());

    if let Ok(session) = Session::load(&session_file) {
        println!("Loaded session from {session_file:?}");
        weibo_api.login_with_session(session).await.unwrap();
    } else {
        println!("No session file found. Starting new login.");

        print!("Please enter your phone number: ");
        io::stdout().flush().unwrap();
        let mut phone_number = String::new();
        io::stdin().read_line(&mut phone_number).unwrap();

        weibo_api.get_sms_code(phone_number).await.unwrap();

        print!("Please enter the SMS code: ");
        io::stdout().flush().unwrap();
        let mut sms_code = String::new();
        io::stdin().read_line(&mut sms_code).unwrap();

        weibo_api.login(sms_code.trim()).await.unwrap();
    };

    let session = weibo_api.session().unwrap();
    session.save(&session_file).unwrap();
    println!("Session saved to {session_file:?}");
    let _favorites = weibo_api.favorites(1).await.unwrap();
}
