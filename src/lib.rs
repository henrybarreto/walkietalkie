//! # Walkietalkie
//!
//! Walkietalkie is an application to help system admins to execute simple payloads in many remote
//! devices at once.
#![feature(slice_as_chunks)]
#![feature(with_options)]

pub mod commander;
pub mod communication;
pub mod config;
pub mod devices;
pub mod radio;
pub mod report;
pub mod seal;
pub mod soldier;
