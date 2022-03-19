#[derive(PartialEq, Clone, Debug)]
pub enum DependencyLifeCycle {
    Transient,
    Singleton,
    Scoped,
}