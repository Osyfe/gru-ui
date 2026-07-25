use crate::{Widget, widget::data::Lensing};
use std::marker::PhantomData;
pub use gru_ui_derive::Lens;
pub use tuples::*;

pub trait Projector<U, T>: Copy
{
    fn project(self, data: &U) -> &T;
    fn project_mut(self, data: &mut U) -> &mut T;

    fn to_lens(self) -> ProjectionLens<U, T, Self> { ProjectionLens::new(self) }
    fn fallback(self, fallback: T) -> impl Lens<Option<U>, T> where Self: Clone + 'static
    {
        Fallback::new
        (
            move |opt_u: &Option<U>| opt_u.as_ref().map(|u| self.project(u)),
            move |opt_u| opt_u.as_mut().map(|u| self.project_mut(u)),
            fallback
        )
    }
}

pub struct ProjectionLens<U, T, P: Projector<U, T>>
{
    p: P,
    _phantom: PhantomData<(U, T)>,
}

impl<U, T, P: Projector<U, T>> Lens<U, T> for ProjectionLens<U, T, P>
{
    fn with<A, F: FnOnce(&T) -> A>(&mut self, data: &U, f: F) -> A
    {
        f(self.p.project(data))
    }

    fn with_mut<A, F: FnOnce(&mut T) -> A>(&mut self, data: &mut U, f: F) -> A
    {
        f(self.p.project_mut(data))
    }
}

impl<U, T, P: Projector<U, T>> ProjectionLens<U, T, P>
{
    pub fn new(projector: P) -> Self { Self { p: projector, _phantom: PhantomData }}
}

pub trait Lens<U, T>
{
    fn with<A, F: FnOnce(&T) -> A>(&mut self, data: &U, f: F) -> A;
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&mut self, data: &mut U, f: F) -> A;
}

pub trait LensExt<U, T>: Lens<U, T> + Sized
{
    fn lens<E, W: Widget<T, E>>(self, widget: W) -> Lensing<U, T, E, W, Self> { Lensing::new(widget, self) }
    fn chain<S, L2: Lens<T, S>>(self, lens2: L2) -> LensChain<U, T, S, Self, L2> { LensChain::new(self, lens2) }
}

impl<U, T, L: Lens<U, T>> LensExt<U, T> for L {}

pub struct LensChain<V, U, T, L1: Lens<V, U>, L2: Lens<U, T>>
{
    lens1: L1,
    lens2: L2,
    _phantom: PhantomData<(V, U, T)>
}

impl<V, U, T, L1: Lens<V, U> + Clone, L2: Lens<U, T> + Clone> Clone for LensChain<V, U, T, L1, L2>
{
    fn clone(&self) -> Self
    {
        Self { lens1: self.lens1.clone(), lens2: self.lens2.clone(), _phantom: PhantomData }
    }
}

impl<V, U, T, L1: Lens<V, U> + Copy, L2: Lens<U, T> + Copy> Copy for LensChain<V, U, T, L1, L2> {}

impl<V, U, T, L1: Lens<V, U>, L2: Lens<U, T>> LensChain<V, U, T, L1, L2>
{
    #[inline]
    pub fn new(lens1: L1, lens2: L2) -> Self
    {
        Self { lens1, lens2, _phantom: PhantomData }
    }
}

impl<V, U, T, L1: Lens<V, U>, L2: Lens<U, T>> Lens<V, T> for LensChain<V, U, T, L1, L2>
{
    #[inline]
    fn with<A, F: FnOnce(&T) -> A>(&mut self, data: &V, f: F) -> A
    {
        self.lens1.with(data, |data| self.lens2.with(data, |data| f(data)))
    }

    #[inline]
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&mut self, data: &mut V, f: F) -> A
    {
        self.lens1.with_mut(data, |data| self.lens2.with_mut(data, |data| f(data)))
    }
}

pub struct Transform<'a, U, T, F: FnMut(&U) -> T + 'a, G: FnMut(&mut U, &T) + 'a>
{
    f: F,
    g: G,
    _phantom: PhantomData<&'a (U, T)>,
}

