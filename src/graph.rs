/* 
   Copyright (c) 2022 ArSysOp.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
  
       http:  www.apache.org/licenses/LICENSE-2.0
  
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
  
   SPDX-License-Identifier: Apache-2.0
  
   Contributors:
     ArSysOp - initial API and implementation
*/

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
    name: Key,
    members: Vec<T>
}

impl<T: Serialize> Serialize for Array<T> {
    fn serialize(&self, mut json: JSONSerializer) -> JSONSerializer {
        if self.members.is_empty() {
            json.render_line_with_bracket(self.name.get_key(), Bracket::RBrace);    
        } else {
            json.render_line_with_bracket(self.name.get_key(), Bracket::LBrace);
            json.prefix.expand();
            let mut first = true;
            for member in &self.members {
                if first {
                    first = false;
                } else {
                    json.render_comma();
                }
                json = member.serialize(json);
            }
            json.prefix.shrink();
            json.render_bracket(Bracket::RBrace);
        }
        json
    }
}

impl<T : Serialize> Array<T> {
    fn new(name: Key, members: Vec<T>) -> Array<T> {
        Array::<T> {
            name: name,
            members: members
        }
    }

    fn push(&mut self, new_member: T) {
        self.members.push(new_member);
    }
}

#[derive(Clone, Debug)]
enum Key {
    Label,
    Kind,
    Name,
    DisplayName,
    Type,
    Usr,
    Location,
    Children,
    Metadata,
    Source,
    Target,
    Edges,
    Nodes,
    Graph,
    CallGraph,
}

