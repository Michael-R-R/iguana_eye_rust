use crate::system::ecs::{entity_manager::EntityManager, entity::Entity};


#[test]
fn create_entities() {
    let mut em = EntityManager::new();

    for i in 1..=10 {
        let e = em.create();
        assert_eq!(i as u64, e.id);
    }

    assert_eq!(10, em.count());
}

#[test]
fn remove_entities() {
    let mut em = EntityManager::new();

    for i in 1..=10 {
        let e = em.create();
        assert_eq!(i as u64, e.id);
    }
    assert_eq!(10, em.count());

    for i in 1..=10 {
        let e = Entity::new(i);
        assert!(em.remove(e));
    }
    assert_eq!(0, em.count());
}

#[test]
fn attach_components() {
    let mut em = EntityManager::new();
    
    let e = em.create();
    for i in 1..=10 {
        assert!(em.attach_component(e, i));
    }

    for i in 1..=10 {
        assert!(em.has_component(e, i));
    }

    assert!(!em.has_component(e, 100));
}

#[test]
fn detach_components() {
    let mut em = EntityManager::new();

    let e = em.create();
    for i in 1..=10 {
        assert!(em.attach_component(e, i));
    }

    for i in 1..=10 {
        assert!(em.detach_component(e, i));
    }

    let attached = em.get_attached(e).unwrap();
    assert_eq!(0, attached.len());
}