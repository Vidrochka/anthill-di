use tokio::runtime::Runtime;

trait TextGetter {
    fn get(&self) -> String;
}

struct StructWithCustomConstuctor {
    #[allow(dead_code)]
    string: String,
}

impl TextGetter for StructWithCustomConstuctor {
    fn get(&self) -> String {
        self.string.clone()
    }
}

impl crate::Injection for StructWithCustomConstuctor {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            string: injector.get_new_instance()?
        })
    }
}

#[test]
fn trait_with_castom_constructor() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_interface::<dyn TextGetter, StructWithCustomConstuctor>()
                .to_constructor(|_| -> _ { Ok(Box::new(StructWithCustomConstuctor {string: "test".to_string()})) } ).build(),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<Box<dyn TextGetter>>().unwrap();
        assert_eq!(obj.get(), "test".to_string());

        let obj = injector.write().await.get_singletone::<Box<dyn TextGetter>>().unwrap();
        assert_eq!(obj.read().await.get(), "test".to_string());
    });
}

#[test]
fn unconfigured_trait_with_castom_constructor() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_unconfigured_interface::<dyn TextGetter, StructWithCustomConstuctor>()
                .build_with_constructor(|_| -> _ { Ok(Box::new(StructWithCustomConstuctor {string: "test".to_string()})) } ),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<Box<dyn TextGetter>>().unwrap();
        assert_eq!(obj.get(), "test".to_string());

        let obj = injector.write().await.get_singletone::<Box<dyn TextGetter>>().unwrap();
        assert_eq!(obj.read().await.get(), "test".to_string());
    });
}