impl Key {
    fn get_key(&self) -> &[u8] {
        match self {
            Key::Label => b"label",
            Key::Kind => b"kind",
            Key::Name => b"name",
            Key::DisplayName => b"displayName",
            Key::Type => b"type",
            Key::Usr => b"usr",
            Key::Location => b"location",
            Key::Children => b"children",
            Key::Metadata => b"metadata",
            Key::Source => b"source",
            Key::Target => b"target",
            Key::Edges => b"edges",
            Key::Nodes => b"nodes",
            Key::Graph => b"graph",
            Key::CallGraph => b"call graph",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node<'tu> {
    entity: Entity<'tu>,
    kind: (Key, String),
    name: (Key, String),
    display_name: (Key, String),
    _type: (Key, String),
    usr: (Key, String),
    location: (Key, String),
    children: Array<Node<'tu>>,
    serialize: bool,
    serialize_children: bool
}

impl<'tu> Serialize for Node<'tu> {
    fn serialize(&self, mut json: JSONSerializer) -> JSONSerializer {
        if self.serialize {
            json.render_line_with_bracket(self.get_node_hash().to_string().as_bytes(), Bracket::LCurly);
            json.prefix.expand();
            json.render_line(Key::Label.get_key(), &self.display_name.1.as_bytes());
            json.render_comma();
            json.render_line_with_bracket(Key::Metadata.get_key(), Bracket::LCurly);
            json.prefix.expand();
            for attr in [&self.kind, &self.name, &self.display_name, &self._type, &self.usr, &self.location] {
                json.render_line(attr.0.get_key(), attr.1.as_bytes());
                json.render_comma();
            }
            if self.serialize_children {
                json = self.children.serialize(json);
            }
            json.prefix.shrink();
            json.render_bracket(Bracket::RCurly);
            json.prefix.shrink();
            json.render_bracket(Bracket::RCurly);
        }
        json
    }
}

impl<'a> Node<'a> {
    pub fn new(node: Entity<'a>, mut ast: &'a mut AST<'a>, serialize_children: bool, exclude_dirs: &Vec<String>, system_headers: bool) -> (Node<'a>, &'a mut AST<'a>) {
        let children_as_entities: Vec<Entity> = node.get_children().into_iter().collect::<Vec<_>>();
        let mut children_as_nodes: Vec<Node> = Vec::new();
        for child in children_as_entities {
            if ast.nodes.contains(&child) {
                let child_as_node_wuth_ast = Node::new(ast.nodes.take(&child).unwrap(), ast, serialize_children, exclude_dirs, system_headers);
                ast = child_as_node_wuth_ast.1;
                children_as_nodes.push(child_as_node_wuth_ast.0); 
            }
        }
        (Node { 
            entity: node,
            kind: (Key::Kind, get_kind_label(node.get_kind())), 
            name: (Key::Name, Name::get_name(&node.get_name())), 
            display_name: (Key::DisplayName, node.get_display_name().get_name()),
            _type: (Key::Type, node.get_type().get_name()),
            usr: (Key::Usr, node.get_usr().get_name()),
            location: (Key::Location, node.get_location().get_name()), 
            children: (
                Array::new(
                    Key::Children,
                    children_as_nodes
                )
            ),
            serialize: !AST::should_be_excluded(node.clone(), exclude_dirs) && (!node.is_in_system_header() || system_headers),
            serialize_children: serialize_children
        },
        ast)
    }

    fn get_node_hash(&self) -> u64 {
        get_hash(self.entity)
    }
}

fn get_hash(entity: Entity<'_>) -> u64 {
    let mut hasher = DefaultHasher::new();
    entity.hash(&mut hasher);
    hasher.finish()
}

pub struct Edge<'tu> {
    source: (Key, Entity<'tu>),
    target: (Key, Entity<'tu>),
}

impl Serialize for Edge<'_> {
    fn serialize(&self, mut json: JSONSerializer) -> JSONSerializer {
        json.render_bracket(Bracket::LCurly);
        json.prefix.expand();
        json.render_line(self.source.0.get_key(), get_hash(self.source.1).to_string().as_bytes());
        json.render_comma();
        json.render_line(self.target.0.get_key(), get_hash(self.target.1).to_string().as_bytes());
        json.prefix.shrink();
        json.render_bracket(Bracket::RCurly);
        json
    }
}

impl<'tu> Edge<'tu> {
    fn new (source: Entity<'tu>, target: Entity<'tu>) -> Edge<'tu> {
        Edge { source: (Key::Source, source), target: (Key::Target, target) }
    }
}

pub fn visit_ast<'a, Task: FnMut(Entity<'a>, Entity<'a>) -> EntityVisitResult, Filter: Fn(Entity<'a>) -> bool>
    (parent: Entity<'a>, mut task: Task, filter: &Filter) {
    let task_filtered = |entity: Entity<'a>, parent: Entity<'a>| {
        if filter(entity) {
            task(entity, parent)
        } else {
            EntityVisitResult::Recurse
        }
    };
    parent.visit_children(task_filtered);
}


#[derive(Clone)]
pub struct AST<'tu> {
    _root: Entity<'tu>,
    pub nodes: HashSet<Entity<'tu>>
}

impl<'a> AST<'a> {
    pub fn new(parent: Entity<'a>) -> AST<'a> {
        let mut nodes = HashSet::new();
        let insertion_task = |entity: Entity<'a>, _: Entity<'a>| 
        { nodes.insert(entity); EntityVisitResult::Recurse };
        let filter = |_: Entity<'a>| { true };
        visit_ast(parent, insertion_task, &filter);
        AST { _root: parent, nodes: nodes }
    }
    
    fn should_be_excluded(entity: Entity, exclude_dirs: &Vec<String>) -> bool {
        exclude_dirs
        .into_iter()
        .map(|path| entity.get_location().get_name().contains(path))
        .collect::<Vec<bool>>()
        .contains(&true) || entity.get_location().get_name().contains("include")
    }
}
pub struct CallGraph<'tu> {
    pub label: String,
    pub nodes: HashSet<Entity<'tu>>,
    pub edges: Array<Edge<'tu>>,
    pub source_ast: AST<'tu>
}

