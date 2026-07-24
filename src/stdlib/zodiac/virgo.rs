use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "serve_god_query", virgo_serve_god_query);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "serve_god_accept_task",
        virgo_serve_god_accept_task
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "serve_god_give_up_task",
        virgo_serve_god_give_up_task
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "serve_god_finish_task",
        virgo_serve_god_finish_task
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "serve_god_buy_unlock",
        virgo_serve_god_buy_unlock
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_serve_god_boss",
        virgo_submit_serve_god_boss
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "serve_god_query_bag",
        virgo_serve_god_query_bag
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "serve_god_upgrade_to_100",
        virgo_serve_god_upgrade_to_100,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "serve_god_upgrade",
        virgo_serve_god_upgrade,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "find_halidom_query",
        virgo_find_halidom_query,
        altar: bool
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "find_halidom_clean",
        virgo_find_halidom_clean,
        relic_index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "find_halidom_list_spirits",
        virgo_find_halidom_list_spirits
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "find_halidom_put_spirit",
        virgo_find_halidom_put_spirit,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "find_halidom_buy_pass",
        virgo_find_halidom_buy_pass
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "find_halidom_buy_search_count",
        virgo_find_halidom_buy_search_count
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "find_halidom_buy_top_level",
        virgo_find_halidom_buy_top_level,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bell_fox_query_status",
        virgo_bell_fox_query_status
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "bell_fox_exchange_item",
        virgo_bell_fox_exchange_item,
        exchange_position: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "bell_fox_buy_tail",
        virgo_bell_fox_buy_tail,
        count: i64
    );
    register_stdlib_fn_0!(module, stdlib, "bell_fox_buy_wish", virgo_bell_fox_buy_wish);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bell_fox_exchange_spirit",
        virgo_bell_fox_exchange_spirit
    );
}
