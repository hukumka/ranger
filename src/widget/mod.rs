mod text;
mod flex;
mod block;

pub use text::Text;
pub use flex::Flex;
pub use block::Block;

use crate::lens::Lens;
use crate::painter::{Painter, Size};
use termion::event::Event;
use std::marker::PhantomData;

pub trait Widget<T: ?Sized> {
    fn layout(&mut self, max_size: Size, data: &T) -> Size;
    fn paint(&mut self, painter: &mut Painter, data: &T);
}

pub trait InputProvider<T> {
    fn input(&mut self, event: &Event, data: &mut T);
}
pub trait WidgetExt<T: ?Sized>: Widget<T> + Sized + 'static {
    fn lens<I, L>(self, lens: L) -> LensWidget<Self, I, T, L> 
    where
        I: ?Sized,
        L: Lens<I, T>,
    {
        LensWidget::<Self, I, T, L> {
            widget: self,
            lens,
            marker1: PhantomData::default(),
            marker2: PhantomData::default(),
        }
    }

    fn boxed(self) -> Box<dyn Widget<T>> {
        Box::new(self)
    }
}

impl<T: ?Sized, W: Widget<T> + 'static> WidgetExt<T> for W {}

pub struct LensWidget<W, I: ?Sized, O: ?Sized, L> {
    widget: W,
    lens: L,
    marker1: PhantomData<I>,
    marker2: PhantomData<O>,
}

impl<W, I, O, L> Widget<I> for LensWidget<W, I, O, L> 
where
    W: Widget<O>,
    I: ?Sized,
    O: ?Sized,
    L: Lens<I, O>,
{
    fn layout(&mut self, max_size: Size, data: &I) -> Size {
        let widget = &mut self.widget;
        self.lens.get(data, |x| widget.layout(max_size, x))
    }

    fn paint(&mut self, painter: &mut Painter, data: &I) {
        let widget = &mut self.widget;
        self.lens.get(data, |x| widget.paint(painter, x))
    }
}
