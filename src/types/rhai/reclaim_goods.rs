use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        ReclaimGoodsItem,
        item_id,
        count,
        unit_price,
        previous_price,
        status
    );
    register_getters!(engine, ReclaimGoodsEgg, spirit_id, catch_time, host_uin);
    register_getters!(engine, ReclaimGoodsPrice, item_id, unit_price);
    register_getters!(
        engine,
        ReclaimGoodsListResult,
        balance,
        tips,
        safe_code_open,
        safe_code_required
    );
    engine.register_get("items", |value: &mut ReclaimGoodsListResult| {
        to_array(&value.items)
    });
    register_getters!(
        engine,
        ReclaimGoodsEggListResult,
        balance,
        safe_code_open,
        safe_code_required
    );
    engine.register_get("eggs", |value: &mut ReclaimGoodsEggListResult| {
        to_array(&value.eggs)
    });
    register_getters!(
        engine,
        ReclaimGoodsPriceResult,
        safe_code_open,
        safe_code_required
    );
    engine.register_get("items", |value: &mut ReclaimGoodsPriceResult| {
        to_array(&value.items)
    });
    register_getters!(engine, ReclaimGoodsSellResult, balance, tips);
}
