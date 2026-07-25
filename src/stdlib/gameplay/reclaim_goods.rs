use crate::stdlib::util::{
    lock_stdlib, parse_typed_array, register_stdlib_fn_0, register_stdlib_fn_1,
    register_stdlib_fn_3, to_rhai_error_in_context,
};
use crate::stdlib::RocoStdLib;
use crate::types::{ReclaimGoodsEgg, ReclaimGoodsItem};
use rhai::{Array, Module, NativeCallContext};
use std::sync::{Arc, Mutex};

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_1!(module, stdlib, "query_goods", reclaim_goods_query_goods, goods_type: i64);
    register_stdlib_fn_3!(module, stdlib, "sell_goods", reclaim_goods_sell_goods, goods_type: i64, item_id: i64, count: i64);
    register_stdlib_fn_0!(module, stdlib, "query_eggs", reclaim_goods_query_eggs);
    register_stdlib_fn_3!(module, stdlib, "sell_egg", reclaim_goods_sell_egg, host_uin: i64, spirit_id: i64, catch_time: i64);
    register_sell_goods_batch(module, stdlib.clone());
    register_sell_eggs_batch(module, stdlib.clone());
    register_stdlib_fn_1!(module, stdlib, "query_prices", reclaim_goods_query_prices, goods_type: i64);
}

fn register_sell_goods_batch<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    module.set_native_fn(
        "sell_goods_batch",
        move |context: NativeCallContext, goods_type: i64, items: Array| {
            let items = parse_typed_array::<ReclaimGoodsItem>("items[]", items, &context)?;
            lock_stdlib(&stdlib)?
                .reclaim_goods_sell_goods_batch(goods_type, items)
                .map_err(|error| to_rhai_error_in_context(error, &context))
        },
    );
}

fn register_sell_eggs_batch<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    module.set_native_fn(
        "sell_eggs_batch",
        move |context: NativeCallContext, eggs: Array| {
            let eggs = parse_typed_array::<ReclaimGoodsEgg>("eggs[]", eggs, &context)?;
            lock_stdlib(&stdlib)?
                .reclaim_goods_sell_eggs_batch(eggs)
                .map_err(|error| to_rhai_error_in_context(error, &context))
        },
    );
}
