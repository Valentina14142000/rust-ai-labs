use super::memory::Memory;

#[derive(Default)]
pub struct MemoryStore {
    memory: Memory,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }
}
