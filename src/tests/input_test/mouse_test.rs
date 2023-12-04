use crate::systems::input::{Input, Button};
use winit::event::{MouseButton, ElementState};

#[test]
fn button_is_pressed() {
    let mut input = Input::new();
    input.mouse.add_button(
        String::from("Shoot"),
        Button::new(MouseButton::Left)
    );

    input.handle_mb_input(&ElementState::Pressed, &MouseButton::Left);

    assert!(input.button_state(String::from("Shoot")));
}

#[test]
fn button_is_released() {
    let mut input = Input::new();
    input.mouse.add_button(
        String::from("Shoot"),
        Button::new(MouseButton::Right)
    );

    input.handle_mb_input(&ElementState::Released, &MouseButton::Right);

    assert!(!input.button_state(String::from("Shoot")));
}

#[test]
fn add_mouse_hotkey() {
    let mut input = Input::new();
    input.mouse.add_button(
        String::from("Shoot"), 
        Button::new(MouseButton::Left));

    assert!(input.mouse.hotkeys.contains_key(&String::from("Shoot")));
}

#[test]
fn remove_mouse_hotkey() {
    let mut input = Input::new();
    input.mouse.remove_button(String::from("Shoot"));

    assert!(!input.mouse.hotkeys.contains_key(&String::from("Shoot")));
}

#[test]
fn modify_mouse_hotkey() {
    let mut input = Input::new();
    input.mouse.add_button(
        String::from("Shoot"), 
        Button::new(MouseButton::Left));
    input.mouse.modify_button(
        String::from("Shoot"), 
        Button::new(MouseButton::Right));

    let hotkey = input.mouse.hotkeys.get(&String::from("Shoot")).unwrap();
    let test_button = Button::new(MouseButton::Right);

    assert_eq!(*hotkey, test_button);
}