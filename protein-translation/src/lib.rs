use std::{collections::HashMap, sync::LazyLock};

static CODONS: LazyLock<HashMap<&[u8; 3], &str>> = LazyLock::new(|| {
    HashMap::from([
        (b"AUG", "Methionine"),
        (b"UUU", "Phenylalanine"),
        (b"UUC", "Phenylalanine"),
        (b"UUA", "Leucine"),
        (b"UUG", "Leucine"),
        (b"UCU", "Serine"),
        (b"UCC", "Serine"),
        (b"UCA", "Serine"),
        (b"UCG", "Serine"),
        (b"UAU", "Tyrosine"),
        (b"UAC", "Tyrosine"),
        (b"UGU", "Cysteine"),
        (b"UGC", "Cysteine"),
        (b"UGG", "Tryptophan"),
        (b"UAA", ""),
        (b"UAG", ""),
        (b"UGA", ""),
    ])
});

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut proteins = Vec::with_capacity(rna.len() / 3);

    for codon in rna.as_bytes().chunks(3) {
        let codon: &[u8; 3] = codon.try_into().ok()?;
        let protein = CODONS.get(codon);

        match protein {
            None => return None,
            Some(&"") => break,
            Some(&protein) => proteins.push(protein),
        }
    }

    Some(proteins)
}
