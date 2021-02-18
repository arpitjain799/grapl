pub use crate::{graph_description::*,
                node_property::Property};

// A helper macro to generate `as_*` methods for the NodeProperty wrapper type, and its internall
// enumeration
macro_rules! impl_as {
    ($base_t:ty, $as_ident:ident, $r:pat, $p:ident, $to_t:ty, $variant:path) => {
        impl $base_t {
            pub fn $as_ident(&self) -> Option<$to_t> {
                match self.property {
                    Some($variant($r)) => Some($p),
                    _ => None,
                }
            }
        }
    };
    ($base_t:ty, $as_ident:ident, &$to_t:ty, $variant:path) => {
        impl_as! {$base_t, $as_ident, ref p, p, &$to_t, $variant}
    };
    ($base_t:ty, $as_ident:ident, $to_t:ty, $variant:path) => {
        impl_as! {$base_t, $as_ident,     p, p, $to_t, $variant}
    };
}

// A helper macro to generate `From` impl boilerplate.
macro_rules ! impl_from_for {
    ($into_t:ty, $field:tt, $from_t:ty) => {
        impl From<$from_t> for $into_t
        {
            fn from(p: $from_t) -> Self {
                let p = p.to_owned().into();
                Self {$field: p}
            }
        }
    };
    ($into_t:ty, $field:tt, $head:ty, $($tail:ty),*) => {
        impl_from_for!($into_t, $field, $head);
        impl_from_for!($into_t, $field, $($tail),*);
    };
    ($from_t:tt, $to_t:ty) => {
        impl From<$from_t> for $to_t {
            fn from($from_t (s): $from_t) -> Self {
                Self:: $from_t (s)
            }
        }
    };
}

pub mod graph_description {
    // TODO: Restructure the Rust modules to better reflect the new
    // Protobuf structure
    include!(concat!(
        env!("OUT_DIR"),
        "/graplinc.grapl.api.graph.v1beta1.rs"
    ));
}

pub use node_property::Property::{DecrementOnlyIntProp as ProtoDecrementOnlyIntProp,
                                  DecrementOnlyUintProp as ProtoDecrementOnlyUintProp,
                                  ImmutableIntProp as ProtoImmutableIntProp,
                                  ImmutableStrProp as ProtoImmutableStrProp,
                                  ImmutableUintProp as ProtoImmutableUintProp,
                                  IncrementOnlyIntProp as ProtoIncrementOnlyIntProp,
                                  IncrementOnlyUintProp as ProtoIncrementOnlyUintProp};

impl GraphDescription {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn add_node(&mut self, node: impl Into<NodeDescription>) {
        let node = node.into();
        match self.nodes.get_mut(&node.node_key) {
            Some(n) => n.merge(&node),
            None => {
                self.nodes.insert(node.clone_node_key(), node);
            }
        };
    }

    pub fn add_edge(
        &mut self,
        edge_name: impl Into<String>,
        from_node_key: impl Into<String>,
        to_node_key: impl Into<String>,
    ) {
        let from_node_key = from_node_key.into();
        let to_node_key = to_node_key.into();
        let edge_name = edge_name.into();
        let edge = Edge {
            from_node_key: from_node_key.clone(),
            to_node_key,
            edge_name,
        };

        let edge_list: &mut Vec<Edge> = &mut self
            .edges
            .entry(from_node_key)
            .or_insert_with(|| EdgeList {
                edges: Vec::with_capacity(1),
            })
            .edges;
        edge_list.push(edge);
    }

