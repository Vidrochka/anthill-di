use std::any::TypeId;


#[derive(Debug)]
pub (crate) struct TypeInfo { pub (crate) type_id: TypeId, pub (crate) type_name: String, }

impl TypeInfo {
    #[must_use] pub (crate) fn new(type_id: TypeId, type_name: String) -> Self { Self { type_id, type_name } }
}