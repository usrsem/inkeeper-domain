#[derive(Debug, PartialEq, Clone)]
pub struct ProgramInfo {
    pub program_name: String,
    pub window_title: String,
    pub file_path: String,
    pub file_name: String,
    pub process_id: u32,
    pub is_minimized: bool,
    pub is_active: bool,
}

impl ProgramInfo {
    pub fn new(program_name: &str, window_title: &str) -> Self {
        Self {
            program_name: String::from(program_name),
            window_title: String::from(window_title),
            file_name: String::new(),
            file_path: String::new(),
            process_id: 0,
            is_minimized: false,
            is_active: true,
        }
    }
}

#[derive(Debug, Eq)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Eq)]
pub enum ProgramName {
    Photoshop,
    Sai,
}
