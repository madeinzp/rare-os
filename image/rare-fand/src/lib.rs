//! rare-fand: fan curves for handhelds whose fan the `ayaneo_ec` driver exposes (SuiPlay0X1,
//! AYANEO 2/2S and relatives). Runs as root, takes presets over the system bus (bus.rs) from
//! the launcher, and hands the fan back to the embedded controller on any error and on exit.
//! It also draws the stick ring lights on the same EC (lights.rs).

pub mod battery;
pub mod control;
pub mod curve;
pub mod daemon;
pub mod hwmon;
pub mod lights;
pub mod store;

#[cfg(target_os = "linux")]
pub mod bus;
