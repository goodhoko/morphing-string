use std::{
    iter::{Copied, Cycle},
    slice,
    time::{Duration, Instant},
};

use eframe::{
    CreationContext,
    egui::{self, Align, CentralPanel, Color32, Layout, RichText},
};
use morphing_string::MorphingString;

const LINES: &[&str] = &[
    "If I must die,",
    "you must live",
    "to tell my story",
    "to sell my things",
    "to buy a piece of cloth",
    "and some strings,",
    "(make it white with a long tail)",
    "so that a child, somewhere in Gaza",
    "while looking heaven in the eye",
    "awaiting his dad who left in a blaze–",
    "and bid no one farewell",
    "not even to his flesh",
    "not even to himself–",
    "sees the kite, my kite you made, flying up above",
    "and thinks for a moment an angel is there",
    "bringing back love",
    "If I must die",
    "let it bring hope",
    "let it be a tale",
    "",
    "A poem by Refaat Alareer. https://ifimustdie.net/",
    "",
];

/// One edit per this much wall-clock time, i.e. one per two frames at 60 FPS.
const MORPH_STEP_DURATION: Duration = Duration::from_micros(16_667 * 2);
/// How long a finished line is held before starting to morphing to the next one.
const LINE_STEP_DURATION: Duration = Duration::from_secs(2);
const FONT_SIZE: f32 = 28.0;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "morphing-string",
        Default::default(),
        Box::new(|cc| Ok(Box::new(PoemApp::new(cc)))),
    )
}

/// The poem, one line at a time.
struct PoemApp {
    current_line: MorphingString,
    poem_lines: Cycle<Copied<slice::Iter<'static, &'static str>>>,
    last_step_at: Instant,
}

impl PoemApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.all_styles_mut(|style| {
            style.visuals.panel_fill = Color32::BLACK;
            style.visuals.override_text_color = Some(Color32::WHITE);
        });

        let mut poem_lines = LINES.iter().copied().cycle();

        Self {
            current_line: MorphingString::with_target(
                "",
                poem_lines.next().expect("this poem is not empty"),
            ),
            poem_lines,
            last_step_at: Instant::now(),
        }
    }

    fn update_text(&mut self) {
        if self.current_line.progress().is_complete()
            && self.last_step_at.elapsed() >= LINE_STEP_DURATION
        {
            self.current_line.set_target(
                self.poem_lines
                    .next()
                    .expect("non-empty poem cycled endlessly"),
            );
            self.last_step_at = Instant::now();
        } else if !self.current_line.progress().is_complete()
            && self.last_step_at.elapsed() >= MORPH_STEP_DURATION
        {
            self.current_line.advance();
            self.last_step_at = Instant::now();
        }
    }

    fn render_text(&self, ui: &mut egui::Ui) {
        CentralPanel::default().show(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let text = RichText::new(self.current_line.value())
                    .family(egui::FontFamily::Monospace)
                    .size(FONT_SIZE);
                ui.label(text);
            });
        });
    }
}

impl eframe::App for PoemApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.update_text();
        self.render_text(ui);
        // Nothing else would ever ask for a repaint, and without that the
        // animation would only move when the window is disturbed.
        ui.ctx().request_repaint();
    }
}
