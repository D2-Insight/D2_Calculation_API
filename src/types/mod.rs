

#[cfg(target_arch = "wasm32")]
pub mod js_types;
#[cfg(not(target_arch = "wasm32"))]
pub mod py_types;

pub mod rs_types;

//make a to_rs trait
//make a to_js trait
//make a to_py trait
trait ToRs {
    fn to_rs<T>(&self) -> T;
}
