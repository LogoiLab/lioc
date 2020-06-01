#[derive(Clone)]
pub struct Ioc {
    pub ioc_type: IocType,
    pub data: String,
}

#[derive(Clone, Debug, PartialEq)]
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

impl PartialEq for Ioc {
    fn eq(&self, other: &Ioc) -> bool {
        return self.data.eq(&other.data);
    }
}
