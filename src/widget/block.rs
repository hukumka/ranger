use super::Widget;
use crate::painter::{Painter, Size, Rect};
use crate::lens::Lens;
use std::io::{Write, self};
use termion::color;

pub struct Block<L, W, T: ?Sized>{
    title_lens: L,
    inner: W,
    _m: std::marker::PhantomData<T>,
}

impl<T: ?Sized, L: Lens<T, str>, W: Widget<T>> Widget<T> for Block<L, W, T> {
    fn layout(&mut self, max_size: Size, data: &T) -> Size {
        let inner_size = Size{
            width: max_size.width - 2,
            height: max_size.height - 2,
        };
        let size = self.inner.layout(inner_size, data);
        Size{
            width: size.width + 2,
            height: size.height + 2,
        }
    }

    fn paint(&mut self, painter: &mut Painter, data: &T) {
        self.title_lens.get(data, |x| Self::paint_title(painter, x)).unwrap();       
        Self::paint_borders(painter).unwrap();
        let Size{width, height} = painter.size();
        painter.with_inner_rect(Rect::new((1, 1), Size::new(width-2, height-2)), |p| {
            self.inner.paint(p, data);  
        })
    }
}

impl<L: Lens<I, str>, W: Widget<I>, I: ?Sized> Block<L, W, I> {
    pub fn new(title_lens: L, inner: W) -> Self {
        Self {
            title_lens,
            inner,
            _m: Default::default(),
        }
    }
}

impl<L, W, T: ?Sized> Block<L, W, T> {
    fn paint_borders(painter: &mut Painter) -> io::Result<()> {
        let Size{width, height} = painter.size();
        for i in 1..height-1 {
            painter.goto((0, i));
            write!(painter.raw_write(), "│")?;
            painter.goto((width-1, i));
            write!(painter.raw_write(), "│")?;
        }
        painter.goto((0, height-1));
        let mut writer = painter.raw_write();
        write!(writer, "└")?;
        for _ in 1..width-1 {
            write!(writer, "─")?;
        }
        write!(writer, "┘")?;
        Ok(())
    }

    fn paint_title(painter: &mut Painter, title: &str) -> io::Result<()>{
        let len = title.chars().count() as u16;
        let width = painter.size().width;
        let left = (width - len) / 2;
        let right = width - len - left;
        painter.goto((0, 0));
        let mut writer = painter.raw_write();
        write!(writer, "┌")?;
        for _ in 1..left-2 {
            write!(writer, "─")?;
        }
        write!(writer, "| {}{}{} |", color::Fg(color::Red), title, color::Fg(color::Reset))?;
        for _ in 1..right-2 {
            write!(writer, "─")?;
        }
        write!(writer, "┐")?;
        Ok(())
    }
}
