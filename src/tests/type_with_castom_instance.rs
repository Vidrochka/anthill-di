struct StructWithCustomInstance {
    #[allow(dead_code)]
    string: String,
}

impl crate::Injection for StructWithCustomInstance {
    fn build_injection(injector: &mut crate::Injector) -> Self {
        Self {
            string: injector.get_new_instance()
        }
    }
}

#[test]
fn type_with_castom_instance() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_type().to_value(StructWithCustomInstance {string: "test".to_string()}).build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_singletone::<StructWithCustomInstance>();
    assert_eq!((*obj.lock().unwrap()).string, "test".to_string());

    let obj = injector.lock().unwrap().get_new_instance::<StructWithCustomInstance>();
    assert_eq!(obj.string, "".to_string());

    let obj = injector.lock().unwrap().get_singletone::<StructWithCustomInstance>();
    assert_eq!((*obj.lock().unwrap()).string, "test".to_string());
}

#[test]
fn unconfigured_type_with_castom_instance_singletone_instance() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_unconfigured_type()
            .build_with_value(StructWithCustomInstance {string: "test".to_string()}),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_singletone::<StructWithCustomInstance>();
    assert_eq!((*obj.lock().unwrap()).string, "test".to_string());
}

#[test]
#[should_panic]
fn unconfigured_type_with_castom_instance_new_instance_panic() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_unconfigured_type()
            .build_with_value(StructWithCustomInstance {string: "test".to_string()}),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<StructWithCustomInstance>();
    assert_eq!(obj.string, "".to_string());
}