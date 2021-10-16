//! # Walkietalkie
//!
//! This crate provides elements to communication between a server and clients, execute commands and
//! send responses.
#![feature(slice_as_chunks)]

pub mod commander;
pub mod communication;
pub mod config;
pub mod devices;
pub mod radio;
pub mod report;
pub mod seal;
pub mod soldier;
