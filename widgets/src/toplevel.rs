//! TopLevel type that interfaces with the backend.
use crate::draw::DrawQueue;
use crate::event::Event;
use crate::geometry::Bounds;
use crate::widget::{WidgetId, WindowAttributes};

/// Defines an object that can be a top level window.
pub trait TopLevel: Bounds {
    fn update(&mut self);

    fn draw(&self, dq: &mut DrawQueue);

    fn push_event(&mut self, event: Event) -> Option<WidgetId>;

    fn get_window_attributes(&self) -> &WindowAttributes;
}

impl<T: TopLevel> TopLevel for Box<T> {
    #[inline]
    fn update(&mut self) {
        (**self).update()
    }

    #[inline]
    fn draw(&self, dq: &mut DrawQueue) {
        (**self).draw(dq)
    }

    #[inline]
    fn push_event(&mut self, event: Event) -> Option<WidgetId> {
        (**self).push_event(event)
    }

    #[inline]
    fn get_window_attributes(&self) -> &WindowAttributes {
        (**self).get_window_attributes()
    }
}
