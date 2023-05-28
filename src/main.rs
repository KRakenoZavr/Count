use ::cosmos_sdk_proto;
use ::tokio;
use bank::balance::get_account_balance;
use bucket::list::get_list_bucket;

mod bank;
mod bucket;
mod pagination;

#[tokio::main]
async fn main() {
    let msg = cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceRequest {
        address: "0x1060D988E6b1235d1Bd0A01E6378A934b6aC763e".to_string(),
        denom: "BNB".to_string(),
    };

    let _res = get_account_balance(msg.address, msg.denom).await;
    let _res = get_list_bucket().await;
}

// bucket list
// storage list
