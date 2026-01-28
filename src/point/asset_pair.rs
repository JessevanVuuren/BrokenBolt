use serde_json::Value;

use crate::{error::error::AssetPairError, point::fetch::fetch_params, types::points::AssetPairs, urls::ASSET_PAIRS_URL, utils::nested_object};

pub async fn get_asset_pair(pair: &str) -> Result<AssetPairs, AssetPairError> {
    let params = vec![("pair", pair.to_owned())];
    let mut data: Value = fetch_params(ASSET_PAIRS_URL, params).await?;
    let path = format!("/result/{}", pair.replace('/', "~1"));
    let assets: AssetPairs = nested_object(&path, &mut data)?;

    Ok(assets)
}
