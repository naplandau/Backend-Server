use bson::Document;
use validator::{ValidationErrors, ValidationErrorsKind};

pub fn get_validate_error(e: ValidationErrors) -> Document {
    let mut doc = Document::new();
    for (s, vlk) in e.into_errors().iter() {
        match vlk {
            ValidationErrorsKind::Field(v) => {
                for i in v.iter() {
                    doc.insert(*s, &*i.message.clone().unwrap());
                }
            }
            _ => unimplemented!(),
        };
    }
    doc
}
