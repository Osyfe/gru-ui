use super::*;
use std::borrow::Borrow;

const DEFAULT_LENGTH: f32 = 10.0;

pub struct Bg<T, E, W: Widget<T, E>>
{
    child: W,
    size: Vec2,
    _phantom: PhantomData<(T, E)>
}

impl<T, E, W: Widget<T, E>> Widget<T, E> for Bg<T, E, W>
{
    impl_event_child!(T);
    impl_update_child!(T);
    impl_widget_compute_child!(T);
    impl_layout_inquire_child!(T);

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        self.size = self.child.layout_compute(ctx, data, size);
        self.size
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        ctx.draw_rect(Rect::new_origin(self.size), ctx.style.bg.get(ctx.state));
        self.child.paint(ctx, data);
    }

    impl_respond_child!(T);
}

impl<T, E, W: Widget<T, E>> Bg<T, E, W>
{
    pub fn new(widget: W) -> Self
    {
        Self { child: widget, size: Vec2::zero(), _phantom: PhantomData }
    }
}

pub struct Label<T: Borrow<str>>
{
    text_size: f32,
    size: Vec2,
    _phantom: PhantomData<T>
}

impl<T: Borrow<str>, E> Widget<T, E> for Label<T>
{
    impl_event_empty!(T);
    impl_update_empty!(T);
    impl_widget_compute_empty!(T);

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        let width = ctx.text_width(data.borrow(), self.text_size);
        self.size = Vec2(width, self.text_size);
        self.size
    }

    #[inline] fn layout_compute(&mut self, _: &mut LayoutComputeCtx, _: &T, _: Vec2) -> Vec2 { self.size }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        ctx.draw_text(Rect::new_origin(self.size), data.borrow(), self.text_size, text::Align::Left, false, ctx.style.text);
    }

    impl_respond_empty!(T);
}

impl<T: Borrow<str>> Label<T>
{
    pub fn new() -> Self
    {
        Self { text_size: 1.0, size: Vec2::zero(), _phantom: PhantomData }
    }

    pub fn size(mut self, text_size: f32) -> Self
    {
        self.text_size = text_size;
        self
    }
}

pub struct Text<T: Borrow<str>>
{
    text_size: f32,
    align: text::Align,
    wish_width: f32,
    actual_size: Vec2,
    _phantom: PhantomData<T>
}

impl<T: Borrow<str>, E> Widget<T, E> for Text<T>
{
    impl_event_empty!(T);
    impl_update_empty!(T);
    impl_widget_compute_empty!(T);

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        let width = self.wish_width / self.text_size;
        let height = ctx.text_height(data.borrow(), text::Layout { width, align: self.align, auto_wrap: true }) as f32 * self.text_size;
        Vec2(width * self.text_size, height)
    }

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        let width = self.wish_width.max(size.0) / self.text_size;
        let height = ctx.text_height(data.borrow(), text::Layout { width, align: self.align, auto_wrap: true }) as f32 * self.text_size;
        self.actual_size = Vec2(width * self.text_size, height);
        self.actual_size
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        ctx.draw_text(Rect::new_origin(self.actual_size), data.borrow(), self.text_size, self.align, true, ctx.style.text);
    }

    impl_respond_empty!(T);
}

impl<T: Borrow<str>> Text<T>
{
    pub fn new() -> Self
    {
        Self { text_size: 1.0, align: text::Align::Block, wish_width: DEFAULT_LENGTH, actual_size: Vec2::zero(), _phantom: PhantomData }
    }

    pub fn size(mut self, text_size: f32) -> Self
    {
        self.text_size = text_size;
        self
    }

    pub fn align(mut self, align: text::Align) -> Self
    {
        self.align = align;
        self
    }

    pub fn default_width(mut self, width: f32) -> Self
    {
        self.wish_width = width;
        self
    }
}

pub struct Check<E>
{
    size: f32,
    _phantom: PhantomData<E>
}

impl<E> Widget<bool, E> for Check<E>
{
    impl_event_empty!(bool);
    impl_update_empty!(bool);
    impl_widget_compute_empty!(bool);

