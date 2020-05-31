pub struct Ioc {
    ioc_type: IocType,
    data: String,
}

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