impl<'tu> CallGraph<'tu> {
    pub fn new(label: String, ast: AST<'tu>) -> CallGraph<'tu> {
        CallGraph { label: label, nodes: HashSet::new(), edges: Array::new(Key::Edges, Vec::new()), source_ast: ast }
    }

    pub fn take_callable_from_ast(&mut self, ast: AST<'tu>) {
        let mut result = HashSet::new();
        for entity in ast.nodes {
            match entity.get_kind() {
                EntityKind::Method
                | EntityKind::Constructor
                | EntityKind::Destructor
                | EntityKind::FunctionDecl
                | EntityKind::LambdaExpr
                => { result.insert(entity); },
                _ => ()
            }
        }
        self.nodes = result;
    }

    fn get_callee(call_expr: Entity<'tu>) -> Entity<'tu> {
        assert_eq!(call_expr.get_kind(), EntityKind::CallExpr);
        let mut result: Entity<'tu> = call_expr.clone();
        match call_expr.get_reference() {
            Some(callee) => { result = callee; },
            None => {
                // may happen at "throw" stmt
                let filter = |entity: Entity| { entity.get_kind() == EntityKind::CallExpr };
                let get_reference = |inner_call_expr: Entity<'tu>, _: Entity<'tu>| { 
                    match inner_call_expr.get_reference() {
                        Some(callee) => {
                            result = callee;
                            EntityVisitResult::Break
                        },
                        None => EntityVisitResult::Recurse
                    }
                };
                visit_ast(call_expr, get_reference, &filter);
            }
        }
        match result.get_definition() {
            Some(callee_definition) => callee_definition,
            None => result
        }
    }


    pub fn add_callees(&mut self, node: Entity<'tu>) {
        let filter = |entity: Entity| { entity.get_kind() == EntityKind::CallExpr };
        let get_callee = |call_expr: Entity<'tu>, _: Entity<'tu>| { 
            self.edges.push(Edge::new(node.clone(), CallGraph::get_callee(call_expr)));
            EntityVisitResult::Recurse
        };
        visit_ast(node, get_callee, &filter);
    }

    fn serialize_node(node: Entity, mut json: JSONSerializer) -> JSONSerializer {
        json.render_line_with_bracket(get_hash(node).to_string().as_bytes(), Bracket::LCurly);
        json.prefix.expand();
        json.render_line(Key::Label.get_key(), node.get_display_name().get_name().as_bytes());
        json.render_comma();
        json.render_line_with_bracket(Key::Metadata.get_key(), Bracket::LCurly);
        json.prefix.expand();
        json.render_line(Key::Usr.get_key(), node.get_usr().get_name().as_bytes());
        json.render_comma();
        json.render_line(Key::Kind.get_key(), get_kind_label(node.get_kind()).as_bytes());
        json.render_comma();
        json.render_line(Key::Location.get_key(), node.get_location().get_name().as_bytes());
        json.prefix.shrink();
        json.render_bracket(Bracket::RCurly);
        json.prefix.shrink();
        json.render_bracket(Bracket::RCurly);
        json
    }

    fn serialize_nodes(&self, mut json: JSONSerializer) -> JSONSerializer {
        json.render_line_with_bracket(Key::Nodes.get_key(), Bracket::LCurly);
        json.prefix.expand();
        let mut first = true;
        for node in self.nodes.clone() {
            if first {
                first = false;
            } else {
                json.render_comma();
            }
            json = CallGraph::serialize_node(node, json);
        }
        json.prefix.shrink();
        json.render_bracket(Bracket::RCurly);
        json.render_comma();
        json
    }

}

impl<'tu> Serialize for CallGraph<'tu> {
    fn serialize(&self, mut json: JSONSerializer) -> JSONSerializer {
        json.render_bracket(Bracket::LCurly);
        json.prefix.expand();
        json.render_line_with_bracket(Key::Graph.get_key(), Bracket::LCurly);
        json.prefix.expand();
        json.render_line(Key::Label.get_key(), self.label.as_bytes());
        json.render_comma();
        json.render_line(Key::Type.get_key(), Key::CallGraph.get_key());
        json.render_comma();
        json = self.serialize_nodes(json);
        json = self.edges.serialize(json);
        json.prefix.shrink();
        json.render_bracket(Bracket::RCurly);
        json.prefix.shrink();
        json.render_bracket(Bracket::RCurly);
        json
    }
}