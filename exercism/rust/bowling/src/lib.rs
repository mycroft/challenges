#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    frame: usize,
    first_shot: bool,
    current_pins: u16,
    score: u16,
    complete: bool,
    has_spare: bool,
    strike_bonus: (u16, u16),
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frame: 10,
            first_shot: true,
            current_pins: 10,
            score: 0,
            complete: false,
            has_spare: false,
            strike_bonus: (0, 0),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let mut strike : bool = false;

        if pins > self.current_pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.complete && self.has_spare {
            self.score += pins;
            self.has_spare = false;

            return Ok(())
        }

        if self.complete && self.strike_bonus.0 > 0 {
            self.score += pins * self.strike_bonus.0;
            self.strike_bonus.0 = self.strike_bonus.1;
            self.strike_bonus.1 = 0;

            if pins != 10 {
                self.current_pins -= pins;
            }

            return Ok(());
        }

        if self.complete {
            return Err(Error::GameComplete);
        }

        if self.first_shot && pins == 10 {
            strike = true;
        }

        self.score += pins * (1 + self.strike_bonus.0);
        if self.has_spare {
            self.score += pins;
            self.has_spare = false;
        }

        self.strike_bonus.0 = self.strike_bonus.1;
        self.strike_bonus.1 = 0;

        if strike {
            self.strike_bonus.0 += 1;
            self.strike_bonus.1 += 1;
        }

        self.current_pins -= pins;

        self.has_spare = !self.first_shot && self.current_pins == 0;

        // end of frame
        if strike || !self.first_shot {
            self.first_shot = true;
            self.current_pins = 10;
            if self.frame > 1 {
                self.frame -= 1;
            } else {
                self.complete = true;
            }

        } else {
            self.first_shot = false;

        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.complete && self.strike_bonus.0 == 0 && self.has_spare == false {
            Some(self.score)
        } else {
            None
        }
    }
}
