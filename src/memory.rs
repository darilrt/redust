use std::collections::HashMap;

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

#[derive(Clone)]
pub struct DBValue {
    pub ttl: i64,
    pub value: Value,
}

pub struct MemoryDb {
    database: HashMap<String, DBValue>,
}

impl MemoryDb {
    pub fn new() -> Self {
        Self {
            database: HashMap::new(),
        }
    }

    pub fn copy(&mut self, key: &str, new_key: &str) {
        if let Some(v) = self.database.get(key) {
            self.database.insert(new_key.to_string(), v.clone());
        }
    }

    pub fn rename(&mut self, key: &str, new_key: &str) {
        if let Some(v) = self.database.remove(key) {
            self.database.insert(new_key.to_string(), v);
        }
    }

    pub fn remove(&mut self, key: &str) {
        self.database.remove(key);
    }

    pub fn get_ttl(&self, key: &str) -> i64 {
        self.database.get(key).map(|v| v.ttl).unwrap_or(-2)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.database.get(key).map(|v| &v.value)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.database.insert(key, DBValue { ttl: -1, value });
    }

    pub fn set_ttl(&mut self, key: String, ttl: i64) {
        if let Some(v) = self.database.get_mut(&key) {
            v.ttl = ttl;
        }
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Str(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
        }
    }
}
