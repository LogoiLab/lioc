#[derive(Clone)]
pub struct Ioc {
    pub ioc_type: IocType,
    pub data: String,
}

#[derive(Clone, Debug)]
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

impl PartialEq for IocType {
    fn eq(&self, other: &IocType) -> bool {
        return self == other;
    }
}

impl PartialEq<IocType> for Ioc {
    fn eq(&self, other: &IocType) -> bool {
        return self.ioc_type == *other;
    }
}

impl PartialEq<Ioc> for IocType {
    fn eq(&self, other: &Ioc) -> bool {
        return *self == other.ioc_type;
    }
}
