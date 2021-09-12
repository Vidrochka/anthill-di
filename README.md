# anthill-di
Rust di containers system

---
## Warning

Library required Rust nightly

---

## Example

```rust

trait TextGetter {
    fn get(&self) -> String;
}

struct StructWithText {
    text: String,
}

impl crate::Injection for StructWithText {
    fn build_injection(_: &mut crate::Injector) -> Self {
        Self {
            text: "test".to_string(),
        }
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

impl crate::Injection for TextBox {
    fn build_injection(injector: &mut crate::Injector) -> Self {
        Self {
            text_getter: injector.get_new_instance(),
        }
    }
}

fn main() {
    let containers = vec![
        crate::builders::ContainerBuilder::bind_interface::<dyn TextGetter, StructWithText>().build(),
        crate::builders::ContainerBuilder::bind_type::<TextBox>().build(),
    ];

    let injector = crate::Injector::new(containers);

    let obj = injector.lock().unwrap().get_new_instance::<TextBox>();

    assert_eq!(obj.text_getter.get(), "test".to_string());
}

```

#### More shared examples present in tests folder