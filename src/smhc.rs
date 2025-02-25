use json::JsonValue;
use smdton::{SmDton, SmDtonBuffer, SmDtonBuilder, SmDtonMap};

use crate::smgv::{SM_CALLOUT, SM_TABLE};
use crate::smitem::{SmItem, SmMethod};
use crate::smker::check_ker;
use crate::smuc::SMU as smu;

static SM_USAGE: &str = "$usage";

fn _could_callout() -> bool {
    return *SM_CALLOUT.read().unwrap();
}

pub struct SmHub {}

impl SmHub {
    pub fn register(&self, define: SmDtonBuffer, method: SmMethod) -> bool {
        check_ker();

        let sd = SmDton::new_from_buffer(&define);

        let op = sd.get_string(SM_USAGE);
        if op.is_none() {
            return false;
        }
        let _usage = op.unwrap();
        if _usage == "" {
            return false;
        }
        if _usage == "smker.callsm" {
            smu.log("--- call out enabled ---");
            if !_could_callout() {
                let mut callout = SM_CALLOUT.write().unwrap();
                *callout = true;
            }
        }
        smu.log(&format!("--- sm register --- {} ---", &_usage));

        let _item = SmItem {
            define: define,
            method: method,
        };

        {
            let mut _m = SM_TABLE.write().unwrap();
            _m.insert(_usage, _item);
        }
        return true;
    }

    pub fn register_by_json(&self, define: &JsonValue, method: SmMethod) -> bool {
        return self.register(smu.build_buffer(define), method);
    }

    pub fn call(&self, input: SmDtonBuffer) -> SmDtonBuffer {
        let _dt_in = SmDton::new_from_buffer(&input);
        let _op_name = _dt_in.get_string(SM_USAGE);
        let name;
        match _op_name {
            Some(_name) => {
                name = _name;
            }
            _ => {
                return SmDtonBuffer::new();
            }
        }

        let _m = SM_TABLE.read().unwrap();
        let _op = _m.get(&name);
        match _op {
            Some(_item) => {
                let _method = _item.method;
                let _ret = _method(&input);
                if smu.is_debug() {
                    let sd1 = SmDton::new_from_buffer(&input);
                    let sd2 = SmDton::new_from_buffer(&_ret);
                    smu.log(&format!(
                        "--- sm call --- {} --- {} --- {}",
                        name,
                        sd1.stringify().unwrap(),
                        sd2.stringify().unwrap()
                    ));
                }
                return _ret;
            }
            _ => {
                if _could_callout() {
                    let _op2 = _m.get("smker.callsm");
                    match _op2 {
                        Some(_item) => {
                            let mut smp = SmDtonMap::new();
                            smp.add_string("name", &name);

                            let _method = _item.method;
                            let _ret = _method(&input);
                            return _ret;
                        }
                        _ => {
                            smu.log(&format!("--- sm call --- no outside --- {} ---", name));
                            return SmDtonBuffer::new();
                        }
                    }
                } else {
                    smu.log(&format!("--- sm call --- nothing --- {} ---", name));
                    return SmDtonBuffer::new();
                }
            }
        }
    }
}

pub static SMH: SmHub = SmHub {};

pub fn sm_get_all(_input: &SmDtonBuffer) -> SmDtonBuffer {
    smu.log(&format!("--- smo --- smker.get.all ---"));
    let mut _jsn = JsonValue::new_object();
    let _m = SM_TABLE.read().unwrap();
    for pair in _m.iter() {
        let name = pair.0;
        let sd = SmDton::new_from_buffer(&pair.1.define);
        let txt = sd.stringify().unwrap();
        let rjsn = json::parse(&txt);
        if rjsn.is_ok() {
            _jsn[name] = rjsn.unwrap();
        }
    }

    let mut _ret = SmDtonBuilder::new_from_json(&_jsn);
    return _ret.build();
}
