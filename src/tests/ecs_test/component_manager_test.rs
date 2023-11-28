use std::any::TypeId;

use crate::system::ecs::component_manager::{ComponentManager, component::{name_component::NameComponent, Componentable}};

#[test]
fn get_component() {
    let mut cm = ComponentManager::new();
    let nc = NameComponent::new();

    cm.add::<NameComponent>(Box::new(nc));

    let nc = cm.get::<NameComponent>().unwrap();

    assert_ne!(TypeId::of::<ComponentManager>(), nc.as_any().type_id());
    assert_eq!(TypeId::of::<NameComponent>(), nc.as_any().type_id());
}