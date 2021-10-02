use tokio::runtime::Runtime;

trait TextGetter : Send + Sync {
    fn get(&self) -> String;
}
struct StructWithText {
    text: String,
}

impl crate::Injection for StructWithText {
    fn build_injection(_: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            text: "test".to_string(),
        })
    }
}

impl TextGetter for StructWithText {
    fn get(&self) -> String {
        self.text.clone()
    }
}

struct TextBox {
    #[allow(dead_code)]
    text_getter: Box<dyn TextGetter>,
}

impl crate::Injection for TextBox {
    fn build_injection(injector: &mut crate::Injector) -> Result<Self, crate::DiError> {
        Ok(Self {
            text_getter: injector.get_new_instance()?,
        })
    }
}

#[test]
fn one_trait_inject() {
    let rt  = Runtime::new().unwrap();  

    rt.block_on(async {
        let containers = vec![
            crate::builders::ContainerBuilder::bind_interface::<dyn TextGetter, StructWithText>().build(),
            crate::builders::ContainerBuilder::bind_type::<TextBox>().build(),
        ];

        let injector = crate::Injector::new(containers).await;

        let obj = injector.write().await.get_new_instance::<TextBox>().unwrap();

        assert_eq!(obj.text_getter.get(), "test".to_string());
    });
}
