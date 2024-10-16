use super::{*, math::{Vec2, Rect}, text, event::{HardwareEvent, LogicEvent, MouseButton, Key}, lens::Lens};
use std::marker::PhantomData;
use macros::*;

pub mod primitive;
pub mod data;
pub mod layout;
pub mod interact;
pub mod compose;
pub mod dynamic;

pub trait WidgetExt<T, E>: Widget<T, E> + Sized
{
    //general
    fn boxed<'a>(self) -> Box<dyn Widget<T, E> + 'a> where Self: 'a { Box::new(self) }
    //primitive
    fn bg(self) -> primitive::Bg<T, E, Self> { primitive::Bg::new(self) }
    //data
    fn lens<U, L: Lens<U, T>>(self, lens: L) -> data::Lensing<U, T, E, Self, L> { data::Lensing::new(self, lens) }
    fn own<U>(self, data: T) -> data::Owning<U, T, E, Self> { data::Owning::new(self, data) }
    fn map<'a, U, F: FnMut(&U) -> T + 'a>(self, f: F) -> data::Map<'a, U, T, E, Self, F> { data::Map::new(self, f) }
    //layout
    fn fix(self) -> layout::Fix<T, E, Self> { layout::Fix::new(self) }
    fn align(self) -> layout::Align<T, E, Self> { layout::Align::new(self) }
    fn pad(self) -> layout::Padding<T, E, Self> { layout::Padding::new(self) }
    //interact
    fn response<'a>(self) -> interact::Response<'a, T, E, Self> where E: Clone { interact::Response::new(self) }
    //composition
    fn maybe<'a, F: FnMut(&mut T) -> bool + 'a>(self, f: F) -> compose::Maybe<'a, T, E, Self, F> { compose::Maybe::new(self, f) }
    fn and<W2: Widget<T, E>>(self, other: W2) -> compose::And<T, E, Self, W2> { compose::And::new(self, other) }
    //dynamic
    fn watch(self) -> dynamic::Watch<T, E, Self> where T: Clone + PartialEq { dynamic::Watch::new(self) }
}

impl<T, E, W: Widget<T, E>> WidgetExt<T, E> for W {}

impl<'a, T, E> Widget<T, E> for Box<dyn Widget<T, E> + 'a>
{
    #[inline] fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T) { self.as_mut().event(ctx, data); }
    #[inline] fn update(&mut self, ctx: &mut UpdateCtx, data: &mut T) { self.as_mut().update(ctx, data); }
    #[inline] fn widget_compute(&mut self, ctx: &mut WidgetComputeCtx, data: &mut T) { self.as_mut().widget_compute(ctx, data); }
    #[inline] fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2 { self.as_mut().layout_inquire(ctx, data) }
    #[inline] fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2 { self.as_mut().layout_compute(ctx, data, size) }
    #[inline] fn paint(&mut self, ctx: &mut PaintCtx, data: &T) { self.as_mut().paint(ctx, data); }
    #[inline] fn respond(&mut self, data: &mut T, button: Option<event::MouseButton>) -> bool { self.as_mut().respond(data, button) }
}

mod macros
{
    #[allow(unused)] macro_rules! impl_event_empty { ($t:ty) => { #[inline] fn event(&mut self, _: &mut EventCtx<E>, _: &mut $t) {} } }
    #[allow(unused)] macro_rules! impl_update_empty { ($t:ty) => { #[inline] fn update(&mut self, _: &mut UpdateCtx, _: &mut $t) {} } }
    #[allow(unused)] macro_rules! impl_widget_compute_empty { ($t:ty) => { #[inline] fn widget_compute(&mut self, _: &mut WidgetComputeCtx, _: &mut $t) {} } }
    #[allow(unused)] macro_rules! impl_layout_inquire_empty { ($t:ty) => { #[inline] fn layout_inquire(&mut self, _: &mut LayoutInquireCtx, _: &$t) -> Vec2 { Vec2::zero() } } }
    #[allow(unused)] macro_rules! impl_layout_compute_empty { ($t:ty) => { #[inline] fn layout_compute(&mut self, _: &mut LayoutComputeCtx, _: &$t, size: Vec2) -> Vec2 { size } } }
    #[allow(unused)] macro_rules! impl_paint_empty { ($t:ty) => { #[inline] fn paint(&mut self, _: &mut PaintCtx, _: &$t) {} } }
    #[allow(unused)] macro_rules! impl_respond_empty { ($t:ty) => { #[inline] fn respond(&mut self, _: &mut $t, _: Option<MouseButton>) -> bool { false } } }
    #[allow(unused)] pub(crate) use impl_event_empty;
    #[allow(unused)] pub(crate) use impl_update_empty;
    #[allow(unused)] pub(crate) use impl_widget_compute_empty;
    #[allow(unused)] pub(crate) use impl_layout_inquire_empty;
    #[allow(unused)] pub(crate) use impl_layout_compute_empty;
    #[allow(unused)] pub(crate) use impl_paint_empty;
    #[allow(unused)] pub(crate) use impl_respond_empty;
    
    #[allow(unused)] macro_rules! impl_event_child { ($t:ty) => { #[inline] fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut $t) { self.child.event(ctx, data); } } }
    #[allow(unused)] macro_rules! impl_update_child { ($t:ty) => { #[inline] fn update(&mut self, ctx: &mut UpdateCtx, data: &mut $t) { self.child.update(ctx, data); } } }
    #[allow(unused)] macro_rules! impl_widget_compute_child { ($t:ty) => { #[inline] fn widget_compute(&mut self, ctx: &mut WidgetComputeCtx, data: &mut $t) { self.child.widget_compute(ctx, data); } } }
    #[allow(unused)] macro_rules! impl_layout_inquire_child { ($t:ty) => { #[inline] fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &$t) -> Vec2 { self.child.layout_inquire(ctx, data) } } }
    #[allow(unused)] macro_rules! impl_layout_compute_child { ($t:ty) => { #[inline] fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &$t, size: Vec2) -> Vec2 { self.child.layout_compute(ctx, data, size) } } }
    #[allow(unused)] macro_rules! impl_paint_child { ($t:ty) => { #[inline] fn paint(&mut self, ctx: &mut PaintCtx, data: &$t) { self.child.paint(ctx, data); } } }
    #[allow(unused)] macro_rules! impl_respond_child { ($t:ty) => { #[inline] fn respond(&mut self, data: &mut $t, button: Option<MouseButton>) -> bool { self.child.respond(data, button) } } }
    #[allow(unused)] pub(crate) use impl_event_child;
    #[allow(unused)] pub(crate) use impl_update_child;
    #[allow(unused)] pub(crate) use impl_widget_compute_child;
    #[allow(unused)] pub(crate) use impl_layout_inquire_child;
    #[allow(unused)] pub(crate) use impl_layout_compute_child;
    #[allow(unused)] pub(crate) use impl_paint_child;
    #[allow(unused)] pub(crate) use impl_respond_child;
}
