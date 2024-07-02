use crate::workflows;


/**
 * https://rosalind.info/problems/scsp
 */
pub fn run() {
    workflows::read_all_lines(|buffer: Vec<String>| solve(buffer))
}

fn solve(buffer: Vec<String>) -> String {
    todo!("Alternate between the two lines' chars")
}

#[cfg(test)]
mod 

tests {
    use super::*;

    #[test]
    fn run_example() {
        let test_data = vec!["ATCTGAT".to_string(), "TGCATA".to_string()];
        let expected = "ATGCATGAT".to_string();

        assert_eq!(solve(test_data), expected);
    }

}