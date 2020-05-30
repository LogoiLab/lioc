use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;
use http::StatusCode;

pub mod apache {

    pub struct AccessLog {
        accessor_ip: std::net::IpAddr,
        accessor_ident: Option<String>,
        accessor_userid: Option<String>,
        timestamp: DateTime,
        request: String,
        status_code: StatusCode,
        response_size: Option<u128>,
        referrer: Option<String>,
        user_agent: Option<String>
    }

    impl AccessLog {
        fn parse(entry: String) -> self {
            let custom = DateTime::parse_from_str("10/Oct/2000:13:55:36 -0700", "%d/%m/%Y:%H:%M %z").unwrap();
        }
    }

}
