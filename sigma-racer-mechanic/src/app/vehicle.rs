//! Vehicle Connect / Diagnosis / Maintenance / Settings / Updates / Logs.

use crate::state::AppState;
use crate::{SettingRow, SigmaRacerMechanic, VitalRow};
use sigma_diagnostics::{
    fetch_channel_latest, list_can_interfaces, request_log_export, LogExportRequest,
    MaintenanceAction, MaintenanceService, OtaConfig, SettingsService, StubMaintenanceService,
    StubSettingsService, VehicleLinkConfig,
};
use slint::{Model, ModelRc, VecModel, Weak};
use std::rc::Rc;
use std::sync::Arc;

pub struct VehicleController {
    state: Arc<AppState>,
    ui: Weak<SigmaRacerMechanic>,
}

impl VehicleController {
    pub fn new(state: Arc<AppState>, ui: Weak<SigmaRacerMechanic>) -> Self {
        Self { state, ui }
    }

    pub fn wire(self: Rc<Self>, ui: &SigmaRacerMechanic) {
        ui.on_refresh_interfaces({
            let t = self.clone();
            move || t.refresh_interfaces()
        });
        ui.on_connect_vehicle({
            let t = self.clone();
            move || t.connect()
        });
        ui.on_disconnect_vehicle({
            let t = self.clone();
            move || t.disconnect()
        });
        ui.on_maintenance_reset_service({
            let t = self.clone();
            move || t.maintenance(MaintenanceAction::ResetServiceInterval)
        });
        ui.on_maintenance_reset_oil({
            let t = self.clone();
            move || t.maintenance(MaintenanceAction::ResetOilLife)
        });
        ui.on_maintenance_clear_warning({
            let t = self.clone();
            move || t.maintenance(MaintenanceAction::ClearMaintenanceWarning)
        });
        ui.on_refresh_settings({
            let t = self.clone();
            move || t.refresh_settings()
        });
        ui.on_check_updates({
            let t = self.clone();
            move || t.check_updates()
        });
        ui.on_download_bundle({
            let t = self.clone();
            move || t.download_bundle()
        });
        ui.on_request_ecu_log({
            let t = self.clone();
            move || t.request_ecu_log()
        });
        ui.on_open_local_mdf4_from_logs({
            let t = self.clone();
            move || t.open_local_mdf4()
        });
    }

    fn with_ui<F: FnOnce(&SigmaRacerMechanic)>(&self, f: F) {
        if let Some(ui) = self.ui.upgrade() {
            f(&ui);
        }
    }

    pub fn refresh_interfaces(&self) {
        let list = list_can_interfaces().unwrap_or_default();
        self.with_ui(|ui| {
            let model = VecModel::from(
                list.iter()
                    .map(|s| slint::SharedString::from(s.as_str()))
                    .collect::<Vec<_>>(),
            );
            ui.set_vehicle_interfaces(ModelRc::new(model));
            if let Some(saved) = self.state.mechanic_session.lock().can_interface.clone() {
                if let Some(idx) = list.iter().position(|i| i == &saved) {
                    ui.set_vehicle_selected_interface(idx as i32);
                }
            }
        });
    }

