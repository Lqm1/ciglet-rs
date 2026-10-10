//! Callback references stay rooted and are used only on their JavaScript thread.
use napi::bindgen_prelude::{
    FnArgs, Function as JsFunction, FunctionRef, JsObjectValue, Object, ObjectRef, ToNapiValue,
    TypeName, Unknown,
};
use napi::{Env, Error, Result, ValueType};
use std::rc::Rc;

pub(crate) struct RootedValue {
    env: Env,
    reference: Option<ObjectRef>,
}
impl Drop for RootedValue {
    fn drop(&mut self) {
        if let Some(reference) = self.reference.take() {
            let _ = reference.unref(&self.env);
        }
    }
}
pub(crate) enum Value {
    Number(f64),
    Other,
    Context(Rc<RootedValue>),
}
impl Value {
    pub(crate) fn new(env: Env, value: Unknown<'_>) -> Result<Self> {
        let mut object = Object::new(&env)?;
        object.set_named_property("value", value)?;
        Ok(Self::Context(Rc::new(RootedValue {
            env,
            reference: Some(object.create_ref()?),
        })))
    }
    pub(crate) fn number(value: f64) -> Self {
        Self::Number(value)
    }
    pub(crate) fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(*value),
            _ => None,
        }
    }
    fn from_result(value: Unknown<'_>) -> Result<Self> {
        if value.get_type()? != ValueType::Number {
            return Ok(Self::Other);
        }
        // SAFETY: the value's type was checked immediately above. It is used
        // only inside the active callback handle scope and is never retained.
        Ok(Self::Number(unsafe { value.cast::<f64>()? }))
    }
}
struct RootedFunction {
    env: Env,
    reference: FunctionRef<(), Unknown<'static>>,
}
#[derive(Clone)]
pub(crate) struct Function {
    root: Rc<RootedFunction>,
}
impl Function {
    pub(crate) fn new(env: Env, value: Unknown<'_>) -> Result<Self> {
        if value.get_type()? != ValueType::Function {
            return Err(Error::from_reason("callback must be callable"));
        }
        // SAFETY: this is a function in the current environment. create_ref
        // roots it before this call's scope ends. Unknown results are inspected
        // immediately, never stored beyond the callback's handle scope.
        let function = unsafe { value.cast::<JsFunction<'_, (), Unknown<'static>>>()? };
        Ok(Self {
            root: Rc::new(RootedFunction {
                env,
                reference: function.create_ref()?,
            }),
        })
    }
    pub(crate) fn call0(&self, _receiver: &()) -> Result<Value> {
        let function = self.root.reference.borrow_back(&self.root.env)?;
        Value::from_result(function.apply((), ())?)
    }
    pub(crate) fn call1(&self, _receiver: &(), value: &Value) -> Result<Value> {
        let value = value
            .as_f64()
            .ok_or_else(|| Error::from_reason("callback argument must be numeric"))?;
        let function = self
            .root
            .reference
            .borrow_back(&self.root.env)?
            .into_unknown(&self.root.env)?;
        // SAFETY: the rooted value was validated as a function in new().
        let function = unsafe { function.cast::<JsFunction<'_, f64, Unknown<'_>>>()? };
        Value::from_result(function.apply((), value)?)
    }
    pub(crate) fn call3(
        &self,
        _receiver: &(),
        first: &Value,
        second: &Value,
        context: &Value,
    ) -> Result<Value> {
        let Value::Context(context) = context else {
            return Err(Error::from_reason("missing callback context"));
        };
        let context = context
            .reference
            .as_ref()
            .ok_or_else(|| Error::from_reason("context has been released"))?
            .get_value(&self.root.env)?
            .get_named_property::<Unknown<'_>>("value")?;
        let first = first
            .as_f64()
            .ok_or_else(|| Error::from_reason("first callback argument must be numeric"))?;
        let second = second
            .as_f64()
            .ok_or_else(|| Error::from_reason("second callback argument must be numeric"))?;
        let function = self
            .root
            .reference
            .borrow_back(&self.root.env)?
            .into_unknown(&self.root.env)?;
        // SAFETY: the rooted value was validated as a function in new().
        let function = unsafe {
            function.cast::<JsFunction<'_, FnArgs<(f64, f64, Unknown<'_>)>, Unknown<'_>>>()?
        };
        Value::from_result(function.apply((), (first, second, context).into())?)
    }
}
pub struct ReturnedFunction(pub(crate) Function);
impl TypeName for ReturnedFunction {
    fn type_name() -> &'static str {
        "Function"
    }
    fn value_type() -> ValueType {
        ValueType::Function
    }
}
impl ToNapiValue for ReturnedFunction {
    unsafe fn to_napi_value(
        env: napi::sys::napi_env,
        value: Self,
    ) -> Result<napi::sys::napi_value> {
        let function = value.0.root.reference.borrow_back(&value.0.root.env)?;
        // SAFETY: the function is rooted in the current environment for the
        // entire conversion. Node retains the returned local handle.
        unsafe { ToNapiValue::to_napi_value(env, function) }
    }
}
