use std::str::Chars;

use nanoserde::{DeJson, DeJsonErr, DeJsonState, DeJsonTok};

#[derive(Debug, Clone)]
pub enum Value {
    Boolean {
        name: String,
        display: i64,
        /// Default value of a Boolean.
        defaults: bool,
    },
    Color {
        name: String,
        display: i64,
        /// Default value of a Color. Format is "#RRGGBBAA".
        defaults: String,
        /// Flag to get whether to include the Alpha component on a Color.
        include_alpha: bool,
    },
    Enum {
        name: String,
        display: i64,
        /// List of choices for the Enum.
        choices: Vec<String>,
        /// Default value of an Enum.
        defaults: i64,
    },
    Filepath {
        name: String,
        display: i64,
        defaults: String,
        extensions: Vec<String>,
    },
    Float {
        name: String,
        display: i64,
        defaults: f64,
        /// Flag to set if the value is bounded with a min/max.
        bounded: bool,
        /// Minimum value of a Float.
        min: f64,
        /// Maximum value of a Float.
        max: f64,
    },
    Integer {
        name: String,
        display: i64,
        defaults: i64,
        /// Flag to set if the value is bounded with a min/max.
        bounded: bool,
        /// Minimum value of an Int.
        min: i64,
        /// Maximum value of an Int.
        max: i64,
    },
    String {
        name: String,
        display: i64,
        defaults: String,
        /// Maximum length of a String.
        max_length: i64,
        /// Flag to set if whitespace should be trimmed.
        trim_whitespace: bool,
    },
    Text {
        name: String,
        display: i64,
        defaults: String,
    },
}

#[derive(Debug, Clone)]
pub enum EntityValue {
    Bool(bool),
    Float(f64),
    Int(i64),
    String(String),
}

