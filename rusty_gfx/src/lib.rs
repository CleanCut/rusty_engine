//! The OpenGL window is (-1.0, -1.0) in the bottom left to (1.0, 1.0) in the top right.

// EXTERNAL
use glium::{
    glutin::{
        dpi::{LogicalSize, PhysicalPosition},
        event::{
            ElementState as GLElementState, Event as GLEvent, MouseButton as GLMouseButton,
            VirtualKeyCode as GLVirtualKeyCode, WindowEvent as GLWindowEvent,
        },
        event_loop::{ControlFlow, EventLoop},
        platform::run_return::EventLoopExtRunReturn,
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    program::ProgramCreationInput,
    texture::{CompressedTexture2d, RawImage2d},
    uniform, Blend, Display, DrawParameters, Frame, IndexBuffer, PolygonMode, Program, Smooth,
    Surface, VertexBuffer,
};
use std::f64::consts::PI;

// RUSTY
use crate::color::Color;
use crate::event::*;
use rusty_core::glm::{self, Vec2};
use rusty_core::prelude::Transform;

// EXPORT
pub mod color;
pub mod event;
pub mod util;
pub mod prelude {
    pub use crate::color::*;
    pub use crate::event::*;
    pub use crate::util::*;
    pub use crate::{Sprite, Window};
}

pub enum ShapeStyle {
    Fill,
    Line,
}

pub enum ThingToDraw<'a> {
    Img(
        &'a VertexBuffer<ImgVertex>,
        &'a IndexBuffer<u16>,
        &'a CompressedTexture2d,
    ),
    Shape(&'a VertexBuffer<ShapeVertex>, &'a NoIndices, PolygonMode),
    ShaderShape(&'a VertexBuffer<ShaderShapeVertex>, &'a NoIndices, &'a f32),
}

pub trait Drawable {
    fn get_shader_items(&self) -> ThingToDraw;
}

pub struct Sprite {
    pub transform: Transform,
    drawable: Box<dyn Drawable>,
}

impl Sprite {
    pub fn smooth_circle(
        window: &Window,
        pos: Vec2,
        direction: f32,
        scale: f32,
        radius: f32,
        color: Color,
    ) -> Self {
        Self::with_drawable_at(
            ShaderShape::new_circle(window, radius, color),
            Transform::at(pos, direction, scale),
        )
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
        Self::with_drawable_at(
            Shape::new_rectangle(&window, width, height, color, shape_style),
            Transform::at(pos, direction, scale),
        )
    }
    pub fn new_image(
        window: &Window,
        pos: Vec2,
        direction: f32,
        scale: f32,
        tint: Option<Color>,
        filename: &str,
    ) -> Self {
        Self::with_drawable_at(
            Img::new(window, tint, filename),
            Transform::at(pos, direction, scale),
        )
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
        Self::with_drawable_at(
            Shape::new_circle(&window, radius, color, shape_style),
            Transform::at(pos, direction, scale),
        )
    }
    pub fn with_drawable<T: 'static + Drawable>(drawable: T) -> Self {
        Self {
            transform: Transform::new(),
            drawable: Box::new(drawable),
        }
    }
    pub fn with_drawable_at<T: 'static + Drawable>(drawable: T, transform: Transform) -> Self {
        Self {
            transform,
            drawable: Box::new(drawable),
        }
    }
    /// You must call a `Window`'s `.drawstart()` before calling this method.  `draw()` will draw your
    /// image to the current off-screen framebuffer.  After the first time a given image value is
    /// drawn it stays on the GPU and during subsequent calls it only sends updated
    /// position/rotation, which is super efficient, so don't destroy and recreate images every
    /// frame! Draw calls draw to the framebuffer in the order that they occur, so the last image
    /// is on top.
    pub fn draw(&mut self, window: &mut Window) {
        if let Some(ref mut target) = window.target {
            let affine = self.transform.get_affine();

            match self.drawable.get_shader_items() {
                ThingToDraw::Img(vertex_buffer, index_buffer, texture) => {
                    let uniforms = uniform! {
                        matrix: affine,
                        tex: texture,
                    };

                    let draw_parameters = DrawParameters {
                        blend: Blend::alpha_blending(),
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
                ThingToDraw::Shape(vertex_buffer, indices, polygon_mode) => {
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
                }
                ThingToDraw::ShaderShape(vertex_buffer, indices, local_scale) => {
                    let uniforms = uniform! {
                        matrix: affine,
                        local_scale: *local_scale,
                    };

                    let draw_parameters = DrawParameters {
                        blend: Blend::alpha_blending(),
                        smooth: Some(Smooth::Nicest),
                        ..Default::default()
                    };
                    target
                        .draw(
                            vertex_buffer,
                            indices,
                            &window.shader_shape_program,
                            &uniforms,
                            &draw_parameters,
                        )
                        .unwrap();
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ShaderShapeVertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(ShaderShapeVertex, position, color);

#[derive(Debug)]
pub struct ShaderShape {
    vertex_buffer: VertexBuffer<ShaderShapeVertex>,
    indices: NoIndices,
    local_scale: f32,
}

impl Drawable for ShaderShape {
    fn get_shader_items(&self) -> ThingToDraw {
        ThingToDraw::ShaderShape(&self.vertex_buffer, &self.indices, &self.local_scale)
    }
}

impl ShaderShape {
    pub fn new_circle(window: &Window, radius: f32, color: Color) -> Self {
        let corner_a = Vec2::new(-6. / (2. * 3.0_f32.sqrt()), -1.0).scale(radius);
        let corner_b = Vec2::new(-0., 2.).scale(radius);
        let corner_c = Vec2::new(6. / (2. * 3.0_f32.sqrt()), -1.0).scale(radius);
        let v = vec![
            ShaderShapeVertex {
                position: corner_a.into(),
                color: color.into(),
            },
            ShaderShapeVertex {
                position: corner_b.into(),
                color: color.into(),
            },
            ShaderShapeVertex {
                position: corner_c.into(),
                color: color.into(),
            },
        ];
        let vertex_buffer = VertexBuffer::new(&window.display, &v).unwrap();
        Self {
            vertex_buffer,
            indices: NoIndices(PrimitiveType::TriangleStrip),
            local_scale: radius,
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
    fn get_shader_items(&self) -> ThingToDraw {
        ThingToDraw::Shape(&self.vertex_buffer, &self.indices, self.polygon_mode)
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
                    color: color.into(),
                },
                ShapeVertex {
                    position: (glm::vec2(width * 0.5, -height * 0.5)).into(),
                    color: color.into(),
                },
                ShapeVertex {
                    position: (glm::vec2(-width * 0.5, -height * 0.5)).into(),
                    color: color.into(),
                },
                ShapeVertex {
                    position: (glm::vec2(-width * 0.5, height * 0.5)).into(),
                    color: color.into(),
                },
            ],
        )
        .unwrap();
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
    pub fn new_circle(window: &Window, radius: f32, color: Color, shape_style: ShapeStyle) -> Self {
        let num_vertices = 63;
        let mut v = Vec::<ShapeVertex>::with_capacity(num_vertices + 1);
        for x in 0..=num_vertices {
            let inner: f64 = 2.0 * PI / num_vertices as f64 * x as f64;
            v.push(ShapeVertex {
                position: [inner.cos() as f32 * radius, inner.sin() as f32 * radius],
                color: color.into(),
            });
        }
        let vertex_buffer = VertexBuffer::new(&window.display, &v).unwrap();
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
    tint: [f32; 3],
    should_tint: u8,
}

implement_vertex!(ImgVertex, position, tex_coords, tint, should_tint);

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
    fn get_shader_items(&self) -> ThingToDraw {
        ThingToDraw::Img(&self.vertex_buffer, &self.index_buffer, &self.texture)
    }
}

impl Img {
    /// Create a new image.  `filename` is relative to the root of the project you are running from.
    /// For example, if you created a `media` subdirectory in the root of your project and then put
    /// `soldier.png` in it, then your filename would be `media/soldier.png`.
    pub fn new(window: &Window, tint: Option<Color>, filename: &str) -> Self {
        let file = std::fs::File::open(filename).unwrap();
        let reader = std::io::BufReader::new(file);
        let image = image::load(reader, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = CompressedTexture2d::new(&window.display, image).unwrap();

        let should_tint = if tint.is_some() { 1 } else { 0 };
        let tint = tint.unwrap_or_else(|| Color::new(1.0, 1.0, 1.0));

        // Size the image relative to the window - calculations assume a square window
        // Assuming 16 game units fit in the (1.0, 1.0) to (-1.0, -1.0) viewspace
        // So: 8 game units per OpenGL unit, a window is 2 OpenGL units high and wide
        let pixels_per_game_unit = window.dimension as f32 * 0.0625;
        let scale_x = image_dimensions.0 as f32 / pixels_per_game_unit / 16.;
        let scale_y = image_dimensions.1 as f32 / pixels_per_game_unit / 16.;

        let vertex_buffer = VertexBuffer::new(
            &window.display,
            &[
                ImgVertex {
                    position: [-scale_x, -scale_y],
                    tex_coords: [0.0, 0.0],
                    tint: tint.into(),
                    should_tint,
                },
                ImgVertex {
                    position: [-scale_x, scale_y],
                    tex_coords: [0.0, 1.0],
                    tint: tint.into(),
                    should_tint,
                },
                ImgVertex {
                    position: [scale_x, scale_y],
                    tex_coords: [1.0, 1.0],
                    tint: tint.into(),
                    should_tint,
                },
                ImgVertex {
                    position: [scale_x, -scale_y],
                    tex_coords: [1.0, 0.0],
                    tint: tint.into(),
                    should_tint,
                },
            ],
        )
        .unwrap();
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
    shader_shape_program: Program,
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
        let context = ContextBuilder::new(); // This is the place to enable multisampling and vsync if we want to
        let display = Display::new(window, context, &event_loop).unwrap();

        let current_monitor = display.gl_window().window().current_monitor();
        let scale_factor: f64 = current_monitor.expect("no monitor found").scale_factor();

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

        let shader_circle_program = Program::new(
            &display,
            ProgramCreationInput::SourceCode {
                vertex_shader: include_str!("shader/circle_solid_vertex.glsl"),
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                fragment_shader: include_str!("shader/circle_solid_frag.glsl"),
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
            in vec3 tint;
            in uint should_tint;
            out vec3 v_tint;
            out vec2 v_tex_coords;
            flat out uint v_should_tint;
            void main() {
                v_should_tint = should_tint;
                v_tint = tint;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
                v_tex_coords = tex_coords;
            }
        "#;

        let fragment_shader_img = r#"
            #version 140
            uniform sampler2D tex;
            in vec2 v_tex_coords;
            in vec3 v_tint;
            flat in uint v_should_tint;
            out vec4 f_color;
            void main() {
                if ((texture(tex, v_tex_coords).a < 0.9) || (v_should_tint == 0u)) {
                    f_color = texture(tex, v_tex_coords);
                } else {
                    f_color = mix(texture(tex, v_tex_coords), vec4(v_tint, 1.0), 0.5);
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
            shader_shape_program: shader_circle_program,
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
            if let GLEvent::WindowEvent { event, .. } = ev {
                match event {
                    // Time to close the app?
                    GLWindowEvent::CloseRequested => events.push(GameEvent::Quit),
                    // Mouse moved
                    GLWindowEvent::CursorMoved { position, .. } => {
                        let mouse_pos = screen_to_opengl(position);
                        events.push(GameEvent::MouseMoved {
                            position: mouse_pos,
                        });
                    }
                    // Keyboard button
                    GLWindowEvent::KeyboardInput { input, .. } => {
                        let button_state = match input.state {
                            GLElementState::Pressed => ButtonState::Pressed,
                            GLElementState::Released => ButtonState::Released,
                        };
                        use GLVirtualKeyCode::*;
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
                                Equals => events.push(GameEvent::Button {
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
                    GLWindowEvent::MouseInput { state, button, .. } => {
                        let button_state = match state {
                            GLElementState::Pressed => ButtonState::Pressed,
                            GLElementState::Released => ButtonState::Released,
                        };
                        events.push(GameEvent::Button {
                            button_state,
                            button_value: {
                                match button {
                                    GLMouseButton::Left => ButtonValue::Action1,
                                    GLMouseButton::Right => ButtonValue::Action2,
                                    GLMouseButton::Middle | GLMouseButton::Other(_) => {
                                        ButtonValue::Action3
                                    }
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
