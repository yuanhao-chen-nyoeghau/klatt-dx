use std::{
    error, fmt,
    ops::{Deref, DerefMut},
};

use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct JsValueError(pub JsValue);
impl fmt::Display for JsValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JsValueError: {:?}", self.0)
    }
}
impl error::Error for JsValueError {}
impl From<JsValue> for JsValueError {
    fn from(value: JsValue) -> Self {
        JsValueError(value)
    }
}
impl From<JsValueError> for JsValue {
    fn from(value: JsValueError) -> Self {
        value.0
    }
}
impl Deref for JsValueError {
    type Target = JsValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for JsValueError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
