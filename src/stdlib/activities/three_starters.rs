use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1};
use crate::stdlib::RocoStdLib;

// Index convention:
// - fire index is 1-based, matching the CGI index sent by the original AS.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "water_query", three_starters_water_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "water_buy",
        three_starters_water_buy,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_water_combat",
        three_starters_submit_water_combat
    );
    register_stdlib_fn_0!(module, stdlib, "submit_water", three_starters_submit_water);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "water_claim_gift",
        three_starters_water_claim_gift,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "water_query_bag",
        three_starters_water_query_bag
    );

    register_stdlib_fn_0!(module, stdlib, "fire_query", three_starters_fire_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_buy",
        three_starters_fire_buy,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_start",
        three_starters_fire_start,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_fire_combat",
        three_starters_submit_fire_combat,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_fire_direct",
        three_starters_submit_fire_direct,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_over",
        three_starters_fire_over,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_claim_gift",
        three_starters_fire_claim_gift,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "fire_query_bag",
        three_starters_fire_query_bag
    );

    register_stdlib_fn_0!(module, stdlib, "sun_query", three_starters_sun_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "sun_buy",
        three_starters_sun_buy,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sun_start_combat",
        three_starters_sun_start_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_sun_combat",
        three_starters_submit_sun_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sun_start_collect",
        three_starters_sun_start_collect
    );
    register_stdlib_fn_0!(module, stdlib, "submit_sun", three_starters_submit_sun);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "sun_claim_gift",
        three_starters_sun_claim_gift,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sun_query_bag",
        three_starters_sun_query_bag
    );
}