    #[inline] fn layout_inquire(&mut self, _: &mut LayoutInquireCtx, _: &bool) -> Vec2 { Vec2(self.size, self.size) }
    #[inline] fn layout_compute(&mut self, _: &mut LayoutComputeCtx, _: &bool, _: Vec2) -> Vec2 { Vec2(self.size, self.size) }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &bool)
    {
        let size1 = self.size;
        let (size2, size3) = (size1 * 0.15, size1 * 0.7);
        let (size4, size5) = (size1 * 0.3, size1 * 0.4);
        ctx.painter.draw_rect(Rect::new_origin(Vec2(size1, size1)), ctx.style.top);
        ctx.painter.draw_rect(Rect::new_size(Vec2(size2, size2), Vec2(size3, size3)), ctx.style.data.get(ctx.state));
        if *data { ctx.painter.draw_rhombus(Rect::new_size(Vec2(size4, size4), Vec2(size5, size5)), ctx.style.top); }
    }

    #[inline]
    fn respond(&mut self, data: &mut bool, button: Option<MouseButton>) -> bool
    {
        if button == Some(MouseButton::Primary) { *data = !*data; true } else { false }
    }
}

impl<E> Check<E>
{
    pub fn new() -> Self
    {
        Self { size: 1.0, _phantom: PhantomData }
    }

    pub fn size(mut self, size: f32) -> Self
    {
        self.size = size;
        self
    }
}

pub struct Slider<E>
{
    min: f32,
    max: f32,
    step: f32,
    wish_size: Vec2,
    actual_size: Vec2,
    dragged: bool,
    _phantom: PhantomData<E>
}

impl<E> Widget<f32, E> for Slider<E>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut f32)
    {
        let size = self.actual_size;
        if !ctx.event.used
        {
            match ctx.event.event
            {
                HardwareEvent::PointerClicked { pos, button: MouseButton::Primary, pressed } =>
                {
                    if pressed
                    {
                        if pos.1 >= 0.0 && pos.1 <= size.1 //height bound
                        {
                            if pos.0 >= -0.5 && pos.0 <= size.0 + 0.5 //relaxed width bound
                            {
                                self.dragged = true;
                                ctx.event.used = true;
                            }
                            let f = pos.0 / size.0;
                            if f >= 0.0 && f <= 1.0 //strict width bound
                            {
                                *data = (f * (self.max - self.min) / self.step).round() * self.step + self.min;
                                ctx.request.paint();
                            }
                        }
                    } else if self.dragged
                    {
                        self.dragged = false;
                        ctx.request.paint();
                        ctx.event.used = true;
                    }                    
                },
                HardwareEvent::PointerGone => if self.dragged
                {
                    self.dragged = false;
                    ctx.request.paint();
                }
                HardwareEvent::PointerMoved { pos, .. } => if self.dragged
                {
                    let f = (pos.0 / size.0).max(0.0).min(1.0);
                    let new = (f * (self.max - self.min) / self.step).round() * self.step + self.min;
                    if new != *data
                    {
                        *data = new;
                        ctx.request.paint();
                    }
                },
                HardwareEvent::Scroll { pos, delta } =>
                {
                    if pos.1 >= 0.0 && pos.1 <= size.1 //height bound
                    && pos.0 >= -0.5 && pos.0 <= size.0 + 0.5 //relaxed width bound
                    {
                        *data = (*data + delta.1 * self.step).max(self.min).min(self.max);
                        ctx.request.paint();
                        ctx.event.used = true;
                    }
                },
                _ => {}
            }
        }
    }

    impl_update_empty!(f32);
    impl_widget_compute_empty!(f32);

    #[inline] fn layout_inquire(&mut self, _: &mut LayoutInquireCtx, _: &f32) -> Vec2 { self.wish_size }

    #[inline]
    fn layout_compute(&mut self, _: &mut LayoutComputeCtx, _: &f32, size: Vec2) -> Vec2
    {
        self.actual_size = Vec2(self.wish_size.0.max(size.0), self.wish_size.1);
        self.actual_size
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &f32)
    {
        let size = self.actual_size;
        let pos = (data - self.min) / (self.max - self.min) * size.0;
        let (x0, x1, x2, x3) = (0.0, pos - 0.5 * size.1, pos + 0.5 * size.1, size.0);
        let (y0, y1, y2, y3) = (0.0, size.1 / 3.0, size.1 / 1.5, size.1);
        ctx.painter.draw_rect(Rect { min: Vec2(x0, y1), max: Vec2(x3, y2) }, ctx.style.top);
        ctx.painter.draw_rhombus(Rect { min: Vec2(x1, y0), max: Vec2(x2, y3) }, if self.dragged { ctx.style.data.hot } else { ctx.style.data.get(ctx.state) });
    }

    impl_respond_empty!(f32);
}

