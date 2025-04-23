trait HasNucleotides {
    const NEUCLEOTIDES: [char; 4];

    fn is_neucleotide(c: &char) -> bool {
        Self::NEUCLEOTIDES.contains(c)
    }

    fn is_valid_sequence(seq: &str) -> Result<(), usize> {
        for (i, c) in seq.chars().enumerate() {
            if !Self::is_neucleotide(&c) {
                return Err(i);
            }
        }

        Ok(())
    }
}

impl HasNucleotides for Dna {
    const NEUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
}

impl HasNucleotides for Rna {
    const NEUCLEOTIDES: [char; 4] = ['U', 'G', 'C', 'A'];
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        Dna::is_valid_sequence(dna)?;

        Ok(Dna {
            sequence: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            sequence: self
                .sequence
                .chars()
                .map(|c| Rna::NEUCLEOTIDES[Dna::NEUCLEOTIDES.iter().position(|&x| x == c).unwrap()])
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        Rna::is_valid_sequence(rna)?;

        Ok(Rna {
            sequence: rna.to_string(),
        })
    }
}
