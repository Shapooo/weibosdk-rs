use simple_logger;
use std::io::{self, Write};
use weibosdk_rs::{favorites::FavoritesAPI, session::Session, weibo_api::WeiboAPIImpl};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let session_file = "session.json";
    let client = weibosdk_rs::client::new_client_with_headers().unwrap();
    let mut weibo_api = WeiboAPIImpl::new(client);

    if let Ok(session) = Session::load(session_file) {
        println!("Loaded session from {}", session_file);
        weibo_api.login_with_session(session).await?;
    } else {
        println!("No session file found. Starting new login.");
        // let send_code_client = SendCode::new(client.clone());

        print!("Please enter your phone number: ");
        io::stdout().flush()?;
        let mut phone_number = String::new();
        io::stdin().read_line(&mut phone_number)?;

        weibo_api.get_sms_code(phone_number).await?;

        print!("Please enter the SMS code: ");
        io::stdout().flush()?;
        let mut sms_code = String::new();
        io::stdin().read_line(&mut sms_code)?;

        weibo_api.login(sms_code.trim()).await?;
    };

    let session = weibo_api.session()?;
    session.save(session_file)?;
    println!("Session saved to {}", session_file);
    let _favorites = weibo_api.favorites(1).await?;

    Ok(())
}
