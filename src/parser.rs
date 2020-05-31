use super::ioc::{Ioc, IocType};

pub fn parse_file() -> Vec<Ioc> {
    return vec![Ioc::new(IocType::Uid, String::from("test"))];
}
