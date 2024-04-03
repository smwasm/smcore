use json::object;

use crate::smgv::SM_KER;
use crate::smhc::{sm_get_all, SMH as smh};
use crate::smuc::SMU as smu;

const SM_PREFIX: &str = "smker";

pub fn check_ker() {
    {
        let _k = SM_KER.read().unwrap();
        if _k.is_init_done() {
            return;
        }
    }
    {
        let mut _k = SM_KER.write().unwrap();
        _k.set_init_done();
    }
    {
        let _k = SM_KER.read().unwrap();
        _k.init();
    }
}

pub struct SmKer {
    init_done: bool,
}

impl SmKer {
    pub fn new() -> SmKer {
        SmKer { init_done: false }
    }

    fn is_init_done(&self) -> bool {
        return self.init_done;
    }

    fn set_init_done(&mut self) {
        self.init_done = true;
    }

    fn init(&self) {
        smu.log(&format!("--- sm init --- {} ---", SM_PREFIX));

        let _define1 = object! {
            "$usage" => smu.build_key(SM_PREFIX, "get.all")
        };

        let smb = smu.build_buffer(&_define1);
        smh.register(smb, sm_get_all);
    }
}
