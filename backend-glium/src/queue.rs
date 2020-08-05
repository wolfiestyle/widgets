use crate::shared_res::SharedRes;
use crate::vertex::Vertex;
use glium::index::PrimitiveType;
use glium::texture::SrgbTexture2d;
use glium::GlObject;
use glium::{uniform, Surface};
use std::fmt;
use std::ops::Range;
use std::rc::Rc;
use widgets::draw::{Color, DrawBackend, TextDrawMode};
use widgets::geometry::{Rect, Size};
use widgets::image::Image;

/// Draw command detail.
#[derive(Debug, Clone)]
struct DrawCmdData {
    /// Range inside the shared vertex buffer.
    idx_range: Range<usize>,
    /// Image to use for this draw command.
    texture: Option<Rc<SrgbTexture2d>>,
    /// Clipping viewport.
    viewport: Rect,
}

impl DrawCmdData {
    #[inline]
    fn compatible_with(&self, viewport: Rect, texture: &Option<Rc<SrgbTexture2d>>) -> bool {
        self.viewport == viewport && self.texture.as_ref().map(|t| t.get_id()) == texture.as_ref().map(|t| t.get_id())
    }
}

/// A single draw command.
#[derive(Debug, Clone)]
enum DrawCommand {
    Clear(Color, Rect),
    Triangles(DrawCmdData),
}

/// Buffer with draw commands to be sent to the backend.
pub struct DrawQueue {
    /// Shared vertex buffer.
    vertices: Vec<Vertex>,
    /// Shared index buffer.
    indices: Vec<u32>,
    /// List of draw commands to be executed.
    commands: Vec<DrawCommand>,
    /// Shared OpenGL resources
    shared_res: Rc<SharedRes>,
    /// Glium context handle
    pub display: glium::Display,
}

impl DrawQueue {
    pub fn new(display: glium::Display, shared_res: Rc<SharedRes>) -> Self {
        Self {
            vertices: Default::default(),
            indices: Default::default(),
            commands: Default::default(),
            shared_res,
            display,
        }
    }

    /// Clears all data from the draw queue.
    #[inline]
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.commands.clear();
        //self.texture_map.remove_expired();  //TODO: GC old textures
    }

    /// Adds a clear command to the draw queue.
    #[inline]
    fn push_clear(&mut self, color: Color, viewport: Rect) {
        self.commands.push(DrawCommand::Clear(color, viewport))
    }

    /// Adds raw elements to the draw queue.
    #[inline]
    fn push_tris<V, I>(&mut self, vertices: V, indices: I, texture: Option<Rc<SrgbTexture2d>>, viewport: Rect)
    where
        V: Iterator<Item = Vertex>,
        I: Iterator<Item = u32>,
    {
        // append vertices to the buffer
        let base_vert = self.vertices.len() as u32;
        self.vertices.extend(vertices);
        // indices are added with an offset pointing to a single vertex buffer
        let base_idx = self.indices.len();
        self.indices.extend(indices.map(|i| i + base_vert));

        // check if the last draw command has the same state of the incoming one
        match self.commands.last_mut() {
            Some(DrawCommand::Triangles(cmd)) if cmd.compatible_with(viewport, &texture) => {
                // ..then we only need to add more indices
                cmd.idx_range.end = self.indices.len();
            }
            _ => {
                // state change, we need to create a new draw command
                self.commands.push(DrawCommand::Triangles(DrawCmdData {
                    idx_range: base_idx..self.indices.len(),
                    texture,
                    viewport,
                }));
            }
        }
    }

    /// Runs the stored draw commands.
    pub fn execute(&self, win_size: Size) {
        let vertex_buf = glium::VertexBuffer::new(&self.display, &self.vertices).unwrap();
        let index_buf = glium::index::IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &self.indices).unwrap();
        let mut draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let mut target = self.display.draw();

        for drawcmd in &self.commands {
            match drawcmd {
                DrawCommand::Clear(color, viewport) => {
                    if let Some(vp) = viewport.clip_inside(win_size.into()) {
                        let rect = to_glium_rect(vp, win_size.h);
                        target.clear(Some(&rect), Some((color.r, color.g, color.b, color.a)), false, None, None);
                    }
                }
                DrawCommand::Triangles(cmd) => {
                    // clip the viewport against the visible window area
                    if let Some(scissor) = cmd.viewport.clip_inside(win_size.into()) {
                        // indices reference a single shared vertex buffer
                        let indices = index_buf.slice(cmd.idx_range.clone()).unwrap();
                        // get texture to use
                        let texture = cmd.texture.as_deref().unwrap_or(&self.shared_res.t_white);
                        // settings for the pipeline
                        let uniforms = uniform! {
                            vp_size: <[f32; 2]>::from(win_size.as_point()),
                            tex: texture.sampled()
                                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                        };
                        draw_params.scissor = Some(to_glium_rect(scissor, win_size.h));
                        // perform the draw command
                        target
                            .draw(&vertex_buf, indices, &self.shared_res.program, &uniforms, &draw_params)
                            .unwrap();
                    }
                }
            }
        }

        target.finish().unwrap();
    }
}

impl DrawBackend for DrawQueue {
    type Vertex = Vertex;

    #[inline]
    fn clear(&mut self, color: Color, viewport: Rect) {
        self.push_clear(color, viewport)
    }

    #[inline]
    fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, image: Option<&Image>, viewport: Rect)
    where
        V: IntoIterator<Item = Self::Vertex>,
        I: IntoIterator<Item = u32>,
    {
        let texture = image.map(|img| self.shared_res.load_texture(img));
        self.push_tris(vertices.into_iter(), indices.into_iter(), texture, viewport)
    }

    #[allow(unused_variables)]
    #[inline]
    fn draw_text(&mut self, text: &str, font_desc: &str, mode: TextDrawMode, color: Color, viewport: Rect) {
        todo!() //TODO: implement text drawing
    }
}

impl fmt::Debug for DrawQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GliumWindow")
            .field("vertices", &self.vertices)
            .field("indices", &self.indices)
            .field("commands", &self.commands)
            .field("shared_res", &self.shared_res)
            .field("display", &format_args!("..."))
            .finish()
    }
}

fn to_glium_rect(rect: widgets::geometry::Rect, win_height: u32) -> glium::Rect {
    glium::Rect {
        left: rect.pos.x as u32,
        bottom: win_height - rect.size.h - rect.pos.y as u32,
        width: rect.size.w,
        height: rect.size.h,
    }
}
