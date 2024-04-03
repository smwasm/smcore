use std::time::SystemTime;

use crate::ISmCoreSupport;

pub struct SmSupportForRust {}

impl ISmCoreSupport for SmSupportForRust {
    fn sm_log(&self, txt: &str) {
        println!("{}", txt);
    }

    fn get_current_ms(&self) -> u128 {
        let _now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("get millis error");
        let _mills = _now.as_millis();
        return _mills;
    }
}
