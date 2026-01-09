use email_address::EmailAddress;

use super::entities::UserError;

pub fn validate_email(email: &str) -> Result<(), UserError> {
    if EmailAddress::is_valid(email) {
        Ok(())
    } else {
        Err(UserError::InvalidEmail)
    }
}
