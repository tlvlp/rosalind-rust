use crate::workflows;


/**
 * https://rosalind.info/problems/rna
 */
pub fn run_with_workflow() {
    workflows::with_default_continuous_buffer(|buffer: &[u8]| solve(buffer))
}

fn solve(buffer: &[u8]) -> Vec<u8> {
    buffer.iter()
        .map(|&b| {
            workflows::check_ascii(b);
            if b as char == 'T' {
                'U' as u8
            } else {
                b
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let test_data = "GATGGAACTTGACTACGTAAATT".as_bytes();
        let expected = "GAUGGAACUUGACUACGUAAAUU".as_bytes();

        assert_eq!(solve(test_data), expected);
    }
}