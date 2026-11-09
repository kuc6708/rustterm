//! rustterm desktop client (native GUI via egui/eframe).
//!
//! Clean-room original implementation.

use eframe::egui;
use rustterm_term::Terminal;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "rustterm",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

struct Tab {
    term: Terminal,
    title: String,
}

struct App {
    tabs: Vec<Tab>,
    active: usize,
}

fn default_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}

impl App {
    fn new() -> Self {
        let shell = default_shell();
        let term = Terminal::spawn(&shell, 24, 80).expect("failed to spawn pty");
        Self {
            tabs: vec![Tab {
                term,
                title: shell,
            }],
            active: 0,
        }
    }

    fn collect_input(ctx: &egui::Context) -> Vec<u8> {
        let events = ctx.input(|i| i.events.clone());
        let mut buf = Vec::new();
        for event in &events {
            match event {
                egui::Event::Text(s) => buf.extend_from_slice(s.as_bytes()),
                egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if modifiers.ctrl {
                        match key {
                            egui::Key::A => buf.push(0x01),
                            egui::Key::C => buf.push(0x03),
                            egui::Key::D => buf.push(0x04),
                            egui::Key::E => buf.push(0x05),
                            egui::Key::L => buf.push(0x0c),
                            egui::Key::Z => buf.push(0x1a),
                            _ => {}
                        }
                    } else {
                        match key {
                            egui::Key::Enter => buf.push(b'\r'),
                            egui::Key::Backspace => buf.push(0x7f),
                            egui::Key::Tab => buf.push(b'\t'),
                            egui::Key::Escape => buf.push(0x1b),
                            egui::Key::ArrowUp => buf.extend_from_slice(b"\x1b[A"),
                            egui::Key::ArrowDown => buf.extend_from_slice(b"\x1b[B"),
                            egui::Key::ArrowRight => buf.extend_from_slice(b"\x1b[C"),
                            egui::Key::ArrowLeft => buf.extend_from_slice(b"\x1b[D"),
                            egui::Key::Home => buf.extend_from_slice(b"\x1b[H"),
                            egui::Key::End => buf.extend_from_slice(b"\x1b[F"),
                            egui::Key::Delete => buf.extend_from_slice(b"\x1b[3~"),
                            egui::Key::PageUp => buf.extend_from_slice(b"\x1b[5~"),
                            egui::Key::PageDown => buf.extend_from_slice(b"\x1b[6~"),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        buf
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for tab in &mut self.tabs {
            tab.term.process_pending();
        }

        let input = Self::collect_input(ctx);
        if !input.is_empty() {
            if let Some(tab) = self.tabs.get_mut(self.active) {
                let _ = tab.term.write(&input);
            }
        }

        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut clicked = None;
                for (i, tab) in self.tabs.iter().enumerate() {
                    if ui.selectable_label(i == self.active, &tab.title).clicked() {
                        clicked = Some(i);
                    }
                }
                if let Some(i) = clicked {
                    self.active = i;
                }
                if ui.button("+").on_hover_text("New local shell").clicked() {
                    let shell = default_shell();
                    if let Ok(term) = Terminal::spawn(&shell, 24, 80) {
                        self.tabs.push(Tab {
                            term,
                            title: shell,
                        });
                        self.active = self.tabs.len() - 1;
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let cols = (size.x / 8.0).clamp(20.0, 500.0) as u16;
            let rows = (size.y / 16.0).clamp(5.0, 300.0) as u16;
            let idx = self.active;
            if let Some(tab) = self.tabs.get_mut(idx) {
                if tab.term.size() != (rows, cols) {
                    tab.term.resize(rows, cols);
                }
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(tab.term.contents()).monospace(),
                            )
                            .selectable(true),
                        );
                    });
            }
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}
