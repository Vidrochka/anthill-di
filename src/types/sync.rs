#[cfg(feature = "async-mode")]
pub (crate) type AnthillRwLock<T> = tokio::sync::RwLock<T>;

#[cfg(not(feature = "async-mode"))]
pub (crate) type AnthillRwLock<T> = std::sync::RwLock<T>;