struct StructWithCustomConstuctorAndInstance {
    #[allow(dead_code)]
    string: String,
}

impl crate::Injection for StructWithCustomConstuctorAndInstance {
    fn build_injection(injector: &mut crate::Injector) -> Self {
        Self {
            string: injector.get_new_instance()
        }
    }
}

#[test]
fn type_with_castom_constructor_and_instance() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_type()
            .to_constructor(|_| -> _ { StructWithCustomConstuctorAndInstance {string: "test 1".to_string()} } )
            .to_value(StructWithCustomConstuctorAndInstance {string: "test 2".to_string()}).build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<StructWithCustomConstuctorAndInstance>();
    assert_eq!(obj.string, "test 1".to_string());

    let obj = injector.lock().unwrap().get_singletone::<StructWithCustomConstuctorAndInstance>();
    assert_eq!(obj.lock().unwrap().string, "test 2".to_string());
}

#[test]
fn unconfigured_type_with_castom_constructor_and_instance() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_unconfigured_type()
            .build_with_constructor_and_value(
                StructWithCustomConstuctorAndInstance {string: "test 2".to_string()},
                |_| -> _ { StructWithCustomConstuctorAndInstance {string: "test 1".to_string()} }
            )
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<StructWithCustomConstuctorAndInstance>();
    assert_eq!(obj.string, "test 1".to_string());

    let obj = injector.lock().unwrap().get_singletone::<StructWithCustomConstuctorAndInstance>();
    assert_eq!(obj.lock().unwrap().string, "test 2".to_string());
}