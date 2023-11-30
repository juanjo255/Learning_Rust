// This file is needed to let Rust know 
// this folder contains some modules
// The use statement is just to bring to scope the modules
// to outter layer and reduce the use statement in other files
pub use request::Request;
pub use method::Method;
pub use request::ParseError;
pub mod request;
pub mod method;