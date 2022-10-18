extern crate chrono;

use self::ProgramName::*;
use chrono::{NaiveDateTime, Utc};
use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub struct ProgramInfo {
    pub program_name: String,
    pub window_title: String,
    pub file_name: String,
    pub is_active: bool,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}

impl ProgramInfo {
    pub fn new(program_name: &str, window_title: &str) -> Self {
        Self {
            program_name: String::from(program_name),
            window_title: String::from(window_title),
            file_name: String::new(),
            is_active: true,
            start_time: Utc::now().naive_local(),
            end_time: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ProgramName {
    Photoshop,
    Sai,
}

impl ProgramName {
    pub fn iterator() -> Iter<'static, ProgramName> {
        static PROGRAM_NAMES: [ProgramName; 2] = [Photoshop, Sai];
        PROGRAM_NAMES.iter()
    }
}
