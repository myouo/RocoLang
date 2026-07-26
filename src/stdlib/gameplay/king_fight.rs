use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::register_stdlib_fn_0;
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "cancel_match", king_fight_cancel_match);
}
