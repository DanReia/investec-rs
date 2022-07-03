use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, HttpRequest, Scope, TokenResponse, TokenUrl};

use oauth2::http::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use oauth2::http::method::Method;
use oauth2::url::Url;
use std::env;

mod payloads;
use payloads::{Account, PayloadAccountBalance, PayloadAccounts,AccountBalance,PayloadTransactions,Transaction};

mod endpoints;
use endpoints::{ACCOUNTS, TOKEN};

pub struct Client {
    token: oauth2::AccessToken,
}

impl Client {
    pub fn new() -> Client {
        let token = Client::get_access_token();
        dbg!(token.secret());
        Client { token }
    }

    pub fn get_access_token() -> oauth2::AccessToken {
        let client_id = env::var("INVESTEC_CLIENT_ID").unwrap();
        let secret = env::var("INVESTEC_SECRET").unwrap();
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(secret)),
            AuthUrl::new(TOKEN.to_string()).unwrap(),
            Some(TokenUrl::new(TOKEN.to_string()).unwrap()),
        );

        let token_result = client
            .exchange_client_credentials()
            .add_scope(Scope::new("accounts".to_string()))
            .request(http_client)
            .unwrap();

        let token = token_result.access_token().to_owned();
        token
    }

    pub fn get_accounts(self) -> Vec<Account> {
        let url = Url::parse(&ACCOUNTS.to_string()).unwrap();
        let method = Method::GET;
        let mut headers = HeaderMap::new();

        let header_authorization = format!("Bearer {}", self.token.secret());
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&header_authorization).unwrap(),
        );

        let body: Vec<u8> = Vec::new();
        let http_request = HttpRequest {
            url,
            method,
            headers,
            body,
        };
        let response = http_client(http_request).unwrap();
        let payload: PayloadAccounts = serde_json::from_slice(&response.body).unwrap();
        payload.data.get("accounts").unwrap().to_vec()
    }

    pub fn get_account_balance(self, account_id: String) -> AccountBalance{
        let acc_url = format!(
            "https://openapi.investec.com/za/pb/v1/accounts/{}/balance",
            account_id
        );
        let url = Url::parse(&acc_url.to_string()).unwrap();
        let method = Method::GET;
        let mut headers = HeaderMap::new();

        let header_authorization = format!("Bearer {}", self.token.secret());

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&header_authorization).unwrap(),
        );

        let body: Vec<u8> = Vec::new();

        let http_request = HttpRequest {
            url,
            method,
            headers,
            body,
        };
        let response = http_client(http_request).unwrap();
        let payload: PayloadAccountBalance = serde_json::from_slice(&response.body).unwrap();
        payload.data
    }

    pub fn get_account_transactions(self,account_id:String,from_date:String,to_date:String,transaction_type:String)->Vec<Transaction>{
        let acc_url = format!(
            "https://openapi.investec.com/za/pb/v1/accounts/{}/transactions?fromDate={}&toDate={}&transactionType={}",
            account_id,
            from_date,
            to_date,
            transaction_type
        );
        let url = Url::parse(&acc_url.to_string()).unwrap();
        let method = Method::GET;
        let mut headers = HeaderMap::new();

        let header_authorization = format!("Bearer {}", self.token.secret());

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&header_authorization).unwrap(),
        );

        let body: Vec<u8> = Vec::new();

        let http_request = HttpRequest {
            url,
            method,
            headers,
            body,
        };
        let response = http_client(http_request).unwrap();
        let payload: PayloadTransactions = serde_json::from_slice(&response.body).unwrap();
        dbg!(&payload);
        payload.data.get("transactions").unwrap().to_vec()
    }
}
