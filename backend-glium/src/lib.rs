use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::{WindowBuilder, WindowId};
use glium::glutin::{ContextBuilder, GlProfile, Robustness};
use glium::index::PrimitiveType;
use glium::{uniform, Surface};
use std::collections::HashMap;
use widgets::draw::{DrawCommand, DrawQueue, Primitive};
use widgets::event::{AxisValue, ButtonState, EvState, EventContext, ModState};
use widgets::geometry::Point;
use widgets::widget::{TopLevel, WidgetId, WindowAttributes};

mod event;
use event::translate_event;

pub struct GliumWindow<T> {
    display: glium::Display,
    program: glium::Program,
    t_white: glium::texture::Texture2d,
    draw_queue: DrawQueue,
    cur_attr: WindowAttributes,
    last_pos: Point<f64>,
    mod_state: ModState,
    button_state: ButtonState,
    window: T,
}

impl<T: TopLevel> GliumWindow<T> {
    pub fn new(window: T, event_loop: &EventLoop<()>) -> Self {
        let win_attr = window.get_window_attributes();
        let size = win_attr.size.nonzero_or(widgets::DEFAULT_WINDOW_SIZE);
        let mut win_builder = WindowBuilder::new()
            .with_title(win_attr.title.clone().unwrap_or_else(|| "Window".into()))
            .with_inner_size(PhysicalSize::new(size.w, size.h))
            .with_resizable(win_attr.resizable)
            .with_maximized(win_attr.maximized)
            .with_transparent(win_attr.transparent)
            .with_always_on_top(win_attr.always_on_top)
            .with_decorations(win_attr.decorations);
        if let Some(size) = win_attr.min_size.get_nonzero() {
            win_builder = win_builder.with_min_inner_size(PhysicalSize::new(size.w, size.h));
        }
        if let Some(size) = win_attr.max_size.get_nonzero() {
            win_builder = win_builder.with_max_inner_size(PhysicalSize::new(size.w, size.h));
        }

        let ctx = ContextBuilder::new()
            .with_gl_profile(GlProfile::Core)
            .with_gl_robustness(Robustness::TryRobustNoResetNotification)
            .with_double_buffer(Some(true));
        let display = glium::Display::new(win_builder, ctx, event_loop).unwrap();

        if let Some(pos) = win_attr.position {
            display
                .gl_window()
                .window()
                .set_outer_position(PhysicalPosition::new(pos.x, pos.y));
        }

        let vert_src = include_str!("widgets.vert.glsl");
        let frag_src = include_str!("widgets.frag.glsl");
        let program = glium::Program::from_source(&display, vert_src, frag_src, None).unwrap();

        let image = glium::texture::RawImage2d::from_raw_rgba(vec![255u8; 4], (1, 1));
        let t_white = glium::texture::Texture2d::new(&display, image).unwrap();

        Self {
            display,
            program,
            t_white,
            draw_queue: DrawQueue::new(),
            cur_attr: win_attr.clone(),
            last_pos: Default::default(),
            mod_state: Default::default(),
            button_state: Default::default(),
            window,
        }
    }

