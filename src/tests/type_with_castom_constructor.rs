use tokio::runtime::Runtime;

struct StructWithCustomConstuctor {
    #[allow(dead_code)]
    string: String,
}

impl crate::Injection for StructWithCustomConstuctor {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            string: injector.get_new_instance()?
        })
    }
}

#[test]
fn type_with_castom_constructor() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_type()
                .to_constructor(|_| -> _ { Ok(StructWithCustomConstuctor {string: "test".to_string()}) } ).build(),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<StructWithCustomConstuctor>().unwrap();
        assert_eq!(obj.string, "test".to_string());

        let obj = injector.write().await.get_singletone::<StructWithCustomConstuctor>().unwrap();
        assert_eq!(obj.read().await.string, "test".to_string());
    });
}

#[test]
fn unconfigured_type_with_castom_constructor() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_unconfigured_type()
                .build_with_constructor(|_| -> _ { Ok(StructWithCustomConstuctor {string: "test".to_string()}) } ),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<StructWithCustomConstuctor>().unwrap();
        assert_eq!(obj.string, "test".to_string());

        let obj = injector.write().await.get_singletone::<StructWithCustomConstuctor>().unwrap();
        assert_eq!(obj.read().await.string, "test".to_string());
    });
}