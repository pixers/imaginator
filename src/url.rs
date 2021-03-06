use nom;
use base64;
use urlencoding;
use crypto::hmac::Hmac;
use crypto::mac::{MacResult, Mac};
use crypto::sha1::Sha1;
use crate::cfg::CONFIG;
use crate::imaginator::url::filter;
use crate::imaginator::filter::Filter;

#[derive(Debug, Fail, Clone)]
pub enum UrlParseError {
    #[fail(display="Invalid signature.")]
    InvalidSignature,
    #[fail(display = "Url parse error. Unparsed data: {}", _0)]
    RemainingData(String),
    #[fail(display = "Incomplete url.")]
    IncompleteUrl,
    #[fail(display = "Url parse error: {}", _0)]
    ParseError(String),
    #[fail(display = "Url decoding error.")]
    UrlDecodingError,
}

fn check_signature(input: &str, rest: &str) -> bool {
    let sig = match base64::decode_config(input, base64::URL_SAFE) {
        Ok(sig) => sig,
        // One way or another, the signature is invalid
        Err(_) => return false
    };
    println!("{:?} {:?}", CONFIG.secret, rest);
    if let Some(ref secret) = CONFIG.secret {
        let mut hmac = Hmac::new(Sha1::new(), secret.as_bytes());
        println!("{}", rest);
        hmac.input(rest.as_bytes());
        hmac.result() == MacResult::new(&sig)
    } else {
        true
    }
}

fn is_base64(input: &str) -> bool {
    base64::decode_config(input, base64::URL_SAFE).is_ok()
}

named!(signature(&str) -> bool, do_parse!(
    sig: terminated!(verify!(take!(28), is_base64), tag!("/")) >>
    rest: peek!(is_not!("")) >>
    (check_signature(sig, rest))
));

named!(full_url(&str) -> (Option<bool>, Filter), do_parse! (
    sig: opt!(complete!(signature)) >>
    filter: call!(filter) >>
    (sig, filter)
));

pub fn parse(input: &str) -> Result<Filter, UrlParseError> {
    let url = match urlencoding::decode(input) {
        Ok(url) => url,
        Err(_) => return Err(UrlParseError::UrlDecodingError)
    };
    match full_url(&url) {
        Ok(("", (sig, filter))) => match sig {
            Some(false) => Err(UrlParseError::InvalidSignature),
            _ => Ok(filter),
        },
        Ok((remaining, _)) => Err(UrlParseError::RemainingData(remaining.to_owned())),
        Err(nom::Err::Incomplete(_)) => Err(UrlParseError::IncompleteUrl),
        Err(e) => Err(UrlParseError::ParseError(format!("{:?}", e))),
    }
}
