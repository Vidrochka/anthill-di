struct StructWithValueIngection {
    #[allow(dead_code)]
    string: String,
    #[allow(dead_code)]
    string_ref: &'static str,
    #[allow(dead_code)]
    u_number: u32,
    #[allow(dead_code)]
    number: i32,
}

impl crate::Injection for StructWithValueIngection {
    fn build_injection(injector: &mut crate::Injector) -> Self {
        StructWithValueIngection {
            string: injector.get_new_instance(),
            string_ref: injector.get_new_instance(),
            u_number: injector.get_new_instance(),
            number: injector.get_new_instance(),
        }
    }
}

#[test]
fn base_types_as_unique_inject() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_type::<StructWithValueIngection>().build(),
        crate::builders::ContainerBuilder::bind_type::<String>().to_constructor(|_| -> String { "test".to_string() }).build(),
        crate::builders::ContainerBuilder::bind_type::<&str>().to_constructor(|_| -> &str { "test 2" }).build(),
        crate::builders::ContainerBuilder::bind_type::<u32>().build(),
        crate::builders::ContainerBuilder::bind_type::<i32>().to_constructor(|_| -> i32 { 1 }).build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<StructWithValueIngection>();

    assert_eq!(obj.string, "test".to_string());
    assert_eq!(obj.string_ref, "test 2".to_string());
    assert_eq!(obj.u_number, 0);
    assert_eq!(obj.number, 1);
}