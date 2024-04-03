use smdton::{SmDtonBuffer, SmDtonPair};

pub type SmMethod = fn(&SmDtonPair) -> SmDtonBuffer;

pub struct SmItem {
    pub define: SmDtonBuffer,
    pub method: SmMethod,
}