    fn connect(&self) {
        self.with_ui(|ui| {
            let ifaces = ui.get_vehicle_interfaces();
            let idx = ui.get_vehicle_selected_interface() as usize;
            let iface = ifaces
                .row_data(idx)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "can0".into());
            let bitrate = ui.get_vehicle_bitrate().parse::<u32>().unwrap_or(500_000);
            let use_m7 = ui.get_vehicle_use_m7_dbc();

            self.state.vehicle.set_config(VehicleLinkConfig {
                interface: iface.clone(),
                bitrate,
                use_m7_draft_dbc: use_m7,
            });

            match self.state.vehicle.connect(&self.state.analysis.diag) {
                Ok(()) => {
                    let _ = self
                        .state
                        .mechanic_session
                        .lock()
                        .set_can_interface(Some(iface.clone()));
                    ui.set_vehicle_connected(true);
                    ui.set_vehicle_status_label("Connected".into());
                    ui.set_vehicle_status_detail(
                        format!("Listening on {iface} (bitrate hint {bitrate})").into(),
                    );
                    ui.set_status_text(format!("Connected to {iface}").into());
                    ui.set_diag_status("Receiving".into());
                }
                Err(e) => {
                    ui.set_vehicle_connected(false);
                    ui.set_vehicle_status_label("Error".into());
                    ui.set_vehicle_status_detail(e.clone().into());
                    ui.set_status_text(e.into());
                }
            }
        });
    }

    fn disconnect(&self) {
        self.state.vehicle.disconnect(&self.state.analysis.diag);
        self.with_ui(|ui| {
            ui.set_vehicle_connected(false);
            ui.set_vehicle_status_label("Disconnected".into());
            ui.set_vehicle_status_detail("".into());
            ui.set_diag_status("Not connected".into());
            ui.set_status_text("Disconnected".into());
        });
    }

    pub fn poll_diagnosis_into(&self, ui: &SigmaRacerMechanic) {
        if !ui.get_vehicle_connected() {
            return;
        }
        let snap = self.state.vehicle.poll_diagnosis(&self.state.analysis.diag);
        ui.set_vehicle_connected(snap.connected);
        ui.set_diag_status(snap.status.into());
        ui.set_diag_rpm(or_dash(&snap.rpm).into());
        ui.set_diag_coolant(or_dash(&snap.coolant_c).into());
        ui.set_diag_oil(or_dash(&snap.oil_c).into());
        ui.set_diag_dtc(or_dash(&snap.dtc_count).into());
        ui.set_diag_gear(or_dash(&snap.gear).into());
        ui.set_diag_side_stand(or_dash(&snap.side_stand).into());
        ui.set_diag_mode(or_dash(&snap.performance_mode).into());
        ui.set_diag_frame_count(snap.frame_count.to_string().into());

        let rows: Vec<VitalRow> = snap
            .vitals
            .iter()
            .map(|v| VitalRow {
                name: v.name.clone().into(),
                value: v.value.clone().into(),
                unit: v.unit.clone().into(),
            })
            .collect();
        ui.set_diag_vitals(ModelRc::new(VecModel::from(rows)));

        if !snap.connected {
            ui.set_vehicle_status_label("Disconnected".into());
        }
    }

    fn maintenance(&self, action: MaintenanceAction) {
        let svc = StubMaintenanceService;
        let msg = match svc.perform(action) {
            Ok(s) => s,
            Err(e) => e,
        };
        self.with_ui(|ui| {
            ui.set_maintenance_status(msg.clone().into());
            ui.set_status_text(msg.into());
        });
    }

    pub fn refresh_settings(&self) {
        let svc = StubSettingsService;
        let (rows, status) = match svc.list() {
            Ok(list) => {
                let rows: Vec<SettingRow> = list
                    .into_iter()
                    .map(|s| SettingRow {
                        key: s.key.into(),
                        value: s.value.into(),
                        read_only: s.read_only,
                    })
                    .collect();
                (
                    rows,
                    "Read-only preview — write protocol pending.".to_string(),
                )
            }
            Err(e) => (Vec::new(), e),
        };
        self.with_ui(|ui| {
            ui.set_vehicle_settings(ModelRc::new(VecModel::from(rows)));
            ui.set_settings_status(status.into());
        });
    }

    pub fn init_ota_labels(&self) {
        let cfg = OtaConfig::from_env();
        self.with_ui(|ui| {
            ui.set_ota_channel(cfg.channel.clone().into());
            ui.set_ota_current(cfg.current_version.clone().into());
            ui.set_ota_status("Idle — check for channel updates.".into());
        });
    }

    fn check_updates(&self) {
        let cfg = OtaConfig::from_env();
        self.with_ui(|ui| {
            ui.set_ota_busy(true);
            ui.set_ota_status("Checking…".into());
        });
        let result = fetch_channel_latest(&cfg);
        self.with_ui(|ui| {
            ui.set_ota_busy(false);
            match result {
                Ok(rel) => {
                    let newer = rel.version != cfg.current_version;
                    ui.set_ota_update_available(newer);
                    ui.set_ota_available(rel.version.clone().into());
                    ui.set_ota_notes(rel.notes.clone().into());
                    ui.set_ota_bundle_url(rel.bundle_url.clone().into());
                    ui.set_ota_status(
                        if newer {
                            format!("Update {} available", rel.version)
                        } else {
                            "Already on catalog version.".into()
                        }
                        .into(),
                    );
                }
                Err(e) => {
                    ui.set_ota_update_available(false);
                    ui.set_ota_status(e.into());
                }
            }
        });
    }

    fn download_bundle(&self) {
        self.with_ui(|ui| {
            let url = ui.get_ota_bundle_url().to_string();
            if url.is_empty() {
                ui.set_ota_status("No bundle URL".into());
                return;
            }
            ui.set_ota_busy(true);
            ui.set_ota_status(format!("Downloading {url} …").into());
            // Shop PC download only — write under config dir.
            let dest_dir = crate::config::SessionConfig::config_dir();
            let status = (|| -> Result<String, String> {
                let dir = dest_dir.ok_or("No config dir")?;
                std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
                let name = url
                    .rsplit('/')
                    .next()
                    .filter(|s| !s.is_empty())
                    .unwrap_or("bundle.raucb");
                let path = dir.join(name);
                let mut reader = ureq::get(&url)
                    .timeout(std::time::Duration::from_secs(120))
                    .call()
                    .map_err(|e| e.to_string())?
                    .into_reader();
                let mut bytes = Vec::new();
                std::io::Read::read_to_end(&mut reader, &mut bytes).map_err(|e| e.to_string())?;
                std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
                Ok(format!("Saved {}", path.display()))
            })();
            ui.set_ota_busy(false);
            match status {
                Ok(s) => ui.set_ota_status(s.into()),
                Err(e) => ui.set_ota_status(e.into()),
            }
        });
    }

    fn request_ecu_log(&self) {
        let msg = match request_log_export(&LogExportRequest::default()) {
            Ok(s) => s,
            Err(e) => e,
        };
        self.with_ui(|ui| {
            ui.set_logs_status(msg.clone().into());
            ui.set_status_text(msg.into());
        });
    }

    fn open_local_mdf4(&self) {
        self.with_ui(|ui| {
            ui.set_active_tab(6);
            ui.set_logs_status(
                "Switched to Analysis → MDF4 — use header MDF4 button or Open.".into(),
            );
            ui.invoke_open_mdf4();
        });
    }
}

fn or_dash(s: &str) -> &str {
    if s.is_empty() {
        "—"
    } else {
        s
    }
}
