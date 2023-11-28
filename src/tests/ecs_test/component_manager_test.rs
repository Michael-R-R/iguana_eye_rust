use std::any::TypeId;
use crate::system::ecs::component_manager::{*, component::{*, name_component::NameComponent}};

#[test]
fn has_component() {
    let cm = ComponentManager::new();

    assert!(cm.has::<name_component::NameComponent>());
}

#[test]
fn insert_component() {
    todo!()
}

#[test]
fn remove_component() {
    todo!()
}

#[test]
fn get_component() {
    let mut cm = ComponentManager::new();

    let nc = cm.get::<name_component::NameComponent>().unwrap();
    assert_ne!(TypeId::of::<ComponentManager>(), nc.as_any().type_id());
    assert_eq!(TypeId::of::<name_component::NameComponent>(), nc.as_any().type_id());
}