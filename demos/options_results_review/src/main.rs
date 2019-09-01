use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct DBError;

type DbResult<T> = Result<T, DBError>;

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in DB Result")
    }
}

impl error::Error for DBError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

//Check an option's value and set if it is None
fn or_else_return_an_option(y: Option<i32>) -> Option<i32> {
    y.or_else(|| Some(0))
}

//Check an option's value, returning it has a Result<i32>
//OR returning a custom Error if it is None
fn or_else_return_a_result(y: Option<i32>) -> DbResult<i32> {
    y.ok_or_else(|| return DBError)
}

fn double_with_result_from_option(y: Option<i32>) -> DbResult<i32> {

    validate = |x| x { if x > i32::Max }
    y.or_else(|| return DBError).and_then(|x| Ok(x*2))
}

#[test]
fn or_else_success_test() {

#[test]
fn or_else_success_test() {
    let test_result = or_else_return_an_option(Some(12));
    assert_eq!(test_result, Some(12));
}

#[test]
fn or_else_fail_test() {
    let test_result = or_else_return_an_option(None);
    assert_ne!(test_result, Some(12));
}

#[test]
fn or_else_success_return_a_result_test() {
    let test_result = or_else_return_a_result(Some(12));
    assert_eq!(test_result, Ok(12));
}

#[test]
fn or_else_fail_return_a_result_test() {
    let test_result = or_else_return_a_result(None);
    assert_eq!(test_result, Err(DBError));
}

