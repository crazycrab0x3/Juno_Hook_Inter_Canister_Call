use junobuild_macros::{
    assert_delete_asset, assert_delete_doc, assert_set_doc, assert_upload_asset, on_delete_asset,
    on_delete_doc, on_delete_many_assets, on_delete_many_docs, on_set_doc, on_set_many_docs,
    on_upload_asset,
};
use junobuild_satellite::{
    include_satellite, AssertDeleteAssetContext, AssertDeleteDocContext, AssertSetDocContext,
    AssertUploadAssetContext, OnDeleteAssetContext, OnDeleteDocContext, OnDeleteManyAssetsContext,
    OnDeleteManyDocsContext, OnSetDocContext, OnSetManyDocsContext, OnUploadAssetContext, set_doc_store, SetDoc
};
use junobuild_utils::{ decode_doc_data, encode_doc_data };
use candid::Principal;
use types::CurrencyData;
mod types;


#[on_set_doc]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    let encoded_data = context.data.data.after.data;
    let decoded_data: types::Note = decode_doc_data(&encoded_data)?;
    let get_rate = ic_cdk::call::<(), (Option<Vec<types::CurrencyData>>,)>(Principal::from_text("5zigg-xyaaa-aaaak-aktyq-cai").unwrap(), "getCurrencies", ()).await;
    
    let mut price_in_used_cent = "dss".to_string();

    match get_rate {
        Ok((Some(currency_data),)) => {
            if let Some(currency) = currency_data.iter().find(|&currency| currency.name == decoded_data.currency) {
                let rate = currency.value;
                price_in_used_cent = format!("{}",(decoded_data.price_cents as f64 * rate));
            } else {
                price_in_used_cent = "Invalid Currency".to_string()
            };
        },
        _ => {
            price_in_used_cent = "Can't detect price".to_string()
        }
    };

    let new_note = types::UpdateNote{
        name: decoded_data.name,
        price_cents: decoded_data.price_cents,
        currency: decoded_data.currency,
        price_usd_cents: price_in_used_cent
    };
    let new_encoded_data = encode_doc_data(&new_note)?;
    set_doc_store(
        context.caller,
        context.data.collection,
        context.data.key,
        SetDoc{
            data: new_encoded_data,
            description: Some("from crazycrab".to_string()),
            version: context.data.data.after.version
        }
    );
    Ok(())
}

#[on_set_many_docs]
async fn on_set_many_docs(_context: OnSetManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_doc]
async fn on_delete_doc(_context: OnDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_docs]
async fn on_delete_many_docs(_context: OnDeleteManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_upload_asset]
async fn on_upload_asset(_context: OnUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_asset]
async fn on_delete_asset(_context: OnDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_assets]
async fn on_delete_many_assets(_context: OnDeleteManyAssetsContext) -> Result<(), String> {
    Ok(())
}

#[assert_set_doc]
fn assert_set_doc(_context: AssertSetDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_doc]
fn assert_delete_doc(_context: AssertDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_upload_asset]
fn assert_upload_asset(_context: AssertUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_asset]
fn assert_delete_asset(_context: AssertDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

include_satellite!();
