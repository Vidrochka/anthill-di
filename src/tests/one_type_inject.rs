struct InnerStruct {
    #[allow(dead_code)]
    pub text: String,
}

impl crate::Injection for InnerStruct {
    fn build_injection(_: &mut crate::Injector) -> Self {
        Self {
            text: "test".to_string(),
        }
    }
}

struct OuterStruct {
    #[allow(dead_code)]
    pub inner: InnerStruct,
}

impl crate::Injection for OuterStruct {
    fn build_injection(injector: &mut crate::Injector) -> Self {
        Self {
            inner: injector.get_new_instance(),
        }
    }
}

#[test]
fn one_type_inject() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_type::<InnerStruct>().build(),
        crate::builders::ContainerBuilder::bind_type::<OuterStruct>().build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<OuterStruct>();

    assert_eq!(obj.inner.text, "test".to_string());
}