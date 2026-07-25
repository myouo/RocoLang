use serde::{Deserialize, Serialize};

use super::RocoOptionalI64;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReclaimGoodsItem {
    pub item_id: i64,
    pub count: i64,
    pub unit_price: i64,
    pub previous_price: i64,
    pub status: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReclaimGoodsEgg {
    pub spirit_id: i64,
    pub catch_time: i64,
    pub host_uin: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReclaimGoodsPrice {
    pub item_id: i64,
    pub unit_price: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReclaimGoodsListResult {
    pub balance: RocoOptionalI64,
    pub tips: String,
    pub safe_code_open: bool,
    pub safe_code_required: bool,
    pub items: Vec<ReclaimGoodsItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReclaimGoodsEggListResult {
    pub balance: RocoOptionalI64,
    pub safe_code_open: bool,
    pub safe_code_required: bool,
    pub eggs: Vec<ReclaimGoodsEgg>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReclaimGoodsPriceResult {
    pub safe_code_open: bool,
    pub safe_code_required: bool,
    pub items: Vec<ReclaimGoodsPrice>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReclaimGoodsSellResult {
    pub balance: RocoOptionalI64,
    pub tips: String,
}
