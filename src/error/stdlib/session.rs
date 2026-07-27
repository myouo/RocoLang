use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSessionMemoryError {
    DailyMemoryDateUnavailable,
    DailyMemoryFailure {
        message: String,
    },
    TypeMismatch {
        key: String,
        expected: ScriptSessionValueKind,
        actual: ScriptSessionValueKind,
    },
}

impl fmt::Display for ScriptSessionMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DailyMemoryDateUnavailable => {
                f.write_str("server date is unavailable for daily memory")
            }
            Self::DailyMemoryFailure { message } => f.write_str(message),
            Self::TypeMismatch {
                key,
                expected,
                actual,
            } => write!(
                f,
                "session key '{key}' has different type: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
        }
    }
}

impl ScriptSessionMemoryError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::DailyMemoryDateUnavailable => "daily_memory_date_unavailable",
            Self::DailyMemoryFailure { .. } => "daily_memory_failure",
            Self::TypeMismatch { .. } => "type_mismatch",
        }
    }

    pub fn key(&self) -> String {
        match self {
            Self::TypeMismatch { key, .. } => key.clone(),
            Self::DailyMemoryDateUnavailable | Self::DailyMemoryFailure { .. } => String::new(),
        }
    }

    pub fn expected_kind_code(&self) -> String {
        match self {
            Self::TypeMismatch { expected, .. } => expected.as_str().to_string(),
            Self::DailyMemoryDateUnavailable | Self::DailyMemoryFailure { .. } => String::new(),
        }
    }

    pub fn actual_kind_code(&self) -> String {
        match self {
            Self::TypeMismatch { actual, .. } => actual.as_str().to_string(),
            Self::DailyMemoryDateUnavailable | Self::DailyMemoryFailure { .. } => String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSessionValueKind {
    Integer,
    String,
    Bool,
}

impl ScriptSessionValueKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Integer => "integer",
            Self::String => "string",
            Self::Bool => "bool",
        }
    }
}
