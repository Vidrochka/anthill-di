trait TextGetter {
    fn get(&self) -> String;
}

struct StructWithCustomConstuctorAndInstance {
    #[allow(dead_code)]
    string: String,
}

impl TextGetter for StructWithCustomConstuctorAndInstance {
    fn get(&self) -> String {
        self.string.clone()
    }
}

impl crate::Injection for StructWithCustomConstuctorAndInstance {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            string: injector.get_new_instance()?
        })
    }
}

#[test]
fn trait_with_castom_constructor_and_instance() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_interface::<dyn TextGetter, StructWithCustomConstuctorAndInstance>()
            .to_constructor(|_| -> _ { Ok(Box::new(StructWithCustomConstuctorAndInstance {string: "test 1".to_string()})) } )
            .to_value(Box::new(StructWithCustomConstuctorAndInstance {string: "test 2".to_string()})).build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.write().unwrap().get_new_instance::<Box<dyn TextGetter>>().unwrap();
    assert_eq!(obj.get(), "test 1".to_string());

    let obj = injector.write().unwrap().get_singletone::<Box<dyn TextGetter>>().unwrap();
    assert_eq!(obj.read().unwrap().get(), "test 2".to_string());
}

#[test]
fn unconfigured_trait_with_castom_constructor_and_instance() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_unconfigured_interface::<dyn TextGetter, StructWithCustomConstuctorAndInstance>()
            .build_with_constructor_and_value(
                Box::new(StructWithCustomConstuctorAndInstance {string: "test 2".to_string()}),
                |_| -> _ { Ok(Box::new(StructWithCustomConstuctorAndInstance {string: "test 1".to_string()})) }
            )
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.write().unwrap().get_new_instance::<Box<dyn TextGetter>>().unwrap();
    assert_eq!(obj.get(), "test 1".to_string());

    let obj = injector.write().unwrap().get_singletone::<Box<dyn TextGetter>>().unwrap();
    assert_eq!(obj.read().unwrap().get(), "test 2".to_string());
}