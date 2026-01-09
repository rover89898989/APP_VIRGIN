use email_address::EmailAddress;

use super::entities::UserError;

#[allow(dead_code)] // Used by repository stubs - will be active when schema.rs exists
pub fn validate_email(email: &str) -> Result<(), UserError> {
    if EmailAddress::is_valid(email) {
        Ok(())
    } else {
        Err(UserError::InvalidEmail)
    }
}
