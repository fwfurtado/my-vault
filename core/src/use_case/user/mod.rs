mod signup;

pub use signup::SignUp;


#[derive(Debug)]
pub enum UserError {
    AlreadyExists(String),
}