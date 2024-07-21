use crate::prelude::*;

pub trait StringExt {
    fn as_name(&'static self) -> Name;
}

impl StringExt for str {
    fn as_name(&'static self) -> Name {
        Name::new(self)
    }
}