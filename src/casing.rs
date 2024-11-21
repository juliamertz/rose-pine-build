use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToTrainCase,
};


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Case {
    Snake,
    ShoutySnake,
    Kebab,
    ShoutyKebab,
    Camel,
    Pascal,
    Title,
    Train,
}

pub trait Casing {
    fn to_case(&self, case: Case) -> String;
}

impl Casing for String {
    fn to_case(&self, case: Case) -> String {
        match case {
            Case::Snake => self.to_snake_case(),
            Case::ShoutySnake => self.to_shouty_snake_case(),
            Case::Kebab => self.to_kebab_case(),
            Case::ShoutyKebab => self.to_shouty_kebab_case(),
            Case::Camel => self.to_lower_camel_case(),
            Case::Pascal => self.to_pascal_case(),
            Case::Title => self.to_title_case(),
            Case::Train => self.to_train_case(),
        }
    }
}
