use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query_status", adventure_query_status);
    register_stdlib_fn_1!(module, stdlib, "start", adventure_start, point: i64);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "claim_reward",
        adventure_claim_reward,
        point: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "start_auto",
        adventure_start_auto,
        point: i64,
        count: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "end_auto",
        adventure_end_auto,
        point: i64
    );
    register_stdlib_fn_0!(module, stdlib, "claim_daily", adventure_claim_daily);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "end_auto_vip",
        adventure_end_auto_vip,
        point: i64
    );
}
