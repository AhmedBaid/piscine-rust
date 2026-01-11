use chrono::Local;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Local::now();
        let date = now.format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            form_values: (field_name, field_value),
            date: date,
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }
        if self.password.is_empty() {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password is empty",
            ));
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }
        let mut symbol: bool = false;
        let mut letter: bool = false;
        let mut number: bool = false;

        for ch in self.password.chars() {
            if !ch.is_alphanumeric() && ch.is_ascii() {
                symbol = true
            }
            if ch.is_numeric() {
                number = true
            }
            if ch.is_alphabetic() {
                letter = true
            }
        }
        if symbol && letter && number {
            return Ok(());
        } else {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }
    }
}
