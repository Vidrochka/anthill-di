use tokio::runtime::Runtime;

struct StructWithCustomInstance {
    #[allow(dead_code)]
    string: String,
}

impl crate::Injection for StructWithCustomInstance {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            string: injector.get_new_instance()?
        })
    }
}

#[test]
fn type_with_castom_instance() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_type().to_value(StructWithCustomInstance {string: "test".to_string()}).build(),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_singletone::<StructWithCustomInstance>().unwrap();
        assert_eq!((*obj.read().await).string, "test".to_string());

        let obj = injector.write().await.get_new_instance::<StructWithCustomInstance>().unwrap();
        assert_eq!(obj.string, "".to_string());

        let obj = injector.write().await.get_singletone::<StructWithCustomInstance>().unwrap();
        assert_eq!(obj.read().await.string, "test".to_string());
    });
}

#[test]
fn unconfigured_type_with_castom_instance_singletone_instance() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_unconfigured_type()
                .build_with_value(StructWithCustomInstance {string: "test".to_string()}),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_singletone::<StructWithCustomInstance>().unwrap();
        assert_eq!(obj.read().await.string, "test".to_string());
    });
}

#[test]
fn unconfigured_type_with_castom_instance_new_instance_panic() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_unconfigured_type()
                .build_with_value(StructWithCustomInstance {string: "test".to_string()}),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<StructWithCustomInstance>();
        assert_eq!(obj.err(), Some(crate::DiError::ConstructorNotDefined{type_name: std::any::type_name::<StructWithCustomInstance>().to_string()}));
    });
}