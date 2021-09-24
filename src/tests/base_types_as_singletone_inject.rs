use std::sync::{Arc, RwLock};

struct StructWithSingletoneValueIngection {
    #[allow(dead_code)]
    string: Arc<RwLock<String>>,
}

impl crate::Injection for StructWithSingletoneValueIngection {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            string: injector.get_singletone()?,
        })
    }
}

struct StructWithSingletoneValueWrapper {
    #[allow(dead_code)]
    struct_with_singletone_1: StructWithSingletoneValueIngection,
    #[allow(dead_code)]
    struct_with_singletone_2: StructWithSingletoneValueIngection,
}

impl crate::Injection for StructWithSingletoneValueWrapper {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            struct_with_singletone_1: injector.get_new_instance()?,
            struct_with_singletone_2: injector.get_new_instance()?,
        })
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

    let obj = injector.write().unwrap().get_new_instance::<StructWithSingletoneValueWrapper>().unwrap();

    *obj.struct_with_singletone_1.string.write().unwrap() = "tested singletone".to_string();
    assert_eq!(
        *obj.struct_with_singletone_2.string.read().unwrap(),
        "tested singletone".to_string()
    );
}
