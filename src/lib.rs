use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct Coin {
    amount: String,
    denom: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct QueryBalance {
    balance: Coin,
}

pub async fn get_account_balance(address: String, denom: String) -> QueryBalance {
    let url = format!("https://gnfd-testnet-fullnode-tendermint-us.bnbchain.org/cosmos/bank/v1beta1/balances/{address}/by_denom?denom={denom}");

    let client = Client::new();

    let response = client.get(url).send().await.unwrap();

    let jsoned = response.json::<QueryBalance>().await.unwrap();

    println!("{:#?}", jsoned);

    jsoned
}
