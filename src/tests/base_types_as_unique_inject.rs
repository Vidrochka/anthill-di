use tokio::runtime::Runtime;

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
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(StructWithValueIngection {
            string: injector.get_new_instance()?,
            string_ref: injector.get_new_instance()?,
            u_number: injector.get_new_instance()?,
            number: injector.get_new_instance()?,
        })
    }
}

#[test]
fn base_types_as_unique_inject() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_type::<StructWithValueIngection>().build(),
            crate::builders::ContainerBuilder::bind_type::<String>().to_constructor(|_| -> _ { Ok("test".to_string()) }).build(),
            crate::builders::ContainerBuilder::bind_type::<&str>().to_constructor(|_| -> _ { Ok("test 2") }).build(),
            crate::builders::ContainerBuilder::bind_type::<u32>().build(),
            crate::builders::ContainerBuilder::bind_type::<i32>().to_constructor(|_| -> _ { Ok(1) }).build(),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<StructWithValueIngection>().unwrap();

        assert_eq!(obj.string, "test".to_string());
        assert_eq!(obj.string_ref, "test 2".to_string());
        assert_eq!(obj.u_number, 0);
        assert_eq!(obj.number, 1);
    });
}