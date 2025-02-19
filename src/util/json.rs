use std::borrow::Cow;
use serde_json::{Map, Value};
use crate::error::Error;

enum PathPart {
    Key(Cow<'static, str>),
    Index(usize)
}
pub(crate) struct JsonCursor<'a> {
    parent: Option<(PathPart, &'a JsonCursor<'a>)>,
    value: &'a Value
}

pub(crate) struct Array<'a> {
    cursor: &'a JsonCursor<'a>,
    items: &'a [Value],
}

pub(crate) struct Object<'a> {
    cursor: &'a JsonCursor<'a>,
    items: &'a Map<String, Value>,
}

impl<'a> JsonCursor<'a> {
    pub(crate) fn new(value: &'a Value) -> Self {
        Self { parent: None, value }
    }
    pub(crate) fn json(&self) -> &Value {
        self.value
    }
    pub(crate) fn path_string(&self) -> String {
        match &self.parent {
            None => { "$".to_string() }
            Some((path_part, parent_cursor)) => {
                let mut path = parent_cursor.path_string();
                match path_part {
                    PathPart::Key(key) => {
                        path.push('.');
                        path.push_str(key.as_ref());
                    }
                    PathPart::Index(index) => {
                        path.push('[');
                        path.push_str(&index.to_string());
                        path.push(']');
                    }
                }
                path
            }
        }
    }
    pub(crate) fn as_array(&self) -> Result<Array, Error> {
        match self.value {
            Value::Array(items) => Ok(Array { cursor: self, items }),
            _ => Err(Error::from(format!("{} is not an array", self.path_string())))
        }
    }
    pub(crate) fn as_object(&self) -> Result<Object, Error> {
        match self.value {
            Value::Object(items) => Ok(Object { cursor: self, items }),
            _ => Err(Error::from(format!("{} is not an object", self.path_string())))
        }
    }
}

impl Array<'_> {
    pub(crate) fn iter(&self) -> impl Iterator<Item = JsonCursor> {
        self.items.iter().enumerate().map(move |(i, value)| {
            JsonCursor {
                parent: Some((PathPart::Index(i), self.cursor)),
                value
            }
        })
    }
}

impl Object<'_> {
    pub(crate) fn get_str(&self, key: &'static str) -> Result<JsonCursor, Error> {
        match self.items.get(key) {
            Some(value) => Ok(JsonCursor {
                parent: Some((PathPart::Key(Cow::Borrowed(key)), self.cursor)),
                value
            }),
            None => Err(Error::from(
                format!("{} does not have key '{}'", self.cursor.path_string(), key)
            ))
        }
    }
    pub(crate) fn get_str_opt(&self, key: &'static str) -> Option<JsonCursor> {
        self.items.get(key).map(|value| JsonCursor {
            parent: Some((PathPart::Key(Cow::Borrowed(key)), self.cursor)),
            value
        })
    }
}