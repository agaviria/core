use iron::Response;
use iron::status::Status;
use iron::modifier::Modifier;
use serde::ser::{Serialize, Serializer};
use serde_json;

use std::fmt;
use std::error::Error;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct IronStatusWrapper(Status);

impl Serialize for IronStatusWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let &IronStatusWrapper(status) = self;
        serializer.serialize_str(&format!("{}", status))
    }
}

// Error Response
#[derive(Eq, PartialEq, Clone, Debug, Serialize)]
pub struct ErrorResponse {
    #[serde(rename="statusCode")]
    pub code: IronStatusWrapper,
    pub error: String,
}

impl Error for ErrorResponse {
    fn description(&self) -> &str {
        "Core Service Error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&*self.return_as_json())
    }
}

impl ErrorResponse {
    pub fn new<S>(code: Status, message: S) -> Self
        where S: Into<String>
    {
        ErrorResponse {
            code: IronStatusWrapper(code),
            error: message.into(),
        }
    }

    pub fn bad_request<S>(message: S) -> ErrorResponse
        where S: Into<String>
    {
        ErrorResponse::new(Status::BadRequest, message)
    }

    pub fn internal_error<S>(message: S) -> ErrorResponse
        where S: Into<String>
    {
        ErrorResponse::new(Status::InternalServerError, message)
    }

    pub fn not_found<S>(message: S) -> ErrorResponse
        where S: Into<String>
    {
        ErrorResponse::new(Status::NotFound, message)
    }

    pub fn unauthorized<S>(message: S) -> ErrorResponse
        where S: Into<String>
    {
        ErrorResponse::new(Status::Unauthorized, message)
    }

    pub fn status(&self) -> Status {
        let IronStatusWrapper(status_code) = self.code;
        status_code
    }

    pub fn return_as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

// Modifies ErrorResponse to JSON Mime type with a pretty print decorator
impl Modifier<Response> for ErrorResponse {
    fn modify(self, response: &mut Response) {
        mime!(Application / Json).modify(response);
        let json = serde_json::to_string_pretty(&self).unwrap();
        json.modify(response)
    }
}
