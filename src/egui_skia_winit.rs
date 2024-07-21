use std::time::Duration;

use egui::Context;
pub use egui_winit;
use egui_winit::winit::window::Window;
use egui_winit::EventResponse;
use skia_safe::Canvas;

use crate::EguiSkia;

pub struct EguiSkiaWinit {
    pub egui_skia: EguiSkia,
    pub egui_winit: egui_winit::State,
}

impl EguiSkiaWinit {
    pub fn new(window: &Window) -> Self {
        let scale_factor = window.scale_factor() as f32;
        let egui_skia = EguiSkia::new(scale_factor);

        let egui_winit = egui_winit::State::new(
            egui_skia.egui_ctx.clone(),
            egui_skia.egui_ctx.viewport_id(),
            window,
            Some(scale_factor),
            None,
        );

        Self {
            egui_winit,
            egui_skia,
        }
    }

    /// Returns `true` if egui wants exclusive use of this event
    /// (e.g. a mouse click on an egui window, or entering text into a text field).
    /// For instance, if you use egui for a game, you want to first call this
    /// and only when this returns `false` pass on the events to your game.
    ///
    /// Note that egui uses `tab` to move focus between elements, so this will always return `true` for tabs.
    pub fn on_event(&mut self, window: &Window, event: &egui_winit::winit::event::WindowEvent) -> EventResponse {
        self.egui_winit.on_window_event(window, event)
    }

    /// Returns a duration after witch egui should repaint.
    ///
    /// Call [`Self::paint`] later to paint.
    pub fn run(&mut self, window: &Window, run_ui: impl FnMut(&Context)) -> Option<Duration> {
        let raw_input = self.egui_winit.take_egui_input(window);

        let (repaint_delay, platform_output) = self.egui_skia.run(raw_input, run_ui);

        self.egui_winit
            .handle_platform_output(window, platform_output);
        repaint_delay
    }

    /// Paint the results of the last call to [`Self::run`].
    pub fn paint(&mut self, canvas: &Canvas) {
        self.egui_skia.paint(canvas);
    }
}
