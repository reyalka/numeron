use std::{error::Error, fmt::Display};

use colored::Colorize;
use inquire::{length, validator::Validation, InquireError, Text};
use rand::Rng;

type Answer = Vec<usize>;

#[derive(Debug)]
pub struct HBResult {
    pub hit: usize,
    pub blow: usize,
}
impl Display for HBResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} hit, {} blow",
            self.hit.to_string().yellow().bold(),
            self.blow.to_string().yellow().bold()
        )
    }
}

pub struct Numeron {
    pub collect_answer: Answer,
    pub length: usize,
}

impl Numeron {
    pub fn new(length: usize) -> Numeron {
        let mut rng = rand::thread_rng();
        let mut collect_answer = Vec::new();
        let mut range = (0..=9).collect::<Vec<usize>>();
        for _ in 1..=length {
            let index = rng.gen_range(0..range.len());
            collect_answer.push(range[index]);
            range.remove(index);
        }
        Numeron {
            collect_answer,
            length,
        }
    }

    pub fn read_answer(&self) -> Result<Answer, InquireError> {
        let digit_validator = |s: &str| {
            if s.chars().all(|c| c.is_digit(10)) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Please enter a number".into()))
            }
        };

        let answer: String = Text::new("Guess the number:")
            .with_validator(length!(self.length))
            .with_validator(digit_validator)
            .prompt()?;
        Ok(answer
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>())
    }

    pub fn check(&self, answer: &Answer) -> HBResult {
        let mut hit = 0;
        let mut blow = 0;
        for (i, &a) in answer.iter().enumerate() {
            if self.collect_answer[i] == a {
                hit += 1
            } else if self.collect_answer.contains(&a) {
                blow += 1
            }
        }
        HBResult { hit, blow }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!(
            "generated number in {} digit.",
            self.length.to_string().green().bold()
        );
        let mut count = 0;
        loop {
            let answer = self.read_answer()?;
            count += 1;
            let hbresult = self.check(&answer);
            println!("{}", hbresult);

            if hbresult.hit == 3 {
                let text = format!("🎉Congratulations, you guessed in {count} times!").bold();
                println!("{}", text);
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_new_no_duplicate() {
    let numeron = Numeron::new(3);
    let correct_answer = &numeron.collect_answer;
    let hash_set = correct_answer.iter().collect::<std::collections::HashSet<_>>();
    assert_eq!(correct_answer.len(), hash_set.len());
}

    #[test]
    fn test_check_all_blow() {
        let answer = vec![2, 0, 1];
        let result = Numeron::check(
            &Numeron {
                collect_answer: vec![1, 2, 0],
                length: 3,
            },
            &answer,
        );
        assert_eq!(result.hit, 0);
        assert_eq!(result.blow, 3);
    }

    #[test]
    fn test_check_all_hit() {
        let answer = vec![0, 1, 2];
        let result = Numeron::check(
            &Numeron {
                collect_answer: vec![0, 1, 2],
                length: 3,
            },
            &answer,
        );
        assert_eq!(result.hit, 3);
        assert_eq!(result.blow, 0);
    }

    #[test]
    fn test_check_hit_and_blow() {
        let answer = vec![0, 2, 1];
        let result = Numeron::check(
            &Numeron {
                collect_answer: vec![0, 1, 2],
                length: 3,
            },
            &answer,
        );
        assert_eq!(result.hit, 1);
        assert_eq!(result.blow, 2);
    }

    #[test]
    fn test_check_with_long_length() {
        let answer = vec![0, 2, 8, 3, 4, 6];
        let result = Numeron::check(
            &Numeron {
                collect_answer:  vec![0, 1, 2, 3, 4, 8],
                length: 6,
            },
            &answer,
        );
        assert_eq!(result.hit, 3);
        assert_eq!(result.blow, 2);
    }

}
