use std::sync::RwLock;

use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::smitem::SmItem;
use crate::smker::SmKer;
use crate::smuc::SmOption;

lazy_static! {
    pub static ref SM_KER: RwLock<SmKer> = RwLock::new(SmKer::new());
    pub static ref SM_WASM: RwLock<SmOption> = RwLock::new(SmOption::new());
    pub static ref SM_TABLE: RwLock<HashMap<String, SmItem>> = RwLock::new(HashMap::default());
    pub static ref SM_CALLOUT: RwLock<bool> = RwLock::new(false);
}
