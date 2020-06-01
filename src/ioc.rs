pub struct Ioc {
    pub ioc_type: IocType,
    pub data: String,
}

#[derive(Debug)]
pub enum IocType {
    Ip,
    Domain,
    Url,
    Hash,
    Uid,
}

impl Ioc {
    pub fn new(ioc_type: IocType, data: String) -> Ioc {
        Ioc {
            ioc_type: ioc_type,
            data: data
        }
    }
}
