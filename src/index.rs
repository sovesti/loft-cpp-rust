use clang::Entity;
use std::collections::HashSet;

fn deref_in_set(set: HashSet<&String>) -> HashSet<String> {
    let mut derefed: HashSet<String> = HashSet::new();
    for element in set {
        derefed.insert(element.to_string());
    }
    derefed
}

pub fn collect_entities(parent: Entity) -> HashSet<String> {
    let mut entities = HashSet::new();
    let parent_usr = parent.get_usr();
    match parent_usr {
        Some(usr) => entities.insert(usr.0),
        None => entities.insert(String::from("")),
    };
    for child in parent.get_children() {
        entities = deref_in_set(entities.union(&collect_entities(child)).collect());
    }
    entities
}