impl EntityValue {
    pub fn as_bool(&self) -> Option<bool> {
        if let EntityValue::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        if let EntityValue::Int(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        if let EntityValue::Float(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let EntityValue::String(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl DeJson for EntityValue {
    fn de_json(s: &mut DeJsonState, i: &mut Chars) -> Result<Self, DeJsonErr> {
        let v = match s.tok {
            DeJsonTok::Bool(b) => EntityValue::Bool(b),
            DeJsonTok::I64(n) => EntityValue::Int(n),
            DeJsonTok::U64(n) => EntityValue::Int(n as i64),
            DeJsonTok::F64(n) => EntityValue::Float(n),
            DeJsonTok::Str => EntityValue::String(s.as_string()?),
            _ => return Err(s.err_token("bool, number or string")),
        };
        s.next_tok(i)?;
        Ok(v)
    }
}

#[derive(Debug, Clone)]
enum Raw {
    Bool(bool),
    Num(f64),
    Str(String),
}

impl Raw {
    fn b(&self, s: &DeJsonState) -> Result<bool, DeJsonErr> {
        if let Raw::Bool(v) = self {
            Ok(*v)
        } else {
            Err(s.err_type("bool"))
        }
    }
    fn i(&self, s: &DeJsonState) -> Result<i64, DeJsonErr> {
        if let Raw::Num(v) = self {
            Ok(*v as i64)
        } else {
            Err(s.err_type("number"))
        }
    }
    fn f(&self, s: &DeJsonState) -> Result<f64, DeJsonErr> {
        if let Raw::Num(v) = self {
            Ok(*v)
        } else {
            Err(s.err_type("number"))
        }
    }
    fn s(self, st: &DeJsonState) -> Result<String, DeJsonErr> {
        if let Raw::Str(v) = self {
            Ok(v)
        } else {
            Err(st.err_type("string"))
        }
    }
}

impl DeJson for Raw {
    fn de_json(s: &mut DeJsonState, i: &mut Chars) -> Result<Self, DeJsonErr> {
        let v = match s.tok {
            DeJsonTok::Bool(b) => Raw::Bool(b),
            DeJsonTok::I64(n) => Raw::Num(n as f64),
            DeJsonTok::U64(n) => Raw::Num(n as f64),
            DeJsonTok::F64(n) => Raw::Num(n),
            DeJsonTok::Str => Raw::Str(s.as_string()?),
            _ => return Err(s.err_token("bool, number or string")),
        };
        s.next_tok(i)?;
        Ok(v)
    }
}

impl DeJson for Value {
    fn de_json(state: &mut DeJsonState, i: &mut Chars) -> Result<Self, DeJsonErr> {
        state.curly_open(i)?;

        let (mut name, mut definition): (Option<String>, Option<String>) = (None, None);
        let mut display: Option<i64> = None;
        let (mut defaults, mut min, mut max): (Option<Raw>, Option<Raw>, Option<Raw>) =
            (None, None, None);
        let (mut include_alpha, mut bounded, mut trim_whitespace): (
            Option<bool>,
            Option<bool>,
            Option<bool>,
        ) = (None, None, None);
        let (mut choices, mut extensions): (Option<Vec<String>>, Option<Vec<String>>) =
            (None, None);
        let mut max_length: Option<i64> = None;

        while state.next_str().is_some() {
            match AsRef::<str>::as_ref(&state.strbuf) {
                "name" => {
                    state.next_colon(i)?;
                    name = Some(DeJson::de_json(state, i)?);
                }
                "definition" => {
                    state.next_colon(i)?;
                    definition = Some(DeJson::de_json(state, i)?);
                }
                "display" => {
                    state.next_colon(i)?;
                    display = Some(DeJson::de_json(state, i)?);
                }
                "defaults" => {
                    state.next_colon(i)?;
                    defaults = Some(DeJson::de_json(state, i)?);
                }
                "includeAlpha" => {
                    state.next_colon(i)?;
                    include_alpha = Some(DeJson::de_json(state, i)?);
                }
                "choices" => {
                    state.next_colon(i)?;
                    choices = Some(DeJson::de_json(state, i)?);
                }
                "extensions" => {
                    state.next_colon(i)?;
                    extensions = Some(DeJson::de_json(state, i)?);
                }
                "bounded" => {
                    state.next_colon(i)?;
                    bounded = Some(DeJson::de_json(state, i)?);
                }
                "min" => {
                    state.next_colon(i)?;
                    min = Some(DeJson::de_json(state, i)?);
                }
                "max" => {
                    state.next_colon(i)?;
                    max = Some(DeJson::de_json(state, i)?);
                }
                "maxLength" => {
                    state.next_colon(i)?;
                    max_length = Some(DeJson::de_json(state, i)?);
                }
                "trimWhitespace" => {
                    state.next_colon(i)?;
                    trim_whitespace = Some(DeJson::de_json(state, i)?);
                }
                _ => {
                    state.next_colon(i)?;
                    state.whole_field(i)?;
                }
            }
            state.eat_comma_curly(i)?;
        }
        state.curly_close(i)?;

        fn req<T>(v: Option<T>, s: &DeJsonState, name: &str) -> Result<T, DeJsonErr> {
            v.ok_or_else(|| s.err_nf(name))
        }

        let name = req(name, state, "name")?;
        let display = req(display, state, "display")?;
        let def = req(definition, state, "definition")?;
        let defaults = req(defaults, state, "defaults");

        Ok(match def.as_str() {
            "Boolean" => Value::Boolean {
                name,
                display,
                defaults: defaults?.b(state)?,
            },
            "Color" => Value::Color {
                name,
                display,
                defaults: defaults?.s(state)?,
                include_alpha: include_alpha.unwrap_or(false),
            },
            "Enum" => Value::Enum {
                name,
                display,
                choices: choices.unwrap_or_default(),
                defaults: defaults?.i(state)?,
            },
            "Filepath" => Value::Filepath {
                name,
                display,
                defaults: defaults?.s(state)?,
                extensions: extensions.unwrap_or_default(),
            },
            "Float" => Value::Float {
                name,
                display,
                defaults: defaults?.f(state)?,
                bounded: bounded.unwrap_or(false),
                min: min.map(|v| v.f(state)).transpose()?.unwrap_or(0.0),
                max: max.map(|v| v.f(state)).transpose()?.unwrap_or(0.0),
            },
            "Integer" => Value::Integer {
                name,
                display,
                defaults: defaults?.i(state)?,
                bounded: bounded.unwrap_or(false),
                min: min.map(|v| v.i(state)).transpose()?.unwrap_or(0),
                max: max.map(|v| v.i(state)).transpose()?.unwrap_or(0),
            },
            "String" => Value::String {
                name,
                display,
                defaults: defaults?.s(state)?,
                max_length: max_length.unwrap_or(0),
                trim_whitespace: trim_whitespace.unwrap_or(true),
            },
            "Text" => Value::Text {
                name,
                display,
                defaults: defaults?.s(state)?,
            },
            other => return Err(state.err_enum(other)),
        })
    }
}
