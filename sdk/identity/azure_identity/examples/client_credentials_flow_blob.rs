// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{credentials::TokenCredential, date, new_http_client, Url};
use azure_identity::ClientSecretCredential;
use std::{
    env::{args, var},
    error::Error,
};
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id = var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret = var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = var("TENANT_ID").expect("Missing TENANT_ID environment variable.");

    let storage_account_name = args()
        .nth(1)
        .expect("please specify the storage account name as first command line parameter");
    let container_name = args()
        .nth(2)
        .expect("please specify the container name as second command line parameter");

    let http_client = new_http_client();

    let credential = ClientSecretCredential::new(
        http_client,
        Url::parse("https://login.microsoftonline.com").unwrap(),
        tenant_id,
        client_id,
        client_secret,
    );

    let token = credential
        .get_token(&[&format!(
            "https://{storage_account_name}.blob.core.windows.net/.default"
        )])
        .await?;

    println!("token received: {token:?}");
    println!("token secret: {}", token.token.secret());

    let dt = OffsetDateTime::now_utc();
    let time = date::to_rfc1123(&dt);
    println!("x-ms-date ==> {time}");

    let resp = reqwest::Client::new()
        .get(&format!(
            "https://{storage_account_name}.blob.core.windows.net/{container_name}?restype=container&comp=list"
        ))
        .header(
            "Authorization",
            format!("Bearer {}", token.token.secret()),
        )
        .header("x-ms-version", "2019-07-07")
        .header("x-ms-date", time)
        .send()
        .await?
        .text()
        .await?;

    println!("\n\nresp {resp:?}");

    Ok(())
}
