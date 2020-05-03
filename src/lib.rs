pub mod widget;
pub mod painter;
pub mod lens;

use crate::painter::{Painter, Size, Rect};
use crate::widget::Widget;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{self, Write};

pub struct App<W: io::Write, R> {
    painter: Painter,
    stdout: RawTerminal<W>,
    stdin: R,
}

impl<W: io::Write, R: io::Read> App<W, R> {
    pub fn new(stdout: W, stdin: R) -> Self {
        let stdout = stdout.into_raw_mode().unwrap();
        let size = termion::terminal_size().unwrap();
        Self{
            stdout,
            stdin,
            painter: Painter::new(Size::new(size.0, size.1)),
        }
    }

    pub fn draw<T>(&mut self, widget: &mut impl Widget<T>, data: &T) -> io::Result<()> {
        let size = termion::terminal_size().unwrap();
        let size = Size::new(size.0, size.1);
        self.painter.resize(size);
        write!(self.painter.raw_write(), "{}", termion::clear::All)?;
        let size = widget.layout(size, data);
        self.painter.with_inner_rect(Rect::from_size(size), |p| {
            widget.paint(p, data);
        });
        self.painter.finalize(&mut self.stdout)?;
        Ok(())
    }
}
