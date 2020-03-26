//! The OpenGL window is (-1.0, -1.0) in the bottom left to (1.0, 1.0) in the top right.

use glium::{glutin::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
    window::WindowBuilder,
    ContextBuilder,
}, implement_vertex, index::{NoIndices, PrimitiveType}, program::ProgramCreationInput, texture::{CompressedTexture2d, RawImage2d}, uniform, Blend, Display, DrawParameters, Frame, IndexBuffer, Program, Smooth, Surface, VertexBuffer, PolygonMode};
use rand::prelude::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::hash::{Hash, Hasher};

use rusty_core::glm::{self, Vec2};
use std::convert::TryInto;

pub mod prelude {
    pub use crate::{GameEvent, Window};
}

pub fn clamp_vec_to_magnitude(v: &mut Vec2, magnitude: f32) {
    if v.magnitude() > magnitude {
        v.data = (v.normalize() * magnitude).data;
    }
}

pub fn angle_facing(v1: &Vec2, v2: &Vec2) -> f32 {
    (v2.data[1] - v1.data[1]).atan2(v2.data[0] - v1.data[0])
}

pub fn new_in_square<T: Rng>(dimension: f32, rng: &mut T) -> Vec2 {
    Vec2::new(
        rng.gen_range(-dimension, dimension),
        rng.gen_range(-dimension, dimension),
    )
}

/// A color with 32-bit float parts from `[0.0, 1.0]` suitable for OpenGL.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Color {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
}

impl Color {
    /// Slightly simpler way to create a color
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.r as u32).hash(state);
        (self.g as u32).hash(state);
        (self.b as u32).hash(state);
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.r == other.r) && (self.g == other.g) && (self.b == other.b)
    }
}

/// Abstracted button values you may receive (arrow keys and WASD keys combined into directions, for
/// example)
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ButtonValue {
    /// An abstracted button that combines: Arrow Up, W, Comma (Dvorak)
    Up,
    /// An abstracted button that combines: Arrow Down, S, O (Dvorak)
    Down,
    /// An abstracted button that combines: Arrow Left, A
    Left,
    /// An abstracted button that combines: Arrow Right, D, E (Dvorak)
    Right,
    /// An abstracted button that combines: Left Mouse Button, Space Bar, Backspace
    Action1,
    /// An abstracted button that combines: Right Mouse Button, Enter, Return
    Action2,
    /// An abstracted button that combines: Any other Mouse Button, Tab
    Action3,
    /// An abstracted button that combines: =/+ key
    Increase,
    /// An abstracted button that combines: -/_ key
    Decrease,
}

/// Whether a button was pressed or released
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ButtonState {
    /// A button was just pressed
    Pressed,
    /// A button was just released
    Released,
}

/// `GameEvent` represents game events caused by a user, such as the mouse moving around, buttons
/// being pushed, or the window being closed.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum GameEvent {
    /// The user pressed Escape or closed the window. We should quit the game.
    Quit,
    /// Indicates the current position the mouse has moved to.  The mouse is now at this location in
    /// OpenGL coordinates.  Note that on some operating systems this event will fire even if the
    /// cursor is outside the bounds of the window.
    MouseMoved { position: Vec2 },
    /// Indicates that a button with variant `ButtonValue` has been either pressed or released
    /// (variant of `ButtonState`).  Note that both mouse buttons and keyboard buttons are
    /// abstracted and collected together into a few logical game buttons.
    Button {
        button_value: ButtonValue,
        button_state: ButtonState,
    },
}

pub enum ShapeStyle {
    Fill,
    Line,
}

pub enum DrawBundle<'a> {
    Img(&'a VertexBuffer<ImgVertex>, &'a IndexBuffer<u16>, &'a CompressedTexture2d),
    Shape(&'a VertexBuffer<ShapeVertex>, &'a NoIndices, PolygonMode),
}

pub trait Drawable {
    fn get_shader_items(&self) -> DrawBundle;
}

pub struct Sprite {
    pub pos: Vec2,
    pub direction: f32,
    pub scale: f32,
    affine: [[f32; 4]; 4],
    affine_cache: (Vec2, f32, f32),
    drawable: Box<dyn Drawable>,
}

