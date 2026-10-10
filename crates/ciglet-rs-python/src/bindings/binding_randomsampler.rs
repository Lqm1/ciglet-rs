use super::*;
#[pyclass(unsendable, module = "ciglet_rs")]
pub struct RandomSampler {
    pub(crate) inner: Option<crate::api::RandomSampler>,
}
#[pymethods]
impl RandomSampler {
    fn close(&mut self) {
        self.inner = None;
    }
    fn free(&mut self) {
        self.inner = None;
    }
    #[getter]
    fn is_closed(&self) -> bool {
        self.inner.is_none()
    }
}
#[pymethods]
impl RandomSampler {
    #[new]
    #[pyo3(signature = (source, maximum))]
    pub fn binding_new(
        mut source: pyo3::PyRefMut<'_, RandomWordSource>,
        maximum: u32,
    ) -> pyo3::PyResult<Self> {
        (crate::api::RandomSampler::new(source.inner.take().ok_or_else(consumed)?, maximum))
            .map(|value| RandomSampler { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "from_seed")]
    #[pyo3(signature = (seed))]
    pub fn binding_from_seed(seed: Vec<u8>) -> pyo3::PyResult<Self> {
        (crate::api::RandomSampler::from_seed(&seed))
            .map(|value| RandomSampler { inner: Some(value) })
    }
    #[getter(maximum)]
    pub fn binding_maximum(&mut self) -> pyo3::PyResult<u32> {
        (self.inner.as_ref().ok_or_else(consumed)?).maximum()
    }
    #[pyo3(name = "draw")]
    #[pyo3(signature = ())]
    pub fn binding_draw(&mut self) -> pyo3::PyResult<u32> {
        (self.inner.as_ref().ok_or_else(consumed)?).draw()
    }
    #[pyo3(name = "into_rng")]
    #[pyo3(signature = ())]
    pub fn binding_into_rng(&mut self) -> pyo3::PyResult<RandomWordSource> {
        Ok(RandomWordSource {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_rng()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&mut self) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?).cloned())
            .map(|value| RandomSampler { inner: Some(value) })
    }
    #[pyo3(name = "uniform_f32")]
    #[pyo3(signature = ())]
    pub fn binding_uniform_f32(&mut self) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).uniform_f32()
    }
    #[pyo3(name = "uniform_f64")]
    #[pyo3(signature = ())]
    pub fn binding_uniform_f64(&mut self) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).uniform_f64()
    }
    #[pyo3(name = "normal_f32")]
    #[pyo3(signature = (mean, variance))]
    pub fn binding_normal_f32(&mut self, mean: f32, variance: f32) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).normal_f32(mean, variance)
    }
    #[pyo3(name = "normal_f64")]
    #[pyo3(signature = (mean, variance))]
    pub fn binding_normal_f64(&mut self, mean: f64, variance: f64) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).normal_f64(mean, variance)
    }
    #[pyo3(name = "normal_with_f32")]
    #[pyo3(signature = (mean, variance, code))]
    pub fn binding_normal_with_f32(
        &mut self,
        mean: f32,
        variance: f32,
        code: u32,
    ) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).normal_with_f32(mean, variance, code)
    }
    #[pyo3(name = "normal_with_f64")]
    #[pyo3(signature = (mean, variance, code))]
    pub fn binding_normal_with_f64(
        &mut self,
        mean: f64,
        variance: f64,
        code: u32,
    ) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).normal_with_f64(mean, variance, code)
    }
    #[pyo3(name = "white_noise_f32")]
    #[pyo3(signature = (amplitude, count))]
    pub fn binding_white_noise_f32(
        &mut self,
        amplitude: f32,
        count: usize,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_ref().ok_or_else(consumed)?).white_noise_f32(amplitude, count)
    }
    #[pyo3(name = "white_noise_f64")]
    #[pyo3(signature = (amplitude, count))]
    pub fn binding_white_noise_f64(
        &mut self,
        amplitude: f64,
        count: usize,
    ) -> pyo3::PyResult<Vec<f64>> {
        (self.inner.as_ref().ok_or_else(consumed)?).white_noise_f64(amplitude, count)
    }
}
