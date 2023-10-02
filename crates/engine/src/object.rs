use serde::Deserialize;

use crate::prelude::AssetHandle;

/// Object holds a handle to both a [`Model`] and a [`Material`]
/// These handles will be used to get the actual asset from the [`AssetRegistry`]
#[derive(Debug, Deserialize)]
pub struct Object {
    model: AssetHandle,
    material: AssetHandle,
}
