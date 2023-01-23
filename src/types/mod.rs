#[cfg(feature = "wasm")]
pub mod js_types;
#[cfg(feature = "python")]
pub mod py_types;

pub mod rs_types;

//make a to_rs trait
//make a to_js trait
//make a to_py trait
trait ToRs {
    fn to_rs<T>(&self) -> T;
}
