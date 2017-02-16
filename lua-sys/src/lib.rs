// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Raw bindings to [Lua](https://www.lua.org/). For higher level Lua(JIT) bindings, see
//! [Moon](https://github.com/SShrike/moon).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

extern crate libc;

use std::{default, ptr};
use libc::{c_int, c_longlong, c_char};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Manual Bindings

// Functions from lua.h //
// #define lua_tonumber(L,i) lua_tonumberx(L,(i),NULL)
#[inline(always)]
pub unsafe fn lua_tonumber(L: *mut lua_State, i: c_int) -> f64 {
    lua_tonumberx(L, i, ptr::null_mut())
}

// #define lua_tointeger(L,i) lua_tointegerx(L,(i),NULL)
#[inline(always)]
pub unsafe fn lua_tointeger(L: *mut lua_State, i: c_int) -> c_longlong {
    lua_tointegerx(L, i, ptr::null_mut())
}

// #define lua_tostring(L,i) lua_tolstring(L, (i), NULL)
#[inline(always)]
pub unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char {
    lua_tolstring(L, i, ptr::null_mut())
}

// #define lua_pop(L,n) lua_settop(L, -(n)-1)
#[inline(always)]
pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -(n)-1);
}

// #define lua_newtable(L) lua_createtable(L, 0, 0)
#[inline(always)]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0);
}

// #define lua_register(L,n,f) (lua_pushcfunction(L, (f)), lua_setglobal(L, (n)))
#[inline(always)]
pub unsafe fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, n);
}

// #define lua_pushcfunction(L,f) lua_pushcclosure(L, (f), 0)
#[inline(always)]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0);
}

// #define lua_pushglobaltable(L) ((void)lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS))
#[inline(always)]
pub unsafe fn lua_pushglobaltable(L: *mut lua_State) {
    lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS as i64);
}

// Functions from lauxlib.h //
// #define luaL_loadfile(L,f) luaL_loadfilex(L,f,NULL)
#[inline(always)]
pub unsafe fn luaL_loadfile(L: *mut lua_State, f: *const c_char) -> c_int {
    luaL_loadfilex(L, f, ptr::null())
}

impl default::Default for lua_Debug {
    fn default() -> lua_Debug {
        lua_Debug {
            event: 0,
            name: ptr::null(),
            namewhat: ptr::null(),
            what: ptr::null(),
            source: ptr::null(),
            currentline: 0,
            linedefined: 0,
            lastlinedefined: 0,
            nups: 0,
            nparams: 0,
            isvararg: 0,
            istailcall: 0,
            short_src: [0; 60],
            i_ci: &mut lua_Debug_CallInfo([0; 0]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newstate() {
        let lua = unsafe { luaL_newstate() };
        assert_eq!(lua.is_null(), false);
    }

    #[test]
    fn test_openlibs() {
       let lua = unsafe { luaL_newstate() };
       unsafe { luaL_openlibs(lua) };
    }
}
