#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Program, Surface, VertexBuffer};
use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode, WindowBuilder};
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::{Uniforms, UniformValue};

#[derive(Clone, Copy, Debug)]
struct V { p: [f32; 2] }
impl std::fmt::Display for V {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({}, {})", self.p[0], self.p[1])
    }
}
implement_vertex!(V, p);

#[derive(Clone, Debug)]
struct DrawParams {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,

    width: f64,
    height: f64,
}
impl DrawParams {
    fn new(dims: (u32, u32)) -> DrawParams {
        DrawParams {
            /*x_min: -2.0,
            x_max: 1.0,
            y_min: -1.0,
            y_max: 1.0,*/
            x_min: -0.7832642347569785,
            x_max: -0.7832642347569401,
            y_min: -0.12973931718080114,
            y_max: -0.12973931718077555,
            width: dims.0 as f64,
            height: dims.1 as f64,
        }
    }
    fn reset(&mut self) {
        self.x_min = -2.0;
        self.x_max = 1.0;
        self.y_min = -1.0;
        self.y_max = 1.0;
    }
    fn scroll(&mut self, x: f64, y: f64) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min += x * s_x;
        self.x_max += x * s_x;
        self.y_min += y * s_y;
        self.y_max += y * s_y;
    }
    fn pan(&mut self, x: i32, y: i32) {
        self.scroll(x as f64 / 100.0,
                    y as f64 / 100.0)
    }
    fn zoom_in(&mut self) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min += s_x;
        self.x_max -= s_x;
        self.y_min += s_y;
        self.y_max -= s_y;
    }
    fn zoom_out(&mut self) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min -= s_x;
        self.x_max += s_x;
        self.y_min -= s_y;
        self.y_max += s_y;
    }
}
impl Uniforms for DrawParams {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("xMin", UniformValue::Double(self.x_min));
        f("xMax", UniformValue::Double(self.x_max));
        f("yMin", UniformValue::Double(self.y_min));
        f("yMax", UniformValue::Double(self.y_max));
        f("width", UniformValue::Double(self.width));
        f("height", UniformValue::Double(self.height));
    }
}

fn main() {
    // Initialize the window.
    let display = WindowBuilder::new()
        .with_multitouch()
        .with_title("Mandelbrot Viewer")
        .with_vsync()
        .build_glium()
        .expect("couldn't open a window");
    // Store the vertices for a rectangle.
    let vertices = [
        V{ p: [1.0, -1.0] },
        V{ p: [-1.0, 1.0] },
        V{ p: [-1.0, -1.0] },
        V{ p: [1.0, 1.0] },
        V{ p: [1.0, -1.0] },
        V{ p: [-1.0, 1.0] },
    ];
    let vertex_buffer = VertexBuffer::new(&display, &vertices)
        .expect("couldn't init vertexbuffer");
    let indices = NoIndices(PrimitiveType::TrianglesList);
    // Load the GLSL program.
    let program = Program::from_source(&display,
            include_str!("vertex.glsl"),
            include_str!("fragment.glsl"),
            None)
        .expect("couldn't compile program");
    // Initialize the display parameters.
    let mut draw_params = DrawParams::new(display.get_window()
        .expect("couldn't get window")
        .get_inner_size_pixels()
        .expect("couldn't get window size"));

    // Input variables.
    let mut mouse_down = false;
    let mut mouse_last = (0, 0);

    // Main loop.
    loop {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &draw_params, &Default::default())
            .expect("couldn't draw triangles");
        target.finish().expect("drawing failed");

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => match code {
                    VirtualKeyCode::Subtract => draw_params.zoom_out(),
                    VirtualKeyCode::Equals => draw_params.zoom_in(),
                    VirtualKeyCode::Space => draw_params.reset(),
                    VirtualKeyCode::Up => draw_params.scroll(0.0, -1.0),
                    VirtualKeyCode::Left => draw_params.scroll(-1.0, 0.0),
                    VirtualKeyCode::Right => draw_params.scroll(1.0, 0.0),
                    VirtualKeyCode::Down => draw_params.scroll(0.0, 1.0),
                    _ => println!("Key: {:?}", code),
                },
                Event::MouseInput(state, MouseButton::Left) => mouse_down = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                },
                Event::MouseMoved(x, y) => {
                    if mouse_down {
                        draw_params.pan(mouse_last.0 - x, mouse_last.1 - y);
                    }
                    mouse_last = (x, y);
                },
                Event::Resized(w, h) => {
                    draw_params.height = h as f64;
                    draw_params.width = w as f64;
                },
                _ => println!("Event: {:?}", ev),
            }
        }
    }
}
