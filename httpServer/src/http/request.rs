use super::method::Method;
use super::method::MethodError;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET ....
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEnconding)){
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }
        // This is the same as above
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEnconding))?;
        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest),
        // }
        // This is the same as  above
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        todo!()
    }
}

fn get_next_word (request:&str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return  Some((&request[..i], &request[i+1..]));
        }
    }
    todo!()
}

// ERROR HANDLING
pub enum ParseError {
    InvalidRequest,
    InvalidEnconding,
    InvalidProtocol,
    InvalidMethod,
}


impl ParseError{
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest =>  "Invalid Request",
            Self::InvalidEnconding =>"Invalid Enconding",
            Self::InvalidProtocol  =>"Invalid Protocol",
            Self::InvalidMethod =>"Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}", self.message())
    }
}

impl From<MethodError> for ParseError{
    fn from (_:MethodError) -> Self{
        Self::InvalidMethod
    }

}

impl Error for ParseError {}

