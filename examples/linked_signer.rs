use std::error::Error;

use ethereal_rust_sdk::signing::{Eip712, SigningContext};
use ethereal_rust_sdk::with_signing_fields;
use ethereal_rust_sdk::{
    apis::{
        linked_signer_api::{
            LinkedSignerControllerLinkSignerParams, LinkedSignerControllerListBySubaccountIdParams,
        },
        subaccount_api::SubaccountControllerListByAccountParams,
    },
    models::{LinkSignerDto, LinkSignerDtoData},
    signable_messages::LinkSigner,
};
use ethers::signers::{LocalWallet, Signer};
use ethers::utils::hex;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (http_client, _) = common::create_test_clients().await?;
    // we first get the linked signers associated with our account
    let subaccount_ids = http_client
        .subaccount()
        .list_by_account(SubaccountControllerListByAccountParams {
            sender: http_client.address.to_string(),
            ..Default::default()
        })
        .await?;

    for subaccount in &subaccount_ids.data {
        println!("Subaccount ID: {}", subaccount.id);
    }
    let subaccount_id = subaccount_ids
        .data
        .first()
        .ok_or_else(|| anyhow::anyhow!("No subaccounts found"))?;
    let linked_signers = http_client
        .linked_signer()
        .list_by_subaccount_id(LinkedSignerControllerListBySubaccountIdParams {
            subaccount_id: subaccount_id.id.to_string(),
            ..Default::default()
        })
        .await?;
    for signer in &linked_signers.data {
        println!(" - Linked Signer Address: {}", signer.signer);
    }
    // adding a new linked signer is as so;

    let new_signer_pk = "43718d12917ba14f08f2d4a424f8406b0ba20adc4f3a15bcff0d593c57f55dc5";
    let wallet = new_signer_pk.parse::<LocalWallet>().unwrap();
    let new_signer = format!("{:?}", wallet.address());
    let ctx = SigningContext::new(&http_client.wallet, subaccount_id);
    let link_signer_msg = with_signing_fields!(
        eip_signing_fields,
        ctx,
        LinkSigner {
            signer: new_signer.parse()?,
        }
    );
    let owner_signature = link_signer_msg.sign(http_client.env, &http_client.wallet)?;
    let signer_signature = link_signer_msg.sign(http_client.env, &wallet)?;

    println!("Linking new signer...");

    let link_signer_dto_data = with_signing_fields!(
        dto_signing_fields,
        ctx,
        LinkSignerDtoData {
            subaccount_id: subaccount_id.id,
            signer: new_signer.to_string(),
        }
    );
    let params = LinkedSignerControllerLinkSignerParams {
        link_signer_dto: LinkSignerDto {
            data: link_signer_dto_data,
            signature: "0x".to_string() + &hex::encode(owner_signature.to_vec()),
            signer_signature: "0x".to_string() + &hex::encode(signer_signature.to_vec()),
        },
    };

    let result = http_client.linked_signer().link_signer(params).await?;
    println!("New linked signer added: {:?}", result);

    Ok(())
}
