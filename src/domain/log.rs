use chrono::Utc;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default, Debug, Serialize, Clone, PartialEq)]
pub struct LogError {
    pub message: String,
}

impl LogError {
    pub fn new(msg: &str) -> LogError {
        LogError {
            message: msg.to_string(),
        }
    }
}

type LogResult<T> = Result<T, LogError>;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq,Eq,Hash)]
pub enum LogType {
    #[default]
    AppLog,
    RequestLog,
    RequestExternalLog,
    PIILog,
    UserAccessLog,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq,Eq,Hash)]
pub enum LogLevel {
    #[default]
    Info,
    Warning,
    Error,
    Debug,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound = "T: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq")]
pub struct RequestObj<T = Value>
where
    T: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq,
{
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "headers")]
    pub headers: HashMap<String, String>,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<T>, // Supports generic or unknown payload
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound = "T: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq")]
pub struct ResponseObj<T = Value>
where
    T: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq,
{
    #[serde(rename = "status_code", default)]
    pub status_code: Option<u16>,
    #[serde(rename = "headers", default)]
    pub headers: Option<HashMap<String, String>>,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<T>, // Supports generic or unknown payload
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound = "T: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq")]
pub struct LogMessage<T = Value, K = Value>
where
    T: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq,
    K: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq,
{
    pub event_date_time: String,
    pub log_type: LogType,
    pub log_level: LogLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_channel_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_user: Option<String>,
    pub correlation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<u64>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<RequestObj<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseObj<K>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<HashMap<String, String>>,
}

impl LogMessage {
    pub fn new(
        log_type: LogType,
        log_level: LogLevel,
        message: &str,
        correlation_id: &str,
    ) -> Self {
        Self {
            event_date_time: Utc::now().to_rfc3339(),
            log_type,
            log_level,
            message: message.to_string(),
            correlation_id: correlation_id.to_string(),
            ..Default::default()
        }
    }

    /// Logs the message using the appropriate log level
    pub fn log(&self) {
        match self.log_level {
            LogLevel::Debug => debug!("{:?}", self),
            LogLevel::Info => info!("{:?}", self),
            LogLevel::Warning => warn!("{:?}", self),
            LogLevel::Error => error!("{:?}", self),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound = "T: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq")]
pub struct LogBuilder<T = Value, K = Value>
where
    T: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq,
    K: Default + Clone + Debug + Serialize + for<'a> Deserialize<'a> + PartialEq,
{
    log: LogMessage<T, K>,
}

impl LogBuilder {
    pub fn new(
        log_type: LogType,
        log_level: LogLevel,
        message: &str,
        correlation_id: &str,
    ) -> Self {
        Self {
            log: LogMessage::new(log_type, log_level, message, correlation_id),
        }
    }

    #[allow(dead_code)]
    pub fn log_type(mut self, log_type: LogType) -> Self {
        self.log.log_type = log_type;
        self
    }

    #[allow(dead_code)]
    pub fn log_level(mut self, log_level: LogLevel) -> Self {
        self.log.log_level = log_level;
        self
    }

    #[allow(dead_code)]
    pub fn message(mut self, message: String) -> Self {
        self.log.message = message;
        self
    }

    #[allow(dead_code)]
    pub fn code_location(mut self, location: String) -> Self {
        self.log.code_location = Some(location);
        self
    }
    #[allow(dead_code)]
    pub fn request(mut self, request: RequestObj) -> Self {
        self.log.request = Some(request);
        self
    }
    #[allow(dead_code)]
    pub fn response(mut self, response: ResponseObj) -> Self {
        self.log.response = Some(response);
        self
    }
    #[allow(dead_code)]
    pub fn add_extra_field(mut self, key: String, value: String) -> Self {
        self.log
            .extra_fields
            .get_or_insert_with(HashMap::new)
            .insert(key, value);
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> LogResult<LogMessage> {
        let log = self.log.clone();

        if matches!(
            log.log_type,
            LogType::RequestLog | LogType::RequestExternalLog
        ) {
            if log.request.is_none() {
                return Err(LogError::new("Request object is missing"));
            }
            if log.response.is_none() {
                return Err(LogError::new("Response object is missing"));
            }
        }

        log.log();
        Ok(log)
    }
}
