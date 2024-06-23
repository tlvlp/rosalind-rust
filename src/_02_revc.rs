use crate::workflows;


/**
 * https://rosalind.info/problems/revc
 */
pub fn run_with_workflow() {
    workflows::with_read_all(|buffer: &[u8]| solve(buffer))
}

fn solve(buffer: &[u8]) -> Vec<u8> {
    buffer.iter()
        .map(|&b| {
            workflows::check_ascii(b);
            match b.to_ascii_uppercase() as char  {
                'T' => 'A' as u8,
                'A' => 'T' as u8,
                'C' => 'G' as u8,
                'G' => 'C' as u8,
                c => panic!("Character '{c}' is not a nucleotide!")
            }
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let test_data = "AAAACCCGGT".as_bytes();
        let expected = "ACCGGGTTTT".as_bytes();

        assert_eq!(solve(test_data), expected);
    }
    
    #[test]
    #[should_panic(expected = "not a nucleotide")]
    fn error_for_non_nucleotide() {
        let test_data = "AAAACXCCGGT".as_bytes();
        solve(test_data);
    }
}