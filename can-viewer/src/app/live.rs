//! Live capture tab controller.

use super::helpers::{pick_save_file, set_status};
use crate::services::{is_capture_running, list_can_interfaces, start_capture, stop_capture};
use crate::state::AppState;
use crate::{LiveErrorRow, LiveFrameRow, LiveMessageRow, LiveSignalRow, SigmaDiagnostics};
use parking_lot::Mutex;
use slint::{Model, SharedString, Timer, TimerMode, Weak};
use std::rc::Rc;
use std::sync::Arc;

/// Live tab: SocketCAN capture with optional MDF4 recording.
pub struct LiveController {
    state: Arc<AppState>,
    ui: Weak<SigmaDiagnostics>,
    capture_session: Arc<Mutex<Option<crate::services::CaptureSession>>>,
    capture_file: Mutex<Option<String>>,
    _timer: Timer,
}

impl LiveController {
    /// Controller bound to the shared state; starts the 100 ms poll timer.
    pub fn new(state: Arc<AppState>, ui: Weak<SigmaDiagnostics>) -> Self {
        let capture_session: Arc<Mutex<Option<crate::services::CaptureSession>>> =
            Arc::new(Mutex::new(None));
        let poll_ui = ui.clone();
        let poll_session = capture_session.clone();
        let timer = Timer::default();
        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(100),
            move || {
                if let Some(session) = poll_session.lock().as_ref() {
                    if let Some(display) = session.poll_update() {
                        if let Some(ui) = poll_ui.upgrade() {
                            update_live_display(&ui, &display);
                        }
                    }
                }
            },
        );

        Self {
            state,
            ui,
            capture_session,
            capture_file: Mutex::new(None),
            _timer: timer,
        }
    }

    /// Hook the live tab callbacks.
    pub fn wire(self: Rc<Self>, ui: &SigmaDiagnostics) {
        #[cfg(not(target_os = "linux"))]
        ui.set_live_linux_only(false);

        self.refresh_interfaces();

        let this = self.clone();
        ui.on_refresh_interfaces({
            let this = this.clone();
            move || this.refresh_interfaces()
        });
        ui.on_start_capture({
            let this = this.clone();
            move || this.start()
        });
        ui.on_stop_capture({
            let this = this.clone();
            move || this.stop()
        });
        ui.on_export_capture({
            let this = this.clone();
            move || this.export_capture()
        });
    }

    fn with_ui<F: FnOnce(&SigmaDiagnostics)>(&self, f: F) {
        if let Some(ui) = self.ui.upgrade() {
            f(&ui);
        }
    }

    fn refresh_interfaces(&self) {
        match list_can_interfaces() {
            Ok(ifaces) => {
                let model: Vec<SharedString> = ifaces.iter().map(|s| s.clone().into()).collect();
                self.with_ui(|ui| {
                    ui.set_live_interfaces(slint::ModelRc::new(slint::VecModel::from(model)));
                    if ui.get_live_selected_interface() as usize >= ifaces.len()
                        && !ifaces.is_empty()
                    {
                        ui.set_live_selected_interface(0);
                    }
                });
            }
            Err(e) => self.with_ui(|ui| set_status(ui, &e)),
        }
    }

    fn start(&self) {
        #[cfg(not(target_os = "linux"))]
        {
            self.with_ui(|ui| set_status(ui, "SocketCAN is only available on Linux"));
            return;
        }

        #[cfg(target_os = "linux")]
        {
            if is_capture_running(&self.state) {
                self.with_ui(|ui| set_status(ui, "Capture already running"));
                return;
            }

            let interface = self.with_ui_get_interface();
            let capture_file = std::env::temp_dir()
                .join(format!("diagnostics-capture-{}.mf4", std::process::id()))
                .display()
                .to_string();

            match start_capture(&interface, &capture_file, false, None, &self.state) {
                Ok(session) => {
                    *self.capture_session.lock() = Some(session);
                    *self.capture_file.lock() = Some(capture_file);
                    self.with_ui(|ui| {
                        ui.set_live_capturing(true);
                        ui.set_live_capture_status("Capturing".into());
                        set_status(ui, &format!("Capturing on {interface}"));
                    });
                }
                Err(e) => self.with_ui(|ui| set_status(ui, &e)),
            }
        }
    }

    fn with_ui_get_interface(&self) -> String {
        let mut iface = "can0".to_string();
        self.with_ui(|ui| {
            let idx = ui.get_live_selected_interface() as usize;
            let model = ui.get_live_interfaces();
            if let Some(name) = model.row_data(idx) {
                iface = name.to_string();
            }
        });
        iface
    }

    fn stop(&self) {
        #[cfg(target_os = "linux")]
        {
            match stop_capture(&self.state) {
                Ok(path) => {
                    *self.capture_session.lock() = None;
                    *self.capture_file.lock() = Some(path.clone());
                    self.with_ui(|ui| {
                        ui.set_live_capturing(false);
                        ui.set_live_capture_status("Stopped".into());
                        set_status(ui, &format!("Capture saved to {path}"));
                    });
                }
                Err(e) => self.with_ui(|ui| set_status(ui, &e)),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            self.with_ui(|ui| set_status(ui, "SocketCAN is only available on Linux"));
        }
    }

    fn export_capture(&self) {
        let path = self.capture_file.lock().clone();
        let Some(src) = path else {
            self.with_ui(|ui| set_status(ui, "No capture file available"));
            return;
        };
        if let Some(dest) = pick_save_file("Export Capture", &[("MDF4", &["mf4"])]) {
            match std::fs::copy(&src, &dest) {
                Ok(_) => self.with_ui(|ui| set_status(ui, &format!("Exported to {dest}"))),
                Err(e) => self.with_ui(|ui| set_status(ui, &format!("Export failed: {e}"))),
            }
        }
    }
}

fn update_live_display(ui: &SigmaDiagnostics, display: &crate::dto::LiveCaptureDisplay) {
    let stats = &display.stats;
    ui.set_live_stats_text(
        format!(
            "{} msgs | {} frames | {:.1} fps | {:.1}s",
            display.message_count, stats.frame_count, stats.frame_rate, stats.elapsed_secs
        )
        .into(),
    );
    crate::apply_live_rows!(ui, display);
}
