trait TextGetter {
    fn get(&self) -> String;
}

struct StructWithCustomInstance {
    #[allow(dead_code)]
    string: String,
}

impl TextGetter for StructWithCustomInstance {
    fn get(&self) -> String {
        self.string.clone()
    }
}

impl crate::Injection for StructWithCustomInstance {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            string: injector.get_new_instance()?
        })
    }
}

#[test]
fn trait_with_castom_instance() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_interface::<dyn TextGetter, StructWithCustomInstance>()
            .to_value(Box::new(StructWithCustomInstance {string: "test".to_string()})).build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.write().unwrap().get_singletone::<Box<dyn TextGetter>>().unwrap();
    assert_eq!((*obj.read().unwrap()).get(), "test".to_string());

    let obj = injector.write().unwrap().get_new_instance::<Box<dyn TextGetter>>().unwrap();
    assert_eq!(obj.get(), "".to_string());

    let obj = injector.write().unwrap().get_singletone::<Box<dyn TextGetter>>().unwrap();
    assert_eq!((*obj.read().unwrap()).get(), "test".to_string());
}

#[test]
fn unconfigured_trait_with_castom_instance_singletone_instance() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_unconfigured_interface::<dyn TextGetter, StructWithCustomInstance>()
            .build_with_value(Box::new(StructWithCustomInstance {string: "test".to_string()})),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.write().unwrap().get_singletone::<Box<dyn TextGetter>>().unwrap();
    assert_eq!((*obj.read().unwrap()).get(), "test".to_string());
}

#[test]
#[should_panic]
fn unconfigured_trait_with_castom_instance_new_instance_panic() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_unconfigured_interface::<dyn TextGetter, StructWithCustomInstance>()
            .build_with_value(Box::new(StructWithCustomInstance {string: "test".to_string()})),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.write().unwrap().get_new_instance::<Box<dyn TextGetter>>().unwrap();
    assert_eq!(obj.get(), "test".to_string());
}