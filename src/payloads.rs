use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Account {
    pub accountId: String,
    pub accountNumber: String,
    pub accountName: String,
    pub referenceName: String,
    pub productName: String,
}

#[derive(Deserialize, Debug)]
pub struct PayloadAccounts {
    pub data: HashMap<String, Vec<Account>>,
    pub links: HashMap<String, String>,
    pub meta: HashMap<String, i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct AccountBalance {
    pub accountId: String,
    pub currentBalance: f64,
    pub availableBalance: f64,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
pub struct PayloadAccountBalance {
    pub data: AccountBalance,
    pub links: HashMap<String, String>,
    pub meta: HashMap<String, i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Transaction {
    pub accountId: String,
    pub r#type: String,
    pub transactionType: String,
    pub status: String,
    pub description: String,
    pub cardNumber: String,
    pub postedOrder: i64,
    pub postingDate: String,
    pub valueDate: String,
    pub actionDate: String,
    pub transactionDate: String,
    pub amount: f64,
    pub runningBalance: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct PayloadTransactions {
    pub data: HashMap<String, Vec<Transaction>>,
    pub links: HashMap<String, String>,
    pub meta: HashMap<String, i32>,
}
