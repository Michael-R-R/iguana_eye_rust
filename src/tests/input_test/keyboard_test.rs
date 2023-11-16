use crate::sys::input::{Input, Key};
use winit::event::{VirtualKeyCode, ModifiersState, KeyboardInput, ElementState};

#[test]
fn key_is_pressed_w_modifier() {
    let mut input = Input::new();
    input.keyboard.add_hotkey(
        String::from("Forward"),
        Key::new(VirtualKeyCode::W, ModifiersState::SHIFT)
    );
    
    let modifier = ModifiersState::SHIFT;
    let key_input = KeyboardInput {
        state: ElementState::Pressed,
        virtual_keycode: Some(VirtualKeyCode::W),
        scancode: 0,
        modifiers: ModifiersState::empty()
    };
    input.handle_modifiers(&modifier);
    input.handle_kb_input(&key_input);

    assert!(input.key_state(String::from("Forward")));
}

#[test]
fn key_is_pressed_wo_modifier() {
    let mut input = Input::new();
    input.keyboard.add_hotkey(
        String::from("Forward"),
        Key::new(VirtualKeyCode::W, ModifiersState::empty())
    );
    
    let modifier = ModifiersState::empty();
    let key_input = KeyboardInput {
        state: ElementState::Pressed,
        virtual_keycode: Some(VirtualKeyCode::W),
        scancode: 0,
        modifiers: ModifiersState::empty()
    };
    input.handle_modifiers(&modifier);
    input.handle_kb_input(&key_input);

    assert!(input.key_state(String::from("Forward")));
}

#[test]
fn key_is_released_w_modifier() {
    let mut input = Input::new();
    input.keyboard.add_hotkey(
        String::from("Forward"),
        Key::new(VirtualKeyCode::W, ModifiersState::SHIFT)
    );
    
    let modifier = ModifiersState::SHIFT;
    let key_input = KeyboardInput {
        state: ElementState::Released,
        virtual_keycode: Some(VirtualKeyCode::W),
        scancode: 0,
        modifiers: ModifiersState::empty()
    };
    input.handle_modifiers(&modifier);
    input.handle_kb_input(&key_input);

    assert!(!input.key_state(String::from("Forward")));
}

#[test]
fn key_is_released_wo_modifier() {
    let mut input = Input::new();
    input.keyboard.add_hotkey(
        String::from("Forward"),
        Key::new(VirtualKeyCode::W, ModifiersState::empty())
    );
    
    let modifier = ModifiersState::empty();
    let key_input = KeyboardInput {
        state: ElementState::Released,
        virtual_keycode: Some(VirtualKeyCode::W),
        scancode: 0,
        modifiers: ModifiersState::empty()
    };
    input.handle_modifiers(&modifier);
    input.handle_kb_input(&key_input);

    assert!(!input.key_state(String::from("Forward")));
}

#[test]
fn add_keyboard_hotkey() {
    let mut input = Input::new();
    input.keyboard.add_hotkey(
        String::from("Forward"),
        Key::new(VirtualKeyCode::W, ModifiersState::empty())
    );

    assert!(input.keyboard.hotkeys.contains_key(&String::from("Forward")));
}

#[test]
fn remove_keyboard_hotkey() {
    let mut input = Input::new();
    input.keyboard.add_hotkey(
        String::from("Forward"),
        Key::new(VirtualKeyCode::W, ModifiersState::empty())
    );
    input.keyboard.remove_hotkey(String::from("Forward"));

    assert!(!input.keyboard.hotkeys.contains_key(&String::from("Forward")));
}

#[test]
fn modify_keyboard_hotkey() {
    let mut input = Input::new();
    input.keyboard.add_hotkey(
        String::from("Forward"),
        Key::new(VirtualKeyCode::W, ModifiersState::empty())
    );
    input.keyboard.modify_hotkey(
        String::from("Forward"), 
        Key::new(VirtualKeyCode::A, ModifiersState::SHIFT));

    let hotkey = input.keyboard.hotkeys.get(&String::from("Forward")).unwrap();
    let test_key = Key::new(VirtualKeyCode::A, ModifiersState::SHIFT);

    assert_eq!(*hotkey, test_key);
}