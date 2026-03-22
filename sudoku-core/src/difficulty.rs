#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Difficulty {
    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Expert => "Expert",
        }
    }

    pub fn givens_range(&self) -> (usize, usize) {
        match self {
            Difficulty::Easy => (38, 43),
            Difficulty::Medium => (32, 37),
            Difficulty::Hard => (26, 31),
            Difficulty::Expert => (20, 25),
        }
    }

    pub fn all() -> &'static [Difficulty] {
        &[
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Hard,
            Difficulty::Expert,
        ]
    }

    pub fn next(&self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Expert,
            Difficulty::Expert => Difficulty::Easy,
        }
    }

    pub fn prev(&self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Expert,
            Difficulty::Medium => Difficulty::Easy,
            Difficulty::Hard => Difficulty::Medium,
            Difficulty::Expert => Difficulty::Hard,
        }
    }
}
