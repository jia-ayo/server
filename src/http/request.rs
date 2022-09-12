use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //Get /search/name=abc&srt=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        //REWRITTEN BEST EFFICINCY
        // match str::from_utf8(buf){
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //    Ok(request) => {}
        //    Err(e) => Err(e),
        // }
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        //GET_NEXT_word
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidEncoding),
        // };
        let request = str::from_utf8(buf)?;
       
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }
        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    //REWRITTEN BEST EFFICINCY
    // let mut iter = request.chars();
    // loop{
    //     let item = iter.next();
    //     match item {
    //         Some(c) =>{},
    //         None => break,
    //     }
    // }
    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c =='\r'{
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}


pub enum  ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest=>  "Invalid Request",
            Self::InvalidEncoding=> "Invalid Encoding",
            Self::InvalidProtocol=> "Invalid Protocol",
            Self::InvalidMethod=>   "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}