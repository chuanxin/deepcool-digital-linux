//! Display module for: AK series / SK700V (Final Corrected Edition)
//!
//! Confirmed SK700V packet layout (reverse-engineered):
//!
//!  D0         REPORT ID        = 16
//!  D1         FIXED HEADER     = 104
//!  D2         FIXED HEADER     = 1
//!  D3         FIXED HEADER     = 4    (SK700V specific)
//!  D4         FIXED HEADER     = 13   (SK700V specific)
//!  D5         FIXED HEADER     = 1
//!  D6         FIXED HEADER     = 2
//!  D7         POWER U16 [high]           Big-Endian uint16 (watts)
//!  D8         POWER U16 [low]            /
//!  D9         POWER PERCENTAGE           0–100 (progress bar beneath power reading)
//!  D10        TEMPERATURE UNIT           0 = °C, 1 = °F
//!  D11~D14    TEMPERATURE F32            Big-Endian IEEE 754 float (in selected unit)
//!  D15        UTILIZATION                0–100 (%)
//!  D16        FREQUENCY U16 [high]       Big-Endian uint16 (MHz)
//!  D17        FREQUENCY U16 [low]        /
//!  D18        CHECKSUM                   (D1~D17 sum) % 256
//!  D19        TERMINATION BYTE           = 22
//!  D20+       NOT USED
use crate::{devices::AUTO_MODE_INTERVAL, monitor::cpu::Cpu};
use super::{device_error, Mode};
use hidapi::HidApi;
use std::{thread::sleep, time::{Duration, Instant}};
use std::cell::Cell;
pub const DEFAULT_MODE: Mode = Mode::CpuTemperature;
pub const TEMP_LIMIT_C: u8 = 90;
pub const TEMP_LIMIT_F: u8 = 194;
// --- Configuration for Smoothing (EMA Alpha values) ---
// Note: 1.0 = No smoothing (Real-time), 0.1 = Very smooth (Slow transition)
pub const POWER_SMOOTHING_ALPHA: f32 = 0.4;
pub const TEMP_SMOOTHING_ALPHA: f32 = 0.8;
// Fallback max power (watts) when RAPL power limit cannot be read from the OS.
// Used as the 100% reference for the D9 progress bar.
pub const FALLBACK_MAX_POWER_W: f32 = 250.0;
pub struct Display {
    cpu: Cpu,
    pub mode: Mode,
    update: Duration,
    fahrenheit: bool,
    alarm: bool,
    // CPU TDP read from RAPL at startup (watts); used as the D9 progress bar ceiling.
    max_power: f32,
    // Storage for the previous second's data (used for EMA smoothing)
    last_power: Cell<f32>,
    last_temp: Cell<f32>,
}
impl Display {
    /// Reads the CPU's peak power limit for the D9 progress bar ceiling.
    ///
    /// Priority:
    ///   1. PL2 / constraint_1 (short-term burst limit) — preferred, reflects true maximum.
    ///      e.g. i7-14700: PL1=65W, PL2=253W → use 253W so the bar doesn't peg at moderate load.
    ///   2. PL1 / constraint_0 (long-term / TDP) — fallback if PL2 is unavailable.
    ///   3. FALLBACK_MAX_POWER_W constant — last resort.
    fn read_rapl_power_limit() -> f32 {
        // (path, label) pairs tried in order — first valid non-zero value wins.
        let candidates: &[(&str, &str)] = &[
            // Intel PL2 (short-term burst)
            ("/sys/class/powercap/intel-rapl:0/constraint_1_power_limit_uw", "Intel PL2 (burst)"),
            // Intel PL1 (long-term / TDP)
            ("/sys/class/powercap/intel-rapl:0/constraint_0_power_limit_uw", "Intel PL1 (TDP)"),
            // AMD
            ("/sys/class/powercap/amd-energy/constraint_1_power_limit_uw",   "AMD PL2 (burst)"),
            ("/sys/class/powercap/amd-energy/constraint_0_power_limit_uw",   "AMD PL1 (TDP)"),
        ];
        for (path, label) in candidates {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(uw) = content.trim().parse::<u64>() {
                    if uw > 0 {
                        let watts = uw as f32 / 1_000_000.0;
                        eprintln!("[SK700V] Progress bar ceiling: {:.1} W  [{}]", watts, label);
                        return watts;
                    }
                }
            }
        }
        eprintln!(
            "[SK700V] RAPL power limit not available, using fallback: {:.1} W",
            FALLBACK_MAX_POWER_W
        );
        FALLBACK_MAX_POWER_W
    }
    pub fn new(cpu: Cpu, mode: &Mode, update: Duration, fahrenheit: bool, alarm: bool) -> Self {
        let mode = match mode {
            Mode::Default => DEFAULT_MODE,
            Mode::Auto => Mode::Auto,
            Mode::CpuTemperature => Mode::CpuTemperature,
            Mode::CpuUsage => Mode::CpuUsage,
            _ => mode.support_error(),
        };
        Display {
            cpu,
            mode,
            update,
            fahrenheit,
            alarm,
            max_power: Self::read_rapl_power_limit(),
            last_power: Cell::new(0.0),
            last_temp: Cell::new(0.0),
        }
    }
    pub fn run(&self, api: &HidApi, vid: u16, pid: u16) {
        let device = api.open(vid, pid).unwrap_or_else(|_| device_error());
        self.cpu.warn_temp();
        let mut data: [u8; 64] = [0; 64];
        data[0] = 16;
        {
            let mut init_data = data.clone();
            init_data[1] = 170;
            device.write(&init_data).unwrap();
        }
        match self.mode {
            Mode::Auto => {
                let mut initial_update = self.update;
                let mut mode = Mode::CpuTemperature;
                loop {
                    device.write(&self.status_message(&data, &mode, initial_update)).unwrap();
                    let timeout = Instant::now() + AUTO_MODE_INTERVAL;
                    while Instant::now() + self.update < timeout {
                        device.write(&self.status_message(&data, &mode, self.update)).unwrap();
                    }
                    initial_update = timeout - Instant::now();
                    mode = match mode {
                        Mode::CpuTemperature => Mode::CpuUsage,
                        Mode::CpuUsage => Mode::CpuTemperature,
                        _ => DEFAULT_MODE,
                    }
                }
            }
            _ => loop {
                device.write(&self.status_message(&data, &self.mode, self.update)).unwrap();
            }
        }
    }
    fn status_message(&self, _inital_data: &[u8; 64], _mode: &Mode, update: Duration) -> [u8; 64] {
        let cpu_instant = self.cpu.read_instant();
        // 1. (RAPL) Read energy before sleep (microjoules)
        let rapl_path = "/sys/class/powercap/intel-rapl:0/energy_uj";
        let energy_before: u64 = std::fs::read_to_string(rapl_path)
            .unwrap_or_default().trim().parse().unwrap_or(0);
        // System pause (usually 1 second)
        sleep(update);
        // 2. (RAPL) Read energy after sleep (microjoules)
        let energy_after: u64 = std::fs::read_to_string(rapl_path)
            .unwrap_or_default().trim().parse().unwrap_or(0);
        let usage = self.cpu.get_usage(cpu_instant);
        // get_temp() already returns the value in the correct unit (°C or °F)
        let temp = self.cpu.get_temp(self.fahrenheit);
        // --- SK700V Packet ---
        let mut data = [0u8; 64];
        // D0~D6: Report ID + Fixed Header
        data[0..7].copy_from_slice(&[16, 104, 1, 4, 13, 1, 2]);
        // --- D7~D8: POWER CONSUMPTION (Big-Endian U16, watts) ---
        let mut current_power: f32 = 0.0;
        if energy_after >= energy_before {
            let delta_uj = energy_after - energy_before;
            let seconds = update.as_secs_f64();
            if seconds > 0.0 {
                current_power = (delta_uj as f64 / 1_000_000.0) as f32 / seconds as f32;
            }
        } else {
            // Fallback: If RAPL counter overflows, use the last known power
            current_power = self.last_power.get();
        }
        let smoothed_power = if self.last_power.get() == 0.0 {
            current_power
        } else {
            current_power * POWER_SMOOTHING_ALPHA + self.last_power.get() * (1.0 - POWER_SMOOTHING_ALPHA)
        };
        self.last_power.set(smoothed_power);
        let power_w = smoothed_power as u16;
        data[7] = ((power_w >> 8) & 0xFF) as u8; // D7: Power high byte (Big-Endian)
        data[8] = (power_w & 0xFF) as u8;         // D8: Power low byte
        // --- D9: POWER PERCENTAGE (progress bar, 0–100) ---
        // self.max_power is read from RAPL PL1 at startup (≈ CPU TDP).
        // Clamped to 100 so boost spikes never overflow the bar.
        let power_pct = ((smoothed_power / self.max_power) * 100.0).clamp(0.0, 100.0) as u8;
        data[9] = power_pct;
        // --- D10: TEMPERATURE UNIT FLAG ---
        // 0 = °C, 1 = °F — must match what get_temp() returns.
        data[10] = if self.fahrenheit { 1 } else { 0 };
        // --- D11~D14: TEMPERATURE (Big-Endian F32) with EMA smoothing ---
        let current_temp = temp as f32;
        let smoothed_temp = if self.last_temp.get() == 0.0 {
            current_temp
        } else {
            current_temp * TEMP_SMOOTHING_ALPHA + self.last_temp.get() * (1.0 - TEMP_SMOOTHING_ALPHA)
        };
        self.last_temp.set(smoothed_temp);
        let temp_bytes = smoothed_temp.to_be_bytes();
        data[11..15].copy_from_slice(&temp_bytes); // D11~D14
        // --- D15: CPU UTILIZATION (No smoothing, real 1-second average) ---
        data[15] = usage as u8;
        // --- D16~D17: CPU FREQUENCY (Big-Endian U16, MHz, no smoothing) ---
        let mut current_freq: u16 = 0;
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            let mut max_f: f32 = 0.0;
            for line in cpuinfo.lines() {
                if line.starts_with("cpu MHz") {
                    if let Some(val) = line.split(':').nth(1) {
                        if let Ok(f) = val.trim().parse::<f32>() {
                            if f > max_f { max_f = f; }
                        }
                    }
                }
            }
            if max_f > 0.0 { current_freq = max_f as u16; }
        }
        data[16] = ((current_freq >> 8) & 0xFF) as u8; // D16: Freq high byte
        data[17] = (current_freq & 0xFF) as u8;         // D17: Freq low byte
        // --- D18: CHECKSUM (sum of D1~D17, mod 256) ---
        let mut checksum: u32 = 0;
        for i in 1..=17 {
            checksum += data[i] as u32;
        }
        data[18] = (checksum % 256) as u8;
        // --- D19: TERMINATION BYTE ---
        data[19] = 22;
        data
    }
}
