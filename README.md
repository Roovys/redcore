# RedCore (`redcore`)
A zero-bloat, open-source replacement for AMD Radeon Software on Windows. Built in Rust with official AMD ADLX SDK bindings—uses under 15 MB RAM, zero background telemetry, and zero embedded browser lag.

## Core Features (v1.0)
- **Native GPU Tuning & Per-Game Profiles:** Direct ADLX control for undervolting, clock offsets, custom fan curves, Radeon Anti-Lag, and Chill that actually persist across reboots.
- **Crash-Safe Undervolt Watchdog:** Detects unexpected power loss or Windows TDR driver timeouts and triggers a mandatory 15-second confirmation prompt before re-applying voltage offsets—preventing boot loops.
- **Crisp HEVC/CQP Instant Replay:** Lightweight background replay buffer utilizing direct AMD AMF hardware encoding (HEVC + CQP rate control) instead of blurry default H.264 presets.
- **100% Offline & Bloat-Free:** No WebView2/Chromium overhead, no news banners, and no telemetry daemons.

## Hardware Safety Guardrails
- **Strict ADLX VBIOS Bounds:** Enforces factory VBIOS voltage and power limits at the driver level—no unsafe registry edits or VBIOS flashing.
- **Thermal Kill-Switch:** Polls GPU hotspot sensors every 500ms; automatically calls `ReleaseToFirmware` to restore stock clocks/voltage if hotspot exceeds 90°C or a driver reset occurs.

## Tech Stack
- **Core / Daemon:** Rust + C++ ADLX FFI bindings
- **UI:** Native lightweight tray & control window (Slint)
- **Media:** Direct AMD AMF / FFmpeg CQP pipeline

## License
GNU General Public License v3.0 (GPL-3.0)
