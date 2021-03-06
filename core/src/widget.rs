//! Widget type and definitions.
mod id;
pub use id::*;

use crate::backend::{DrawBackend, Resources};
use crate::draw::DrawContext;
use crate::event::{Event, EventContext, EventResult};
use crate::geometry::{Bounds, Position, Rect};
use crate::visitor::Visitable;

/// Defines an object that can be drawn and viewed inside a window.
pub trait Widget: ObjectId + Bounds + Visitable {
    /// Update the object's layout.
    fn update_layout<R: Resources>(&mut self, parent_rect: Rect, resources: &mut R);

    /// Draws the contents of this object.
    //TODO: invalidate mechanics to avoid overdraw
    fn draw<B: DrawBackend>(&self, dc: DrawContext<B>);

    /// Handles an event sent to this widget.
    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult;

    /// Event consumed notification.
    fn event_consumed(&mut self, event: &Event, ctx: &EventContext);

    /// Coordinate of the widget's viewport origin (top-left).
    ///
    /// The default implementation returns `(0, 0)`.
    fn viewport_origin(&self) -> Position {
        Default::default()
    }

    /// Indicates if this widget's drawing area is clipped against it's bounds.
    ///
    /// The default implementation returns `true`.
    fn is_clipped(&self) -> bool {
        true
    }
}

impl Widget for () {
    #[inline]
    fn update_layout<R: Resources>(&mut self, _: Rect, _: &mut R) {}

    #[inline]
    fn draw<B: DrawBackend>(&self, _: DrawContext<B>) {}

    #[inline]
    fn handle_event(&mut self, _: &Event, _: EventContext) -> EventResult {
        EventResult::Pass
    }

    #[inline]
    fn event_consumed(&mut self, _: &Event, _: &EventContext) {}
}
