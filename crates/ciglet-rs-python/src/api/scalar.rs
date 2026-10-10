use crate::callbacks::Function;
use pyo3::PyErr as BindingError;
pub fn scalar_math_constants() -> Vec<f64> {
    vec![
        crate::scalar::LEGACY_EPSILON,
        crate::scalar::LEGACY_INFINITY_SENTINEL,
        std::f64::consts::PI,
        std::f64::consts::FRAC_PI_2,
    ]
}
pub fn scalar_add_f32(first: f32, second: f32) -> f32 {
    first + second
}
pub fn scalar_accumulate_square_f32(accumulator: f32, value: f32) -> f32 {
    accumulator + value * value
}
pub fn scalar_fold_f32(
    values: &[f32],
    initial: f32,
    callback: Option<Function>,
    context: &crate::callbacks::Value,
) -> Result<f32, BindingError> {
    if values.is_empty() {
        return Ok(initial);
    }
    let callback = callback.ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err("nonempty scalar fold requires a callback")
    })?;
    values
        .iter()
        .copied()
        .try_fold(initial, |accumulator, value| {
            callback
                .call3(
                    &(),
                    &crate::callbacks::Value::number(accumulator as f64),
                    &crate::callbacks::Value::number(value as f64),
                    context,
                )?
                .as_f64()
                .map(|result| result as f32)
                .ok_or_else(|| {
                    pyo3::exceptions::PyValueError::new_err(
                        "scalar fold callback must return a number",
                    )
                })
        })
}
pub fn scalar_add_f64(first: f64, second: f64) -> f64 {
    first + second
}
pub fn scalar_accumulate_square_f64(accumulator: f64, value: f64) -> f64 {
    accumulator + value * value
}
pub fn scalar_fold_f64(
    values: &[f64],
    initial: f64,
    callback: Option<Function>,
    context: &crate::callbacks::Value,
) -> Result<f64, BindingError> {
    if values.is_empty() {
        return Ok(initial);
    }
    let callback = callback.ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err("nonempty scalar fold requires a callback")
    })?;
    values
        .iter()
        .copied()
        .try_fold(initial, |accumulator, value| {
            callback
                .call3(
                    &(),
                    &crate::callbacks::Value::number(accumulator),
                    &crate::callbacks::Value::number(value),
                    context,
                )?
                .as_f64()
                .ok_or_else(|| {
                    pyo3::exceptions::PyValueError::new_err(
                        "scalar fold callback must return a number",
                    )
                })
        })
}
