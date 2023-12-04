use crate::systems::ecs::{ECS, entity::Entity};

#[test]
fn create_entity() {
    let mut ecs = ECS::new();

    for i in 1..=50 {
        let e = ecs.create_entity().unwrap();
        assert_eq!(i, e.id);
        assert!(ecs.does_entity_exist(e));
    }
}

#[test]
fn remove_entity() {
    let mut ecs = ECS::new();

    for _ in 1..=50 {
        _ = ecs.create_entity().unwrap();
    }

    for i in 1..=50 {
        assert!(ecs.remove_entity(Entity::new(i)).is_ok())
    }

    assert_eq!(0, ecs.count());
}