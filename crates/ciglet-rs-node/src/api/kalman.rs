use crate::kalman::{self, Estimate};
use napi::Error as BindingError;
/// Owned complete native estimate with independently mutable public fields.
pub struct KalmanEstimateF32 {
    value: Estimate<f32>,
}
impl KalmanEstimateF32 {
    pub fn new(means: &[f32], variances: &[f32], log_likelihood: f32) -> Self {
        Self {
            value: Estimate {
                means: means.to_vec(),
                variances: variances.to_vec(),
                log_likelihood,
            },
        }
    }
    pub fn means(&self) -> Vec<f32> {
        self.value.means.clone()
    }
    pub fn set_means(&mut self, values: &[f32]) {
        self.value.means = values.to_vec();
    }
    pub fn variances(&self) -> Vec<f32> {
        self.value.variances.clone()
    }
    pub fn set_variances(&mut self, values: &[f32]) {
        self.value.variances = values.to_vec();
    }
    pub fn log_likelihood(&self) -> f32 {
        self.value.log_likelihood
    }
    pub fn set_log_likelihood(&mut self, value: f32) {
        self.value.log_likelihood = value;
    }
    /// Preserve every native scalar bit without JavaScript Number conversion.
    pub fn likelihood_value(&self) -> Vec<f32> {
        vec![self.value.log_likelihood]
    }
    pub fn set_likelihood_value(&mut self, values: &[f32]) -> Result<(), BindingError> {
        let [value]: [f32; 1] = values.try_into().map_err(|_| {
            napi::Error::from_reason("Kalman likelihood requires exactly one value")
        })?;
        self.value.log_likelihood = value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
pub fn kalman_filter_f32(
    measurements: &[f32],
    process_variances: &[f32],
    measurement_variances: &[f32],
) -> Result<KalmanEstimateF32, BindingError> {
    kalman::filter(measurements, process_variances, measurement_variances)
        .map(|value| KalmanEstimateF32 { value })
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn kalman_filter_with_initial_f32(
    measurements: &[f32],
    process_variances: &[f32],
    measurement_variances: &[f32],
    initial_mean: f32,
) -> Result<KalmanEstimateF32, BindingError> {
    kalman::filter_with_initial(
        measurements,
        process_variances,
        measurement_variances,
        initial_mean,
    )
    .map(|value| KalmanEstimateF32 { value })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn kalman_smooth_f32(
    means: &[f32],
    posterior_variances: &[f32],
    process_variances: &[f32],
) -> Result<Vec<f32>, BindingError> {
    kalman::smooth(means, posterior_variances, process_variances)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Owned complete native estimate with independently mutable public fields.
pub struct KalmanEstimateF64 {
    value: Estimate<f64>,
}
impl KalmanEstimateF64 {
    pub fn new(means: &[f64], variances: &[f64], log_likelihood: f64) -> Self {
        Self {
            value: Estimate {
                means: means.to_vec(),
                variances: variances.to_vec(),
                log_likelihood,
            },
        }
    }
    pub fn means(&self) -> Vec<f64> {
        self.value.means.clone()
    }
    pub fn set_means(&mut self, values: &[f64]) {
        self.value.means = values.to_vec();
    }
    pub fn variances(&self) -> Vec<f64> {
        self.value.variances.clone()
    }
    pub fn set_variances(&mut self, values: &[f64]) {
        self.value.variances = values.to_vec();
    }
    pub fn log_likelihood(&self) -> f64 {
        self.value.log_likelihood
    }
    pub fn set_log_likelihood(&mut self, value: f64) {
        self.value.log_likelihood = value;
    }
    /// Preserve every native scalar bit without JavaScript Number conversion.
    pub fn likelihood_value(&self) -> Vec<f64> {
        vec![self.value.log_likelihood]
    }
    pub fn set_likelihood_value(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [value]: [f64; 1] = values.try_into().map_err(|_| {
            napi::Error::from_reason("Kalman likelihood requires exactly one value")
        })?;
        self.value.log_likelihood = value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
pub fn kalman_filter_f64(
    measurements: &[f64],
    process_variances: &[f64],
    measurement_variances: &[f64],
) -> Result<KalmanEstimateF64, BindingError> {
    kalman::filter(measurements, process_variances, measurement_variances)
        .map(|value| KalmanEstimateF64 { value })
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn kalman_filter_with_initial_f64(
    measurements: &[f64],
    process_variances: &[f64],
    measurement_variances: &[f64],
    initial_mean: f64,
) -> Result<KalmanEstimateF64, BindingError> {
    kalman::filter_with_initial(
        measurements,
        process_variances,
        measurement_variances,
        initial_mean,
    )
    .map(|value| KalmanEstimateF64 { value })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn kalman_smooth_f64(
    means: &[f64],
    posterior_variances: &[f64],
    process_variances: &[f64],
) -> Result<Vec<f64>, BindingError> {
    kalman::smooth(means, posterior_variances, process_variances)
        .map_err(|error| napi::Error::from_reason(error.0))
}
