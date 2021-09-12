use std::sync::{Arc, Mutex};

struct StructWithSingletoneValueIngection {
    #[allow(dead_code)]
    string: Arc<Mutex<String>>,
}

impl crate::Injection for StructWithSingletoneValueIngection {
    fn build_injection(injector: &mut crate::Injector) -> Self {
        Self {
            string: injector.get_singletone(),
        }
    }
}

struct StructWithSingletoneValueWrapper {
    #[allow(dead_code)]
    struct_with_singletone_1: StructWithSingletoneValueIngection,
    #[allow(dead_code)]
    struct_with_singletone_2: StructWithSingletoneValueIngection,
}

impl crate::Injection for StructWithSingletoneValueWrapper {
    fn build_injection(injector: &mut crate::Injector) -> Self {
        Self {
            struct_with_singletone_1: injector.get_new_instance(),
            struct_with_singletone_2: injector.get_new_instance(),
        }
    }
}

#[test]
fn base_types_singletone_inject() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_type::<StructWithSingletoneValueIngection>()
            .build(),
        crate::builders::ContainerBuilder::bind_type::<StructWithSingletoneValueWrapper>().build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<StructWithSingletoneValueWrapper>();

    *obj.struct_with_singletone_1.string.lock().unwrap() = "tested singletone".to_string();
    assert_eq!(
        *obj.struct_with_singletone_2.string.lock().unwrap(),
        "tested singletone".to_string()
    );
}