impl Sprite {
    pub fn new_image(
        window: &Window,
        pos: Vec2,
        direction: f32,
        scale: f32,
        color: Option<Color>,
        filename: &str,
    ) -> Self {
        Self {
            pos,
            direction,
            scale,
            affine: [[0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.]],
            affine_cache: (glm::vec2(std::f32::NAN, std::f32::NAN), std::f32::NAN, std::f32::NAN),
            drawable: Box::new(Img::new(&window, color, filename)),
        }

    }
    pub fn new_circle(
        window: &Window,
        pos: Vec2,
        direction: f32,
        scale: f32,
        radius: f32,
        color: Color,
        shape_style: ShapeStyle,
    ) -> Self {
        Self {
            pos,
            direction,
            scale,
            affine: [[0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.]],
            affine_cache: (glm::vec2(std::f32::NAN, std::f32::NAN), std::f32::NAN, std::f32::NAN),
            drawable: Box::new(Shape::new_circle(&window, radius, color, shape_style)),
        }
    }
    pub fn new_rectangle(
        window: &Window,
        pos: Vec2,
        direction: f32,
        scale: f32,
        width: f32,
        height: f32,
        color: Color,
        shape_style: ShapeStyle,
    ) -> Self {
        Self {
            pos,
            direction,
            scale,
            affine: [[0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.]],
            affine_cache: (glm::vec2(std::f32::NAN, std::f32::NAN), std::f32::NAN, std::f32::NAN),
            drawable: Box::new(Shape::new_rectangle(&window, width, height, color, shape_style)),
        }
    }
    /// You must call a `Window`'s `.drawstart()` before calling this method.  `draw()` will draw your
    /// image to the current off-screen framebuffer.  After the first time a given image value is
    /// drawn it stays on the GPU and during subsequent calls it only sends updated
    /// position/rotation, which is super efficient, so don't destroy and recreate images every
    /// frame! Draw calls draw to the framebuffer in the order that they occur, so the last image
    pub fn draw(&mut self, window: &mut Window) {
        if let Some(ref mut target) = window.target {
            let affine = if self.affine_cache == (self.pos, self.direction, self.scale) {
                self.affine
            } else {
                let translated = glm::translation(&glm::vec2_to_vec3(&self.pos));
                let rotated = glm::rotate(&translated, self.direction, &glm::vec3(0.0f32, 0., 1.));
                let scaled = glm::scale(&rotated, &glm::vec3(self.scale, self.scale, self.scale));
                self.affine_cache = (self.pos, self.direction, self.scale);
                self.affine = scaled.try_into().unwrap();
                self.affine
            };

            match self.drawable.get_shader_items() {
                DrawBundle::Img(vertex_buffer, index_buffer, texture) => {
                    let uniforms = uniform! {
                        matrix: affine,
                        tex: texture,
                    };

                    let draw_parameters = DrawParameters {
                        blend: Blend::alpha_blending(),
//                        smooth: Some(Smooth::Nicest),
                        ..Default::default()
                    };

                    target
                        .draw(
                            vertex_buffer,
                            index_buffer,
                            &window.img_program,
                            &uniforms,
                            &draw_parameters,
                        )
                        .unwrap();
                }
                DrawBundle::Shape(vertex_buffer, indices, polygon_mode) => {
                    let uniforms = uniform! {
                            matrix: affine,
                        };

                    let draw_parameters = DrawParameters {
                        blend: Blend::alpha_blending(),
                        smooth: Some(Smooth::Nicest),
                        polygon_mode,
                        ..Default::default()
                    };
                    target
                        .draw(
                            vertex_buffer,
                            indices,
                            &window.shape_program,
                            &uniforms,
                            &draw_parameters,
                        )
                        .unwrap();
                },
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ShapeVertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(ShapeVertex, position, color);

/// A `Shape` can be drawn to a `Window` using its `draw_shape()` method. Use the provided `new_*`
/// methods to make a `Shape`.
#[derive(Debug)]
pub struct Shape {
    vertex_buffer: VertexBuffer<ShapeVertex>,
    indices: NoIndices,
    polygon_mode: PolygonMode,
}

impl Drawable for Shape {
    fn get_shader_items(&self) -> DrawBundle {
        DrawBundle::Shape(&self.vertex_buffer, &self.indices, self.polygon_mode)
    }
}

impl Shape {
    // todo: document
    pub fn new_rectangle(
        window: &Window,
        width: f32,
        height: f32,
        color: Color,
        shape_style: ShapeStyle,
    ) -> Self {
        let vertex_buffer = VertexBuffer::new(
            &window.display,
            &vec![
                ShapeVertex {
                    position: (glm::vec2(width * 0.5, height * 0.5)).into(),
                    color: [color.r, color.g, color.b],
                },
                ShapeVertex {
                    position: (glm::vec2(width * 0.5, -height * 0.5)).into(),
                    color: [color.r, color.g, color.b],
                },
                ShapeVertex {
                    position: (glm::vec2(-width * 0.5, -height * 0.5)).into(),
                    color: [color.r, color.g, color.b],
                },
                ShapeVertex {
                    position: (glm::vec2(-width * 0.5, height * 0.5)).into(),
                    color: [color.r, color.g, color.b],
                },
            ]).unwrap();
        let (primitive_type, polygon_mode) = match shape_style {
            ShapeStyle::Fill => (PrimitiveType::TriangleFan, PolygonMode::Fill),
            ShapeStyle::Line => (PrimitiveType::LineLoop, PolygonMode::Line),
        };
        Self {
            vertex_buffer,
            indices: NoIndices(primitive_type),
            polygon_mode,
        }
    }
    /// Create a solid circle with a stripe that always faces `direction`.
    pub fn new_circle(
        window: &Window,
        radius: f32,
        color: Color,
        shape_style: ShapeStyle,
    ) -> Self {
        let num_vertices = 63;
        let mut v = Vec::<ShapeVertex>::with_capacity(num_vertices + 1);
        for x in 0..=num_vertices {
            let inner: f64 = 2.0 * PI / num_vertices as f64 * x as f64;
            v.push(ShapeVertex {
                position: [inner.cos() as f32 * radius, inner.sin() as f32 * radius],
                color: [color.r, color.g, color.b],
            });
        }
        let vertex_buffer =
            VertexBuffer::new(&window.display, &v).unwrap();
        let (primitive_type, polygon_mode) = match shape_style {
            ShapeStyle::Fill => (PrimitiveType::TriangleFan, PolygonMode::Fill),
            ShapeStyle::Line => (PrimitiveType::LineLoop, PolygonMode::Line),
        };
        Self {
            vertex_buffer,
            indices: NoIndices(primitive_type),
            polygon_mode,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ImgVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 3],
    tint: u8,
}

implement_vertex!(ImgVertex, position, tex_coords, color, tint);

/// An image that can be drawn using the `Window.draw()` method.  Currently only PNG format is
/// supported.
///
/// If you are looking at an image in Photoshop, the "right" direction is the "front" of the
/// image.  `direction` is the angle in radians that the image will be rotated.
///
/// If you want your image to have transparency without getting white borders, export as a PNG-8
/// with Transparency checked, and Matte set to None.  See `media/png-settings-screenshot.png` in
/// the repository for a screenshot of the Photoshop "Export > Save for Web" settings that are known
/// to work.  Or just exporting as a 24-bit PNG might work.
#[derive(Debug)]
pub struct Img {
    vertex_buffer: VertexBuffer<ImgVertex>,
    index_buffer: IndexBuffer<u16>,
    texture: CompressedTexture2d,
}

impl Drawable for Img {
    fn get_shader_items(&self) -> DrawBundle {
        DrawBundle::Img(&self.vertex_buffer, &self.index_buffer, &self.texture)
    }
}


impl Img {
    /// Create a new image.  `filename` is relative to the root of the project you are running from.
    /// For example, if you created a `media` subdirectory in the root of your project and then put
    /// `soldier.png` in it, then your filename would be `media/soldier.png`.
    pub fn new(
        window: &Window,
        color: Option<Color>,
        filename: &str,
    ) -> Self {
        let file = std::fs::File::open(filename).unwrap();
        let reader = std::io::BufReader::new(file);
        let image = image::load(reader, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = CompressedTexture2d::new(&window.display, image).unwrap();

        let tint = if color.is_some() { 1 } else { 0 };
        let color = color.unwrap_or_else(|| Color::new(1.0, 1.0, 1.0));

        // Size the image relative to the window - calculations assume a square window
        // Assuming 16 game units fit in the (1.0, 1.0) to (-1.0, -1.0) viewspace
        // So: 8 game units per OpenGL unit, a window is 2 OpenGL units high and wide
        let pixels_per_game_unit = window.dimension as f32 * 0.0625;
        let scale_x = image_dimensions.0 as f32 / pixels_per_game_unit / 16.;
        let scale_y = image_dimensions.1 as f32 / pixels_per_game_unit / 16.;

        let vertex_buffer =
            VertexBuffer::new(
                &window.display,
                &[
                    ImgVertex {
                        position: [-scale_x, -scale_y],
                        tex_coords: [0.0, 0.0],
                        color: [color.r, color.g, color.b],
                        tint,
                    },
                    ImgVertex {
                        position: [-scale_x, scale_y],
                        tex_coords: [0.0, 1.0],
                        color: [color.r, color.g, color.b],
                        tint,
                    },
                    ImgVertex {
                        position: [scale_x, scale_y],
                        tex_coords: [1.0, 1.0],
                        color: [color.r, color.g, color.b],
                        tint,
                    },
                    ImgVertex {
                        position: [scale_x, -scale_y],
                        tex_coords: [1.0, 0.0],
                        color: [color.r, color.g, color.b],
                        tint,
                    },
                ],
            ).unwrap();
        let index_buffer = IndexBuffer::new(
            &window.display,
            PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3],
        )
            .unwrap();
        Self {
            vertex_buffer,
            index_buffer,
            texture,
        }
    }
}

/// An OpenGL window for displaying graphics. Also the object through which you'll receive input
/// events (mouse, keyboard, etc.)
pub struct Window {
    event_loop: EventLoop<()>,
    display: Display,
    shape_program: Program,
    img_program: Program,
    screen_to_opengl: Box<dyn Fn(PhysicalPosition<f64>) -> Vec2>,
    dimension: u32,
    target: Option<Frame>,
}

impl Window {
    /// By default, this will be a square window with a dimension of `1024` logical pixels.  You can
    /// override the dimension by providing a value for override_dimension, for example: `Some(1200)`.
    ///
    /// `window_title` is for the OS to use on the bar above your window.
    pub fn new(override_dimension: Option<u32>, window_title: &str) -> Self {
        let event_loop = EventLoop::<()>::new();
        let dimension = override_dimension.unwrap_or(1024);
        let logical_size = LogicalSize::new(dimension, dimension);
        let window = WindowBuilder::new()
            .with_inner_size(logical_size)
            .with_title(window_title);
        let context = ContextBuilder::new();
        let display = Display::new(window, context, &event_loop).unwrap();

        let current_monitor = display.gl_window().window().current_monitor();
        let scale_factor: f64 = current_monitor.scale_factor();

        // Create a closure that captures current screen information to use to
        // do local screen coordinate conversion for us.
        let inverse_half_dimension = 1.0 / (dimension as f32 * 0.5);
        let screen_to_opengl = Box::new(move |pos: PhysicalPosition<f64>| -> Vec2 {
            let logical_pos: (f64, f64) = pos.to_logical::<f64>(scale_factor).into();
            let x = (logical_pos.0 as f32 * inverse_half_dimension) - 1.0;
            let y = 1.0 - (logical_pos.1 as f32 * inverse_half_dimension);
            Vec2::new(x, y)
        });

        // For drawing shapes
        let shape_vertex_shader = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 v_color;

        uniform mat4 matrix;

        void main() {
            v_color = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
        "#;

        let shape_fragment_shader = r#"
            #version 140

            in vec3 v_color;
            out vec4 color;

            void main() {
                color = vec4(v_color, 1.0);
            }
        "#;

        let shape_program = Program::new(
            &display,
            ProgramCreationInput::SourceCode {
                vertex_shader: shape_vertex_shader,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                fragment_shader: shape_fragment_shader,
                transform_feedback_varyings: None,
                outputs_srgb: true,
                uses_point_size: true,
            },
        )
            .unwrap();

        // For drawing images
        let vertex_shader_img = r#"
            #version 140
            uniform mat4 matrix;
            in vec2 position;
            in vec2 tex_coords;
            in vec3 color;
            in uint tint;
            out vec3 v_color;
            out vec2 v_tex_coords;
            flat out uint u_tint;
            void main() {
                u_tint = tint;
                v_color = color;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
                v_tex_coords = tex_coords;
            }
        "#;

        let fragment_shader_img = r#"
            #version 140
            uniform sampler2D tex;
            in vec2 v_tex_coords;
            in vec3 v_color;
            flat in uint u_tint;
            out vec4 f_color;
            void main() {
                if ((texture(tex, v_tex_coords).a < 0.9) || (u_tint == 0u)) {
                    f_color = texture(tex, v_tex_coords);
                } else {
                    f_color = mix(texture(tex, v_tex_coords), vec4(v_color, 1.0), 0.5);
                }
            }
        "#;

        let img_program = Program::new(
            &display,
            ProgramCreationInput::SourceCode {
                vertex_shader: vertex_shader_img,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                fragment_shader: fragment_shader_img,
                transform_feedback_varyings: None,
                outputs_srgb: true,
                uses_point_size: true,
            },
        )
            .unwrap();

        Self {
            event_loop,
            display,
            shape_program,
            img_program,
            screen_to_opengl,
            dimension,
            target: None,
        }
    }

    /// Call `drawstart()` when you are ready to draw a new frame. It will initialize the next
    /// off-screen framebuffer and clear it to black.
    pub fn drawstart(&mut self) {
        self.target = Some(self.display.draw());
        if let Some(ref mut target) = self.target {
            target.clear_color(0.0, 0.0, 0.0, 1.0);
        }
    }

    /// Call `drawfinish()` when you are ready to finalize the frame and show it.  You will need to
    /// call `drawstart()` again before you can `draw()` any shapes in a new frame.  I _think_ this
    /// method blocks until the hardware is ready for a frame (vsync), so an unconstrained loop
    /// (that runs fast enough) should run at 60fps on most displays.
    pub fn drawfinish(&mut self) {
        self.target.take().unwrap().finish().unwrap();
    }

    /// For convenience this method abstracts all the possible mouse and keyboard events to
    /// `GameEvent`s, which are the events we care about for the game.
    /// The WASD and arrow keys map to directions, mouse clicks and space bar map to attacks, and
    /// the Escape key maps to quitting.  Any number of events could have occurred since we last
    /// looked, so a `Vec<GameEvent>` is returned.
    pub fn poll_game_events(&mut self) -> Vec<GameEvent> {
        let screen_to_opengl = &mut (self.screen_to_opengl);
        let mut events = Vec::<GameEvent>::new();
        self.event_loop.run_return(|ev, _, control_flow| {
            *control_flow = ControlFlow::Exit;
            if let Event::WindowEvent { event, .. } = ev {
                match event {
                    // Time to close the app?
                    WindowEvent::CloseRequested => events.push(GameEvent::Quit),
                    // Mouse moved
                    WindowEvent::CursorMoved { position, .. } => {
                        let mouse_pos = screen_to_opengl(position);
                        events.push(GameEvent::MouseMoved {
                            position: mouse_pos,
                        });
                    }
                    // Keyboard button
                    WindowEvent::KeyboardInput { input, .. } => {
                        let button_state = match input.state {
                            ElementState::Pressed => ButtonState::Pressed,
                            ElementState::Released => ButtonState::Released,
                        };
                        use VirtualKeyCode::*;
                        if let Some(vkey) = input.virtual_keycode {
                            match vkey {
                                W | Up | Comma => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Up,
                                }),
                                S | Down | O => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Down,
                                }),
                                A | Left => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Left,
                                }),
                                D | Right | E => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Right,
                                }),
                                Escape => events.push(GameEvent::Quit),
                                Space | Delete => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Action1,
                                }),
                                NumpadEnter | Return => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Action2,
                                }),
                                Tab => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Action3,
                                }),
                                // Equals covers the +/= key.
                                Equals => events.push( GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Increase,
                                }),
                                // Minus covers the -/_ key.
                                Minus => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Decrease,
                                }),
                                _ => (),
                            }
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        let button_state = match state {
                            ElementState::Pressed => ButtonState::Pressed,
                            ElementState::Released => ButtonState::Released,
                        };
                        events.push(GameEvent::Button {
                            button_state,
                            button_value: {
                                match button {
                                    MouseButton::Left => ButtonValue::Action1,
                                    MouseButton::Right => ButtonValue::Action2,
                                    MouseButton::Middle | MouseButton::Other(_) => ButtonValue::Action3,
                                }
                            },
                        });
                    }
                    _ => (),
                }
            }
        });
        events
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
