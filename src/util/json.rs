use crate::error::Error;
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::sync::Arc;

enum PathPart {
    Key(Cow<'static, str>),
    Index(usize),
}

struct CursorCore<'a> {
    parent: Option<(PathPart, JsonCursor<'a>)>,
    value: &'a Value,
}
#[derive(Clone)]
pub(crate) struct JsonCursor<'a> {
    core: Arc<CursorCore<'a>>,
}

pub(crate) struct Array<'a> {
    cursor: JsonCursor<'a>,
    items: &'a [Value],
}

pub(crate) struct Object<'a> {
    cursor: JsonCursor<'a>,
    items: &'a Map<String, Value>,
}

impl<'a> JsonCursor<'a> {
    pub(crate) fn new(value: &'a Value) -> Self {
        let core = Arc::new(CursorCore {
            parent: None,
            value,
        });
        Self { core }
    }
    pub(crate) fn path_string(&self) -> String {
        match &self.core.parent {
            None => "$".to_string(),
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
    pub(crate) fn as_array<'b>(&'b self) -> Result<Array<'a>, Error> {
        match self.core.value {
            Value::Array(items) => Ok(Array {
                cursor: self.clone(),
                items,
            }),
            _ => Err(Error::from(format!(
                "{} is not an array",
                self.path_string()
            ))),
        }
    }
    pub(crate) fn as_object(&self) -> Result<Object<'a>, Error> {
        match self.core.value {
            Value::Object(items) => Ok(Object {
                cursor: self.clone(),
                items,
            }),
            _ => Err(Error::from(format!(
                "{} is not an object",
                self.path_string()
            ))),
        }
    }
    pub(crate) fn as_str(&self) -> Result<&'a str, Error> {
        match self.core.value {
            Value::String(s) => Ok(s),
            _ => Err(Error::from(format!(
                "{} is not a string",
                self.path_string()
            ))),
        }
    }
    pub(crate) fn as_str_opt(&self) -> Option<&'a str> {
        match self.core.value {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

impl Array<'_> {
    pub(crate) fn iter(&self) -> impl Iterator<Item = JsonCursor> {
        self.items.iter().enumerate().map(move |(i, value)| {
            let core = Arc::new(CursorCore {
                parent: Some((PathPart::Index(i), self.cursor.clone())),
                value,
            });
            JsonCursor { core }
        })
    }
}

impl<'a> Object<'a> {
    pub(crate) fn get(&self, key: &'static str) -> Result<JsonCursor<'a>, Error> {
        match self.items.get(key) {
            Some(value) => {
                let core = Arc::new(CursorCore {
                    parent: Some((PathPart::Key(Cow::Borrowed(key)), self.cursor.clone())),
                    value,
                });
                Ok(JsonCursor { core })
            }
            None => Err(Error::from(format!(
                "{} does not have key '{}'",
                self.cursor.path_string(),
                key
            ))),
        }
    }
    pub(crate) fn get_opt(&self, key: &'static str) -> Option<JsonCursor<'a>> {
        self.items.get(key).map(|value| {
            let core = Arc::new(CursorCore {
                parent: Some((PathPart::Key(Cow::Borrowed(key)), self.cursor.clone())),
                value,
            });
            JsonCursor { core }
        })
    }
}
