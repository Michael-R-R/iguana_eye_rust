use crate::systems::ecs::{component_manager::component::{hierarchy_component, Componentable}, entity::Entity};

#[test]
fn attach_component() {
    let mut hc = hierarchy_component::HierarchyComponent::new();

    let count = 10;
    for i in 1..=count {
        let e = Entity::new(i as u64);
        assert!(hc.attach(e).is_ok());
    }

    assert_eq!(count, hc.component.entities.len());
}

#[test]
fn detach_component() {
    let mut hc = hierarchy_component::HierarchyComponent::new();

    let count = 10;
    for i in 1..=count {
        let e = Entity::new(i as u64);
        assert!(hc.attach(e).is_ok());
    }

    for i in 1..=count {
        let e = Entity::new(i as u64);
        assert!(hc.detach(e).is_ok());
    }

    assert!(hc.is_empty());
}

#[test]
fn add_child() {
    let mut hc = hierarchy_component::HierarchyComponent::new();

    let e = Entity::new(1);
    let p_index = hc.attach(e).unwrap();
    for i in 2..=11 {
        let e = Entity::new(i);
        let c_index = hc.attach(e).unwrap();
        assert!(hc.add_child(p_index, c_index).is_ok());
    }

    let c_list = hc.get_children(p_index).unwrap();
    assert_eq!(10, c_list.len());
    for i in 2..=11 {
        let e = Entity::new(i);
        assert!(c_list.contains(&e));
    }
}

#[test]
fn move_child() {
    let mut hc = hierarchy_component::HierarchyComponent::new();

    let e = Entity::new(1);
    let p1_index = hc.attach(e).unwrap();

    let e = Entity::new(2);
    let p2_index = hc.attach(e).unwrap();

    let c = Entity::new(3);
    let c_index = hc.attach(c).unwrap();

    {
        assert!(hc.add_child(p1_index, c_index).is_ok());
        let c_list = hc.get_children(p1_index).unwrap();
        assert!(c_list.contains(&c));
    }

    {
        assert!(hc.move_child(p1_index, p2_index, c_index).is_ok());
        let c1_list = hc.get_children(p1_index).unwrap();
        let c2_list = hc.get_children(p2_index).unwrap();

        assert_eq!(0, c1_list.len());
        assert!(c2_list.contains(&c));
    }
}

#[test]
fn remove_child() {
    let mut hc = hierarchy_component::HierarchyComponent::new();

    let e = Entity::new(1);
    let p_index = hc.attach(e).unwrap();
    for i in 2..=11 {
        let e = Entity::new(i);
        let c_index = hc.attach(e).unwrap();
        assert!(hc.add_child(p_index, c_index).is_ok());
    }

    for i in 2..=11 {
        let e = Entity::new(i);
        let c_index = hc.component.find_index(&e).unwrap();
        assert!(hc.remove_child(p_index, c_index).is_ok());
    }

    let c_list = hc.get_children(p_index).unwrap();
    assert_eq!(0, c_list.len());
}