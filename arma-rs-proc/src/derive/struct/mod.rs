mod from;
mod into;

use syn::{Error, Result};

pub use from::from_impl_body;
pub use into::into_impl_body;

use super::{Attribute, ContainerAttributes, DataStruct, FieldNamed, FieldUnnamed};

pub fn validate_attributes(data: &DataStruct, attributes: &ContainerAttributes) -> Result<()> {
    if *attributes.transparent.value() {
        match data {
            DataStruct::Map(fields) => {
                if fields.len() > 1 {
                    return Err(attributes
                        .transparent
                        .error("#[arma(transparent)] structs must have exactly one field"));
                }
            }
            _ => {
                return Err(attributes
                    .transparent
                    .error("#[arma(transparent)] can only be used on map-like structs"));
            }
        }
    }

    if let Some(attr) = get_default_attr(data, attributes) {
        if matches!(data, DataStruct::Map(_)) && *attributes.transparent.value() {
            return Err(attributes
                .transparent
                .error("#[arma(transparent)] and #[arma(default)] cannot be used together"));
        }

        if matches!(data, DataStruct::NewType(_)) {
            return Err(attr.error("#[arma(default)] can't be used on new type structs"));
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
                        return Err(Error::new_spanned(&field.ty,
                            format!("field must have #[arma(default)] because previous field {} has #[arma(default)]", index)
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

fn get_default_attr<'a>(
    data: &'a DataStruct,
    attributes: &'a ContainerAttributes,
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
