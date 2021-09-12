struct StructWithCustomConstuctor {
    #[allow(dead_code)]
    string: String,
}

impl crate::Injection for StructWithCustomConstuctor {
    fn build_injection(injector: &mut crate::Injector) -> Self {
        Self {
            string: injector.get_new_instance()
        }
    }
}

#[test]
fn type_with_castom_constructor() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_type()
            .to_constructor(|_| -> _ { StructWithCustomConstuctor {string: "test".to_string()} } ).build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<StructWithCustomConstuctor>();
    assert_eq!(obj.string, "test".to_string());

    let obj = injector.lock().unwrap().get_singletone::<StructWithCustomConstuctor>();
    assert_eq!(obj.lock().unwrap().string, "test".to_string());
}

#[test]
fn unconfigured_type_with_castom_constructor() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_unconfigured_type()
            .build_with_constructor(|_| -> _ { StructWithCustomConstuctor {string: "test".to_string()} } ),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<StructWithCustomConstuctor>();
    assert_eq!(obj.string, "test".to_string());

    let obj = injector.lock().unwrap().get_singletone::<StructWithCustomConstuctor>();
    assert_eq!(obj.lock().unwrap().string, "test".to_string());
}