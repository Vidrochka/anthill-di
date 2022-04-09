# anthill-di

Rust ioc system

The library is for deep tree of dependencies

Advantages:

* async constructors (parallel building)
* runtime injection
* runtime check dependency cycles
* 3 type life cycle (transient/singleton/scoped)
* 3 injection way (simple trait constructor, trait as interface for type with constructor, closure as constructor)
* extensible dependency injection logic

Deficiencies:

* runtime check dependency cycles take some time for synchronize
* async building take some time for synchronize

---

## Warning

Library required Rust nightly for trait as interface (Unsize)

---

## Little overview

1. Register dependency + how construct type + lifecycle
    *automatically generate Component + LifecycleBuilder + fake Service as component to component*
2. Register addition Service component to implemented trait (maby later something else like closure buildings)
3. Request dependency
 *validate link -> take first Service (or collection in future) -> call LifecycleBuilder by TypeId from Service -> LifecycleBuilder build Component as CycledInstance (empty/with Arc/with Weak) -> call Service with CycledInstance -> return Service*

---

## Example

```rust

use async_trait::async_trait;
use tokio::runtime::Runtime;
use async_trait::async_trait;

use anthill_di::{
    Constructor,
    types::BuildDependencyResult,
    DependencyContext,
    DependencyLifeCycle
};

struct TransientDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for TransientDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for TransientDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[tokio::main]
fn main() {
    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(DependencyLifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();
    
    let dependency = root_context.resolve::<Box<dyn GetStr>>().await.unwrap();

    assert_eq!(dependency.get(), "test".to_string());
}

```

### More shared examples present in src/tests folder

---

#### Refs

* [crate.io](https://crates.io/crates/anthill-di)