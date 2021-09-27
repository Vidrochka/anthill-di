#[derive(Debug, PartialEq)]
pub enum DiError {
    CustomInjectTimeError(String),
    ContainerNotFound{type_name: String},
    IvalidDiCast{from: String, to: String},
    ConstructorNotDefined{type_name: String}
}