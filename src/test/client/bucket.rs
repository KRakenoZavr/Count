#[cfg(test)]
mod bank_tests {
    use crate::client::RpcClient;
    use cosmos_sdk_proto::cosmos::bank::v1beta1::DenomUnit;

    #[tokio::test]
    async fn balances() -> Result<(), anyhow::Error> {
        let client =
            RpcClient::new("https://gnfd-testnet-fullnode-tendermint-us.bnbchain.org:443")?;

        let balances = client.bucket.list_bucket().await?;

        println!("{:#?}", balances);

        // assert!(!balances.balances.is_empty());
        Ok(())
    }
}
