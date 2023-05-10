fn main() {
    let a  = proteins_from_rna("UUAUGUUGACAC");
    println!("your protein sequence will be displayed here =>{:?}",a)
}
// match all 20 proteins
pub fn codon_to_protein(codon: &str) -> Result<&'static str, &'static str> {//taken input as string reference and return result of Ok or Err
    match codon {
        "AUG" => Ok("Methionine"),
        "UUU" | "UUC" => Ok("Phenylalanine"),
        "UUA" | "UUG" => Ok("Leucine"),
        "UCU" | "UCC" | "UCA" | "UCG" => Ok("Serine"),
        "UAU" | "UAC" => Ok("Tyrosine"),
        "UGU" | "UGC" => Ok("Cysteine"),
        "UGG" => Ok("Tryptophan"),
        "CUU" | "CUC" | "CUA" | "CUG" => Ok("Leucine"),
        "CCU" | "CCC" | "CCA" | "CCG" => Ok("Proline"),
        "CAU" | "CAC" => Ok("Histidine"),
        "CAA" | "CAG" => Ok("Glutamine"),
        "CGU" | "CGC" | "CGA" | "CGG" => Ok("Arginine"),
        "AUU" | "AUC" | "AUA" => Ok("Isoleucine"),
        "ACU" | "ACC" | "ACA" | "ACG" => Ok("Threonine"),
        "AAU" | "AAC" => Ok("Asparagine"),
        "AAA" | "AAG" => Ok("Lysine"),
        "AGU" | "AGC" => Ok("Serine"),
        "AGA" | "AGG" => Ok("Arginine"),
        "GUU" | "GUC" | "GUA" | "GUG" => Ok("Valine"),
        "GCU" | "GCC" | "GCA" | "GCG" => Ok("Alanine"),
        "GAU" | "GAC" => Ok("Aspartic acid"),
        "GAA" | "GAG" => Ok("Glutamic acid"),
        "GGU" | "GGC" | "GGA" | "GGG" => Ok("Glycine"),
        "UAA" | "UAG" | "UGA" => Ok("STOP codon"),
        _ => Err("Invalid codon"),
    }
}

pub fn proteins_from_rna(rna: &str) -> Result<Vec<&'static str>, &'static str> {//taken input as string reference and return result of Vec Ok or Err
    let mut proteins = Vec::new();
    for codon in rna.as_bytes().chunks(3) {//running loop for every three characters
        if let Ok(codon_str) = std::str::from_utf8(codon) { //check to convert bytes to utf-8 string encoding
            if let Ok(protein) = codon_to_protein(codon_str) {
                if protein == "STOP codon" {
                    break; //break loop if stop codon encountered
                } else {
                    proteins.push(protein); //push each protein after loop to vec
                }
            } else {
                return Err("Invalid codon");
            }
        } else {
            return Err("Invalid sequence");
        }
    }
    Ok(proteins) //returns proteins vec
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_proteins_from_rna1() {
        assert_eq!(proteins_from_rna("AUGUUUUCU"), Ok(vec!["Methionine", "Phenylalanine", "Serine"]));//test case without stop codon
    }
    #[test]
    fn test_proteins_from_rna2() {
        assert_eq!(proteins_from_rna("AUGUUUUCUUAAAUG"), Ok(vec!["Methionine", "Phenylalanine", "Serine"]));//test case with stop codon proteins before and after stop will show only before ones
    }
    #[test]
    fn test_proteins_from_rna3() {
        assert_eq!(proteins_from_rna("AFSFRS"), Err("Invalid codon"));//Any gibberish value to throw error
    }
}
