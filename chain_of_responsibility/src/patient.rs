#[derive(Default, Debug, Clone)]
pub struct Patient {
    name: String,
    level: u32,
    log_stack: Vec<String>,
    fee: u32,
}

impl Patient {
    pub fn new(name: String, level: u32) -> Self {
        Self {
            name,
            level,
            log_stack: Vec::new(),
            fee: 0,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn level(&self) -> u32 {
        self.level
    }
    pub fn log_stack(&self) -> &[String] {
        &self.log_stack
    }
    pub fn log_stack_mut(&mut self) -> &mut Vec<String> {
        &mut self.log_stack
    }
    pub fn fee(&self) -> u32 {
        self.fee
    }
    pub fn fee_mut(&mut self) -> &mut u32 {
        &mut self.fee
    }
}
