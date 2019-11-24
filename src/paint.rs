//! Paint trait and implementations.

use crate::{canvas::Canvas, path::ClosedPath, P2};
use arrayvec::{ArrayVec, IntoIter};
use euclid::{Rect, UnknownUnit};
use lyon_geom::LineSegment;
use lyon_path::PathEvent;

/// A trait for types which can be represented on a `Canvas`.
pub trait Paint {
    /// Paints self in the composition.
    fn paint(&self, canvas: &mut Canvas);
}

/// Paints a path with a fill pattern.
pub struct Filled<D>(pub D);

impl<P: Paint> Paint for Filled<P> {
    fn paint(&self, comp: &mut Canvas) {
        self.0.paint(comp);
        comp.fill();
    }
}

/// Paints a path with a stroke.
pub struct Stroked<D> {
    pub element: D,
    pub thickness: f32,
}

impl<P: Paint> Paint for Stroked<P> {
    fn paint(&self, comp: &mut Canvas) {
        self.element.paint(comp);
        comp.set_stroke_thickness(self.thickness);
        comp.stroke();
    }
}

impl<P> Paint for P
where
    P: Iterator<Item = PathEvent> + Clone,
{
    fn paint(&self, canvas: &mut Canvas) {
        let mut iter = self.clone();
        iter.for_each(|p| match p {
            PathEvent::Line(line) => canvas.line_to(line.to),
            PathEvent::Quadratic(q) => canvas.quadratic_to(q.ctrl, q.to),
            PathEvent::Cubic(c) => canvas.cubic_to(c.ctrl1, c.ctrl2, c.to),
            PathEvent::MoveTo(p) => canvas.move_to(p),
            PathEvent::Close(_) => canvas.close(),
        });
    }
}
