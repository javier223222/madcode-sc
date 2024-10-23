#![no_std]

use sails_rs::prelude::*;

struct MadCodeService(());

#[sails_rs::service]
impl MadCodeService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn do_something(&mut self) -> String {
        "Hello from MadCode!".to_string()
    }

    // Service's query
    pub fn get_something(&self) -> String {
        "Hello from MadCode!".to_string()
    }    
}

pub struct MadCodeProgram(());

#[sails_rs::program]
impl MadCodeProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn mad_code(&self) -> MadCodeService {
        MadCodeService::new()
    }
}
