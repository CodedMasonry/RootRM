///# Append added modules here (else they just won't work)

/// Module for Debug purposes
pub mod misc;
pub mod handlers;

#[cfg(feature = "gui")]
/// GUI module is feature if someone wants it (not implemented and increases compile time)
pub mod gui;