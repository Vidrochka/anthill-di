[![build & tests](https://github.com/Vidrochka/anthill-di/actions/workflows/rust.yml/badge.svg)](https://github.com/Vidrochka/anthill-di/actions/workflows/rust.yml)

# anthill-di

Rust ioc system

The library is for deep tree of dependencies

Advantages:

* two work mode async (with optional blocking fn versions)/sync
* sync/async constructors (async ctor only in async mode)
* parallel building
* runtime injection
* runtime check dependency cycles [optional by default ```loop-check``` feature]
* 3 type life cycle (transient/singleton/context dependent)
* 3 injection way: simple trait constructor, async/sync closure as constructor, instance (async closure only in async mode)
* full displayed debug info, just print the context (you can explore more information with addition ```debug-type-info``` feature)
* flexibility: you can add or delete components/services on any context nesting, check component/service existence

Deficiencies:

* runtime check dependency cycles take some time for synchronize (you can remove ```loop-check``` feature)
* async building take some time for synchronize (async is optional, for disable set ```default-features = false```)
* non global context, just tree structure with root/child context

---

## Warning

Library required Rust nightly

---

## Basic concepts

The components source is DependencyContext

``` rust
/* creating root DependencyContext */

fn _() {
    let root_context = DependencyContext::new_root()
}
```

---

There are several ways to register components

``` rust
/* all way for component registration */

async fn _() {
    //let root_context = DependencyContext::new_root()

    // adds component, which implement Constructor trait
    root_context.register_type::<SomeComponent>(DependencyLifeCycle::Transient).await.unwrap();

    // adds component from closure
    root_context.register_closure(|_| Ok(SomeComponent {}), DependencyLifeCycle::Transient).await.unwrap();

    // adds component from async closure
    root_context.register_async_closure(
        move |_: crate::DependencyContext| { async move { Ok(SomeComponent {}) }},
        DependencyLifeCycle::Transient
    ).await.unwrap();

    // adds an existing component
    root_context.register_instance(RwLock::new(instance)).await.unwrap();
}
```

---

Each type of component registration (except instance registration) takes 3 life times    
Instance registration is always a Singleton

```rust
/* Life times */

async fn _() {
    //let root_context = DependencyContext::new_root()

    // instance per call
    root_context.register_type::<SomeComponent>(DependencyLifeCycle::Transient).await.unwrap();
    // single instance
    root_context.register_type::<SomeComponent>(DependencyLifeCycle::Singleton).await.unwrap();
    // instance per local context
    root_context.register_type::<SomeComponent>(DependencyLifeCycle::ContextDependent).await.unwrap();
}

```

---

To register a component through a type, you need to implement Constructor trait

``` rust
/* Constructor implementation */

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SomeComponent {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok( Self { } )
    }
}
```

---

Components are given a unique context when they are built    
You can store the context in a struct field and get dependencies anywhere in the component    
Nested (child) dependencies can be requested from the context

``` rust
/* Resolve nested service */

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SomeComponent {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok( Self {
            nested_service1: ctx.resolve().await?,
            nested_service2: ctx.resolve().await?
        } )
    }
}
```

---

Components context is the same as the root context, which means it can register dependencies

``` rust
/* register new dependency */

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SomeComponent {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        ctx.register_type::<SomeComponent2>(DependencyLifeCycle::Transient).await
            .map_err(|e| BuildDependencyError::AddDependencyError { err: e })?;

        Ok( Self {
            nested_service1: ctx.resolve::<SomeComponent2>().await?,
            nested_service2: ctx.resolve().await?
        } )
    }
}
```

---

You can resolve the ```first``` (by TypeId), all matching dependencies as ```vector```, or by ```TypeId```

``` rust
/* dependency resolving way */

async fn _() {
    //let root_context = DependencyContext::new_root()
    //root_context.register_type::<SomeComponent>(DependencyLifeCycle::Transient).await.unwrap();

    // return first (by TypeId) component
    let mut dependency = root_context.resolve::<SomeComponent>().await.unwrap();

    // return all match as Vector<SomeComponent> (look at service mappings section)
    let mut dependency_vector = root_context.resolve_collection::<Box<dyn SomeTrait>>().await.unwrap();

    // return service with component by type_id
    root_context.resolve_by_type_id::<Box<dyn GetStr>>(TypeId::of::<TransientDependency>()).await.unwrap()
}

```

---

Each life time resolve in a different way

``` rust
/* dependency resolving match */

async fn _() {
    //let root_context = DependencyContext::new_root()
    //root_context.register_type::<SomeComponent1>(DependencyLifeCycle::Transient).await.unwrap();
    //root_context.register_type::<SomeComponent2>(DependencyLifeCycle::Singleton).await.unwrap();
    //root_context.register_type::<SomeComponent3>(DependencyLifeCycle::ContextDependent).await.unwrap();

    // resolve transient
    let mut dependency = root_context.resolve::<SomeComponent1>().await.unwrap();

    // resolve singleton
    let mut dependency2 = root_context.resolve::<Arc<SomeComponent2>>().await.unwrap();

    // resolve local context dependency
    let mut dependency3 = root_context.resolve::<Weak<SomeComponent3>>().await.unwrap();

    // To get a mutable singleton you need to register with RwLock/Lock
    // Constructor trait implemented by default for tokio::sync::RwLock<T>, std::sync::RwLock<T>
}
```

---

You can map a component to a service    
By default, only the component's mapping to itself is created    
You are not limited in the number of mappings

``` rust
/* component to service mapping */

async fn _() {
    //let root_context = DependencyContext::new_root()

    // mapping at creation time
    root_context.register_type::<SomeComponent>(DependencyLifeCycle::Transient).await.unwrap()
        .map_as::<dyn SomeImplementedTrait1>().await.unwrap();

    // map after creation
    root_context.map_component::<SomeComponent, dyn SomeImplementedTrait2>().await.unwrap();
}
```

---

Service is resolved in Box\<T\>

``` rust
/* lifetime resolving */

async fn _() {
    //let root_context = DependencyContext::new_root()
    //root_context.register_type::<SomeComponent>(DependencyLifeCycle::Transient).await.unwrap()
    //   .map_as::<dyn SomeImplementedTrait>().await.unwrap();

    let service = root_context.resolve::<Box<dyn SomeImplementedTrait>>().await.unwrap();
}
```

---

Context dependent components live until all Arc of local context are removed    
Child instance contain local context of parent    

If the parent changes local context before the child instance is created, the child instance will be created with the new parent local context

You can always create a new local context, or set an old one

``` rust
/* local context manipulation */

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SomeComponent {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        // take old local context, we can save it for using two local context
        let old_context = ctx.get_context();

        //instance from old local context
        let instance1 = ctx.resolve::<SomeInstance>().await?;

        // return new local context after create
        let new_context = ctx.set_empty_context();

        //instance from new local context
        let instance2 = ctx.resolve::<SomeInstance>().await?;

        // set old local context
        ctx.set_context(old_context);

        //instance from old local context
        let instance3 = ctx.resolve::<SomeInstance>().await?;

        Ok( Self { } )
    }
}
```

---

You can delete Transient and Singleton components

``` rust
/* delete component */

async fn _() {
    //let root_context = DependencyContext::new_root()
    //root_context.register_type::<SomeComponent>(DependencyLifeCycle::Transient).await.unwrap()
    //   .map_as::<dyn SomeImplementedTrait>().await.unwrap();

    root_context.delete_component::<SomeComponent>().await.unwrap();
}
```

---

You can check component/service existence

``` rust
/* check service */

async fn _() {
    //let root_context = DependencyContext::new_root()
    //root_context.register_type::<SomeComponent>(DependencyLifeCycle::Transient).await.unwrap()
    //   .map_as::<dyn SomeImplementedTrait>().await.unwrap();

    let is_exist = root_context.is_component_exist::<TransientDependency>().await;
    
    let is_exist = root_context.is_component_with_type_id_exist(TypeId::of::<TransientDependency>()).await;

    let is_exist = root_context.is_service_exist::<Box<dyn SomeImplementedTrait>>().await;

    let is_exist = root_context.is_service_with_type_id_exist(TypeId::of::<Box<dyn SomeImplementedTrait>>()).await;
}
```

---

Global context verifies link of the requested dependencies and return error in case of a circular dependency    
If the check is successful, all subsequent requests for this pair link will not check for cycling
You can disable this behavior with ```loop-check``` feature

---

You can debug inner state
If you add ```debug-type-info``` feature output will contain some addition type_info fields

``` rust
async fn _() {
    let root_context = DependencyContext::new_root()
    root_context.register_type::<SomeComponent>(DependencyLifeCycle::Transient).await.unwrap()
       .map_as::<dyn SomeImplementedTrait>().await.unwrap();

    println!("{root_context:#?}");
}

```
---

## Basic example

```rust


use anthill_di::{
    Constructor,
    types::BuildDependencyResult,
    DependencyContext,
    DependencyLifeCycle
};

struct TransientDependency1 {
    pub d1: TransientDependency2,
    pub d2: TransientDependency2,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve().await?,
            d2: ctx.resolve().await?,
        })
    }
}

struct TransientDependency2 {
    pub str: String,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency2 {
    async fn ctor(_: DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::main]
fn main() {
    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(DependencyLifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency2>(DependencyLifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().await.unwrap();

    assert_eq!(dependency.d1.str, "test".to_string());
    assert_eq!(dependency.d2.str, "test".to_string());
}
```

### More shared examples present in src/tests folder

---
## Benchmarks

```Test on i5 6500, 24gb```

|settings|transient set|transient get*|transient delete|singleton set|singleton get*|singleton delete|context dependent set|context dependent get*|
|--|--|--|--|--|--|--|--|--|
|sync mode|~2,1us|~5,9us|~1,9us|~2,1us|~4,5us|~2us|~2us|~4,9us|
|sync mode + loop-check|~2,2us|~6us|~2us|~2,3us|~4,8us|~2,5us|~2,3us|~5us|
|async mode|~1,6us|~6,4us|~2.3us|~1,8us|~4,8us|~2,4us|~1,8us|~5,3us|
|async mode + loop-check|~1,8us|~6,4us|~2,6us|~2us|~4,8us|~2,7us|~2us|~5,3us|

\* first request check cycling and then save result as checked, next requests +/- equal no loop-check versions

---

## Features

* ```loop-check``` - check cycled resolve [enable by default]
* ```debug-type-info``` - add some addition ```TypeInfo``` fields, for extended debug display
* ```async-mode``` - switch to async mode, if disable, all function will be no async [enable by default]
* ```blocking``` - add ```blocking_``` function versions, required ```async-mode```

---

## Little architecture overview

1. Register dependency + how construct type + lifecycle
    *automatically generate Component + LifecycleBuilder + fake Service as component to component*
2. Register addition Service component to implemented trait (may be later something else like closure buildings)
3. Request dependency
 *validate link -> take first Service (or collection in future) -> call LifecycleBuilder by TypeId from Service -> LifecycleBuilder build Component as CycledInstance (empty/with Arc/with Weak) -> call Service with CycledInstance -> return Service*

---

#### Refs

* [crate.io](https://crates.io/crates/anthill-di)