impl<E> Slider<E>
{
    pub fn new() -> Self
    {
        Self { min: 0.0, max: 1.0, step: 0.1, wish_size: Vec2(DEFAULT_LENGTH, 1.0), actual_size: Vec2::zero(), dragged: false, _phantom: PhantomData }
    }

    pub fn min(mut self, min: f32) -> Self
    {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self
    {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self
    {
        self.step = step;
        self
    }

    pub fn default_width(mut self, width: f32) -> Self
    {
        self.wish_size.0 = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self
    {
        self.wish_size.1 = height;
        self
    }
}

pub struct VSlider<E>
{
    min: f32,
    max: f32,
    step: f32,
    wish_size: Vec2,
    actual_size: Vec2,
    dragged: bool,
    _phantom: PhantomData<E>
}

impl<E> Widget<f32, E> for VSlider<E>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut f32)
    {
        let size = self.actual_size;
        if !ctx.event.used
        {
            match ctx.event.event
            {
                HardwareEvent::PointerClicked { pos, button: MouseButton::Primary, pressed } =>
                {
                    if pressed
                    {
                        if pos.0 >= 0.0 && pos.0 <= size.0 //width bound
                        {
                            if pos.1 >= -0.5 && pos.1 <= size.1 + 0.5 //relaxed height bound
                            {
                                self.dragged = true;
                                ctx.event.used = true;
                            }
                            let f = pos.1 / size.1;
                            if f >= 0.0 && f <= 1.0 //strict height bound
                            {
                                *data = (f * (self.max - self.min) / self.step).round() * self.step + self.min;
                                ctx.request.paint();
                            }
                        }
                    } else if self.dragged
                    {
                        self.dragged = false;
                        ctx.request.paint();
                        ctx.event.used = true;
                    }
                },
                HardwareEvent::PointerGone => if self.dragged
                {
                    self.dragged = false;
                    ctx.request.paint();
                }
                HardwareEvent::PointerMoved { pos, .. } => if self.dragged
                {
                    let f = (pos.1 / size.1).max(0.0).min(1.0);
                    let new = (f * (self.max - self.min) / self.step).round() * self.step + self.min;
                    if new != *data
                    {
                        *data = new;
                        ctx.request.paint();
                    }
                },
                HardwareEvent::Scroll { pos, delta } =>
                {
                    if pos.0 >= 0.0 && pos.0 <= size.0 //width bound
                    && pos.1 >= -0.5 && pos.1 <= size.1 + 0.5 //relaxed height bound
                    {
                        *data = (*data - delta.1 * self.step).max(self.min).min(self.max);
                    }
                    ctx.request.paint();
                },
                _ => {}
            }
        }
    }

    impl_update_empty!(f32);
    impl_widget_compute_empty!(f32);

    #[inline] fn layout_inquire(&mut self, _: &mut LayoutInquireCtx, _: &f32) -> Vec2 { self.wish_size }

    #[inline]
    fn layout_compute(&mut self, _: &mut LayoutComputeCtx, _: &f32, size: Vec2) -> Vec2
    {
        self.actual_size = Vec2(self.wish_size.0, self.wish_size.1.max(size.1));
        self.actual_size
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &f32)
    {
        let size = self.actual_size;
        let pos = (data - self.min) / (self.max - self.min) * size.1;
        let (x0, x1, x2, x3) = (0.0, size.0 / 3.0, size.0 / 1.5, size.0);
        let (y0, y1, y2, y3) = (0.0, pos - 0.5 * size.0, pos + 0.5 * size.0, size.1);
        ctx.painter.draw_rect(Rect { min: Vec2(x1, y0), max: Vec2(x2, y3) }, ctx.style.top);
        ctx.painter.draw_rhombus(Rect { min: Vec2(x0, y1), max: Vec2(x3, y2) }, if self.dragged { ctx.style.data.hot } else { ctx.style.data.get(ctx.state) });
    }

    impl_respond_empty!(f32);
}

impl<E> VSlider<E>
{
    pub fn new() -> Self
    {
        Self { min: 0.0, max: 1.0, step: 0.1, wish_size: Vec2(1.0, DEFAULT_LENGTH), actual_size: Vec2::zero(), dragged: false, _phantom: PhantomData }
    }

