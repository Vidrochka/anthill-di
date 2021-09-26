use tokio::runtime::Runtime;

struct StructWithCustomConstuctorAndInstance {
    #[allow(dead_code)]
    string: String,
}

impl crate::Injection for StructWithCustomConstuctorAndInstance {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            string: injector.get_new_instance()?
        })
    }
}

#[test]
fn type_with_castom_constructor_and_instance() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_type()
                .to_constructor(|_| -> _ { Ok(StructWithCustomConstuctorAndInstance {string: "test 1".to_string()}) } )
                .to_value(StructWithCustomConstuctorAndInstance {string: "test 2".to_string()}).build(),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<StructWithCustomConstuctorAndInstance>().unwrap();
        assert_eq!(obj.string, "test 1".to_string());

        let obj = injector.write().await.get_singletone::<StructWithCustomConstuctorAndInstance>().unwrap();
        assert_eq!(obj.read().await.string, "test 2".to_string());
    });
}

#[test]
fn unconfigured_type_with_castom_constructor_and_instance() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_unconfigured_type()
                .build_with_constructor_and_value(
                    StructWithCustomConstuctorAndInstance {string: "test 2".to_string()},
                    |_| -> _ { Ok(StructWithCustomConstuctorAndInstance {string: "test 1".to_string()}) }
                )
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<StructWithCustomConstuctorAndInstance>().unwrap();
        assert_eq!(obj.string, "test 1".to_string());

        let obj = injector.write().await.get_singletone::<StructWithCustomConstuctorAndInstance>().unwrap();
        assert_eq!(obj.read().await.string, "test 2".to_string());
    });
}