#[derive(Clone, Debug, PartialEq)]
pub enum SeverityLevel {
    Info,
    Warning,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Notification {
    pub msg: String,
    pub level: SeverityLevel,
}

impl Default for Notification {
    fn default() -> Self {
        return Notification {
            msg: String::from(""),
            level: SeverityLevel::Info,
        };
    }
}

impl Notification {
    pub fn new(msg: String, level: SeverityLevel) -> Self {
        return Notification { msg, level };
    }

    pub fn get_class_name(&self) -> String {
        match self.level {
            SeverityLevel::Info => String::from("snackbar-info"),
            SeverityLevel::Warning => String::from("snackbar-warning"),
        }
    }
}