    pub fn min(mut self, min: f32) -> Self
    {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self
    {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self
    {
        self.step = step;
        self
    }

    pub fn width(mut self, width: f32) -> Self
    {
        self.wish_size.0 = width;
        self
    }

    pub fn default_height(mut self, height: f32) -> Self
    {
        self.wish_size.1 = height;
        self
    }
}

pub struct Edit<'a, E>
{
    active: bool,
    filter: Box<dyn FnMut(char) -> bool + 'a>,
    max_length: Option<usize>,
    wish_size: Vec2,
    actual_size: Vec2,
    _phantom: PhantomData<E>
}

impl<'a, E> Widget<String, E> for Edit<'a, E>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut String)
    {
        if self.active && !ctx.event.used
        {
            if let HardwareEvent::Char(ch) = ctx.event.event
            {
                ctx.event.used = true;
                if (self.filter)(ch) && self.max_length.map(|max| data.chars().count() < max).unwrap_or(true) { data.push(ch); }
                ctx.request.paint();
            }
            if let HardwareEvent::Key { key, pressed: true } = ctx.event.event
            {
                if key == Key::Back
				{
					ctx.event.used = true;
					data.pop();
					ctx.request.paint();
				}
            }
        }
    }

    impl_update_empty!(String);
    impl_widget_compute_empty!(String);

    #[inline] fn layout_inquire(&mut self, _: &mut LayoutInquireCtx, _: &String) -> Vec2 { self.wish_size }

    #[inline]
    fn layout_compute(&mut self, _: &mut LayoutComputeCtx, _: &String, size: Vec2) -> Vec2
    {
        self.actual_size = Vec2(self.wish_size.0.max(size.0), self.wish_size.1);
        self.actual_size
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &String)
    {
        let size = self.actual_size;
        let rect = Rect::new_origin(size);
        ctx.painter.draw_rect(rect, ctx.style.data.get(ctx.state));
        let display_data = data.clone() + if self.active && self.max_length.map_or(true, |ml| data.len() < ml) { "_" } else { "" };
        ctx.painter.draw_text(rect, &display_data, size.1, text::Align::Left, false, ctx.style.text);
    }

    #[inline]
    fn respond(&mut self, data: &mut String, button: Option<MouseButton>) -> bool
    {
        self.active = button.is_some();
        if let Some(MouseButton::Secondary) = button
        {
            use copypasta::{ClipboardContext, ClipboardProvider};
            let mut ctx = ClipboardContext::new().unwrap();
            if let Ok(contents) = ctx.get_contents() { for ch in contents.chars().filter(|ch| (self.filter)(*ch)) { data.push(ch); } }
        }
        true
    }
}

impl<'a, E> Edit<'a, E>
{
    pub fn new() -> Self
    {
        Self { active: false, filter: Box::new(|_| true), max_length: None, wish_size: Vec2(DEFAULT_LENGTH, 1.0), actual_size: Vec2::zero(), _phantom: PhantomData }
    }

    pub fn filter(mut self, filter: impl FnMut(char) -> bool + 'a) -> Self
    {
        self.filter = Box::new(filter) as Box<dyn FnMut(char) -> bool>;
        self
    }

    pub fn max_length(mut self, length: usize) -> Self
    {
        self.max_length = Some(length);
        self
    }

    pub fn default_width(mut self, width: f32) -> Self
    {
        self.wish_size.0 = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self
    {
        self.wish_size.1 = height;
        self
    }
}
