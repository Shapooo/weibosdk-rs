use reqwest::Client;

use super::login::LoginResponse;

//-------------------------------------------------------------
//---------------------- WeiboClient --------------------------
//-------------------------------------------------------------

#[derive(Debug)]
pub struct WeiboAPI {
    client: Client,
    login_response: LoginResponse,
}

impl WeiboAPI {
    pub fn new(client: Client, login_response: LoginResponse) -> Self {
        WeiboAPI {
            client,
            login_response,
        }
    }
}
