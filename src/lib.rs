pub mod layout;
pub mod event;
pub mod widget;
pub mod paint;
pub mod style;

const DEFAULT_SCALE: f32 = 20.0;

pub use gru_misc::{math, text_sdf as text, color};

pub struct Request<'a>
{
    widget: &'a mut bool,
    layout: &'a mut bool,
    paint: &'a mut bool
}

impl Request<'_>
{
    pub fn widget(&mut self)
    {
        *self.widget = true;
        *self.layout = true;
        *self.paint = true;
    }

    pub fn layout(&mut self)
    {
        *self.layout = true;
        *self.paint = true;
    }

    pub fn paint(&mut self)
    {
        *self.paint = true;
    }
}

pub struct EventCtx<'a, 'b>
{
    pub request: &'a mut Request<'b>,
    pub event: &'a mut event::EventPod
}

pub struct UpdateCtx<'a, 'b>
{
    pub request: &'a mut Request<'b>
}

pub struct WidgetComputeCtx
{
}

pub struct LayoutInquireCtx<'a>
{
    painter: &'a mut paint::Painter
}

pub struct LayoutComputeCtx<'a>
{
    fits: &'a mut bool,
    painter: &'a mut paint::Painter
}

pub struct PaintCtx<'a>
{
    painter: &'a mut paint::Painter
}

impl<'a> LayoutInquireCtx<'a>
{
    #[inline] pub fn text_with(&mut self, text: &str, size: f32) -> f32 { self.painter.text_width(text, size) }
}

impl<'a> LayoutComputeCtx<'a>
{
    #[inline] pub fn does_not_fit(&mut self) { *self.fits = false; }
    #[inline] pub fn text_with(&mut self, text: &str, size: f32) -> f32 { self.painter.text_width(text, size) }
}

impl<'a> PaintCtx<'a>
{
    #[inline] pub fn add_offset(&mut self, offset: math::Vec2) { self.painter.add_offset(offset); }
    #[inline] pub fn draw_rect(&mut self, rect: math::Rect, color: color::Color) { self.painter.draw_rect(rect, color); }
    #[inline] pub fn draw_rhombus(&mut self, rect: math::Rect, color: color::Color) { self.painter.draw_rhombus(rect, color); }
    #[inline] pub fn draw_text(&mut self, rect: math::Rect, text: &str, size: f32, align: text::Align, auto_wrap: bool, color: color::Color) { self.painter.draw_text(rect, text, size, align, auto_wrap, color); }
}

pub trait Widget<T>
{
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T);
    fn update(&mut self, ctx: &mut UpdateCtx, data: &mut T);
    fn widget_compute(&mut self, ctx: &mut WidgetComputeCtx, data: &mut T);
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> layout::SizeWish;
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: math::Vec2);
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, style: &style::StyleSet);
}

macro_rules! impl_event_empty { () => { #[inline] fn event(&mut self, _: &mut EventCtx, _: &mut T) {} } }
macro_rules! impl_update_empty { () => { #[inline] fn update(&mut self, _: &mut UpdateCtx, _: &mut T) {} } }
macro_rules! impl_widget_compute_empty { () => { #[inline] fn widget_compute(&mut self, _: &mut WidgetComputeCtx, _: &mut T) {} } }
//macro_rules! impl_layout_inquire_empty { () => { #[inline] fn layout_inquire(&mut self, _: &mut LayoutInquireCtx, _: &T) {} } }
macro_rules! impl_layout_compute_empty { () => { #[inline] fn layout_compute(&mut self, _: &mut LayoutComputeCtx, _: &T, _: Vec2) {} } }
macro_rules! impl_paint_empty { () => { #[inline] fn paint(&mut self, _: &mut PaintCtx, _: &mut T, _: &StyleSet) {} } }
pub(crate) use impl_event_empty;
pub(crate) use impl_update_empty;
pub(crate) use impl_widget_compute_empty;
//pub(crate) use impl_layout_inquire_empty;
pub(crate) use impl_layout_compute_empty;
pub(crate) use impl_paint_empty;

#[derive(Clone, PartialEq)]
pub struct UiConfig
{
    pub size: math::Vec2,
    pub scale: f32,
    pub display_scale_factor: f32
}

pub struct Frame<'a>
{
    pub fits: bool,
    pub events: &'a mut [event::EventPod],
    pub paint: paint::Frame<'a>
}

pub struct Ui<'a, T: 'a>
{
    widget: Box<dyn Widget<T> + 'a>,
    config: Option<UiConfig>,
    events: Vec<event::EventPod>,
    painter: paint::Painter,
    style: style::StyleSet
}

impl<'a, T: 'a> Ui<'a, T>
{
    pub fn new<W: Widget<T> + 'a>(font: text::Font, widget: W) -> Self
    {
        let widget = Box::new(widget);
        let config = None;
        let events = Vec::new();
        let painter = paint::Painter::new(font);
        let style = Default::default();
        Self { widget, config, events, painter, style }
    }

    pub fn style(&mut self) -> &mut style::StyleSet
    {
        &mut self.style
    }

    pub fn frame<'b>(&mut self, config: UiConfig, data: &mut T, events: impl Iterator<Item = &'b event::Event>) -> Frame
    {
        let (mut update_widget, mut update_layout, mut update_paint) = (false, false, false);
        let mut request = Request { widget: &mut update_widget, layout: &mut update_layout, paint: &mut update_paint };
        //config
        let config = Some(config);
        if self.config != config { request.layout(); }
        self.config = config;
        let config = self.config.as_ref().unwrap();
        let scale = config.scale * config.display_scale_factor * DEFAULT_SCALE;
        let size = config.size / scale;

        //events
        {
            self.events.clear();
            for event in events
            {
                let mut event = event::EventPod::new(event.clone());
                let mut ctx = EventCtx { request: &mut request, event: &mut event };

                ctx.event.event.scale(1.0 / scale);
                self.widget.event(&mut ctx, data);
                ctx.event.event.scale(scale);

                self.events.push(event);
            }
        }
        //update check
        {
            let mut ctx = UpdateCtx { request: &mut request };
            self.widget.update(&mut ctx, data);
        }
        //compute widgets
        if update_widget
        {
            let mut ctx = WidgetComputeCtx { };
            self.widget.widget_compute(&mut ctx, data);
        }
        //comput layout
        let mut fits = true;
        if update_layout
        {
            let mut ctx = LayoutInquireCtx { painter: &mut self.painter };
            let wish = self.widget.layout_inquire(&mut ctx, data);
            if !wish.fits(size) { fits = false; }
            //size logic
            let mut ctx = LayoutComputeCtx { fits: &mut fits, painter: &mut self.painter };
            self.widget.layout_compute(&mut ctx, data, size);
        }
        //compute painting
        if update_paint
        {
            self.painter.clear_frame(scale);
            let mut ctx = PaintCtx { painter: &mut self.painter };
            self.widget.paint(&mut ctx, data, &self.style);
        }

        //return
        let events = &mut self.events;
        let paint = self.painter.get_frame();
        Frame { fits, events, paint }
    }
}
