use crate::{settings::MapType, value::ValueType};


#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// Container variants that hold inner values of type `V`
#[derive(Debug, Clone)]
pub enum ContainerValue<Value> {
    /// List
    Vec(Vec<Value>),
    /// Key: Value
    Map(MapType<Value, Value>),
}

impl<V> ContainerValue<V> {
    #[must_use]
    /// Gets the [`ValueType`] of the current item
    pub const fn get_value_type(&self) -> ValueType {
        match self {
            Self::Vec(_) => ValueType::Vec,
            Self::Map(_) => ValueType::Map,
        }
    }
}
impl<Value: PartialOrd + Eq> PartialOrd for ContainerValue<Value> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Vec(a), Self::Vec(b)) => a.partial_cmp(b),
            (Self::Map(a), Self::Map(b)) => Some(a.len().cmp(&b.len())),
            (Self::Vec(_), Self::Map(_)) => Some(std::cmp::Ordering::Less),
            (Self::Map(_), Self::Vec(_)) => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl<Value: Ord + Eq> Ord for ContainerValue<Value> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Vec(a), Self::Vec(b)) => a.cmp(b),
            (Self::Map(a), Self::Map(b)) => a.len().cmp(&b.len()),
            (Self::Vec(_), Self::Map(_)) => std::cmp::Ordering::Less,
            (Self::Map(_), Self::Vec(_)) => std::cmp::Ordering::Greater,
        }
    }
}
impl<Value: Eq + PartialEq> PartialEq for ContainerValue<Value> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Vec(items1), Self::Vec(items2)) => items1.eq(items2),
            (Self::Map(index_map1), Self::Map(index_map2)) => {
                index_map1.eq(index_map2)
            }
            _ => false,
        }
    }
}
impl<V: Eq + PartialEq> Eq for ContainerValue<V> {}

// ── Hash for ContainerValue ───────────────────────────────────────────────────

impl<V: std::hash::Hash + Ord> std::hash::Hash for ContainerValue<V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            Self::Vec(v) => v.hash(state),
            Self::Map(m) => {
                let mut items: Vec<_> = m.iter().collect();
                items.sort_by(|a, b| a.0.cmp(b.0));
                for (k, v) in items {
                    k.hash(state);
                    v.hash(state);
                }
            }
        }
    }
}
