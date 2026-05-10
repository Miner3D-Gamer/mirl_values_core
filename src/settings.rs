// /// The map used for object like things (Unordered)
// #[cfg(not(feature = "preserve_entries"))]
// pub type MapType<K, V> = std::collections::BTreeMap<K, V>;
/// The map used for object like things (Ordered)
/// TODO: Make the uniqueness configurable
// #[cfg(feature = "preserve_entries")]
pub type MapType<K, V> = mirl_collections::VecMap<K, V, false>;
