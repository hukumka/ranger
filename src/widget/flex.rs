use super::Widget;
use crate::painter::{Painter, Size, Rect, Direction};

pub struct Flex<T> {
    direction: Direction,
    flex_children: Vec<Box<dyn Widget<T>>>,
    flexes: Vec<f32>,
    fixed_children: Vec<Box<dyn Widget<T>>>,
    child_type: Vec<ChildType>,

    fixed_sizes: Vec<Size>,
    flex_sizes: Vec<Size>,
}

enum ChildType {
    Fixed,
    Flex,
}

impl<T> Widget<T> for Flex<T> {
    fn layout(&mut self, max_size: Size, data: &T) -> Size {
        let mut max_secondary_size = 0;

        let dir = self.direction;
        // Compute sizes for fixed widgets.
        self.fixed_sizes.clear();
        let mut remaining_size = max_size;
        let fs = self.fixed_children.iter_mut()
            .map(|child| {
                let size = child.layout(remaining_size, data.clone());
                *remaining_size.dir_mut(dir) -= size.dir(dir);
                max_secondary_size = max_secondary_size.max(*size.dir(dir.transpose()));
                size
            });
        self.fixed_sizes.extend(fs);
        // Flexes 
        let total_available = *remaining_size.dir(dir);
        let total_flex: f32 = self.flexes.iter().sum();
        let mut total_flex_u16 = 0; // Since lenght is integer, total length of all flexes
        // might not add up to required length due to rounding.
        // To avoid it count how much was used, and fix size of last flex.
        self.flex_sizes.clear();
        self.flex_sizes.extend(self.flexes.iter().map(|flex| {
            let mut size = remaining_size;
            let len = (total_available as f32 * flex / total_flex) as u16;
            *size.dir_mut(dir) = len;
            total_flex_u16 += len;
            size
        }));
        if let Some(x) = self.flex_sizes.last_mut() {
            *x.dir_mut(dir) += remaining_size.dir(dir) - total_flex_u16;
        }
        for (size, child) in self.flex_sizes.iter().zip(&mut self.flex_children) {
            let size = child.layout(*size, data);
            max_secondary_size = max_secondary_size.max(*size.dir(dir.transpose()));
        }
        
        let mut size = max_size;
        *size.dir_mut(dir.transpose()) = max_secondary_size;
        if self.flexes.is_empty() {
            *size.dir_mut(dir) -= total_available;
        }
        eprintln!("{} {}", size.width, size.height);
        size
    }

    fn paint(&mut self, painter: &mut Painter, data: &T) {
        let mut rect = Rect::from_size(Size::new(0, 0));
        let mut flexes = self.flex_children.iter_mut().zip(&self.flex_sizes);
        let mut fixed = self.fixed_children.iter_mut().zip(&self.fixed_sizes);
        for type_ in &self.child_type {
            let (child, size) = match type_ {
                ChildType::Flex => flexes.next().unwrap(),
                ChildType::Fixed => fixed.next().unwrap(),
            };
            rect = rect.tile(*size, self.direction);
            painter.with_inner_rect(rect, |painter| {
                child.paint(painter, data);
            });
        }
    }
}

impl<T> Flex<T> {
    pub fn column() -> Self {
        Self::new(Direction::Vertical)
    }
    
    pub fn row() -> Self {
        Self::new(Direction::Horizontal)
    }

    fn new(direction: Direction) -> Self {
        Self {
            direction,
            flex_children: vec![],
            flexes: vec![],
            flex_sizes: vec![],

            fixed_children: vec![],
            fixed_sizes: vec![],

            child_type: vec![],
        }
    }

    pub fn add_child(&mut self, widget: Box<dyn Widget<T>>) -> &mut Self {
        self.fixed_children.push(widget);
        self.child_type.push(ChildType::Fixed);
        self
    }

    pub fn add_flex_child(&mut self, widget: Box<dyn Widget<T>>, flex: f32) -> &mut Self {
        self.flex_children.push(widget);
        self.flexes.push(flex);
        self.child_type.push(ChildType::Flex);
        self
    }
}
