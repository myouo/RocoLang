use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_vanguard_lord(module, stdlib.clone());
    register_little_angel_angie(module, stdlib.clone());
    register_red_lotus_beast(module, stdlib.clone());
    register_blue_water_beast(module, stdlib.clone());
    register_ice_crystal_tiger(module, stdlib.clone());
    register_dark_pioneer_dragon(module, stdlib.clone());
    register_bat_prince(module, stdlib.clone());
    register_rock_armor_lord(module, stdlib.clone());
    register_golden_mantis(module, stdlib.clone());
    register_loki(module, stdlib.clone());
    register_magic_tail_cat(module, stdlib.clone());
    register_nether_fox(module, stdlib.clone());
    register_drill_man(module, stdlib);
}

fn register_vanguard_lord<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_query",
        magic_pioneer_vanguard_lord_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_vanguard_lord",
        magic_pioneer_submit_vanguard_lord
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_put",
        magic_pioneer_vanguard_lord_put
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_buy",
        magic_pioneer_vanguard_lord_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_claim_reward",
        magic_pioneer_vanguard_lord_claim_reward
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "vanguard_lord_exchange",
        magic_pioneer_vanguard_lord_exchange,
        index: i64,
        num: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_complete",
        magic_pioneer_vanguard_lord_complete
    );
}

fn register_little_angel_angie<T: RocoStdLib + 'static>(
    module: &mut Module,
    stdlib: Arc<Mutex<T>>,
) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "little_angel_angie_query",
        magic_pioneer_little_angel_angie_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "little_angel_angie_buy",
        magic_pioneer_little_angel_angie_buy
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_little_angel_angie",
        magic_pioneer_submit_little_angel_angie,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "little_angel_angie_claim_reward",
        magic_pioneer_little_angel_angie_claim_reward
    );
}

fn register_red_lotus_beast<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "red_lotus_beast_query",
        magic_pioneer_red_lotus_beast_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "red_lotus_beast_buy",
        magic_pioneer_red_lotus_beast_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_red_lotus_beast",
        magic_pioneer_submit_red_lotus_beast
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "red_lotus_beast_start_combat",
        magic_pioneer_red_lotus_beast_start_combat,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_red_lotus_beast_combat",
        magic_pioneer_submit_red_lotus_beast_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "red_lotus_beast_claim_gift",
        magic_pioneer_red_lotus_beast_claim_gift
    );
}

fn register_blue_water_beast<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_query",
        magic_pioneer_blue_water_beast_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_one_key",
        magic_pioneer_blue_water_beast_one_key
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_check",
        magic_pioneer_blue_water_beast_check
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_blue_water_beast_game",
        magic_pioneer_submit_blue_water_beast_game
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_blue_water_beast_combat",
        magic_pioneer_submit_blue_water_beast_combat,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_claim_gift",
        magic_pioneer_blue_water_beast_claim_gift
    );
}

fn register_ice_crystal_tiger<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "ice_crystal_tiger_query",
        magic_pioneer_ice_crystal_tiger_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "ice_crystal_tiger_one_key",
        magic_pioneer_ice_crystal_tiger_one_key
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_ice_crystal_tiger_game",
        magic_pioneer_submit_ice_crystal_tiger_game
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "submit_ice_crystal_tiger_chain",
        magic_pioneer_submit_ice_crystal_tiger_chain,
        index: i64,
        number: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "ice_crystal_tiger_start_indexed_combat",
        magic_pioneer_ice_crystal_tiger_start_indexed_combat,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_ice_crystal_tiger_combat",
        magic_pioneer_submit_ice_crystal_tiger_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "ice_crystal_tiger_claim_gift",
        magic_pioneer_ice_crystal_tiger_claim_gift
    );
}

