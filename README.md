# anthill-di
Rust di containers system

---
## Warning

Library required Rust nightly

---

## Example

```rust

use anthill_di::{
    builders::ContainerBuilder,
    Injector,
    Injection,
    DiError
};

trait TextGetter {
    fn get(&self) -> String;
}

struct StructWithText {
    text: String,
}

impl Injection for StructWithText {
    fn build_injection(_: &mut Injector) -> Result<Self,DiError> {
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
    text_getter: Box<dyn TextGetter>,
}

impl Injection for TextBox {
    fn build_injection(injector: &mut Injector) -> Result<Self,DiError> {
        Ok(Self {
            text_getter: injector.get_new_instance()?,
        })
    }
}

fn main() {
    let containers = vec![
        ContainerBuilder::bind_interface::<dyn TextGetter, StructWithText>().build(),
        ContainerBuilder::bind_type::<TextBox>().build(),
    ];

    let injector = Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<TextBox>().unwrap();

    assert_eq!(obj.text_getter.get(), "test".to_string());
}

```

#### More shared examples present in tests folder