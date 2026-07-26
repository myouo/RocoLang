use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, SummonRewardItem, id, item_type, count);
    register_getters!(
        engine,
        SummonPoolState,
        pool_index,
        version,
        token_item_id,
        token_count,
        today_draw_count,
        times,
        show,
        wish_index,
        succeeded,
    );
    register_getters!(
        engine,
        SummonPoolConfig,
        pool_index,
        version,
        title,
        vip_limit,
        start_time,
        end_time,
        daily_max,
        recommend,
        info,
        reward_text,
    );
    engine.register_get("rewards", |value: &mut SummonPoolConfig| {
        to_array(&value.rewards)
    });
    register_getters!(
        engine,
        SummonPoolReward,
        name,
        id,
        item_type,
        count,
        probability_type,
        add,
        wishable,
    );
    register_getters!(engine, SummonRecycleState, version);
    engine.register_get("day_times", |value: &mut SummonRecycleState| {
        to_array(&value.day_times)
    });
    engine.register_get("counts", |value: &mut SummonRecycleState| {
        to_array(&value.counts)
    });
    register_getters!(
        engine,
        SummonExchangeItem,
        index,
        reward,
        cost,
        need,
        max,
        day_max,
        times,
        day_times,
        add,
    );
    register_getters!(engine, SummonExchangeGroup, kind);
    engine.register_get("items", |value: &mut SummonExchangeGroup| {
        to_array(&value.items)
    });
    register_getters!(
        engine,
        SummonRecord,
        pool_version,
        title,
        name,
        id,
        item_type,
        count,
        year,
        month,
        day,
    );
    register_getters!(
        engine,
        SummonInfo,
        result_code,
        message,
        diamond,
        vip,
        magic,
        count,
        show,
    );
    engine.register_get("pools", |value: &mut SummonInfo| to_array(&value.pools));
    engine.register_get("config_pools", |value: &mut SummonInfo| {
        to_array(&value.config_pools)
    });
    engine.register_get("exchange_groups", |value: &mut SummonInfo| {
        to_array(&value.exchange_groups)
    });
    engine.register_get("recycle_states", |value: &mut SummonInfo| {
        to_array(&value.recycle_states)
    });
    engine.register_get("rewards", |value: &mut SummonInfo| to_array(&value.rewards));
    engine.register_get("records", |value: &mut SummonInfo| to_array(&value.records));
}
