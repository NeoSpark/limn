use std::ops::DerefMut;

use cassowary::strength::*;

use limn_layout::linear_layout::{LinearLayout, Orientation};
use limn_layout::grid_layout::GridLayout;
use limn_layout::constraint::*;

use resources::WidgetId;

use app::App;

use widget::{WidgetRef, WidgetBuilder};
use event::{EventHandler, EventArgs};
use ui::ChildrenUpdatedEvent;

pub use self::solver::LimnSolver;
pub use limn_layout::*;

impl<T> EventHandler<ChildrenUpdatedEvent> for T where T: LayoutContainer {
    fn handle(&mut self, event: &ChildrenUpdatedEvent, args: EventArgs) {
        args.widget.update_layout(|layout| {
            match *event {
                ChildrenUpdatedEvent::Added(ref child) => {
                    child.update_layout(|child_layout| {
                        self.add_child_layout(layout, child_layout);
                    });
                },
                ChildrenUpdatedEvent::Removed(ref child) => {
                    child.update_layout(|child_layout| {
                        self.remove_child_layout(layout, child_layout);
                    });
                },
            }
        });
    }
}

impl WidgetBuilder {
    pub fn vbox(&mut self, padding: f32, expand: bool) -> &mut Self {
        let handler = LinearLayout::new(self.layout().deref_mut(), Orientation::Vertical, padding, expand);
        self.set_container(handler)
    }
    pub fn hbox(&mut self, padding: f32, expand: bool) -> &mut Self {
        let handler = LinearLayout::new(self.layout().deref_mut(), Orientation::Horizontal, padding, expand);
        self.set_container(handler)
    }
    pub fn grid(&mut self, num_columns: usize) {
        let container = GridLayout::new(self.layout().deref_mut(), num_columns);
        self.set_container(container);
    }
}

#[derive(Default)]
pub struct Frame {
    padding: f32,
}

impl LayoutContainer for Frame {
    fn add_child_layout(&mut self, parent: &mut Layout, child: &mut Layout) {
        child.add(constraints![
            bound_by(&parent).padding(self.padding),
            match_layout(&parent).strength(STRONG),
        ]);
    }
}

pub struct ExactFrame;

impl LayoutContainer for ExactFrame {
    fn add_child_layout(&mut self, parent: &mut Layout, child: &mut Layout) {
        child.add(match_layout(&parent));
    }
}

#[derive(Clone)]
pub struct UpdateLayout(pub WidgetRef);
pub struct ResizeWindow;
pub struct LayoutChanged(pub Vec<(usize, VarType, f64)>);
pub struct LayoutUpdated;

impl App {
    pub fn add_layout_handlers(&mut self) {
        self.add_handler_fn(|_: &ResizeWindow, args| {
            args.ui.resize_window_to_fit();
        });
        self.add_handler_fn(|event: &UpdateLayout, args| {
            let event = event.clone();
            let UpdateLayout(widget_ref) = event;
            let mut widget_mut = widget_ref.widget_mut();
            let layout = &mut widget_mut.layout;
            args.ui.solver.update_layout(layout);
            args.ui.check_layout_changes();
        });
        self.add_handler_fn(|event: &LayoutChanged, args| {
            let changes = &event.0;
            for &(widget_id, var, value) in changes {
                let widget_id = WidgetId(widget_id);
                if let Some(widget) = args.ui.get_widget(widget_id) {
                    {
                        let widget = &mut *widget.widget_mut();
                        let value = value as f32;
                        debug!("{:?}: {:?} = {}", widget.name(), var, value);
                        match var {
                            VarType::Left => widget.bounds.origin.x = value,
                            VarType::Top => widget.bounds.origin.y = value,
                            VarType::Width => widget.bounds.size.width = value,
                            VarType::Height => widget.bounds.size.height = value,
                            _ => (),
                        }
                    }
                    widget.event(LayoutUpdated);
                }
            }
            // redraw everything when layout changes, for now
            args.ui.redraw();
        });
    }
}
