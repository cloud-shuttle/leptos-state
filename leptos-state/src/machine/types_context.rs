use std::collections::HashMap;

/// Context for machine execution
#[derive(Clone, Debug, Default)]
pub struct Context {
    pub data: HashMap<String, ContextValue>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T: Into<ContextValue>>(&mut self, key: String, value: T) {
        self.data.insert(key, value.into());
    }

    pub fn get(&self, key: &str) -> Option<&ContextValue> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut ContextValue> {
        self.data.get_mut(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<ContextValue> {
        self.data.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Values that can be stored in machine context
#[derive(Clone, Debug, PartialEq)]
pub enum ContextValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ContextValue>),
    Object(HashMap<String, ContextValue>),
}

impl From<String> for ContextValue {
    fn from(s: String) -> Self {
        ContextValue::String(s)
    }
}

impl From<&str> for ContextValue {
    fn from(s: &str) -> Self {
        ContextValue::String(s.to_string())
    }
}

impl From<f64> for ContextValue {
    fn from(n: f64) -> Self {
        ContextValue::Number(n)
    }
}

impl From<i32> for ContextValue {
    fn from(n: i32) -> Self {
        ContextValue::Number(n as f64)
    }
}

impl From<bool> for ContextValue {
    fn from(b: bool) -> Self {
        ContextValue::Boolean(b)
    }
}

impl From<Vec<ContextValue>> for ContextValue {
    fn from(v: Vec<ContextValue>) -> Self {
        ContextValue::Array(v)
    }
}

impl From<HashMap<String, ContextValue>> for ContextValue {
    fn from(m: HashMap<String, ContextValue>) -> Self {
        ContextValue::Object(m)
    }
}