    pub fn merge(&mut self, other: &Self) {
        for (node_key, other_node) in other.nodes.iter() {
            match self.nodes.get_mut(node_key) {
                Some(n) => n.merge(other_node),
                None => {
                    self.nodes.insert(node_key.clone(), other_node.clone());
                }
            };
        }

        for edge_list in other.edges.values() {
            for edge in edge_list.edges.iter() {
                self.add_edge(
                    edge.edge_name.clone(),
                    edge.from_node_key.clone(),
                    edge.to_node_key.clone(),
                );
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }
}

impl IdentifiedGraph {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn add_node(&mut self, node: IdentifiedNode) {
        match self.nodes.get_mut(&node.node_key) {
            Some(n) => n.merge(&node),
            None => {
                self.nodes.insert(node.clone_node_key(), node);
            }
        };
    }

    pub fn add_edge(
        &mut self,
        edge_name: impl Into<String>,
        from_node_key: impl Into<String>,
        to_node_key: impl Into<String>,
    ) {
        let from_node_key = from_node_key.into();
        let to_node_key = to_node_key.into();
        let edge_name = edge_name.into();
        let edge = Edge {
            from_node_key: from_node_key.clone(),
            to_node_key,
            edge_name,
        };

        let edge_list: &mut Vec<Edge> = &mut self
            .edges
            .entry(from_node_key)
            .or_insert_with(|| EdgeList {
                edges: Vec::with_capacity(1),
            })
            .edges;
        edge_list.push(edge);
    }

    pub fn merge(&mut self, other: &Self) {
        for (node_key, other_node) in other.nodes.iter() {
            match self.nodes.get_mut(node_key) {
                Some(n) => n.merge(other_node),
                None => {
                    self.nodes.insert(node_key.clone(), other_node.clone());
                }
            };
        }

        for edge_list in other.edges.values() {
            for edge in edge_list.edges.iter() {
                self.add_edge(
                    edge.edge_name.clone(),
                    edge.from_node_key.clone(),
                    edge.to_node_key.clone(),
                );
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }
}

impl MergedGraph {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn add_node(&mut self, node: MergedNode) {
        match self.nodes.get_mut(&node.node_key) {
            Some(n) => n.merge(&node),
            None => {
                self.nodes.insert(node.clone_node_key(), node);
            }
        };
    }

    pub fn add_merged_edge(&mut self, edge: MergedEdge) {
        let from_node_key = edge.from_node_key.clone();
        let edge_list: &mut Vec<MergedEdge> = &mut self
            .edges
            .entry(from_node_key)
            .or_insert_with(|| MergedEdgeList {
                edges: Vec::with_capacity(1),
            })
            .edges;
        edge_list.push(edge);
    }

    pub fn add_edge(
        &mut self,
        edge_name: impl Into<String>,
        from_node_key: impl Into<String>,
        from_uid: impl Into<String>,
        to_node_key: impl Into<String>,
        to_uid: impl Into<String>,
    ) {
        let edge_name = edge_name.into();
        let from_node_key = from_node_key.into();
        let from_uid = from_uid.into();
        let to_node_key = to_node_key.into();
        let to_uid = to_uid.into();

        let edge = MergedEdge {
            from_node_key: from_node_key.clone(),
            from_uid: from_uid.clone(),
            to_node_key,
            to_uid,
            edge_name,
        };

        let edge_list: &mut Vec<MergedEdge> = &mut self
            .edges
            .entry(from_node_key)
            .or_insert_with(|| MergedEdgeList {
                edges: Vec::with_capacity(1),
            })
            .edges;
        edge_list.push(edge);
    }

    pub fn merge(&mut self, other: &Self) {
        for (node_key, other_node) in other.nodes.iter() {
            match self.nodes.get_mut(node_key) {
                Some(n) => n.merge(other_node),
                None => {
                    self.nodes.insert(node_key.clone(), other_node.clone());
                }
            };
        }

        for edge_list in other.edges.values() {
            for edge in edge_list.edges.iter() {
                self.add_edge(
                    edge.edge_name.clone(),
                    edge.from_node_key.clone(),
                    edge.from_uid.clone(),
                    edge.to_node_key.clone(),
                    edge.to_uid.clone(),
                );
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }
}

impl NodeDescription {
    pub fn merge(&mut self, other: &Self) {
        debug_assert_eq!(self.node_type, other.node_type);
        debug_assert_eq!(self.node_key, other.node_key);
        for (prop_name, prop_value) in other.properties.iter() {
            match self.properties.get_mut(prop_name) {
                Some(self_prop) => self_prop.merge(prop_value),
                None => {
                    self.properties
                        .insert(prop_name.clone(), prop_value.clone());
                }
            }
        }
    }
    pub fn get_node_key(&self) -> &str {
        self.node_key.as_str()
    }

    pub fn clone_node_key(&self) -> String {
        self.node_key.clone()
    }
}

impl IdentifiedNode {
    pub fn into_json(self) -> serde_json::Value {
        let mut json_value = serde_json::Value::default();
        for (prop_name, prop_value) in self.properties {
            if let Some(prop_value) = prop_value.property {
                json_value[prop_name] = prop_value.into_json();
            }
        }

        json_value["node_key"] = self.node_key.into();
        json_value["dgraph.type"] = self.node_type.into();

        json_value
    }

    pub fn merge(&mut self, other: &Self) {
        debug_assert_eq!(self.node_type, other.node_type);
        debug_assert_eq!(self.node_key, other.node_key);
        for (prop_name, prop_value) in other.properties.iter() {
            match self.properties.get_mut(prop_name) {
                Some(self_prop) => self_prop.merge(prop_value),
                None => {
                    self.properties
                        .insert(prop_name.clone(), prop_value.clone());
                }
            }
        }
    }
}

impl NodeProperty {
    pub fn merge(&mut self, other: &Self) {
        match (self.property.as_mut(), other.property.as_ref()) {
            (
                Some(ProtoIncrementOnlyUintProp(ref mut self_prop)),
                Some(ProtoIncrementOnlyUintProp(ref other_prop)),
            ) => {
                *self_prop = std::cmp::max(*self_prop, *other_prop);
            }
            (
                Some(ProtoImmutableUintProp(ref mut self_prop)),
                Some(ProtoImmutableUintProp(ref other_prop)),
            ) => {
                debug_assert_eq!(*self_prop, *other_prop);
            }
            (
                Some(ProtoDecrementOnlyUintProp(ref mut self_prop)),
                Some(ProtoDecrementOnlyUintProp(ref other_prop)),
            ) => {
                *self_prop = std::cmp::min(*self_prop, *other_prop);
            }
            (
                Some(ProtoDecrementOnlyIntProp(ref mut self_prop)),
                Some(ProtoDecrementOnlyIntProp(ref other_prop)),
            ) => {
                *self_prop = std::cmp::min(*self_prop, *other_prop);
            }
            (
                Some(ProtoIncrementOnlyIntProp(ref mut self_prop)),
                Some(ProtoIncrementOnlyIntProp(ref other_prop)),
            ) => {
                *self_prop = std::cmp::max(*self_prop, *other_prop);
            }
            (
                Some(ProtoImmutableIntProp(ref mut self_prop)),
                Some(ProtoImmutableIntProp(ref other_prop)),
            ) => {
                debug_assert_eq!(*self_prop, *other_prop);
            }
            (
                Some(ProtoImmutableStrProp(ref mut self_prop)),
                Some(ProtoImmutableStrProp(ref other_prop)),
            ) => {
                debug_assert_eq!(*self_prop, *other_prop);
            }
            (None, op) => {
                debug_assert!(false, "Unhandled property merge, self is None: {:?}", op);
                tracing::warn!("Unhandled property merge, self is None: {:?}", op);
            }
            (p, None) => {
                debug_assert!(false, "Unhandled property merge, other is None: {:?}", p);
                tracing::warn!("Unhandled property merge, other is None: {:?}", p);
            }
            // technically we could improve type safety here by exhausting the combinations,
            // but I'm not going to type that all out right now
            (p, op) => {
                debug_assert!(false, "Unhandled property merge: {:?} {:?}", p, op);
                tracing::warn!("Unhandled property merge: {:?} {:?}", p, op);
            }
        }
    }
}

impl From<Static> for IdStrategy {
    fn from(strategy: Static) -> IdStrategy {
        IdStrategy {
            strategy: Some(id_strategy::Strategy::Static(strategy)),
        }
    }
}

impl From<Session> for IdStrategy {
    fn from(strategy: Session) -> IdStrategy {
        IdStrategy {
            strategy: Some(id_strategy::Strategy::Session(strategy)),
        }
    }
}

impl std::string::ToString for NodeProperty {
    fn to_string(&self) -> String {
        let prop = match &self.property {
            Some(node_property::Property::IncrementOnlyUintProp(increment_only_uint_prop)) => {
                increment_only_uint_prop.to_string()
            }
            Some(node_property::Property::ImmutableUintProp(immutable_uint_prop)) => {
                immutable_uint_prop.to_string()
            }
            Some(node_property::Property::DecrementOnlyUintProp(decrement_only_uint_prop)) => {
                decrement_only_uint_prop.to_string()
            }
            Some(node_property::Property::DecrementOnlyIntProp(decrement_only_int_prop)) => {
                decrement_only_int_prop.to_string()
            }
            Some(node_property::Property::IncrementOnlyIntProp(increment_only_int_prop)) => {
                increment_only_int_prop.to_string()
            }
            Some(node_property::Property::ImmutableIntProp(immutable_int_prop)) => {
                immutable_int_prop.to_string()
            }
            Some(node_property::Property::ImmutableStrProp(immutable_str_prop)) => {
                immutable_str_prop.to_string()
            }
            None => panic!("Invalid property : {:?}", self),
        };
        prop
    }
}

impl Property {
    pub fn into_json(self) -> serde_json::Value {
        match self {
            ProtoIncrementOnlyUintProp(increment_only_uint_prop) => increment_only_uint_prop.into(),
            ProtoImmutableUintProp(immutable_uint_prop) => immutable_uint_prop.into(),
            ProtoDecrementOnlyUintProp(decrement_only_uint_prop) => decrement_only_uint_prop.into(),
            ProtoDecrementOnlyIntProp(decrement_only_int_prop) => decrement_only_int_prop.into(),
            ProtoIncrementOnlyIntProp(increment_only_int_prop) => increment_only_int_prop.into(),
            ProtoImmutableIntProp(immutable_int_prop) => immutable_int_prop.into(),
            ProtoImmutableStrProp(immutable_str_prop) => immutable_str_prop.into(),
        }
    }
}

impl NodeDescription {
    pub fn get_property(&self, name: impl AsRef<str>) -> Option<&NodeProperty> {
        self.properties.get(name.as_ref())
    }

    pub fn set_property(&mut self, name: impl Into<String>, value: impl Into<NodeProperty>) {
        self.properties.insert(name.into(), value.into().into());
    }

    pub fn set_key(&mut self, key: String) {
        self.node_key = key;
    }
}

impl<T> From<T> for NodeProperty
where
    T: Into<Property>,
{
    fn from(t: T) -> Self {
        NodeProperty {
            property: Some(t.into()),
        }
    }
}

impl From<NodeDescription> for IdentifiedNode {
    fn from(n: NodeDescription) -> Self {
        // todo: Remove any properties that were used *only* for identification purposes
        IdentifiedNode {
            properties: n.properties,
            node_key: n.node_key,
            node_type: n.node_type,
        }
    }
}

impl MergedNode {
    pub fn from(n: IdentifiedNode, uid: u64) -> Self {
        Self {
            uid,
            properties: n.properties,
            node_key: n.node_key,
            node_type: n.node_type,
        }
    }

    pub fn merge(&mut self, other: &Self) {
        debug_assert_eq!(self.node_type, other.node_type);
        debug_assert_eq!(self.node_key, other.node_key);
        for (prop_name, prop_value) in other.properties.iter() {
            match self.properties.get_mut(prop_name) {
                Some(self_prop) => self_prop.merge(prop_value),
                None => {
                    self.properties
                        .insert(prop_name.clone(), prop_value.clone());
                }
            }
        }
    }

    pub fn get_node_key(&self) -> &str {
        self.node_key.as_str()
    }

    pub fn clone_node_key(&self) -> String {
        self.node_key.clone()
    }
}

impl IdentifiedNode {
    pub fn into(self, uid: u64) -> MergedNode {
        MergedNode {
            uid,
            properties: self.properties,
            node_key: self.node_key,
            node_type: self.node_type,
        }
    }
    pub fn get_node_key(&self) -> &str {
        self.node_key.as_str()
    }

    pub fn clone_node_key(&self) -> String {
        self.node_key.clone()
    }
}

// We use separate types here because it makes a lot of the codegen easier
pub struct IncrementOnlyIntProp(pub i64);

pub struct DecrementOnlyIntProp(pub i64);

pub struct ImmutableIntProp(pub i64);

pub struct IncrementOnlyUintProp(pub u64);

pub struct DecrementOnlyUintProp(pub u64);

pub struct ImmutableUintProp(pub u64);

pub struct ImmutableStrProp(pub String);

impl_from_for!(
    ImmutableUintProp,
    0,
    u64,
    u32,
    u16,
    u8,
    &u64,
    &u32,
    &u16,
    &u8
);
impl_from_for!(
    IncrementOnlyUintProp,
    0,
    u64,
    u32,
    u16,
    u8,
    &u64,
    &u32,
    &u16,
    &u8
);
impl_from_for!(
    DecrementOnlyUintProp,
    0,
    u64,
    u32,
    u16,
    u8,
    &u64,
    &u32,
    &u16,
    &u8
);
impl_from_for!(
    ImmutableIntProp,
    0,
    i64,
    i32,
    i16,
    i8,
    &i64,
    &i32,
    &i16,
    &i8
);
impl_from_for!(
    IncrementOnlyIntProp,
    0,
    i64,
    i32,
    i16,
    i8,
    &i64,
    &i32,
    &i16,
    &i8
);
impl_from_for!(
    DecrementOnlyIntProp,
    0,
    i64,
    i32,
    i16,
    i8,
    &i64,
    &i32,
    &i16,
    &i8
);
impl_from_for!(ImmutableStrProp, 0, String, &String, &str);
impl_from_for!(ImmutableStrProp, Property);
impl_from_for!(IncrementOnlyIntProp, Property);
impl_from_for!(DecrementOnlyIntProp, Property);
impl_from_for!(ImmutableIntProp, Property);
impl_from_for!(IncrementOnlyUintProp, Property);
impl_from_for!(DecrementOnlyUintProp, Property);
impl_from_for!(ImmutableUintProp, Property);

impl_as! {NodeProperty, as_increment_only_uint, u64, node_property::Property::IncrementOnlyUintProp}
impl_as! {NodeProperty, as_immutable_uint, u64, node_property::Property::ImmutableUintProp}
impl_as! {NodeProperty, as_decrement_only_uint, u64, node_property::Property::DecrementOnlyUintProp}
impl_as! {NodeProperty, as_decrement_only_int, i64, node_property::Property::DecrementOnlyIntProp}
impl_as! {NodeProperty, as_increment_only_int, i64, node_property::Property::IncrementOnlyIntProp}
impl_as! {NodeProperty, as_immutable_int, i64, node_property::Property::ImmutableIntProp}
impl_as! {NodeProperty, as_immutable_str, &str, node_property::Property::ImmutableStrProp }
