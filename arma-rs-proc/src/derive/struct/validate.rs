use syn::Error;

use crate::derive::{
    attributes::{Attribute, ContainerAttributes, FieldAttributes},
    data::DataStruct,
    CombinedError,
};

pub fn validate_attributes(
    errors: &mut CombinedError,
    attributes: &ContainerAttributes,
    data: &DataStruct,
) {
    if *attributes.transparent.value() {
        match data {
            DataStruct::Map(fields) if fields.len() > 1 => {
                errors.add(
                    attributes
                        .transparent
                        .error("#[arma(transparent)] structs must have exactly one field"),
                );
            }
            DataStruct::Tuple(_) => {
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
        if matches!(data, DataStruct::Map(_)) && *attributes.transparent.value() {
            errors.add(
                attributes
                    .transparent
                    .error("#[arma(transparent)] and #[arma(default)] cannot be used together"),
            );
        }

        if matches!(data, DataStruct::NewType(_)) {
            errors.add(attr.error("#[arma(default)] cannot be used on new type structs"));
        }
    }

    if let DataStruct::Tuple(fields) = data {
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
    data: &'a DataStruct,
) -> Option<&'a Attribute<bool>> {
    if *attributes.default.value() {
        return Some(&attributes.default);
    }

    let field_attributes: Vec<&FieldAttributes> = match data {
        DataStruct::Map(fields) => fields.iter().map(|f| &f.attributes).collect(),
        DataStruct::Tuple(fields) => fields.iter().map(|f| &f.attributes).collect(),
        DataStruct::NewType(field) => vec![&field.attributes],
    };

    field_attributes
        .iter()
        .find(|attr| *attr.default.value())
        .map(|f| &f.default)
}
