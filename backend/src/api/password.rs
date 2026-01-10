// ==============================================================================
// PASSWORD HASHING WITH ARGON2
// ==============================================================================
//
// Uses Argon2id for password hashing - the winner of the Password Hashing
// Competition and recommended by OWASP for password storage.
//
// SECURITY:
// - Argon2id is resistant to GPU attacks and side-channel attacks
// - Uses random salt per password (stored in the hash string)
// - Memory-hard: requires significant RAM, defeating parallel attacks
//
// ==============================================================================

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use super::ApiError;

// ==============================================================================
// PASSWORD HASHING
// ==============================================================================

/// Hash a password using Argon2id.
/// 
/// # Arguments
/// * `password` - The plaintext password to hash
/// 
/// # Returns
/// * `Ok(String)` - The PHC-formatted hash string (includes salt, params, hash)
/// * `Err(ApiError)` - Hashing failed
/// 
/// # Example
/// ```
/// let hash = hash_password("user_password")?;
/// // hash looks like: $argon2id$v=19$m=19456,t=2,p=1$salt$hash
/// ```
pub fn hash_password(password: &str) -> Result<String, ApiError> {
    // Validate password before hashing
    validate_password_strength(password)?;
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default(); // Uses recommended params
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Password hashing failed: {}", e);
            ApiError::InternalError("Password hashing failed".to_string())
        })?;
    
    Ok(password_hash.to_string())
}

/// Verify a password against a stored hash.
/// 
/// # Arguments
/// * `password` - The plaintext password to verify
/// * `hash` - The stored PHC-formatted hash string
/// 
/// # Returns
/// * `Ok(true)` - Password matches
/// * `Ok(false)` - Password does not match
/// * `Err(ApiError)` - Verification failed (malformed hash, etc.)
/// 
/// # Timing Safety
/// This function uses constant-time comparison to prevent timing attacks.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, ApiError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| {
            tracing::error!("Failed to parse password hash: {}", e);
            ApiError::InternalError("Password verification failed".to_string())
        })?;
    
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false), // Wrong password
        Err(e) => {
            tracing::error!("Password verification error: {}", e);
            Err(ApiError::InternalError("Password verification failed".to_string()))
        }
    }
}

// ==============================================================================
// PASSWORD VALIDATION
// ==============================================================================

/// Minimum password length (NIST SP 800-63B recommends 8+)
const MIN_PASSWORD_LENGTH: usize = 8;

/// Maximum password length (prevent DoS via huge passwords)
const MAX_PASSWORD_LENGTH: usize = 128;

/// Validate password strength.
/// 
/// # Rules
/// - Minimum 8 characters (NIST recommendation)
/// - Maximum 128 characters (prevent DoS)
/// - Must contain at least one letter and one number
/// 
/// # Note
/// NIST SP 800-63B recommends against complexity rules (special chars, etc.)
/// in favor of length and checking against common password lists.
pub fn validate_password_strength(password: &str) -> Result<(), ApiError> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ApiError::BadRequest(format!(
            "Password must be at least {} characters",
            MIN_PASSWORD_LENGTH
        )));
    }
    
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ApiError::BadRequest(format!(
            "Password must be at most {} characters",
            MAX_PASSWORD_LENGTH
        )));
    }
    
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    
    if !has_letter || !has_digit {
        return Err(ApiError::BadRequest(
            "Password must contain at least one letter and one number".to_string()
        ));
    }
    
    Ok(())
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_and_verify_password() {
        let password = "SecurePass123";
        let hash = hash_password(password).unwrap();
        
        // Should start with argon2id
        assert!(hash.starts_with("$argon2id$"));
        
        // Correct password should verify
        assert!(verify_password(password, &hash).unwrap());
        
        // Wrong password should not verify
        assert!(!verify_password("WrongPass123", &hash).unwrap());
    }
    
    #[test]
    fn test_different_passwords_different_hashes() {
        let hash1 = hash_password("Password123").unwrap();
        let hash2 = hash_password("Password123").unwrap();
        
        // Same password should produce different hashes (different salts)
        assert_ne!(hash1, hash2);
    }
    
    #[test]
    fn test_password_too_short() {
        let result = hash_password("Short1");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_password_no_letter() {
        let result = hash_password("12345678");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_password_no_digit() {
        let result = hash_password("NoDigitsHere");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_valid_password_accepted() {
        let result = validate_password_strength("ValidPass1");
        assert!(result.is_ok());
    }
}
