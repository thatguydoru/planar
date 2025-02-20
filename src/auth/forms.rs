use validator::Validate;
use serde::Deserialize;

#[derive(Debug, Validate, Deserialize)]
pub struct SignupForm {
    #[validate(length(min = 8, max = 255))]
    username: String,
    #[validate(length(min = 8, max = 256))]
    password: String,
}
