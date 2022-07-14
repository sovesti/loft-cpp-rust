use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use crate::json::JSONSerializer;
use crate::json::Bracket;
use crate::kind::get_kind_label;
use clang::*;
use crate::get_name::*;

pub trait Serialize {
    fn serialize(&self, json: JSONSerializer) -> JSONSerializer;
}

#[derive(Clone, Debug)]
pub struct Array<T: Serialize> {
    name: String,
    members: Vec<T>
}

impl<T: Serialize> Serialize for Array<T> {
    fn serialize(&self, mut json: JSONSerializer) -> JSONSerializer {
        json.render_line_without_value(self.name.as_bytes());
        json.render_bracket(Bracket::LBrace);
        json.prefix.expand();
        for member in &self.members {
            json = member.serialize(json);
        }
        json.prefix.shrink();
        json.render_bracket(Bracket::RBrace);
        json
    }
}

#[derive(Clone, Debug)]
enum Key {
    Kind,
    Name,
    DisplayName,
    Type,
    Usr,
    Location,
    Children,
}

impl Key {
    fn get_key(&self) -> &[u8] {
        match self {
            Key::Kind => b"kind",
            Key::Name => b"name",
            Key::DisplayName => b"displayName",
            Key::Type => b"type",
            Key::Usr => b"usr",
            Key::Location => b"location",
            Key::Children => b"children",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    kind: (Key, String),
    name: (Key, String),
    display_name: (Key, String),
    // _type: (Key, String),
    usr: (Key, String),
    location: (Key, String),
    children: Array<Node>
}

impl Serialize for Node {
    fn serialize(&self, mut json: JSONSerializer) -> JSONSerializer {
        json.render_bracket(Bracket::LCurly);
        json.prefix.expand();
        for attr in [&self.kind, &self.name, &self.display_name, /* &self._type, */ &self.usr, &self.location] {
            json.render_line(attr.0.get_key(), attr.1.as_bytes());
        }
        json = self.children.serialize(json);
        json.prefix.shrink();
        json.render_bracket(Bracket::RCurly);
        json
    }
}

impl Node {
    pub fn new(node: Entity) -> Node {
        let children_as_entities: Vec<Entity> = node.get_children().into_iter().collect::<Vec<_>>();
        let mut children_as_nodes: Vec<Node> = Vec::new();
        for child in children_as_entities {
                children_as_nodes.push(Node::new(child)); 
        }
        Node { 
            kind: (Key::Kind, get_kind_label(node.get_kind())), 
            name: (Key::Name, return_empty_if_null(node.get_name())), 
            display_name: (Key::DisplayName, return_empty_if_null(node.get_display_name())),
            // _type: (Key::Type, node.get_type().get_name()),
            usr: (Key::Usr, node.get_usr().get_name()),
            location: (Key::Location, node.get_location().get_name()), 
            children: (Array { 
                name: (String::from_utf8(Key::Children.get_key().to_vec()).expect("failed to make string from &[u8]")), 
                members: (children_as_nodes) 
            }), 
        }
    }
}

pub fn visit_ast<'a, F: FnMut(Entity<'a>)>(parent: Entity<'a>, mut task_and_registry: (F, HashSet<u64>)) -> (F, HashSet<u64>) {
    task_and_registry.0(parent);
    let mut hasher = DefaultHasher::new();
    parent.hash(&mut hasher);
    if task_and_registry.1.insert(hasher.finish()) {
        for child in parent.get_children() {
            task_and_registry = visit_ast(child, task_and_registry);
        }
    }
    task_and_registry
}

#[derive(Clone)]
pub struct AST<'tu> {
    root: Entity<'tu>,
    pub nodes: HashSet<Entity<'tu>>
}

impl<'a> AST<'a> {
    pub fn new(parent: Entity<'a>) -> AST<'a> {
        let mut nodes = HashSet::new();
        let insertion_task = |entity: Entity<'a>| { nodes.insert(entity); };
        visit_ast(parent, (insertion_task, HashSet::new()));
        AST { root: parent, nodes: nodes }
    }
}