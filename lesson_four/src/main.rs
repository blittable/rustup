//Write some code that does the following:
//1: uses an Option<T>
//2: uses Result<T, E> type
//3: uses 'match'
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ScoreError(String);

impl fmt::Display for ScoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for ScoreError {}

struct ExamResult {
    course_name: String,
    score: Option<u8>,
}

fn calculate_grade(score: Option<u8>) -> Result<String, Box<dyn Error>> {
    if score > Some(100) {
        return Result::Err(Box::new(ScoreError("Score should not be more than 100".into())))
    } else if score > Some(90) {
        return Ok("A".to_string())
    } else if score > Some(80) {
        return Ok("B".to_string())
    } else if score > Some(70) {
        return Ok("C".to_string())
    } else if score > Some(60) {
        return Ok("D".to_string())
    } else {
        return Ok("F".to_string())
    }
}

fn main() {
    let computer_101: ExamResult = ExamResult {
        course_name: "Computer 101".to_string(),
        score: Some(101),
    };

    match calculate_grade(computer_101.score) {
        Ok(result) => println!("You've got {}", result),
        Err(err) => println!("Error: {:?}", err),
    }
}
