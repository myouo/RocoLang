use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_1!(module, stdlib, "invite", pk_invite, target_uin: i64);
    register_stdlib_fn_0!(module, stdlib, "accept", pk_accept);
    register_stdlib_fn_0!(module, stdlib, "reject", pk_reject);
    register_stdlib_fn_0!(module, stdlib, "cancel_waiting", pk_cancel_waiting);
}
