#[derive(Debug, Clone)]
pub struct Memory {
    entries: Vec<String>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
