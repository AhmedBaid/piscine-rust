#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "CEO" => Self::CEO,
            "Manager" => Self::Manager,
            _ => Self::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let worker = Worker {
            name: name.to_string(),
            role: Role::from(role),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(worker))
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let k = self.grade.take()?;
        self.grade = k.next;
        Some(k.name)
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        let last = self.grade.as_ref()?;
        Some((last.name.clone(), last.role))
    }
}
