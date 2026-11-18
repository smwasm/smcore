use std::sync::RwLock;

use std::sync::LazyLock;
use std::collections::HashMap;

use crate::smitem::SmItem;
use crate::smker::SmKer;
use crate::smuc::SmOption;

pub static SM_KER: LazyLock<RwLock<SmKer>> =
    LazyLock::new(|| RwLock::new(SmKer::new()));

pub static SM_WASM: LazyLock<RwLock<SmOption>> =
    LazyLock::new(|| RwLock::new(SmOption::new()));

pub static SM_TABLE: LazyLock<RwLock<HashMap<String, SmItem>>> =
    LazyLock::new(|| RwLock::new(HashMap::default()));

pub static SM_CALLOUT: LazyLock<RwLock<bool>> =
    LazyLock::new(|| RwLock::new(false));