use cloudflare_lib::accounts::*;
use cloudflare_lib::*;
use std::collections::HashMap;

// RUST_LOG=error,cloudflare_lib=trace,accounts=trace cargo test --test accounts --features with-accounts
#[tokio::test]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = ClientBuilder::new(ApiAuth::default()).build().unwrap();

    let account_id = ::std::env::var("CLOUDFLARE_ACCOUNT_ID").unwrap_or_default();

    test_accounts(&client, account_id).await;

    tracing::info!("all done");
}

async fn test_accounts(client: &Client, account_id: String) {
    let list = client.accounts_list(None).await.unwrap();
    tracing::info!("accounts: {:#?}", list);
}
