use cgmath::num_traits::sign;

use crate::system::ecs::component_manager::component::{name_component, Componentable};
use crate::system::ecs::entity::Entity;
use crate::util::hash;

#[test]
fn add_entities() {
    let mut nc = name_component::NameComponent::new();

    let count = 10;
    for i in 1..=count {
        let e = Entity::new(i as u64);
        let _ = nc.attach(e).unwrap();
    }

    let size = nc.component.entities.len();

    assert_eq!(count, size)
}

#[test]
fn remove_entities() {
    let mut nc = name_component::NameComponent::new();

    let count = 10;
    for i in 1..=count {
        let e = Entity::new(i as u64);
        let _ = nc.attach(e).unwrap();
    }

    for i in 1..=count {
        let e = Entity::new(i as u64);
        let _ = nc.detach(e);
    }

    let size = nc.component.entities.len();
    assert_eq!(0, size)
}

#[test]
fn entity_swap_matches() {
    let mut nc = name_component::NameComponent::new();

    let count = 3;
    for i in 1..=count {
        let e = Entity::new(i as u64);
        let _ = nc.attach(e).unwrap();
    }

    let index = nc.component.entities[&Entity::new(3)];
    assert_eq!(2, index);

    let _ = nc.detach(Entity::new(1));
    let index = nc.component.entities[&Entity::new(3)];

    assert_eq!(0, index)
}

#[test]
fn name_matches() {
    let mut nc = name_component::NameComponent::new();

    let hash = hash::hasher(&String::from("name"));
    let e = Entity::new(1 as u64);
    let index = nc.attach(e).unwrap();
    nc.set_name(index, String::from("name"));
    let pair = nc.get_name(index).unwrap();
    assert_eq!(hash, pair.0);
    assert_eq!(String::from("name"), pair.1);

    let hash = hash::hasher(&String::from("name_0"));
    let e = Entity::new(2 as u64);
    let index = nc.attach(e).unwrap();
    nc.set_name(index, String::from("name"));
    let pair = nc.get_name(index).unwrap();
    assert_eq!(hash, pair.0);
    assert_eq!(String::from("name_0"), pair.1);
}

#[test]
fn set_name() {
    let mut nc = name_component::NameComponent::new();

    let count = 10;
    for i in 1..=count {
        let e = Entity::new(i as u64);
        let index = nc.attach(e).unwrap();

        assert!(!nc.set_name(index+1, String::from("temp_name")));
        assert!(nc.set_name(index, String::from("temp_name")));
    }
}

#[test]
fn add_tag() {
    let mut nc = name_component::NameComponent::new();

    let count = 10;
    for i in 1..=count {
        let e = Entity::new(i as u64);
        let index = nc.attach(e).unwrap();

        assert!(nc.add_tag(index, String::from("player")));
        assert!(nc.has_tag(index, String::from("player")));
    }
}

#[test]
fn remove_tag() {
    let mut nc = name_component::NameComponent::new();

    let count = 10;
    for i in 1..=count {
        let e = Entity::new(i as u64);
        let index = nc.attach(e).unwrap();

        assert!(nc.add_tag(index, String::from("player")));
        assert!(nc.remove_tag(index, String::from("player")));
        assert!(!nc.has_tag(index, String::from("player")));
    }
}