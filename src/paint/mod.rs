pub mod style;

use gru_misc::{math::{Vec2, Rect}, text_sdf::{Font, AtlasBuilder, Align, Layout}};

pub const TEXTURE_SIZE: u32 = 1024;
const TEXTURE_PADDING: u32 = 5;

pub use gru_misc::color::Color;

pub struct Vertex
{
    pub position: Vec2,
    pub color: Color,
    pub tex_coords: Option<(f32, f32, u32)>
}

pub struct Frame<'a>
{
    pub new: bool,
    pub vertices: &'a [Vertex],
    pub indices: &'a [u16],
    pub font_version: u64,
    pub font_data: &'a Vec<Vec<u8>>
}

pub(crate) struct Painter
{
    text_version: u64,
    text: Option<AtlasBuilder>,
    origin: Vec2,
    scale: f32,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    new: bool
}

enum Corner
{
    TopLeft,
    BottomLeft,
    BottomRight,
    TopRight,
}

impl Corner
{
    fn of(self, rect: Rect) -> Vec2 {
        match self
        {
            Corner::TopLeft => rect.min,
            Corner::BottomLeft => Vec2(rect.min.0, rect.max.1),
            Corner::BottomRight => rect.max,
            Corner::TopRight => Vec2(rect.max.0, rect.min.1),
        }
    }

    fn all_of(rect: Rect) -> [Vec2; 4]
    {
        [
            Corner::TopLeft,
            Corner::BottomLeft,
            Corner::BottomRight,
            Corner::TopRight,
        ].map(|corner| corner.of(rect))   
    }
}

impl Painter
{
    fn atlas_builder(font: Font, scale: f32) -> AtlasBuilder
    {
        let mut builder = AtlasBuilder::new(font, 3.0 * scale, TEXTURE_SIZE, TEXTURE_PADDING);
        builder.add(Font::digits().chain(Font::all_letters()).chain(Font::text_special_characters()));
        builder.atlas_mut().default(Some('?'));
        builder
    }

    pub fn new(font: Font) -> Self
    {
        Self
        {
            text_version: 0,
            text: Some(Self::atlas_builder(font, 1.0)),
            origin: Vec2(0.0, 0.0),
            scale: 1.0,
            vertices: Vec::new(),
            indices: Vec::new(),
            new: true
        }
    }

    pub fn add_offset(&mut self, offset: Vec2)
    {
        self.origin += offset;
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color)
    {
        let rect = (rect + self.origin) * self.scale;
        let i0 = self.vertices.len() as u16;
        for position in Corner::all_of(rect) { self.vertices.push(Vertex { position, color, tex_coords: None }); }
        for i in [0, 1, 2, 2, 3, 0] { self.indices.push(i0 + i); }
    }

    pub fn draw_frame(&mut self, outside: Rect, inside: Rect, color: Color)
    {
        let outside = (outside + self.origin) * self.scale;
        let inside = (inside + self.origin) * self.scale;
        // Trapezoidal pattern
        // Vertex order (outside-inside back-and-forth) counterclockwise
        use Corner::*;
        let all_corners = [
            TopLeft.of(inside),      //0
            TopLeft.of(outside),     //1
            BottomLeft.of(outside),  //2
            BottomLeft.of(inside),   //3
            BottomRight.of(inside),  //4
            BottomRight.of(outside), //5
            TopRight.of(outside),    //6
            TopRight.of(inside),     //7
        ];

        let i0 = self.vertices.len() as u16;
        for position in all_corners { self.vertices.push(Vertex { position, color, tex_coords: None }); }
        
        let left_side = [0, 1, 2, 2, 3, 0];   // 0 1 2 3 -> 0 1 2 3
        let bottom_side = [3, 2, 5, 5, 4, 3]; // 0 1 2 3 -> 3 2 5 4
        let right_side = [4, 5, 6, 6, 7, 4];  // 0 1 2 3 -> 4 5 6 7
        let top_side = [7, 6, 1, 1, 0, 7];    // 0 1 2 3 -> 7 6 1 0
        for side in [left_side, bottom_side, right_side, top_side]
        {
            for i in side { self.indices.push(i0 + i); }
        }
    }

    pub fn draw_rhombus(&mut self, rect: Rect, color: Color)
    {
        let rect = (rect + self.origin) * self.scale;
        let radius = rect.size() / 2.0;
        let min = rect.min;
        let max = rect.max;
        let i0 = self.vertices.len() as u16;
        for position in
        [
            Vec2(min.0, min.1 + radius.1), //left
            Vec2(min.0 + radius.0, max.1), //bottom
            Vec2(max.0, min.1 + radius.1), //right
            Vec2(min.0 + radius.0, min.1) //top
        ] { self.vertices.push(Vertex { position, color, tex_coords: None }); }
        for i in [0, 1, 2, 2, 3, 0] { self.indices.push(i0 + i); }
    }

    pub fn draw_text(&mut self, rect: Rect, text: &str, size: f32, align: Align, auto_wrap: bool, color: Color)
    {
        self.add_glyphs(text);
        let atlas_builder = self.text.as_mut().unwrap();
        let width = (rect.max.0 - rect.min.0) / size;
        let offset = self.origin + rect.min + Vec2(0.0, (rect.max.1 - rect.min.1 - size) / 2.0);
        let i0 = self.vertices.len() as u16;
        atlas_builder.atlas().text
        (
            text,
            Layout { width, align, auto_wrap },
            |index| self.indices.push(i0 + index as u16),
            |tex_coords, position| self.vertices.push(Vertex { position: (Vec2::from(position) * size + offset) * self.scale, color, tex_coords: Some(tex_coords) })
        );
    }

    pub fn clear_frame(&mut self, scale: f32)
    {
        if self.scale != scale
        {
            self.text = Some(Self::atlas_builder(self.text.take().unwrap().into_font(), scale));
            self.text_version += 1;
        }
        self.origin = Vec2(0.0, 0.0);
        self.scale = scale;
        self.vertices.clear();
        self.indices.clear();
        self.new = true;
    }

    pub fn get_frame<'a>(&'a mut self) -> Frame<'a>
    {
        let new = self.new;
        self.new = false;
        Frame { new, vertices: &self.vertices, indices: &self.indices, font_version: self.text_version, font_data: self.text.as_ref().unwrap().sdf() }
    }

    pub fn text_width(&mut self, text: &str, size: f32) -> f32
    {
        self.add_glyphs(text);
        self.text.as_ref().unwrap().atlas().width(text) * size
    }

    pub fn text_height(&mut self, text: &str, layout: Layout) -> u32
    {
        self.add_glyphs(text);
        self.text.as_ref().unwrap().atlas().height(text, layout)
    }

    fn add_glyphs(&mut self, text: &str)
    {
        if self.text.as_mut().unwrap().add(text.chars()) { self.text_version += 1; }
    }
}
