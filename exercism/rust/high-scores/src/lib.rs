#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scores: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.len() > 0 {
            Some(*self.scores.last().unwrap())
        } else {
            None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() > 0 {
            Some(*self.scores.iter().max().unwrap())
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone();
        scores.sort_by(|a, b| b.partial_cmp(a).unwrap());
        scores.iter().take(3).cloned().collect::<Vec<u32>>()
    }
}
