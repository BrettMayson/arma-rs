use syn::Error;

use crate::derive::{
    attributes::{Attribute, ContainerAttributes, FieldAttributes},
    data::StructData,
    CombinedErrors,
};

pub fn validate_attributes(
    errors: &mut CombinedErrors,
    attributes: &ContainerAttributes,
    data: &StructData,
) {
    if *attributes.transparent.value() {
        match data {
            StructData::Map(fields) if fields.len() > 1 => {
                errors.add(
                    attributes
                        .transparent
                        .error("#[arma(transparent)] structs must have exactly one field"),
                );
            }
            StructData::Tuple(_) => {
                errors.add(
                    attributes
                        .transparent
                        .error("#[arma(transparent)] cannot be used on tuple like structs"),
                );
            }
            _ => {}
        }
    }

    if let Some(attr) = get_default_attr(attributes, data) {
        match data {
            StructData::Map(_) if *attributes.transparent.value() => {
                errors.add(
                    attr.error("#[arma(default)] and #[arma(transparent)] cannot be used together"),
                );
            }
            StructData::NewType(_) => {
                errors.add(attr.error("#[arma(default)] cannot be used on new type structs"));
            }
            _ => {}
        }
    }

    if let StructData::Tuple(fields) = data {
        let mut index_first_default = None;
        for (index, field) in fields.iter().enumerate() {
            match index_first_default {
                None => {
                    if field.attributes.default.is_set() {
                        index_first_default = Some(index);
                    }
                }
                Some(index) => {
                    if !field.attributes.default.is_set() {
                        errors.add(Error::new_spanned(&field.ty,
                            format!("field must have #[arma(default)] because previous field {} has #[arma(default)]", index)
                        ));
                    }
                }
            }
        }
    }
}

fn get_default_attr<'a>(
    attributes: &'a ContainerAttributes,
    data: &'a StructData,
) -> Option<&'a Attribute<bool>> {
    if *attributes.default.value() {
        return Some(&attributes.default);
    }

    field_attributes(data)
        .iter()
        .find(|attr| *attr.default.value())
        .map(|f| &f.default)
}

fn field_attributes(data: &StructData) -> Vec<&FieldAttributes> {
    match data {
        StructData::Map(fields) => fields.iter().map(|f| &f.attributes).collect(),
        StructData::Tuple(fields) => fields.iter().map(|f| &f.attributes).collect(),
        StructData::NewType(field) => vec![&field.attributes],
    }
}
