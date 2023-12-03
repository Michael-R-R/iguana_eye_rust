use std::time::Duration;

use imgui::*;
use imgui::{Context, ConfigFlags, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;
use winit::{window::Window, dpi::PhysicalSize};
use winit::event::{KeyboardInput, ModifiersState, MouseButton, Event, ElementState};
use crate::app::{Viewport, Frame};
use crate::system::game::Game;
use crate::util::file;

pub struct UI {
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    dockspace_flags: WindowFlags
}

impl UI {
    pub fn new(window: &Window, viewport: &Viewport) -> Self {
        let mut imgui = imgui::Context::create();
        imgui.io_mut().config_flags = 
            ConfigFlags::DOCKING_ENABLE |
            ConfigFlags::VIEWPORTS_ENABLE;
        imgui.io_mut().font_global_scale = (1.0 / window.scale_factor()) as f32;

        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(), 
            &window, 
            imgui_winit_support::HiDpiMode::Default);

        // Set imgui ini path
        match file::absolute_path("./config/editor/imgui") {
            Ok(val) => {
                let mut abs_path = String::from(val);
                abs_path.push_str("/imgui.ini");
                let path = std::path::PathBuf::from(abs_path);
                imgui.set_ini_filename(path);
            },
            Err(e) => {
                eprintln!("{e} - [editor::ui::new()][ini path]");
                imgui.set_ini_filename(None)
            }
        };
        
        let font_size = (13.0 * window.scale_factor()) as f32;
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                    size_pixels: font_size,
                ..Default::default()
            })
        }]);

        let config = RendererConfig {
            texture_format: viewport.config.format,
            ..Default::default()
        };
        let renderer = Renderer::new(
            &mut imgui, 
            &viewport.device,
            &viewport.queue,
            config);

        let dockspace_flags = 
            WindowFlags::MENU_BAR | WindowFlags::NO_DOCKING | WindowFlags::NO_TITLE_BAR |
            WindowFlags::NO_COLLAPSE | WindowFlags::NO_RESIZE | WindowFlags::NO_BACKGROUND |
            WindowFlags::NO_NAV_FOCUS | WindowFlags::NO_MOVE| WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS;

        Self {
            imgui,
            platform,
            renderer,
            dockspace_flags
        }
    }

    pub fn handle_update(&mut self, _window: &Window, dt: f32) {
        self.imgui
            .io_mut()
            .update_delta_time(Duration::from_secs_f32(dt));
    }

    pub fn handle_render<'a>(&'a mut self, 
        window: &Window, 
        viewport: &Viewport, 
        _game: &Game,
        frame: &mut Frame, 
        _dt: f32) {

        self.platform.prepare_frame(self.imgui.io_mut(), window)
            .expect("ERROR::editor::ui::render()::failed to prepare frame");

        let dockspace_pos: [f32; 2] = self.imgui.main_viewport().pos;
        let dock_size: [f32; 2] = [viewport.config.width as f32, viewport.config.height as f32];
        let ui = self.imgui.frame();
        {
            ui.dockspace_over_main_viewport();

            let rounding = ui.push_style_var(StyleVar::WindowRounding(0.0));
            let border = ui.push_style_var(StyleVar::WindowBorderSize(0.0));
            let padding = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
            ui.window("Dockspace")
                .position(dockspace_pos, Condition::Always)
                .size(dock_size, Condition::Always)
                .flags(self.dockspace_flags)
                .build(|| {
                    padding.pop();
                    border.pop();
                    rounding.pop();

                    // --- Draw here --- //

                    let mut is_open = true;
                    ui.show_demo_window(&mut is_open);

                    // ----------------- //
                });
        }

        // Rendering done here
        {
            let mut rp = frame.render_pass_ui();

            self.platform.prepare_render(ui, window);
            self.renderer.render(
                self.imgui.render(),
                &viewport.queue,
                &viewport.device,
                &mut rp
            ).expect("ERROR::editor::ui::render()::failed to render");
        }
        
    }

    pub fn handle_modifiers(&self, _mod: &ModifiersState) {

    }

    pub fn handle_resize(&self, _size: PhysicalSize<u32>) {

    }

    pub fn handle_kb_input(&self, _input: &KeyboardInput) {

    }

    pub fn handle_mb_input(&mut self, _state: &ElementState, _input: &MouseButton) {

    }

    pub fn handle_event(&mut self, window: &Window, event: &Event<()>) {
        self.platform.handle_event(self.imgui.io_mut(), window, event);
    }
}