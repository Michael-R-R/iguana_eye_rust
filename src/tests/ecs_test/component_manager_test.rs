use std::any::TypeId;
use crate::system::ecs::component_manager::{*, component::{*, name_component, hierarchy_component}};

#[test]
fn add_component() {
    let mut cm = ComponentManager::new();

    assert!(cm.add(Box::new(name_component::NameComponent::new())).is_ok());
    assert!(cm.add(Box::new(name_component::NameComponent::new())).is_err());
}

#[test]
fn insert_component() {
    let mut cm = ComponentManager::new();
    _ = cm.add(Box::new(name_component::NameComponent::new()));

    assert!(cm.insert(0, Box::new(hierarchy_component::HierarchyComponent::new())).is_ok());
    assert_eq!(0, cm.find_index::<hierarchy_component::HierarchyComponent>().unwrap());
    assert_eq!(1, cm.find_index::<name_component::NameComponent>().unwrap());
}

#[test]
fn remove_component() {
    let mut cm = ComponentManager::new();
    _ = cm.add(Box::new(name_component::NameComponent::new()));
    _ = cm.add(Box::new(hierarchy_component::HierarchyComponent::new()));

    assert!(cm.remove::<name_component::NameComponent>().is_ok());
    assert_eq!(0, cm.find_index::<hierarchy_component::HierarchyComponent>().unwrap());
}

#[test]
fn get_component() {
    let mut cm = ComponentManager::new();
    _ = cm.add(Box::new(name_component::NameComponent::new()));

    let nc = cm.get::<name_component::NameComponent>().unwrap();
    assert_ne!(TypeId::of::<ComponentManager>(), nc.as_any().type_id());
    assert_eq!(TypeId::of::<name_component::NameComponent>(), nc.as_any().type_id());
}