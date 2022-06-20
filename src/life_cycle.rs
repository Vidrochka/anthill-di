#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LifeCycle {
    Transient,
    Singleton,
    ContextDependent,
}