use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonInfo {
    pub result_code: i64,
    pub message: String,
    pub diamond: i64,
    pub vip: i64,
    pub magic: i64,
    pub count: i64,
    pub show: i64,
    pub pools: Vec<SummonPoolState>,
    pub config_pools: Vec<SummonPoolConfig>,
    pub exchange_groups: Vec<SummonExchangeGroup>,
    pub recycle_states: Vec<SummonRecycleState>,
    pub rewards: Vec<SummonRewardItem>,
    pub records: Vec<SummonRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonPoolState {
    pub pool_index: i64,
    pub version: i64,
    pub token_item_id: i64,
    pub token_count: i64,
    pub today_draw_count: i64,
    pub times: i64,
    pub show: i64,
    pub wish_index: i64,
    pub succeeded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonPoolConfig {
    pub pool_index: i64,
    pub version: i64,
    pub title: String,
    pub vip_limit: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub daily_max: i64,
    pub recommend: String,
    pub info: String,
    pub reward_text: String,
    pub rewards: Vec<SummonPoolReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonPoolReward {
    pub name: String,
    pub id: i64,
    pub item_type: i64,
    pub count: i64,
    pub probability_type: i64,
    pub add: i64,
    pub wishable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonRecycleState {
    pub version: i64,
    pub day_times: Vec<i64>,
    pub counts: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonExchangeGroup {
    pub kind: String,
    pub items: Vec<SummonExchangeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonExchangeItem {
    pub index: i64,
    pub reward: SummonRewardItem,
    pub cost: SummonRewardItem,
    pub need: i64,
    pub max: i64,
    pub day_max: i64,
    pub times: i64,
    pub day_times: i64,
    pub add: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummonRewardItem {
    pub id: i64,
    pub item_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonRecord {
    pub pool_version: i64,
    pub title: String,
    pub name: String,
    pub id: i64,
    pub item_type: i64,
    pub count: i64,
    pub year: i64,
    pub month: i64,
    pub day: i64,
}
