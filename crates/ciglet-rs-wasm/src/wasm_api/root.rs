use wasm_bindgen::prelude::*;

macro_rules! root_api {
    ($scalar:ty, $name:ident) => {
        /// Find a native bracketed root using a selected-precision callback.
        /// JavaScript exceptions retain their original value. Non-number results
        /// and nonfinite values are ordinary errors; captures replace C context.
        #[wasm_bindgen]
        pub fn $name(
            callback: &js_sys::Function,
            start: $scalar,
            end: $scalar,
        ) -> Result<$scalar, JsValue> {
            let mut exception = None;
            let result = crate::root::find(
                |value: $scalar| match callback
                    .call1(&JsValue::UNDEFINED, &JsValue::from_f64(value.into()))
                {
                    Ok(result) => match result.as_f64() {
                        Some(number) => number as $scalar,
                        None => {
                            exception =
                                Some(JsValue::from_str("root callback must return a number"));
                            <$scalar>::NAN
                        }
                    },
                    Err(error) => {
                        exception = Some(error);
                        <$scalar>::NAN
                    }
                },
                start,
                end,
            );
            match exception {
                Some(error) => Err(error),
                None => result.map_err(|error| JsValue::from_str(error.0)),
            }
        }
    };
}
root_api!(f32, root_find_f32);
root_api!(f64, root_find_f64);
