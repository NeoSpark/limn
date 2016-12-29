use super::EventHandler;
use super::layout::WidgetLayout;
use input::{Event, EventId, MouseScrollEvent};
use std::any::Any;
use super::super::event;
//use util::Point;
use cassowary::{Solver, Constraint};
use util::*;

pub struct ScrollHandler {
    offset: Point,
}
impl ScrollHandler {
    pub fn new() -> Self {
        ScrollHandler { offset: Point { x: 0.0, y: 0.0 }}
    }
}
impl EventHandler for ScrollHandler {
    fn event_id(&self) -> EventId {
        event::MOUSE_SCROLL
    }
    fn handle_event(&mut self, event: &Event, state: &mut Any, layout: &mut WidgetLayout, parent_layout: &WidgetLayout, solver: &mut Solver) -> Option<EventId> {
        if let Some(scroll) = event.mouse_scroll_args() {
            let scroll: Point = scroll.into();
            let widget_bounds = layout.bounds(solver);
            let parent_bounds = parent_layout.bounds(solver);
            if solver.has_edit_variable(&layout.left) {
                self.offset.x += scroll.x * 13.0;
                self.offset.x = f64::min(0.0, f64::max(parent_bounds.width - widget_bounds.width, self.offset.x));
                solver.suggest_value(layout.left, parent_bounds.left + self.offset.x).unwrap();
            }
            if solver.has_edit_variable(&layout.top) {
                self.offset.y += scroll.y * 13.0;
                self.offset.y = f64::min(0.0, f64::max(parent_bounds.height - widget_bounds.height, self.offset.y));
                solver.suggest_value(layout.top, parent_bounds.top + self.offset.y).unwrap();
            }
        }
        None
    }
}