use std::{sync::{Arc, Weak}, any::{TypeId, type_name}};

use tokio::sync::RwLock;

use crate::types::BuildDependencyError;

#[allow(dead_code)]
struct ScopedDependency {
    pub str: String,
}

impl ScopedDependency {
    fn new() ->  Self {
        Self { str: "test".to_string() }
    }
}

#[tokio::test]
async fn scoped_instance() {
    use crate::DependencyContext;

    let mut root_context = DependencyContext::new_root();
    let instance = ScopedDependency::new();
    root_context.add_scoped_instance(instance).await.unwrap();

    let dependency = root_context.get_scoped::<ScopedDependency>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());

    let dependency2 = root_context.get_scoped::<ScopedDependency>().await.unwrap();

    assert!(Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency2.upgrade().unwrap())); // ссылки на scoped объекты созданные в одном scope совпадают

    let _old_scope = root_context.get_scope(); // сохраняем scope, т.к. при удалении ссылок на scope все scoped зависимости удаленного scope удаяются
    let _new_scope = root_context.set_empty_scope(); // устанавливаем новый чистый scope

    let dependency3 = root_context.get_scoped::<ScopedDependency>().await;

    assert_eq!(dependency3.err(), Some(BuildDependencyError::NotFound {
        id: TypeId::of::<Weak<RwLock<ScopedDependency>>>(),
        name: type_name::<Weak<RwLock<ScopedDependency>>>().to_string()
    })); // dependency отсутствует, т.к. не был задан конструктор
}