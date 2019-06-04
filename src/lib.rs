mod utils;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct ValidationResult {
    pub disableFormat : bool,
    pub errors : Vec<i32>,
    pub instance : serde_json::Value,
    pub propertyPath : String,
    pub schema : serde_json::Value,
    pub throwError: bool,
    pub valid : bool,
}

#[wasm_bindgen]
pub fn validate(instance: JsValue, schema: JsValue) -> JsValue {
    let instanceValue : serde_json::Value = instance.into_serde().unwrap();
    let schemaValue : serde_json::Value = schema.into_serde().unwrap();

    let ret = ValidationResult {
        disableFormat : false,
        errors: vec![], //#Todo fill
        instance: instanceValue.clone(),
        propertyPath: "instance".to_string(),
        schema: schemaValue.clone(),
        throwError: false,
        valid : jsonschema_valid::is_valid(&instanceValue, &schemaValue, None, true)

    };
    JsValue::from_serde(&ret).unwrap()
}