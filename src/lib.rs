mod for_rust;
mod smgv;
mod smhc;
mod smitem;
mod smker;
mod smuc;

// SystemTime HashMap lazy_static RwLock json::
// JsonValue

// ISmCoreSupport
pub trait ISmCoreSupport: Sync + Send {
    fn sm_log(&self, txt: &str);
    fn get_current_ms(&self) -> u128;
}

pub use smhc::SMH as smh;
pub use smuc::SMU as smu;
