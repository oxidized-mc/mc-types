//! [`ResourceKey`] — a typed reference to a registry entry.
//!
//! Wraps a pair of [`ResourceLocation`]s (registry + location) with a phantom
//! type parameter for compile-time safety against mixing keys from different
//! registries.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::resource_location::ResourceLocation;

/// A typed reference to a registry entry, providing compile-time safety
/// against mixing keys from different registries.
///
/// `T` is a phantom type representing the registry (e.g., `Block`, `Item`).
/// Two `ResourceKey`s are equal only if both their registry and location match.
///
/// Matches vanilla `net.minecraft.resources.ResourceKey`.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::{ResourceKey, ResourceLocation};
///
/// struct Block; // phantom type
/// let key = ResourceKey::<Block>::create(
///     ResourceLocation::minecraft("block"),
///     ResourceLocation::minecraft("stone"),
/// );
/// assert_eq!(key.location().to_string(), "minecraft:stone");
/// ```
pub struct ResourceKey<T> {
    registry: ResourceLocation,
    location: ResourceLocation,
    _marker: PhantomData<fn() -> T>,
}

impl<T> fmt::Debug for ResourceKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResourceKey")
            .field("registry", &self.registry)
            .field("location", &self.location)
            .finish()
    }
}

impl<T> Clone for ResourceKey<T> {
    fn clone(&self) -> Self {
        Self {
            registry: self.registry.clone(),
            location: self.location.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> ResourceKey<T> {
    /// Creates a new resource key for the given registry and location.
    pub fn create(registry: ResourceLocation, location: ResourceLocation) -> Self {
        Self {
            registry,
            location,
            _marker: PhantomData,
        }
    }

    /// The registry this key belongs to.
    #[inline]
    pub fn registry(&self) -> &ResourceLocation {
        &self.registry
    }

    /// The specific entry within the registry.
    #[inline]
    pub fn location(&self) -> &ResourceLocation {
        &self.location
    }

    /// Whether this key belongs to the given registry.
    #[inline]
    pub fn is_for(&self, registry: &ResourceLocation) -> bool {
        self.registry == *registry
    }
}

impl<T> PartialEq for ResourceKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.registry == other.registry && self.location == other.location
    }
}

impl<T> Eq for ResourceKey<T> {}

impl<T> Hash for ResourceKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.registry.hash(state);
        self.location.hash(state);
    }
}

impl<T> fmt::Display for ResourceKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.registry, self.location)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::collections::HashSet;

    use super::*;

    struct Block;

    fn block_registry() -> ResourceLocation {
        ResourceLocation::minecraft("block")
    }

    fn item_registry() -> ResourceLocation {
        ResourceLocation::minecraft("item")
    }

    fn stone() -> ResourceLocation {
        ResourceLocation::minecraft("stone")
    }

    // ── Construction ────────────────────────────────────────────────────

    #[test]
    fn test_resource_key_create() {
        let key = ResourceKey::<Block>::create(block_registry(), stone());
        assert_eq!(key.registry(), &block_registry());
        assert_eq!(key.location(), &stone());
    }

    #[test]
    fn test_resource_key_is_for() {
        let key = ResourceKey::<Block>::create(block_registry(), stone());
        assert!(key.is_for(&block_registry()));
        assert!(!key.is_for(&item_registry()));
    }

    // ── Equality ────────────────────────────────────────────────────────

    #[test]
    fn test_resource_key_equality_same_type() {
        let a = ResourceKey::<Block>::create(block_registry(), stone());
        let b = ResourceKey::<Block>::create(block_registry(), stone());
        assert_eq!(a, b);
    }

    #[test]
    fn test_resource_key_inequality_different_location() {
        let a =
            ResourceKey::<Block>::create(block_registry(), ResourceLocation::minecraft("stone"));
        let b = ResourceKey::<Block>::create(block_registry(), ResourceLocation::minecraft("dirt"));
        assert_ne!(a, b);
    }

    #[test]
    fn test_resource_key_inequality_different_registry() {
        let a = ResourceKey::<Block>::create(block_registry(), stone());
        let b = ResourceKey::<Block>::create(item_registry(), stone());
        assert_ne!(a, b);
    }

    // ── Hash consistency ────────────────────────────────────────────────

    #[test]
    fn test_resource_key_hash_consistent_with_eq() {
        let a = ResourceKey::<Block>::create(block_registry(), stone());
        let b = ResourceKey::<Block>::create(block_registry(), stone());
        let mut set = HashSet::new();
        set.insert(a);
        assert!(set.contains(&b));
    }

    #[test]
    fn test_resource_key_hash_different_keys() {
        let a =
            ResourceKey::<Block>::create(block_registry(), ResourceLocation::minecraft("stone"));
        let b = ResourceKey::<Block>::create(block_registry(), ResourceLocation::minecraft("dirt"));
        let mut set = HashSet::new();
        set.insert(a);
        set.insert(b);
        assert_eq!(set.len(), 2);
    }

    // ── Display ─────────────────────────────────────────────────────────

    #[test]
    fn test_resource_key_display() {
        let key = ResourceKey::<Block>::create(block_registry(), stone());
        let s = format!("{key}");
        assert_eq!(s, "minecraft:block:minecraft:stone");
    }
}
