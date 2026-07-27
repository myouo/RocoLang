use std::sync::{Arc, Mutex};

use rhai::{ImmutableString, Map, Module};

use crate::stdlib::util::{lock_stdlib, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("today", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_today().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("clear", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_clear().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("combat_observed_started", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_combat_observed_started()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("combat_observed_completed", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_combat_observed_completed()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("combat_tracking_since", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_combat_tracking_since()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("combat_limit_reached", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_combat_limit_reached()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("combat_limit", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_combat_limit().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("combat_limit_return_code", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_combat_limit_return_code()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("combat_limit_message", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_combat_limit_message()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_int", move |key: &str, default_value: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_get_int(key, default_value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("set_int", move |key: &str, value: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_set_int(key, value).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("increment_int", move |key: &str, delta: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_increment_int(key, delta)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_string", move |key: &str, default_value: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_get_string(key, default_value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("set_string", move |key: &str, value: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_set_string(key, value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_bool", move |key: &str, default_value: bool| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_get_bool(key, default_value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("set_bool", move |key: &str, value: bool| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_set_bool(key, value).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("delete", move |key: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.daily_memory_delete(key).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_keys", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            let keys = lib.daily_memory_list_keys().map_err(to_rhai_error)?;
            let mut map = Map::new();
            for (key, value_type) in keys {
                map.insert(
                    ImmutableString::from(key).into(),
                    ImmutableString::from(value_type).into(),
                );
            }
            Ok(map)
        });
    }
}
