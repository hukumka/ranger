use std::io::{Write, self};
use termion::cursor;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height
        }
    }

    pub fn dir_mut(&mut self, direction: Direction) -> &mut u16 {
        match direction {
            Direction::Vertical => &mut self.height,
            Direction::Horizontal => &mut self.width,
        }
    }

    pub fn dir(&self, direction: Direction) -> &u16 {
        match direction {
            Direction::Vertical => &self.height,
            Direction::Horizontal => &self.width,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Direction {
    pub fn transpose(self) -> Direction {
        match self {
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Rect {
    pos: (u16, u16),
    size: Size,
}

impl Rect {
    pub fn new(pos: (u16, u16), size: Size) -> Self {
        Self {
            pos,
            size
        }
    }

    pub fn from_size(size: Size) -> Rect {
        Rect {
            pos: (0, 0),
            size,
        }
    }

    pub fn tile(self, size: Size, direction: Direction) -> Rect {
        let pos = match direction {
            Direction::Vertical => (self.pos.0, self.pos.1 + self.size.height),
            Direction::Horizontal => (self.pos.0 + self.size.width, self.pos.1),
        };
        Rect {
            pos,
            size
        }
    }
}

pub struct Painter {
    command_buffer: Vec<u8>,
    boxes: Vec<Rect>,
}

impl Painter {
    pub(crate) fn new(size: Size) -> Self {
        Self {
            command_buffer: Vec::with_capacity(1024),
            boxes: vec![Rect{pos: (0, 0), size}],
        }
    }

    pub(crate) fn resize(&mut self, size: Size) {
        self.boxes[0] = Rect{pos: (0, 0), size};
    }

    pub(crate) fn finalize<W: Write>(&mut self, mut stdout: W) -> io::Result<()>{
        stdout.write_all(&self.command_buffer)?;
        stdout.flush()?;
        self.command_buffer.clear();
        Ok(())
    }

    fn rect(&self) -> Rect {
        *self.boxes.last().expect("Painter must contain current box")
    }

    pub fn goto(&mut self, pos: (u16, u16)) {
        assert!(pos.0 < self.rect().size.width);
        assert!(pos.1 < self.rect().size.height);
        let (x, y) = self.abs_pos(pos);
        write!(&mut self.command_buffer, "{}", cursor::Goto(x + 1, y + 1)).expect("Writing into vec should not fail");
    }

    fn abs_pos(&self, pos: (u16, u16)) -> (u16, u16) {
        let x = self.rect().pos.0 + pos.0;
        let y = self.rect().pos.1 + pos.1;
        (x, y)
    }

    pub fn with_inner_rect<T, F: FnOnce(&mut Painter) -> T>(&mut self, rect: Rect, f: F) -> T {
        assert!(rect.size.width <= self.rect().size.width);
        assert!(rect.size.height <= self.rect().size.height);
        let rect = Rect{pos: self.abs_pos(rect.pos), size: rect.size};
        self.boxes.push(rect);
        let res = f(self);
        self.boxes.pop();
        res
    }

    pub fn raw_write<'a>(&'a mut self) -> RawWriter<'a> {
        RawWriter(self)
    }

    pub fn size(&self) -> Size {
        self.rect().size
    }
}

pub struct RawWriter<'a>(&'a mut Painter);

impl<'a> Write for RawWriter<'a> {
    fn write(&mut self, block: &[u8]) -> io::Result<usize> {
        self.0.command_buffer.write(block)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
