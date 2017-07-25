use hyper;
use hyper::client;
use std::option::Option;
use std;



#[derive(Debug, Clone)]
pub struct Api {
    code: Errcode,
    message: String
}


#[derive(Debug, Copy, Clone)]
pub enum Errcode {
    // UNKNOWN_ERROR
    UNKNOWN_ERROR,

    // 1xxx errors
    API_VERSION_NOT_FOUND,
    API_VERSION_DEPRECATED,
    API_CLASS_NOT_FOUND,
    API_FUNCTION_NOT_FOUND,
    KEY_INSUFFICIENT_RIGHTS,
    INVALID_TOKEN,
    API_FUNCTION_DISABLED,
    PROXER_MAINTENANCE,
    API_MAINTENANCE,

    // 2xxx errors
    IP_BLOCKED,
    NEWS_ERROR,

    // 3xxx errors
    MISSING_LOGIN_CREDENTIALS,
    INVALID_LOGIN_CREDENTIALS,
    USER_NOT_LOGGED_IN,
    USER_ID_NOT_FOUND,

    USER_ALREADER_LOGGED_IN,
}


impl Api {
    pub fn new<T: Into<Errcode>>(code: T, msg: String) -> Self {
        Api {
            code: code.into(),
            message: msg
        }
    }
}


impl Errcode {
    pub fn from_code(code: i64) -> Self {

        match code {
            1000 => Errcode::API_VERSION_NOT_FOUND,
            1001 => Errcode::API_VERSION_DEPRECATED,
            1002 => Errcode::API_CLASS_NOT_FOUND,
            1003 => Errcode::API_FUNCTION_NOT_FOUND,
            1004 => Errcode::KEY_INSUFFICIENT_RIGHTS,
            1005 => Errcode::INVALID_TOKEN,
            1006 => Errcode::API_FUNCTION_DISABLED,
            1007 => Errcode::PROXER_MAINTENANCE,
            1008 => Errcode::API_MAINTENANCE,


            2000 => Errcode::IP_BLOCKED,
            2001 => Errcode::NEWS_ERROR,


            3000 => Errcode::MISSING_LOGIN_CREDENTIALS,
            3001 => Errcode::INVALID_LOGIN_CREDENTIALS,
            3002 => Errcode::USER_NOT_LOGGED_IN,
            3003 => Errcode::USER_ID_NOT_FOUND,
            3004 => Errcode::USER_NOT_LOGGED_IN,
            3009 => Errcode::USER_NOT_LOGGED_IN,
            3012 => Errcode::USER_ALREADER_LOGGED_IN,
            3023 => Errcode::USER_NOT_LOGGED_IN,
            3034 => Errcode::USER_NOT_LOGGED_IN,

            _ => Errcode::UNKNOWN_ERROR,
        }
    }
}



impl From<i64> for Errcode {
    fn from(code: i64) -> Errcode {
        Errcode::from_code(code)
    }
}
