use std::fmt::Display;

use strum_macros::{Display, EnumIter};

pub(crate) trait Substitutable {
    fn substitute(&mut self, replacement: &Self, start: usize, end: usize);
}

impl Substitutable for Vec<char> {
    fn substitute(&mut self, new: &Self, start: usize, end: usize) {
        let mut out = Vec::from(&self[0..start]);
        out.extend(new);
        if end != self.len() - 1 {
            out.extend(&self[end..self.len()]);
        }
        *self = out;
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display, Default)]
pub enum Case {
    #[default]
    Snake,
    Kebab,
    Camel,
    Pascal,
    Title,
    Train,
    Lower,
    Upper,
}

pub trait Casing {
    fn to_case(&self, case: Case) -> String;
}

impl<T: ?Sized + Display> Casing for T {
    fn to_case(&self, case: Case) -> String {
        use heck::{
            ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTitleCase, ToTrainCase,
        };

        let val = self.to_string();
        match case {
            Case::Snake => val.to_snake_case(),
            Case::Kebab => val.to_kebab_case(),
            Case::Camel => val.to_lower_camel_case(),
            Case::Pascal => val.to_pascal_case(),
            Case::Title => val.to_title_case(),
            Case::Train => val.to_train_case(),
            Case::Lower => val.to_lowercase(),
            Case::Upper => val.to_uppercase(),
        }
    }
}
