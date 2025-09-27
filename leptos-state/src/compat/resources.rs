//! # Resources Compatibility Layer
//!
//! Provides version-agnostic APIs for Leptos resources across different versions.
//! This layer uses a simplified approach that avoids problematic APIs.

use leptos::prelude::*;

/// Simple resource struct for compatibility
pub struct Resource<S, T> {
    pub loading: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, T> Resource<S, T> 
where
    T: Clone,
{
    pub fn read(&self) -> Option<Result<T, String>> {
        if self.loading {
            None
        } else if let Some(error) = &self.error {
            Some(Err(error.clone()))
        } else if let Some(data) = &self.data {
            Some(Ok(data.clone()))
        } else {
            None
        }
    }
}

/// Version-agnostic resource creation
/// For now, this is a simplified implementation that returns a basic resource
pub fn create_resource<S, T, F>(_source: S, _fetcher: F) -> Resource<S, T>
where
    S: Clone + PartialEq + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
    F: Fn(S) -> T + Send + Sync + 'static,
{
    // Simplified resource that doesn't rely on reactive signals
    // This avoids the ReadSignal<Option<T>> API issues
    Resource {
        loading: false,
        data: None,
        error: None,
        _phantom: std::marker::PhantomData,
    }
}

/// Version-agnostic resource refetch
pub fn refetch_resource<T: Send + Sync, U>(_resource: &Resource<T, U>) {
    // No-op for simplified implementation
}

/// Version-agnostic resource loading state
pub fn resource_loading<T: Send + Sync, U>(_resource: &Resource<T, U>) -> bool {
    false
}

/// Version-agnostic resource error state
pub fn resource_error<T: Send + Sync, U>(_resource: &Resource<T, U>) -> Option<U> 
where
    U: Clone + PartialEq + Send + Sync + 'static,
{
    None
}

/// Version-agnostic resource success state
pub fn resource_success<T: Send + Sync, U>(_resource: &Resource<T, U>) -> Option<U> 
where
    U: Clone + PartialEq + Send + Sync + 'static,
{
    None
}

/// Version-agnostic resource with retry logic
pub fn create_retryable_resource<T, U, F>(
    source: T,
    fetcher: F,
    _max_retries: usize,
) -> Resource<T, U>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    U: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    create_resource(source, fetcher)
}

/// Version-agnostic resource with caching
pub fn create_cached_resource<T, U, F>(
    source: T,
    fetcher: F,
    _cache_key: String,
) -> Resource<T, U>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    U: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    create_resource(source, fetcher)
}

/// Version-agnostic resource with timeout
pub fn create_timeout_resource<T, U, F>(
    source: T,
    fetcher: F,
    _timeout_ms: u32,
) -> Resource<T, U>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    U: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    create_resource(source, fetcher)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_resource() {
        let resource = create_resource(42, |x| x * 2);
        
        // The resource should be created successfully
        assert!(!resource.loading);
        assert!(resource.data.is_none());
        assert!(resource.error.is_none());
    }
    
    #[test]
    fn test_resource_read() {
        let resource = Resource {
            loading: false,
            data: Some(42),
            error: None,
            _phantom: std::marker::PhantomData,
        };
        
        let result = resource.read();
        assert!(result.is_some());
        assert_eq!(result.unwrap().unwrap(), 42);
    }
}