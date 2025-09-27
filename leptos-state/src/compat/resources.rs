//! # Resources Compatibility Layer
//!
//! Provides version-agnostic APIs for Leptos resources across different versions.
//! This layer uses direct imports for better reliability.

use leptos::prelude::*;

/// Simple resource struct for compatibility
pub struct Resource<S, T> {
    pub loading: ReadSignal<bool>,
    pub data: ReadSignal<Option<T>>,
    pub error: ReadSignal<Option<String>>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, T> Resource<S, T> {
    pub fn read(&self) -> Option<Result<T, String>> {
        if self.loading.get() {
            None
        } else if let Some(error) = self.error.get() {
            Some(Err(error))
        } else if let Some(data) = self.data.get() {
            Some(Ok(data))
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
    // For now, create a simple resource using the available Leptos API
    // This is a placeholder implementation that will need to be updated
    // based on the actual Leptos 0.8.9 resource API
    let (loading, _) = signal(true);
    let (data, _) = signal(None::<T>);
    let (error, _) = signal(None::<String>);
    
    Resource {
        loading,
        data,
        error,
    }
}

/// Version-agnostic resource refetch
pub fn refetch_resource<T: Send + Sync, U>(_resource: &Resource<T, U>) {
    // Resource refetch API has changed significantly in Leptos 0.7
    // For now, this is a no-op. In a real implementation, you'd need
    // to handle the different APIs properly.
}

/// Version-agnostic resource loading state
pub fn resource_loading<T: Send + Sync, U>(_resource: &Resource<T, U>) -> ReadSignal<bool> {
    // Resource loading API has changed significantly in Leptos 0.7
    // For now, return a simple signal. In a real implementation, you'd need
    // to handle the different APIs properly.
    let (loading, _) = signal(false);
    loading
}

/// Version-agnostic resource error state
pub fn resource_error<T: Send + Sync, U>(_resource: &Resource<T, U>) -> ReadSignal<Option<U>> 
where
    U: Clone + PartialEq + Send + Sync + 'static,
{
    // Resource API has changed significantly in Leptos 0.7
    // For now, return a simple signal. In a real implementation, you'd need
    // to handle the different APIs properly.
    let (error, _) = signal(None::<U>);
    error
}

/// Version-agnostic resource success state
pub fn resource_success<T: Send + Sync, U>(_resource: &Resource<T, U>) -> ReadSignal<Option<U>> 
where
    U: Clone + PartialEq + Send + Sync + 'static,
{
    // Resource API has changed significantly in Leptos 0.7
    // For now, return a simple signal. In a real implementation, you'd need
    // to handle the different APIs properly.
    let (success, _) = signal(None::<U>);
    success
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
    // For now, just create a regular resource
    // In a real implementation, you'd add retry logic here
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
    // For now, just create a regular resource
    // In a real implementation, you'd add caching logic here
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
    // For now, just create a regular resource
    // In a real implementation, you'd add timeout logic here
    create_resource(source, fetcher)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_local_resource() {
        let resource = create_local_resource(|| {
            Box::pin(async move {
                // Simulate async work
                42
            })
        });
        
        // The resource should be created successfully
        assert!(resource.loading().get());
    }
    
    #[test]
    fn test_resource_loading_state() {
        let resource = create_local_resource(|| {
            Box::pin(async move {
                42
            })
        });
        
        let loading = resource_loading(&resource);
        assert!(loading.get());
    }
}
