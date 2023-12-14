// This file is needed to let Rust know
// this folder contains some modules
// The use statement is just to bring to scope the modules
// to outter layer and reduce the use statement in other files
pub use method::Method;
pub use request::ParseError;
pub use request::Request;
pub mod method;
pub mod query_string;
pub mod request;
pub use query_string::{QueryString, Value as queryStringValue};
