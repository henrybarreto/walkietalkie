use std::error::Error;

use log::error;

use crate::{commander::command::Command, soldier::report::Report};

pub trait Communication {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>>
    where
        Self: Sized;
    fn from_bytes_to_vec(bytes: Vec<u8>) -> Result<Vec<Self>, Box<dyn Error>>
    where
        Self: Sized;
    fn from_vec_to_bytes(communication: Vec<Self>) -> Result<Vec<u8>, Box<dyn Error>>
    where
        Self: Sized;
}
impl Communication for Report {
    fn from_bytes(bytes: Vec<u8>) -> Result<Report, Box<dyn Error>> {
        match bincode::deserialize(&bytes) {
            Ok(report) => {
                let report: Report = report;
                Ok(report)
            }
            Err(error) => {
                error!("Could not convert from bytes to Report");
                Err(error)
            }
        }
    }
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        match bincode::serialize(self) {
            Ok(bytes) => Ok(bytes),
            Err(error) => {
                error!("Could not convert from Report to bytes");
                Err(error)
            }
        }
    }
    fn from_bytes_to_vec(bytes: Vec<u8>) -> Result<Vec<Report>, Box<dyn Error>> {
        match bincode::deserialize(&bytes) {
            Ok(reports) => {
                let reports: Vec<Report> = reports;
                Ok(reports)
            }
            Err(error) => {
                error!("Could not convert from bytes to Vec<Report>");
                Err(error)
            }
        }
    }
    fn from_vec_to_bytes(reports: Vec<Report>) -> Result<Vec<u8>, Box<dyn Error>> {
        match bincode::serialize(&reports) {
            Ok(bytes) => {
                let bytes: Vec<u8> = bytes;
                Ok(bytes)
            }
            Err(error) => {
                error!("Could not convert from Vec<Report> to bytes");
                Err(error)
            }
        }
    }
}

impl Communication for Command {
    fn from_bytes(bytes: Vec<u8>) -> Result<Command, Box<dyn Error>> {
        match bincode::deserialize(&bytes) {
            Ok(command) => {
                let command: Command = command;
                Ok(command)
            }
            Err(error) => {
                error!("Could not convering from bytes to Command");
                Err(error)
            }
        }
    }
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        match bincode::serialize(&self) {
            Ok(bytes) => Ok(bytes),
            Err(error) => {
                error!("Could not convering from Command to bytes");
                Err(error)
            }
        }
    }
    fn from_bytes_to_vec(bytes: Vec<u8>) -> Result<Vec<Command>, Box<dyn Error>> {
        match bincode::deserialize(&bytes) {
            Ok(commands) => {
                let commands: Vec<Command> = commands;
                Ok(commands)
            }
            Err(error) => {
                error!("Could not convert from bytes to Vec<Command>");
                Err(error)
            }
        }
    }

    fn from_vec_to_bytes(commands: Vec<Command>) -> Result<Vec<u8>, Box<dyn Error>> {
        match bincode::serialize(&commands) {
            Ok(bytes) => {
                let bytes: Vec<u8> = bytes;
                Ok(bytes)
            }
            Err(error) => {
                error!("Could not convert from Vec<Command> to bytes");
                Err(error)
            }
        }
    }
}