impl<'a, U, T, F: FnMut(&U) -> T + 'a, G: FnMut(&mut U, &T) + 'a> Lens<U, T> for Transform<'a, U, T, F, G>
{
    #[inline]
    fn with<A, H: FnOnce(&T) -> A>(&mut self, data: &U, h: H) -> A
    {
        let data_t = (self.f)(data);
        h(&data_t)
    }

    #[inline]
    fn with_mut<A, H: FnOnce(&mut T) -> A>(&mut self, data: &mut U, h: H) -> A
    {
        let mut data_t = (self.f)(data);
        let a = h(&mut data_t);
        (self.g)(data, &data_t);
        a
    }
}

impl<'a, U, T, F: FnMut(&U) -> T + 'a, G: FnMut(&mut U, &T) + 'a> Transform<'a, U, T, F, G>
{
    pub fn new(f: F, g: G) -> Self
    {
        Self { f, g, _phantom: PhantomData }
    }
}

pub struct Fallback<U, T, F: FnMut(&U) -> Option<&T>, FM: FnMut(&mut U) -> Option<&mut T>>
{
    f: F,
    f_mut: FM,
    fallback: T,
    _phantom: PhantomData<U>,
}

impl<U, T, F: FnMut(&U) -> Option<&T>, FM: FnMut(&mut U) -> Option<&mut T>> Lens<U, T> for Fallback<U, T, F, FM>
{
    #[inline]
    fn with<A, H: FnOnce(&T) -> A>(&mut self, data: &U, h: H) -> A
    {
        let data = match (self.f)(data)
        {
            Some(data) => data,
            None => &self.fallback,
        };
        h(data)
    }

    #[inline]
    fn with_mut<A, H: FnOnce(&mut T) -> A>(&mut self, data: &mut U, h: H) -> A
    {
        let data = match (self.f_mut)(data)
        {
            Some(data) => data,
            None => &mut self.fallback,
        };
        h(data)
    }
}

impl<U, T, F: FnMut(&U) -> Option<&T>, FM: FnMut(&mut U) -> Option<&mut T>> Fallback<U, T, F, FM>
{
    pub fn new(f: F, f_mut: FM, fallback: T) -> Self
    {
        Self { f, f_mut, fallback, _phantom: PhantomData }
    }
}

pub struct OptionRef;
pub struct OptionMut;

impl<'a, T> FnOnce<(&'a Option<T>,)> for OptionRef
{
    type Output = Option<&'a T>;
    extern "rust-call" fn call_once(self, (opt,): (&'a Option<T>,)) -> Self::Output { opt.as_ref() }
}

impl<'a, T> FnMut<(&'a Option<T>,)> for OptionRef
{
    extern "rust-call" fn call_mut(&mut self, (opt,): (&'a Option<T>,)) -> Self::Output { opt.as_ref() }
}

impl<'a, T> FnOnce<(&'a mut Option<T>,)> for OptionMut
{
    type Output = Option<&'a mut T>;
    extern "rust-call" fn call_once(self, (opt,): (&'a mut Option<T>,)) -> Self::Output { opt.as_mut() }
}

impl<'a, T> FnMut<(&'a mut Option<T>,)> for OptionMut
{
    extern "rust-call" fn call_mut(&mut self, (opt,): (&'a mut Option<T>,)) -> Self::Output { opt.as_mut() }
}

impl<T> Fallback<Option<T>, T, OptionRef, OptionMut>
{
    pub fn new_option(fallback: T) -> Self
    {
        Self { f: OptionRef, f_mut: OptionMut, fallback, _phantom: PhantomData }
    }
}

#[derive(Clone)]
pub struct LensSlice<T>(pub usize, pub T);

impl<U: AsRef<[T]> + AsMut<[T]>, T> Lens<U, T> for LensSlice<T>
{
    #[inline]
    fn with<A, F: FnOnce(&T) -> A>(&mut self, data: &U, f: F) -> A
    {
        let data = data.as_ref().get(self.0).unwrap_or_else(|| &self.1);
        f(&data)
    }

    #[inline]
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&mut self, data: &mut U, f: F) -> A
    {
        let data = data.as_mut().get_mut(self.0).unwrap_or_else(|| &mut self.1);
        f(data)
    }
}

mod tuples
{
    use super::Lens;
    
