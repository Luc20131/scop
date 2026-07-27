use glfw::{Action, Context, Key, MouseButton, SwapInterval, WindowEvent};
use std::collections::HashMap;

use crate::graphics::mesh::toggle_texture_mode;

pub struct MouseState {
    pub mouse_buttons: HashMap<MouseButton, bool>,
    pub cursor_pos: (f64, f64),
}

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
    frame_count: u32,
    pub keys_state: HashMap<Key, bool>,
    pub mouse_state: MouseState,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        Window {
            glfw,
            window_handle: window,
            events,
            frame_count: 0,
            keys_state: HashMap::from([
                (Key::D, false),
                (Key::A, false),
                (Key::W, false),
                (Key::S, false),
                (Key::Q, false),
                (Key::E, false),
                (Key::R, false),
                (Key::V, false),
                (Key::B, false),
                (Key::N, false),
                (Key::T, false),
                (Key::Left, false),
                (Key::Right, false),
                (Key::Space, false),
                (Key::LeftShift, false),
                (Key::Kp4, false),
                (Key::Kp6, false),
                (Key::KpAdd, false),
                (Key::KpSubtract, false),
            ]),
            mouse_state: MouseState {
                mouse_buttons: HashMap::from([
                    (MouseButton::Left, false),
                    (MouseButton::Right, false),
                    (MouseButton::Middle, false),
                ]),
                cursor_pos: (0.0, 0.0),
            },
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|symbol| {
            self.window_handle
                .get_proc_address(symbol)
                .map_or(std::ptr::null(), |f| f as *const _)
        });
        self.glfw.set_swap_interval(SwapInterval::Sync(1));
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    self.keys_state.entry(Key::D).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    self.keys_state.entry(Key::D).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    self.keys_state.entry(Key::A).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
                    self.keys_state.entry(Key::A).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    self.keys_state.entry(Key::W).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
                    self.keys_state.entry(Key::W).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    self.keys_state.entry(Key::S).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    self.keys_state.entry(Key::S).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
                    self.keys_state.entry(Key::Q).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::Q, _, Action::Release, _) => {
                    self.keys_state.entry(Key::Q).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::E, _, Action::Press, _) => {
                    self.keys_state.entry(Key::E).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::E, _, Action::Release, _) => {
                    self.keys_state.entry(Key::E).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => {
                    self.keys_state.entry(Key::R).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::R, _, Action::Release, _) => {
                    self.keys_state.entry(Key::R).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::V, _, Action::Press, _) => {
                    self.keys_state.entry(Key::V).insert_entry(true);
                    self.keys_state.entry(Key::B).insert_entry(false);
                    self.keys_state.entry(Key::N).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::B, _, Action::Press, _) => {
                    self.keys_state.entry(Key::V).insert_entry(false);
                    self.keys_state.entry(Key::B).insert_entry(true);
                    self.keys_state.entry(Key::N).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::N, _, Action::Press, _) => {
                    self.keys_state.entry(Key::V).insert_entry(false);
                    self.keys_state.entry(Key::B).insert_entry(false);
                    self.keys_state.entry(Key::N).insert_entry(true);
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    self.mouse_state.cursor_pos = (x, y);
                }
                glfw::WindowEvent::MouseButton(MouseButton::Left, Action::Press, _) => {
                    self.mouse_state
                        .mouse_buttons
                        .entry(MouseButton::Left)
                        .insert_entry(true);
                }
                glfw::WindowEvent::MouseButton(MouseButton::Left, Action::Release, _) => {
                    self.mouse_state
                        .mouse_buttons
                        .entry(MouseButton::Left)
                        .insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::T, _, Action::Press, _) => {
                    self.keys_state
                        .entry(Key::T)
                        .insert_entry(toggle_texture_mode(true));
                }
                glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) => {
                    self.keys_state.entry(Key::Left).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::Left, _, Action::Release, _) => {
                    self.keys_state.entry(Key::Left).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                    self.keys_state.entry(Key::Right).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::Right, _, Action::Release, _) => {
                    self.keys_state.entry(Key::Right).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    self.keys_state.entry(Key::Space).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    self.keys_state.entry(Key::Space).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::LeftShift, _, Action::Press, _) => {
                    self.keys_state.entry(Key::LeftShift).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::LeftShift, _, Action::Release, _) => {
                    self.keys_state.entry(Key::LeftShift).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::Kp4, _, Action::Press, _) => {
                    self.keys_state.entry(Key::Kp4).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::Kp4, _, Action::Release, _) => {
                    self.keys_state.entry(Key::Kp4).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::Kp6, _, Action::Press, _) => {
                    self.keys_state.entry(Key::Kp6).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::Kp6, _, Action::Release, _) => {
                    self.keys_state.entry(Key::Kp6).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::KpAdd, _, Action::Press, _) => {
                    self.keys_state.entry(Key::KpAdd).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::KpAdd, _, Action::Release, _) => {
                    self.keys_state.entry(Key::KpAdd).insert_entry(false);
                }
                glfw::WindowEvent::Key(Key::KpSubtract, _, Action::Press, _) => {
                    self.keys_state.entry(Key::KpSubtract).insert_entry(true);
                }
                glfw::WindowEvent::Key(Key::KpSubtract, _, Action::Release, _) => {
                    self.keys_state.entry(Key::KpSubtract).insert_entry(false);
                }
                _ => {}
            }
        }
    }

    pub fn get_time(&mut self) -> f64 {
        self.glfw.get_time()
    }

    pub fn update_title(&mut self, last_time: f64) -> f64 {
        self.frame_count += 1;
        let time = self.glfw.get_time();
        if time - last_time >= 1.0 {
            let fps = self.frame_count.to_string();
            let fps_str: &str = &fps;
            self.window_handle.set_title(fps_str);
            self.frame_count = 0;
            return time;
        }
        last_time
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.window_handle.get_size()
    }

    // fn create_cursor(&mut self) {
    // unsafe {
    // let cursor: Cursor = Cursor {
    //     ptr: glfwCreateStandardCursor(StandardCursor::Arrow as i32),
    // };
    // let cursor2 =
    // let cursor = Cursor::standard(glfw::StandardCursor::Arrow);
    // let cursor = glfw::ffi::glfwCreateStandardCursor(StandardCursor::Arrow as i32);
    // }
    // }
}
