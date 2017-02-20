//! A direct port to Rust of http://lua-users.org/wiki/SimpleLuaApiExample.

extern crate lua_sys;

use lua_sys::*;
use std::path::PathBuf;
use std::ffi::{CString, CStr};

fn main() {
    unsafe {
        // All Lua contexts are held in this structure. We work with it almost
        // all the time.
        let lua = luaL_newstate();

        luaL_openlibs(lua); // Load Lua libraries.

        // Get the path to the file containing the script.
        let this_file = PathBuf::from(file!());
        let dir = this_file.parent().unwrap();
        let file = dir.join("simple.lua");
        let file = file.to_str().unwrap();
        let file = CString::new(file).unwrap();

        // Load the file containing the script we are going to run.
        let status = luaL_loadfile(lua, file.as_ptr());
        if status != 0 {
            // If something went wrong, error message is at the top of
            // the stack.
            let error = lua_tostring(lua, -1);
            let error = CStr::from_ptr(error).to_string_lossy().into_owned();

            panic!("Couldn't load file: {}", error);
        }

        // Ok, now here we go: We pass data to the lua script on the stack.
        // That is, we first have to prepare Lua's virtual stack the way we
        // want the script to receive it, then ask Lua to run it.
        lua_newtable(lua);

        // To put values into the table, we first push the index, then the
        // value, and then call lua_rawset() with the index of the table in the
        // stack. Let's see why it's -3: In Lua, the value -1 always refers to
        // the top of the stack. When you create the table with lua_newtable(),
        // the table gets pushed into the top of the stack. When you push the
        // index and then the cell value, the stack looks like:
        //
        // <- [stack bottom] -- table, index, value [top]
        //
        // So the -1 will refer to the cell value, thus -3 is used to refer to
        // the table itself. Note that lua_rawset() pops the two last elements
        // of the stack, so that after it has been called, the table is at the
        // top of the stack.
        for i in 1..6 {
           lua_pushnumber(lua, i as f64); // Push the table index.
           lua_pushnumber(lua, i as f64 * 2.0); // Push the cell value.
           lua_rawset(lua, -3); // Stores the pair in the table.
        }

        // By what name is the script going to reference our table?
        let table_name = CString::new("foo").unwrap();
        lua_setglobal(lua, table_name.as_ptr());

        // Ask Lua to run our little script.
        let result = lua_pcall(lua, 0, LUA_MULTRET, 0);
        if result != 0 {
            let error = lua_tostring(lua, -1);
            let error = CStr::from_ptr(error).to_string_lossy().into_owned();

            panic!("Failed to run script: {}", error);
        }

        // Get the returned value at the top of the stack (index -1).
        let sum = lua_tonumber(lua, -1);

        println!("Script returned: {}", sum);

        lua_pop(lua, 1); // Take the returned value out of the stack.
        lua_close(lua); // Cya, Lua.
    }
}
