use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2, register_stdlib_fn_3,
};
use crate::stdlib::RocoStdLib;

// Index convention:
// - submit_second/second_repair use 0-based CGI indices from ui1914.
// - submit_third uses 0-based boss index from ui1910 COMBAT_NPCID.
// - third_complete/third_get_item use 0-based reward/level indices from AS.
// - second_view/third_query_bag kind is raw CGI type: see AS comments in backend protocol.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query", pisces_first_query);
    register_stdlib_fn_0!(module, stdlib, "submit_first", pisces_submit_first);
    register_stdlib_fn_0!(module, stdlib, "first_claim_gift", pisces_first_claim_gift);
    register_stdlib_fn_0!(module, stdlib, "first_exchange", pisces_first_exchange);
    register_stdlib_fn_0!(module, stdlib, "first_buy", pisces_first_buy);

    register_stdlib_fn_0!(module, stdlib, "second_query", pisces_second_query);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "submit_second",
        pisces_submit_second,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_second_without_spirit",
        pisces_submit_second_without_spirit
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_claim_gift",
        pisces_second_claim_gift,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_second_combat",
        pisces_submit_second_combat,
        combat_index: i64
    );
    register_stdlib_fn_1!(module, stdlib, "second_repair", pisces_second_repair, repair_index: i64);
    register_stdlib_fn_1!(module, stdlib, "second_view", pisces_second_view, kind: i64);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_evolution",
        pisces_second_evolution,
        spirit_id: i64,
        catch_time: i64
    );

    register_stdlib_fn_0!(module, stdlib, "third_query", pisces_third_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_third",
        pisces_submit_third,
        boss_index: i64
    );
    register_stdlib_fn_3!(
        module,
        stdlib,
        "third_buy",
        pisces_third_buy,
        kind: i64,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_full_level",
        pisces_third_full_level,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_complete",
        pisces_third_complete,
        level_index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_get_item",
        pisces_third_get_item,
        reward_index: i64
    );
    register_stdlib_fn_1!(module, stdlib, "third_query_bag", pisces_third_query_bag, kind: i64);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_up",
        pisces_third_up,
        spirit_id: i64,
        catch_time: i64
    );
}
