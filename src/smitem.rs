use smdton::SmDtonBuffer;

pub type SmMethod = fn(&SmDtonBuffer) -> SmDtonBuffer;

pub struct SmItem {
    pub define: SmDtonBuffer,
    pub method: SmMethod,
}
