use investec_rs::Client;

fn main() {
    let client = Client::new();
    let res = client.get_account_transactions(
        "8183271355710189127359663".to_string(),
        "2022-06-03".to_string(),
        "2022-07-03".to_string(),
        "".to_string(),
    );
    dbg!(res);
}
