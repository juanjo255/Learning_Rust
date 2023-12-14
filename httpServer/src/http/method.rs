use std::str::FromStr;

pub enum Method {
    GET,
    POST,
    DELETE,
}    

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST) ,
            "DELETE" => Ok(Self::DELETE),
            _ => return Err(MethodError), 
        }
    }
}

pub struct MethodError;