    fn draw_elements(&self, target: &mut glium::Frame) {
        let win_size = self.window.get_size();
        let vertex_buf = glium::VertexBuffer::new(&self.display, &self.draw_queue.vertices).unwrap();

        for drawcmd in &self.draw_queue.commands {
            match drawcmd {
                DrawCommand::Clear(color) => target.clear_color(color.r, color.g, color.b, color.a),
                DrawCommand::Primitives(cmd) => {
                    // clip the viewport against the visible window area
                    if let Some(scissor) = cmd.viewport.clip_inside(win_size.into()) {
                        let mode = match cmd.primitive {
                            Primitive::Points => PrimitiveType::Points,
                            Primitive::Lines => PrimitiveType::LinesList,
                            Primitive::LineStrip => PrimitiveType::LineStrip,
                            Primitive::Triangles => PrimitiveType::TrianglesList,
                            Primitive::TriangleStrip => PrimitiveType::TriangleStrip,
                            Primitive::TriangleFan => PrimitiveType::TriangleFan,
                        };
                        // indices reference a single shared vertex buffer
                        let indices = &self.draw_queue.indices[cmd.idx_offset..cmd.idx_offset + cmd.idx_len];
                        let index_buf = glium::IndexBuffer::new(&self.display, mode, indices).unwrap();
                        // settings for the pipeline
                        let uniforms = uniform! {
                            vp_size: win_size.as_pointf().components(),
                            tex: &self.t_white,
                        };
                        let draw_params = glium::DrawParameters {
                            blend: glium::Blend::alpha_blending(),
                            scissor: Some(glium::Rect {
                                left: scissor.x() as u32,
                                bottom: win_size.h - scissor.h() - scissor.y() as u32,
                                width: scissor.w(),
                                height: scissor.h(),
                            }),
                            ..Default::default()
                        };
                        // perform the draw command
                        target
                            .draw(&vertex_buf, &index_buf, &self.program, &uniforms, &draw_params)
                            .unwrap();
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        self.draw_queue.clear();
        self.window.draw(&mut self.draw_queue);
        let mut target = self.display.draw();
        self.draw_elements(&mut target);
        target.finish().unwrap();
    }

    pub fn update(&mut self) {
        self.window.update()
        //TODO: compare `self.cur_attr` with `self.window.get_window_attributes()` to make changes to real window
    }

    pub fn redraw(&self) {
        self.display.gl_window().window().request_redraw();
    }

    pub fn get_id(&self) -> WindowId {
        self.display.gl_window().window().id()
    }

    pub fn push_event(&mut self, event: WindowEvent) -> Option<WidgetId> {
        use widgets::event::Event;

        if let WindowEvent::ModifiersChanged(mod_state) = event {
            self.mod_state = ModState {
                shift: mod_state.shift(),
                ctrl: mod_state.ctrl(),
                alt: mod_state.alt(),
                meta: mod_state.logo(),
            };
        }

        translate_event(event).and_then(|event| {
            match event {
                Event::MouseMoved(AxisValue::Position(pos)) => {
                    self.last_pos = pos;
                }
                Event::MouseButton {
                    state: EvState::Pressed,
                    button,
                } => {
                    self.button_state.set(button);
                }
                Event::MouseButton {
                    state: EvState::Released,
                    button,
                } => {
                    self.button_state.unset(button);
                }
                Event::Resized(size) => {
                    self.cur_attr.set_size(size);
                    self.window.set_size(size);
                }
                Event::Moved(pos) => {
                    self.cur_attr.set_position(pos);
                    self.window.set_position(pos);
                }
                _ => (),
            }
            self.window.push_event(
                event,
                EventContext::new(self.last_pos, self.button_state, self.mod_state),
            )
        })
    }
}

pub struct GliumApplication<T> {
    event_loop: EventLoop<()>,
    window_map: HashMap<WindowId, GliumWindow<T>>,
}

impl<T: TopLevel + 'static> GliumApplication<T> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn add_window(&mut self, window: T) {
        let gl_win = GliumWindow::new(window, &self.event_loop);
        self.window_map.insert(gl_win.get_id(), gl_win);
    }

    pub fn run(self) -> ! {
        use glium::glutin::event::Event;

        let event_loop = self.event_loop;
        let mut window_map = self.window_map;

        event_loop.run(move |event, _, cf| {
            *cf = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, window_id } => {
                    //TODO: handle close request
                    if let WindowEvent::CloseRequested = event {
                        *cf = ControlFlow::Exit;
                    }

                    if let Some(window) = window_map.get_mut(&window_id) {
                        if window.push_event(event).is_some() {
                            window.redraw();
                        }
                    }
                }
                Event::MainEventsCleared => {
                    for window in window_map.values_mut() {
                        window.update();
                    }
                }
                Event::RedrawRequested(window_id) => {
                    if let Some(window) = window_map.get_mut(&window_id) {
                        window.draw();
                    }
                }
                _ => (),
            }
        })
    }
}

impl GliumApplication<Box<dyn TopLevel>> {
    #[inline]
    pub fn new_dyn() -> Self {
        Default::default()
    }
}

impl<T> Default for GliumApplication<T> {
    #[inline]
    fn default() -> Self {
        Self {
            event_loop: EventLoop::new(),
            window_map: Default::default(),
        }
    }
}