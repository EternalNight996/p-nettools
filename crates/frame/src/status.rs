pub enum Status {
    Success,
    InvalidExist,
    InvalidCommand,
    InvalidAuth,
}
impl ToString for Status {
    fn to_string(&self) -> String {
        match *self {
            Self::Success => "00000".to_owned(),
            Self::InvalidExist => "00001".to_owned(),
            Self::InvalidCommand => "00400".to_owned(),
            Self::InvalidAuth => "00404".to_owned(),
        }
    }
}
