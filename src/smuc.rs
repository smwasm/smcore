use json::JsonValue;
use smdton::{SmDtonMap, SmDtonBuffer, SmDtonBuilder};

use crate::for_rust;
use crate::smgv::SM_WASM;
use crate::ISmCoreSupport;

pub struct SmOption {
    support: Option<Box<dyn ISmCoreSupport>>,
    wasm_type: i32,
    in_debug: bool,
}

impl SmOption {
    pub fn new() -> Self {
        SmOption {
            support: None,
            wasm_type: 0,
            in_debug: false,
        }
    }
}

pub struct SmUtil {}

impl SmUtil {
    pub fn build_key(&self, pre: &str, sub: &str) -> String {
        return format!("{}.{}", pre, sub);
    }

    pub fn build_buffer(&self, define: &JsonValue) -> SmDtonBuffer {
        let mut db = SmDtonBuilder::new_from_json(define);
        return db.build();
    }

    pub fn new_sm_map<'a>(&self, value: &'a str) -> SmDtonMap<'a> {
        let mut obj = SmDtonMap::new();
        obj.add_string("$usage", value);
        return obj;
    }

    pub fn get_string(&self, jsn: &JsonValue, key: &str) -> Option<String> {
        let _name = &jsn[key];
        let _op = _name.as_str();
        match _op {
            Some(_txt) => {
                return Some(_txt.to_string());
            }
            _ => {
                return None;
            }
        }
    }

    pub fn set_wasm(&self, wasm_type: i32, support: Option<Box<dyn ISmCoreSupport>>) {
        {
            let mut smo = SM_WASM.write().unwrap();
            smo.wasm_type = wasm_type;
            if wasm_type < 10 && support.is_none() {
                let s = for_rust::SmSupportForRust {};
                smo.support = Some(Box::new(s));
            } else {
                smo.support = support;
            }
        }
        self.log(&format!("--- sm wasm --- {} ---", wasm_type));
    }

    // 0 ~ 9 : Rust
    // 10 ~ 19 : wasm web
    pub fn wasm_type(&self) -> i32 {
        return SM_WASM.read().unwrap().wasm_type;
    }

    pub fn set_debug(&self, in_debug: bool) {
        {
            SM_WASM.write().unwrap().in_debug = in_debug;
        }
        self.log(&format!("--- sm debug --- {} ---", in_debug));
    }

    pub fn is_debug(&self) -> bool {
        return SM_WASM.read().unwrap().in_debug;
    }

    pub fn has_support(&self) -> bool {
        let opsupport = &SM_WASM.read().unwrap().support;
        match opsupport {
            Some(_support) => {
                return true;
            }
            _ => {}
        }
        return false;
    }

    pub fn get_current_ms(&self) -> u128 {
        let opsupport = &SM_WASM.read().unwrap().support;
        match opsupport {
            Some(support) => {
                return support.get_current_ms();
            }
            _ => {}
        }
        return 0;
    }

    pub fn log(&self, txt: &str) {
        let opsupport = &SM_WASM.read().unwrap().support;
        match opsupport {
            Some(support) => {
                support.sm_log(txt);
            }
            _ => {}
        }
    }
}

pub static SMU: SmUtil = SmUtil {};
