use super::Hint;

pub struct HintAccumulator {
    hints: Vec<Hint>,
}

impl HintAccumulator {
    pub fn new() -> Self {
        Self { hints: Vec::new() }
    }

    pub fn add(&mut self, hint: Hint) {
        self.hints.push(hint);
    }

    pub fn is_empty(&self) -> bool {
        self.hints.is_empty()
    }

    pub fn len(&self) -> usize {
        self.hints.len()
    }

    pub fn first(&mut self) -> Option<Hint> {
        if self.hints.is_empty() {
            None
        } else {
            Some(self.hints.remove(0))
        }
    }

    pub fn take_all(&mut self) -> Vec<Hint> {
        std::mem::take(&mut self.hints)
    }
}

impl Default for HintAccumulator {
    fn default() -> Self {
        Self::new()
    }
}
