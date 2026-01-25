use std::ops::Sub;

use ethereal_rust_sdk::{apis::{linked_signer_api::{LinkedSignerControllerLinkSignerParams, LinkedSignerControllerListBySubaccountIdParams}, subaccount_api::SubaccountControllerListByAccountParams}, models::{LinkSignerDto, LinkSignerDtoData}, signing::{get_nonce, get_now}};

mod common;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (http_client, ws_client) = common::create_test_clients().await?;
    // we first get the linked signers associated with our account
    let subaccount_ids = http_client
        .subaccount()
        .list_by_account(
                SubaccountControllerListByAccountParams {
                    sender: http_client.address.to_string(),
                ..Default::default()
            }
        )
        .await?;

    for subaccount in &subaccount_ids.data {
        println!("Subaccount ID: {}", subaccount.id);
    }
    let subaccount_id = subaccount_ids.data.get(0).ok_or_else(|| anyhow::anyhow!("No subaccounts found"))?;
    let linked_signers = http_client
        .linked_signer()
        .list_by_subaccount_id(
            LinkedSignerControllerListBySubaccountIdParams {
                subaccount_id: subaccount_id.id.to_string(),
                ..Default::default()
            }
        )
        .await?;
    for signer in &linked_signers.data {
        println!(" - Linked Signer Address: {}", signer.signer);
    }
    // adding a new linked signer is as so;

    let new_signer = "0x1234567890abcdef1234567890abcdef12345678";
    let nonce = get_nonce();
    let now = get_now();
    let link_signer_dto = LinkSignerDtoData {
        subaccount_id: subaccount_id.id,
        sender: http_client.address.clone(),
        signer: new_signer.to_string(),
        nonce: nonce.to_string(),
        signed_at: now as i32,
        subaccount: subaccount_id.name.clone(),
    };
    let signature = link_signer_dto.sign(&http_client.env, &http_client.wallet)?;
    println!("Linking new signer...");
    let new_params = LinkedSignerControllerLinkSignerParams{
        link_signer_dto: LinkSignerDto {
            subaccount_id: subaccount_id.id.to_string(),
            signer: new_signer.to_string(),
        },
    };

    let linked_signer = http_client
        .linked_signer()
        .link_signer(new_params)
        .await?;

    Ok(())
}
