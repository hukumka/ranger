use super::{Widget, Size};
use crate::painter::Painter;
use std::io::Write;

pub struct Text {
    lines: Vec<Line>,
}

impl Widget<str> for Text {
    fn layout(&mut self, max_size: Size, data: &str) -> Size {
        self.lines.clear();
        let mut height = 0;
        let mut max_width = 0;
        Text::warp_text(data, max_size.width, |s| {
            if height >= max_size.height {
                return;
            }
            max_width = max_width.max(s.len);
            self.lines.push(s);
            height += 1;
        });
        Size::new(max_width, height)
    }

    fn paint(&mut self, painter: &mut Painter, data: &str) {
        for (i, line) in self.lines.iter().enumerate() {
            painter.goto((0, i as u16));
            write!(painter.raw_write(), "{}", &data[line.range.clone()]).unwrap_or_else(|_| unreachable!());
        }
    }
}

impl Text {
    pub fn new() -> Self{
        Self {
            lines: vec![],
        }
    }

    fn warp_text<F: FnMut(Line)>(string: &str, max_width: u16, mut writer: F) {
        let mut splitter = Splitter::new();
        let mut iter = string.char_indices().peekable();
        while let Some((_, c)) = iter.next() {
            let end = iter.peek()
                .map(|(i, _)| *i)
                .unwrap_or(string.len());
            
            splitter.advance_char(end);
            if c == '\n' {
                writer(splitter.new_line(end));
            } else if splitter.line_len > max_width {
                let line = splitter.split().expect("Splitting on empty string");
                writer(line);
            } else {
                // Current split strategy is just split at certain width.
                // No word concederation
                splitter.new_split_point();
            }
        }
        writer(splitter.new_line(string.len()));
    }
}

struct Splitter {
    pos: usize,
    line_len: u16,
    line_start: usize,
    possible_split_pos: usize,
    possible_split_len: u16,
}

struct Line {
    range: std::ops::Range<usize>,
    len: u16,
}

impl Splitter {
    fn new() -> Self {
        Self {
            pos: 0,
            line_len: 0,
            line_start: 0,
            possible_split_pos: 0,
            possible_split_len: 0,
        }
    }

    #[inline]
    fn new_split_point(&mut self) {
        self.possible_split_pos = self.pos;
        self.possible_split_len = self.line_len;
    }

    #[inline]
    fn advance_char(&mut self, new_pos: usize) {
        self.pos = new_pos;
        self.line_len += 1;
    }

    #[inline]
    fn split(&mut self) -> Option<Line> {
        if self.possible_split_len == 0 {
            return None;
        }
        let range = self.line_start..self.possible_split_pos;
        let len = self.possible_split_len;
        self.line_start = self.possible_split_pos;
        self.line_len -= self.possible_split_len;
        self.possible_split_len = 0;
        Some(Line{
            range,
            len
        })
    }

    fn new_line(&mut self, line_start: usize) -> Line {
        let range = self.line_start..self.pos;
        let len = self.line_len;
        self.pos = line_start;
        self.possible_split_pos = line_start;
        self.possible_split_len = 0;
        self.line_start = line_start;
        self.line_len = 0;
        Line{
            range,
            len,
        }
    }
}
