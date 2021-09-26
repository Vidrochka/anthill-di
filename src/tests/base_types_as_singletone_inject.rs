use std::sync::Arc;

use tokio::{runtime::Runtime, sync::RwLock};

struct StructWithSingletoneValueIngection {
    #[allow(dead_code)]
    string: Arc<RwLock<String>>,
}

impl crate::Injection for StructWithSingletoneValueIngection {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            string: injector.get_singletone()?,
        })
    }
}

struct StructWithSingletoneValueWrapper {
    #[allow(dead_code)]
    struct_with_singletone_1: StructWithSingletoneValueIngection,
    #[allow(dead_code)]
    struct_with_singletone_2: StructWithSingletoneValueIngection,
}

impl crate::Injection for StructWithSingletoneValueWrapper {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            struct_with_singletone_1: injector.get_new_instance()?,
            struct_with_singletone_2: injector.get_new_instance()?,
        })
    }
}

#[test]
fn base_types_singletone_inject() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_type::<StructWithSingletoneValueIngection>()
                .build(),
            crate::builders::ContainerBuilder::bind_type::<StructWithSingletoneValueWrapper>().build(),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<StructWithSingletoneValueWrapper>().unwrap();

        *obj.struct_with_singletone_1.string.write().await = "tested singletone".to_string();
        
        assert_eq!(
            *obj.struct_with_singletone_2.string.read().await,
            "tested singletone".to_string()
        );
    });
}
