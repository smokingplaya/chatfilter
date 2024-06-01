#![feature(c_unwind)]
#[macro_use] extern crate gmod;

use gmod::lua::State;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref FILTER: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

trait LuaTable {
    unsafe fn collect_array(&self) -> Vec<String>;
}

impl LuaTable for State {
    unsafe fn collect_array(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        self.push_nil();

        while self.next(-2) != 0 {
            let key = self.to_integer(-2);
            let value = self.check_string(-1);

            result.insert((key as usize)-1, value.to_string());

            self.pop();
        }

        result
    }
}

#[allow(unused_must_use)]
unsafe extern "C-unwind" fn set_filter(lua: State) -> i32 {
    lua.check_table(1); // self
    lua.check_table(2); // таблица с фильтром

    let mut filter_static = FILTER.lock().unwrap();

    *filter_static = lua.collect_array()
        .iter()
        .map(|str| {str.to_lowercase()}).collect::<Vec<String>>();


    0
}

unsafe extern "C-unwind" fn check(lua: State) -> i32 {
    lua.check_table(1);

    let message = lua.check_string(2).to_string()
        .to_lowercase()
        .replace(" ", "");

    let result = FILTER.lock().unwrap()
        .iter()
        .any(|ban_word| message.contains(ban_word));

    lua.push_boolean(result);

    1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    let name = lua_string!("chatfilter");

    lua.get_global(name);

    if lua.is_nil(-1) {
        lua.pop();
        lua.new_table();
    }

    lua.push_function(set_filter);
    lua.set_field(-2, lua_string!("SetFilter"));

    lua.push_function(check);
    lua.set_field(-2, lua_string!("Check"));

    lua.set_global(name);

    0
}

#[gmod13_close]
fn gmod13_close(_: State) -> i32 {
    0
}