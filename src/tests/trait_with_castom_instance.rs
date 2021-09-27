use tokio::runtime::Runtime;

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
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_interface::<dyn TextGetter, StructWithCustomInstance>()
                .to_value(Box::new(StructWithCustomInstance {string: "test".to_string()})).build(),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_singletone::<Box<dyn TextGetter>>().unwrap();
        assert_eq!((*obj.read().await).get(), "test".to_string());

        let obj = injector.write().await.get_new_instance::<Box<dyn TextGetter>>().unwrap();
        assert_eq!(obj.get(), "".to_string());

        let obj = injector.write().await.get_singletone::<Box<dyn TextGetter>>().unwrap();
        assert_eq!((*obj.read().await).get(), "test".to_string());
    });
}

#[test]
fn unconfigured_trait_with_castom_instance_singletone_instance() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_unconfigured_interface::<dyn TextGetter, StructWithCustomInstance>()
                .build_with_value(Box::new(StructWithCustomInstance {string: "test".to_string()})),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_singletone::<Box<dyn TextGetter>>().unwrap();
        assert_eq!((*obj.read().await).get(), "test".to_string());
    });
}

#[test]
fn unconfigured_trait_with_castom_instance_new_instance_panic() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_unconfigured_interface::<dyn TextGetter, StructWithCustomInstance>()
                .build_with_value(Box::new(StructWithCustomInstance {string: "test".to_string()})),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<Box<dyn TextGetter>>();
        assert_eq!(obj.err(), Some(crate::DiError::ConstructorNotDefined{type_name: std::any::type_name::<Box<dyn TextGetter>>().to_string()}));
    });
}