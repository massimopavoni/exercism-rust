use std::collections::HashMap;

fn is_nucleotide(c: char) -> bool {
    matches!(c, 'A' | 'C' | 'G' | 'T')
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    dna.chars().try_fold(0, |acc, n| {
        if !is_nucleotide(n) {
            return Err(n);
        }

        if n == nucleotide {
            Ok(acc + 1)
        } else {
            Ok(acc)
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    dna.chars().try_for_each(|n| {
        if !is_nucleotide(n) {
            return Err(n);
        }

        *counts.entry(n).or_insert(0) += 1;

        Ok(())
    })?;

    Ok(counts)
}