    macro_rules! lens_tuple
    {
        ($name: ident, $i:tt, $it:ty ; $($j:tt),+) =>
        {
            impl<$($j),+> Lens<($($j),+), $it> for $name
            {
                #[inline]
                fn with<A, F: FnOnce(&$it) -> A>(&mut self, data: &($($j),+), f: F) -> A
                {
                    f(&data.$i)
                }

                #[inline]
                fn with_mut<A, F: FnOnce(&mut $it) -> A>(&mut self, data: &mut ($($j),+), f: F) -> A
                {
                    f(&mut data.$i)
                }
            }
        };
    }

    #[derive(Clone, Copy)]
    pub struct LensTuple0;
    #[derive(Clone, Copy)]
    pub struct LensTuple1;
    #[derive(Clone, Copy)]
    pub struct LensTuple2;
    #[derive(Clone, Copy)]
    pub struct LensTuple3;
    #[derive(Clone, Copy)]
    pub struct LensTuple4;
    #[derive(Clone, Copy)]
    pub struct LensTuple5;
    #[derive(Clone, Copy)]
    pub struct LensTuple6;
    
    lens_tuple!(LensTuple0, 0, U0 ; U0, U1);
    lens_tuple!(LensTuple1, 1, U1 ; U0, U1);
    
    lens_tuple!(LensTuple0, 0, U0 ; U0, U1, U2);
    lens_tuple!(LensTuple1, 1, U1 ; U0, U1, U2);
    lens_tuple!(LensTuple2, 2, U2 ; U0, U1, U2);
    
    lens_tuple!(LensTuple0, 0, U0 ; U0, U1, U2, U3);
    lens_tuple!(LensTuple1, 1, U1 ; U0, U1, U2, U3);
    lens_tuple!(LensTuple2, 2, U2 ; U0, U1, U2, U3);
    lens_tuple!(LensTuple3, 3, U3 ; U0, U1, U2, U3);
    
    lens_tuple!(LensTuple0, 0, U0 ; U0, U1, U2, U3, U4);
    lens_tuple!(LensTuple1, 1, U1 ; U0, U1, U2, U3, U4);
    lens_tuple!(LensTuple2, 2, U2 ; U0, U1, U2, U3, U4);
    lens_tuple!(LensTuple3, 3, U3 ; U0, U1, U2, U3, U4);
    lens_tuple!(LensTuple4, 4, U4 ; U0, U1, U2, U3, U4);
    
    lens_tuple!(LensTuple0, 0, U0 ; U0, U1, U2, U3, U4, U5);
    lens_tuple!(LensTuple1, 1, U1 ; U0, U1, U2, U3, U4, U5);
    lens_tuple!(LensTuple2, 2, U2 ; U0, U1, U2, U3, U4, U5);
    lens_tuple!(LensTuple3, 3, U3 ; U0, U1, U2, U3, U4, U5);
    lens_tuple!(LensTuple4, 4, U4 ; U0, U1, U2, U3, U4, U5);
    lens_tuple!(LensTuple5, 5, U5 ; U0, U1, U2, U3, U4, U5);
    
    lens_tuple!(LensTuple0, 0, U0 ; U0, U1, U2, U3, U4, U5, U6);
    lens_tuple!(LensTuple1, 1, U1 ; U0, U1, U2, U3, U4, U5, U6);
    lens_tuple!(LensTuple2, 2, U2 ; U0, U1, U2, U3, U4, U5, U6);
    lens_tuple!(LensTuple3, 3, U3 ; U0, U1, U2, U3, U4, U5, U6);
    lens_tuple!(LensTuple4, 4, U4 ; U0, U1, U2, U3, U4, U5, U6);
    lens_tuple!(LensTuple5, 5, U5 ; U0, U1, U2, U3, U4, U5, U6);
    lens_tuple!(LensTuple6, 6, U6 ; U0, U1, U2, U3, U4, U5, U6);
}
    