//! Cached async store implementation

use super::async_store_core::AsyncStore;
use crate::utils::StateResult;
use std::marker::PhantomData;

/// Cached async store that persists data between loads
pub struct CachedAsyncStore<A: AsyncStore> {
    _cache_key: String,
    _phantom: PhantomData<A>,
}

impl<A: AsyncStore> CachedAsyncStore<A> {
    pub fn new(cache_key: String) -> Self {
        Self {
            _cache_key: cache_key,
            _phantom: PhantomData,
        }
    }
}

#[cfg(feature = "persist")]
impl<A: AsyncStore> CachedAsyncStore<A>
where
    A::LoaderOutput: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    /// Load with caching support
    pub async fn load_cached(&self, input: A::LoaderInput) -> StateResult<A::LoaderOutput> {
        // Try to load from cache first
        if let Ok(cached_data) =
            crate::store::load_from_storage::<A::LoaderOutput>(&self._cache_key)
        {
            return Ok(cached_data);
        }

        // Load from network/async source
        let data = A::load(input).await?;

        // Cache the result
        if let Err(e) = crate::store::save_to_storage(&self._cache_key, &data) {
            tracing::warn!("Failed to cache async store data: {:?}", e);
        }

        Ok(data)
    }
}
