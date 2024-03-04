pub trait BuilderExt {
    fn types_attributes(self, paths: &[&str], attributes: &[&str]) -> Self;
    fn fields_attributes(self, path: &[&str], fields: &[&str], attributes: &[&str]) -> Self;
}

impl BuilderExt for tonic_build::Builder {
    fn types_attributes(self, paths: &[&str], attributes: &[&str]) -> Self {
        paths.iter().fold(self, |acc, path| {
            attributes
                .iter()
                .fold(acc, |acc, attribute| acc.type_attribute(path, attribute))
        })
    }

    fn fields_attributes(self, path: &[&str], fields: &[&str], attributes: &[&str]) -> Self {
        path.iter().fold(self, |acc, path| {
            fields.iter().fold(acc, |acc, field| {
                attributes.iter().fold(acc, |acc, attribute| {
                    acc.field_attribute(format!("{}.{}", path, field), attribute)
                })
            })
        })
    }
}
