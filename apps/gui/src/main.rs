use mrlyui::face::{HEIGHT, WIDTH};
use mrlyweb::drive::{Driver, KeyPress};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

mod audio;

const BEAT: Duration = Duration::from_millis(125);
const WHEEL_STEP: f64 = 22.0;

fn main() {
    let mut face = Face {
        driver: Driver::wall(),
        audio: audio::Audio::new(),
        window: None,
        context: None,
        surface: None,
        route: String::new(),
        next: Instant::now(),
        port: (0, 0, 1),
        cursor: (-1.0, -1.0),
        pressed: false,
    };
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut face).unwrap();
}

// LOOP

struct Face {
    driver: Driver,
    audio: Option<audio::Audio>,
    window: Option<Arc<Window>>,
    context: Option<Context<Arc<Window>>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    route: String,
    next: Instant,
    port: (usize, usize, usize),
    cursor: (f64, f64),
    pressed: bool,
}

impl ApplicationHandler for Face {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let size = LogicalSize::new((WIDTH * 2) as f64, (HEIGHT * 2) as f64);
        let attrs = Window::default_attributes()
            .with_title("mrly")
            .with_inner_size(size);
        let window = Arc::new(el.create_window(attrs).unwrap());
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
        self.sync();
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => self.press(el, event),
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = (position.x, position.y);
                if self.pressed {
                    let (sx, sy) = self.sheet_clamped();
                    self.driver.move_to(sx, sy);
                } else {
                    self.driver.hover(self.sheet_at());
                    self.sync();
                }
            }
            WindowEvent::CursorLeft { .. } => {
                self.cursor = (-1.0, -1.0);
                self.driver.hover(None);
                self.sync();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y as f64 * WHEEL_STEP,
                    MouseScrollDelta::PixelDelta(p) => p.y,
                };
                self.driver.wheel(dy, self.sheet_at());
                self.sync();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        if let Some((sx, sy)) = self.sheet_at() {
                            self.pressed = true;
                            self.driver.press(sx, sy);
                        }
                    }
                    ElementState::Released => {
                        if self.pressed {
                            self.pressed = false;
                            let (sx, sy) = self.sheet_clamped();
                            self.driver.release(sx, sy);
                        }
                    }
                }
                self.sync();
            }
            WindowEvent::RedrawRequested => self.paint(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, el: &ActiveEventLoop) {
        match self.driver.os().frame(None).view.and_then(|v| v.beat) {
            Some(_) => {
                let now = Instant::now();
                if now >= self.next {
                    self.driver.beat(1);
                    self.next = now + BEAT;
                    self.sync();
                }
                el.set_control_flow(ControlFlow::WaitUntil(self.next));
            }
            None => el.set_control_flow(ControlFlow::Wait),
        }
    }
}

// INPUT

impl Face {
    fn sheet_at(&self) -> Option<(usize, usize)> {
        let (ox, oy, scale) = self.port;
        let x = self.cursor.0 as isize - ox as isize;
        let y = self.cursor.1 as isize - oy as isize;
        if x < 0 || y < 0 || scale == 0 {
            return None;
        }
        let sx = x as usize / scale;
        let sy = y as usize / scale;
        if sx >= WIDTH || sy >= HEIGHT {
            return None;
        }
        Some((sx, sy))
    }

    fn sheet_clamped(&self) -> (usize, usize) {
        let (ox, oy, scale) = self.port;
        let x = (self.cursor.0 as isize - ox as isize).max(0) as usize / scale.max(1);
        let y = (self.cursor.1 as isize - oy as isize).max(0) as usize / scale.max(1);
        (x.min(WIDTH - 1), y.min(HEIGHT - 1))
    }

    fn press(&mut self, el: &ActiveEventLoop, event: KeyEvent) {
        if event.state != ElementState::Pressed {
            return;
        }
        if self.driver.editing() {
            match event.logical_key.as_ref() {
                Key::Named(NamedKey::Escape) => self.driver.key(KeyPress::Escape),
                Key::Named(NamedKey::Enter) => self.driver.key(KeyPress::Enter),
                Key::Named(NamedKey::Backspace) => self.driver.key(KeyPress::Backspace),
                Key::Named(NamedKey::Space) => self.driver.key(KeyPress::Char(' ')),
                Key::Character(ch) => {
                    for c in ch.chars() {
                        self.driver.key(KeyPress::Char(c));
                    }
                }
                _ => {}
            }
            self.sync();
            return;
        }
        match event.logical_key.as_ref() {
            Key::Named(NamedKey::ArrowUp) => self.driver.bound("up"),
            Key::Named(NamedKey::ArrowDown) => self.driver.bound("down"),
            Key::Named(NamedKey::ArrowLeft) => self.driver.bound("left"),
            Key::Named(NamedKey::ArrowRight) => self.driver.bound("right"),
            Key::Named(NamedKey::Escape) => self.driver.escape(),
            Key::Character("q") => {
                el.exit();
                return;
            }
            Key::Character(ch) => match ch {
                "w" | "W" => self.driver.bound("up"),
                "s" | "S" => self.driver.bound("down"),
                "a" | "A" => self.driver.bound("left"),
                "d" | "D" => self.driver.bound("right"),
                _ => {
                    if let Some(digit) = ch.chars().next().and_then(|c| c.to_digit(10)) {
                        if digit >= 1 {
                            self.driver.nth(digit as usize - 1);
                        }
                    }
                }
            },
            _ => {}
        }
        self.sync();
    }
}

// RENDER

fn pack(px: [u8; 4]) -> u32 {
    ((px[0] as u32) << 16) | ((px[1] as u32) << 8) | px[2] as u32
}

impl Face {
    fn sync(&mut self) {
        for effect in self.driver.drain_effects() {
            if effect.kind == "sound" {
                if let Some(audio) = &self.audio {
                    audio.perform(&effect.data);
                }
            }
        }
        let route = self.driver.route().to_string();
        if route != self.route {
            self.route = route;
            if let Some(window) = &self.window {
                window.set_title(&format!("mrly \u{b7} {}", self.route));
            }
        }
        if self.driver.dirty() {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn paint(&mut self) {
        let (Some(window), Some(surface)) = (&self.window, self.surface.as_mut()) else {
            return;
        };
        let size = window.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else {
            return;
        };
        if surface.resize(w, h).is_err() {
            return;
        }
        let frame = &self.driver.scene().frame;
        let Some(colors) = frame.composite().cell.colors else {
            return;
        };
        let fw = frame.width;
        let fh = frame.height;
        let Ok(mut buffer) = surface.buffer_mut() else {
            return;
        };
        let bw = size.width as usize;
        let bh = size.height as usize;
        let scale = (bw / fw).min(bh / fh).max(1);
        let sw = fw * scale;
        let sh = fh * scale;
        let ox = bw.saturating_sub(sw) / 2;
        let oy = bh.saturating_sub(sh) / 2;
        self.port = (ox, oy, scale);
        let bg = pack(colors[0]);
        for y in 0..bh {
            let row = y * bw;
            let inside = y >= oy && y < oy + sh;
            for x in 0..bw {
                buffer[row + x] = if inside && x >= ox && x < ox + sw {
                    let fy = (y - oy) / scale;
                    let fx = (x - ox) / scale;
                    pack(colors[fy * fw + fx])
                } else {
                    bg
                };
            }
        }
        buffer.present().ok();
    }
}
