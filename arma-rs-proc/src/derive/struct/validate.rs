use syn::Error;

use crate::derive::{
    attributes::{Attribute, ContainerAttributes},
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
            DataStruct::Map(fields) => {
                if fields.len() > 1 {
                    errors.add(
                        attributes
                            .transparent
                            .error("#[arma(transparent)] structs must have exactly one field"),
                    );
                }
            }
            _ => {
                errors.add(
                    attributes
                        .transparent
                        .error("#[arma(transparent)] can only be used on map-like structs"),
                );
            }
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
            errors.add(attr.error("#[arma(default)] can't be used on new type structs"));
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

    match data {
        DataStruct::Map(fields) => fields
            .iter()
            .find(|f| *f.attributes.default.value())
            .map(|f| &f.attributes.default),
        DataStruct::Tuple(fields) => fields
            .iter()
            .find(|f| *f.attributes.default.value())
            .map(|f| &f.attributes.default),
        DataStruct::NewType(field) => field
            .attributes
            .default
            .value()
            .then_some(&field.attributes.default),
    }
}
