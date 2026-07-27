use std::sync::{Arc, Mutex};

use rhai::{Array, Module};

use crate::stdlib::util::{lock_stdlib, parse_i64_array_at, to_array, to_rhai_error_in_context};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_items", move |context: rhai::NativeCallContext| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_items()
                .map(|items| to_array(&items))
                .map_err(|error| to_rhai_error_in_context(error, &context))
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "get_cached_scene_roles",
            move |context: rhai::NativeCallContext| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.get_cached_scene_roles()
                    .map(|roles| to_array(&roles))
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "try_change_avatar",
            move |context: rhai::NativeCallContext, avatar: Array| {
                let avatar = parse_i64_array_at("avatar[]", avatar, context.call_position())?;
                let mut lib = lock_stdlib(&stdlib)?;
                lib.try_change_avatar(avatar)
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "try_change_avatar_slot",
            move |context: rhai::NativeCallContext, avatar_position: i64, avatar_id: i64| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.try_change_avatar_slot(avatar_position, avatar_id)
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
}