fn register_dark_pioneer_dragon<T: RocoStdLib + 'static>(
    module: &mut Module,
    stdlib: Arc<Mutex<T>>,
) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_query",
        magic_pioneer_dark_pioneer_dragon_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_buy",
        magic_pioneer_dark_pioneer_dragon_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_dark_pioneer_dragon",
        magic_pioneer_submit_dark_pioneer_dragon
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_add",
        magic_pioneer_dark_pioneer_dragon_add
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_claim_reward",
        magic_pioneer_dark_pioneer_dragon_claim_reward
    );
}

fn register_bat_prince<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bat_prince_query",
        magic_pioneer_bat_prince_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bat_prince_one_key",
        magic_pioneer_bat_prince_one_key
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "bat_prince_start_indexed_combat",
        magic_pioneer_bat_prince_start_indexed_combat,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_bat_prince",
        magic_pioneer_submit_bat_prince
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bat_prince_claim_gift",
        magic_pioneer_bat_prince_claim_gift
    );
}

fn register_rock_armor_lord<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "rock_armor_lord_query",
        magic_pioneer_rock_armor_lord_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "rock_armor_lord_buy",
        magic_pioneer_rock_armor_lord_buy
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "submit_rock_armor_lord",
        magic_pioneer_submit_rock_armor_lord,
        index: i64,
        success: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "rock_armor_lord_add",
        magic_pioneer_rock_armor_lord_add
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "rock_armor_lord_claim_reward",
        magic_pioneer_rock_armor_lord_claim_reward
    );
}

fn register_golden_mantis<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_query",
        magic_pioneer_golden_mantis_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_buy",
        magic_pioneer_golden_mantis_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_start_combat",
        magic_pioneer_golden_mantis_start_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_golden_mantis_combat",
        magic_pioneer_submit_golden_mantis_combat
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_golden_mantis",
        magic_pioneer_submit_golden_mantis,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_claim_gift",
        magic_pioneer_golden_mantis_claim_gift
    );
}

fn register_loki<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "loki_query", magic_pioneer_loki_query);
    register_stdlib_fn_0!(module, stdlib, "loki_buy", magic_pioneer_loki_buy);
    register_stdlib_fn_0!(module, stdlib, "submit_loki", magic_pioneer_submit_loki);
    register_stdlib_fn_0!(module, stdlib, "loki_add", magic_pioneer_loki_add);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "loki_claim_gift",
        magic_pioneer_loki_claim_gift
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "loki_query_all",
        magic_pioneer_loki_query_all
    );
}

fn register_magic_tail_cat<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_query",
        magic_pioneer_magic_tail_cat_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_buy",
        magic_pioneer_magic_tail_cat_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_start_combat",
        magic_pioneer_magic_tail_cat_start_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_magic_tail_cat_combat",
        magic_pioneer_submit_magic_tail_cat_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_learn",
        magic_pioneer_magic_tail_cat_learn
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_magic_tail_cat",
        magic_pioneer_submit_magic_tail_cat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_claim_gift",
        magic_pioneer_magic_tail_cat_claim_gift
    );
}

fn register_nether_fox<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "nether_fox_query",
        magic_pioneer_nether_fox_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "nether_fox_buy",
        magic_pioneer_nether_fox_buy
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "nether_fox_start_combat",
        magic_pioneer_nether_fox_start_combat,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_nether_fox_combat",
        magic_pioneer_submit_nether_fox_combat
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_nether_fox",
        magic_pioneer_submit_nether_fox,
        number: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "nether_fox_claim_gift",
        magic_pioneer_nether_fox_claim_gift
    );
}

fn register_drill_man<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_query",
        magic_pioneer_drill_man_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_one_key",
        magic_pioneer_drill_man_one_key
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_start_combat",
        magic_pioneer_drill_man_start_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_drill_man_combat",
        magic_pioneer_submit_drill_man_combat
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "submit_drill_man",
        magic_pioneer_submit_drill_man,
        index: i64,
        success: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_claim_gift",
        magic_pioneer_drill_man_claim_gift
    );
}
