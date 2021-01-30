#[derive(Debug, PartialEq)]
pub struct Dna {
    string: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    string: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let valid_chars = ['G', 'C', 'T', 'A'];

        for (i, c) in dna.chars().enumerate() {
            if !valid_chars.contains(&c) {
                return Err(i);
            }
        }

        Ok(Dna{ string: dna.to_string() })
    }

    pub fn into_rna(self) -> Rna {
        let mut out = String::from("");

        for c in self.string.chars() {
            out.push(match c {
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
        }

        Rna{ string: out }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let valid_chars = ['G', 'C', 'U', 'A'];

        for (i, c) in rna.chars().enumerate() {
            if !valid_chars.contains(&c) {
                return Err(i);
            }
        }

        Ok(Rna{ string: rna.to_string() })
    }
}
