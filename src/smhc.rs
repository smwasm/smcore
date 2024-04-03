use json::JsonValue;
use smdton::{SmDton, SmDtonBuffer, SmDtonBuilder, SmDtonMap, SmDtonPair};

use crate::smgv::{SM_CALLOUT, SM_TABLE};
use crate::smitem::{SmItem, SmMethod};
use crate::smker::check_ker;
use crate::smuc::SMU as smu;

static SM_USAGE: &str = "$usage";
static SM_CASE: &str = "$case";

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

    pub fn add_case(&self, define: &SmDtonBuffer) -> bool {
        let sd = SmDton::new_from_buffer(define);
        let op1 = sd.get_string(SM_USAGE);
        let op2 = sd.get_string(SM_CASE);
        if op1.is_none() || op2.is_none() {
            return false;
        }
        let _usage = op1.unwrap();
        let _case = op2.unwrap();
        if _usage == "" || _case == "" {
            return false;
        }

        let mut _m = SM_TABLE.write().unwrap();
        let _op3 = _m.get(&_usage);
        match _op3 {
            Some(_usage_item) => {
                let pair = SmDtonPair::new(_usage_item.define.clone(), define.clone());
                let sd_raw = SmDton::new_from_pair(&pair);
                let optxt = sd_raw.stringify();
                if optxt.is_none() {
                    return false;
                }
                let para = optxt.unwrap();
                let opjsn = json::parse(&para);
                if opjsn.is_err() {
                    return false;
                }
                let jsn = opjsn.unwrap();
                let mut smbd = SmDtonBuilder::new_from_json(&jsn);
                let smb = smbd.build();

                smu.log(&format!("--- sm addcase --- {} --- {}", _case, para));
                let _item = SmItem {
                    define: smb,
                    method: _usage_item.method,
                };
                _m.insert(_case, _item);
                return true;
            }
            _ => {}
        }

        return false;
    }

    pub fn get(&self, name: &str) -> SmDtonBuffer {
        let _m = SM_TABLE.read().unwrap();
        let _op = _m.get(name);
        match _op {
            Some(_item) => {
                if smu.is_debug() {
                    let sd = SmDton::new_from_buffer(&_item.define);
                    smu.log(&format!(
                        "--- sm get --- {} --- {}",
                        name,
                        sd.stringify().unwrap()
                    ));
                }
                return _item.define.clone();
            }
            _ => {
                smu.log(&format!("--- sm get --- nothing --- {} ---", name));
                return SmDtonBuffer::new();
            }
        }
    }

    pub fn call(&self, name: &str, input: SmDtonBuffer) -> SmDtonBuffer {
        let _m = SM_TABLE.read().unwrap();
        let _op = _m.get(name);
        match _op {
            Some(_item) => {
                let pair = SmDtonPair::new(_item.define.clone(), input);
                let _method = _item.method;
                let _ret = _method(&pair);
                if smu.is_debug() {
                    let sd1 = SmDton::new_from_pair(&pair);
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
                            smp.add_string("name", name);
                            let pair = SmDtonPair::new(smp.build(), input);

                            let _method = _item.method;
                            let _ret = _method(&pair);
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

pub fn sm_get_all(_input: &SmDtonPair) -> SmDtonBuffer {
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
