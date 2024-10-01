pub mod math { pub use gru_misc::math::{Vec2, Rect}; }
pub mod text { pub use gru_misc::text_sdf::{Font, Align, Layout}; }
pub mod event;
pub mod paint;
pub mod widget;
pub mod lens;

use paint::style;
use widget::*;

const DEFAULT_SCALE: f32 = 20.0;

pub struct Request
{
    widget: bool,
    layout: bool,
    paint: bool
}

impl Request
{
    pub fn widget(&mut self)
    {
        self.widget = true;
        self.layout = true;
        self.paint = true;
    }

    pub fn layout(&mut self)
    {
        self.layout = true;
        self.paint = true;
    }

    pub fn paint(&mut self)
    {
        self.paint = true;
    }

    fn reset(&mut self)
    {
        self.widget = false;
        self.layout = false;
        self.paint = false;
    }
}

pub struct EventCtx<'a, L>
{
    pub request: &'a mut Request,
    pub event: &'a mut event::EventPod,
    events: &'a mut Vec<event::Event<L>>
}

pub struct UpdateCtx<'a>
{
    pub request: &'a mut Request
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
    painter: &'a mut paint::Painter,
    style: &'a mut style::StyleSet,
    state: interact::WidgetState
}

impl<'a, E> EventCtx<'a, E>
{
    #[inline] pub fn emit(&mut self, event: event::LogicEvent<E>) { self.events.push(event::Event::Logic(event)); }
}

impl<'a> LayoutInquireCtx<'a>
{
    #[inline] pub fn text_width(&mut self, text: &str, size: f32) -> f32 { self.painter.text_width(text, size) }
    #[inline] pub fn text_height(&mut self, text: &str, layout: text::Layout) -> u32 { self.painter.text_height(text, layout) }
}

impl<'a> LayoutComputeCtx<'a>
{
    #[inline] pub fn does_not_fit(&mut self) { *self.fits = false; }
    #[inline] pub fn text_width(&mut self, text: &str, size: f32) -> f32 { self.painter.text_width(text, size) }
    #[inline] pub fn text_height(&mut self, text: &str, layout: text::Layout) -> u32 { self.painter.text_height(text, layout) }
}

impl<'a> PaintCtx<'a>
{
    #[inline] pub fn add_offset(&mut self, offset: math::Vec2) { self.painter.add_offset(offset); }
    #[inline] pub fn draw_rect(&mut self, rect: math::Rect, color: paint::Color) { self.painter.draw_rect(rect, color); }
    #[inline] pub fn draw_rhombus(&mut self, rect: math::Rect, color: paint::Color) { self.painter.draw_rhombus(rect, color); }
    #[inline] pub fn draw_text(&mut self, rect: math::Rect, text: &str, size: f32, align: text::Align, auto_wrap: bool, color: paint::Color) { self.painter.draw_text(rect, text, size, align, auto_wrap, color); }
}

pub trait Widget<T, E>
{
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T);
    fn update(&mut self, ctx: &mut UpdateCtx, data: &mut T);
    fn widget_compute(&mut self, ctx: &mut WidgetComputeCtx, data: &mut T);
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> math::Vec2;
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: math::Vec2) -> math::Vec2;
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T);
    fn respond(&mut self, data: &mut T, button: Option<event::MouseButton>) -> bool; //update
}

#[derive(Clone, PartialEq)]
pub struct UiConfig
{
    pub size: math::Vec2,
    pub scale: f32,
    pub display_scale_factor: f32
}

pub struct Frame<'a, E>
{
    pub fits: bool,
    pub events: &'a mut [event::Event<E>],
    pub paint: paint::Frame<'a>,
    pub request: &'a mut Request
}

pub struct Ui<'a, T: 'a, E>
{
    widget: Box<dyn Widget<T, E> + 'a>,
    config: Option<UiConfig>,
    request: Request,
    events: Vec<event::Event<E>>,
    painter: paint::Painter,
    style: style::StyleSet
}

impl<'a, T: 'a, E> Ui<'a, T, E>
{
    pub fn new<W: Widget<T, E> + 'a>(font: text::Font, widget: W) -> Self
    {
        let widget = Box::new(widget);
        let config = None;
        let request = Request { widget: true, layout: true, paint: true };
        let events = Vec::new();
        let painter = paint::Painter::new(font);
        let style = Default::default();
        Self { widget, config, request, events, painter, style }
    }

    pub fn request(&mut self) -> &mut Request
    {
        &mut self.request
    }

    pub fn style(&mut self) -> &mut style::StyleSet
    {
        &mut self.style
    }

    pub fn frame<'b>(&mut self, config: UiConfig, data: &mut T, events: impl Iterator<Item = &'b event::HardwareEvent>) -> Frame<E>
    {
        //config
        let config = Some(config);
        if self.config != config { self.request.layout(); }
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
                let mut ctx = EventCtx { request: &mut self.request, event: &mut event , events: &mut self.events };

                ctx.event.event.scale(1.0 / scale);
                self.widget.event(&mut ctx, data);
                ctx.event.event.scale(scale);

                self.events.push(event::Event::Hardware(event));
            }
        }
        //update check
        {
            let mut ctx = UpdateCtx { request: &mut self.request };
            self.widget.update(&mut ctx, data);
        }
        //compute widgets
        if self.request.widget
        {
            let mut ctx = WidgetComputeCtx { };
            self.widget.widget_compute(&mut ctx, data);
        }
        //compute layout
        let mut fits = true;
        if self.request.layout
        {
            let mut ctx = LayoutInquireCtx { painter: &mut self.painter };
            let min_size = self.widget.layout_inquire(&mut ctx, data);
            if min_size.0 < size.0 || min_size.1 < size.1 { fits = false; }
            //size logic
            let mut ctx = LayoutComputeCtx { fits: &mut fits, painter: &mut self.painter };
            self.widget.layout_compute(&mut ctx, data, size);
        }
        //compute painting
        if self.request.paint
        {
            self.painter.clear_frame(scale);
            let mut ctx = PaintCtx { painter: &mut self.painter, style: &mut self.style, state: interact::WidgetState::Cold };
            self.widget.paint(&mut ctx, data);
        }

        //return
        self.request.reset();
        let events = &mut self.events;
        let paint = self.painter.get_frame();
        let request = &mut self.request;
        Frame { fits, events, paint, request }
    }
}
