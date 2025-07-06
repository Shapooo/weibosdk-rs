use reqwest::Client;

use super::login::LoginResponse;

//-------------------------------------------------------------
//---------------------- WeiboClient --------------------------
//-------------------------------------------------------------

#[derive(Debug)]
pub struct WeiboClient {
    client: Client,
    login_response: LoginResponse,
}

impl WeiboClient {
    pub fn new(client: Client, login_response: LoginResponse) -> Self {
        WeiboClient {
            client,
            login_response,
        }
    }
}
