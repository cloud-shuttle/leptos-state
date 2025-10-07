//! Authentication credentials for connections

/// Authentication credentials
#[derive(Debug, Clone, PartialEq)]
pub enum Credentials {
    /// No authentication
    None,
    /// Basic authentication (username/password)
    Basic { username: String, password: String },
    /// Bearer token authentication
    Bearer { token: String },
    /// API key authentication
    ApiKey { key: String, header_name: String },
    /// Custom authentication
    Custom { auth_type: String, parameters: std::collections::HashMap<String, String> },
}

impl Credentials {
    /// Create basic auth credentials
    pub fn basic<S: Into<String>>(username: S, password: S) -> Self {
        Self::Basic {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Create bearer token credentials
    pub fn bearer<S: Into<String>>(token: S) -> Self {
        Self::Bearer { token: token.into() }
    }

    /// Create API key credentials
    pub fn api_key<S: Into<String>>(key: S, header_name: S) -> Self {
        Self::ApiKey {
            key: key.into(),
            header_name: header_name.into(),
        }
    }

    /// Create custom credentials
    pub fn custom<S: Into<String>>(auth_type: S, parameters: std::collections::HashMap<String, String>) -> Self {
        Self::Custom {
            auth_type: auth_type.into(),
            parameters,
        }
    }

    /// Get authentication type
    pub fn auth_type(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Basic { .. } => "basic",
            Self::Bearer { .. } => "bearer",
            Self::ApiKey { .. } => "api_key",
            Self::Custom { .. } => "custom",
        }
    }

    /// Validate credentials
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::None => Ok(()),
            Self::Basic { username, password } => {
                if username.trim().is_empty() {
                    return Err("username cannot be empty".to_string());
                }
                if password.trim().is_empty() {
                    return Err("password cannot be empty".to_string());
                }
                // Additional validation for password strength could be added here
                Ok(())
            }
            Self::Bearer { token } => {
                if token.trim().is_empty() {
                    return Err("token cannot be empty".to_string());
                }
                // Could validate token format (e.g., JWT) here
                Ok(())
            }
            Self::ApiKey { key, header_name } => {
                if key.trim().is_empty() {
                    return Err("API key cannot be empty".to_string());
                }
                if header_name.trim().is_empty() {
                    return Err("header name cannot be empty".to_string());
                }
                // Validate header name format
                if !header_name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                    return Err("header name must contain only alphanumeric characters, hyphens, and underscores".to_string());
                }
                Ok(())
            }
            Self::Custom { auth_type, parameters } => {
                if auth_type.trim().is_empty() {
                    return Err("auth_type cannot be empty".to_string());
                }
                if parameters.is_empty() {
                    return Err("custom auth must have parameters".to_string());
                }
                // Validate parameter keys
                for key in parameters.keys() {
                    if key.trim().is_empty() {
                        return Err("parameter keys cannot be empty".to_string());
                    }
                }
                Ok(())
            }
        }
    }

    /// Check if credentials are secure (have actual authentication)
    pub fn is_secure(&self) -> bool {
        !matches!(self, Self::None)
    }

    /// Get credential summary (without sensitive data)
    pub fn summary(&self) -> String {
        match self {
            Self::None => "No authentication".to_string(),
            Self::Basic { username, .. } => format!("Basic auth for user '{}'", username),
            Self::Bearer { .. } => "Bearer token authentication".to_string(),
            Self::ApiKey { header_name, .. } => format!("API key authentication (header: {})", header_name),
            Self::Custom { auth_type, .. } => format!("Custom authentication: {}", auth_type),
        }
    }

    /// Get the authorization header value for HTTP requests
    pub fn authorization_header(&self) -> Option<String> {
        match self {
            Self::None => None,
            Self::Basic { username, password } => {
                let credentials = format!("{}:{}", username, password);
                let encoded = base64::encode(credentials);
                Some(format!("Basic {}", encoded))
            }
            Self::Bearer { token } => Some(format!("Bearer {}", token)),
            Self::ApiKey { key, header_name } => Some(format!("{} {}", header_name, key)),
            Self::Custom { auth_type, parameters } => {
                // For custom auth, we might need to construct a custom header
                // This is a simplified implementation
                Some(format!("{} {}", auth_type, parameters.values().next().unwrap_or(&"".to_string())))
            }
        }
    }

    /// Check if credentials can be used for a specific protocol
    pub fn supports_protocol(&self, protocol: &str) -> bool {
        match (self, protocol) {
            (Self::None, _) => true,
            (_, "http" | "https") => true,
            (_, "ws" | "wss") => matches!(self, Self::Bearer { .. } | Self::ApiKey { .. }),
            (_, "tcp") => matches!(self, Self::Custom { .. }),
            _ => false,
        }
    }

    /// Redact sensitive information for logging
    pub fn redact(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Basic { username, .. } => Self::Basic {
                username: username.clone(),
                password: "***REDACTED***".to_string(),
            },
            Self::Bearer { .. } => Self::Bearer {
                token: "***REDACTED***".to_string(),
            },
            Self::ApiKey { header_name, .. } => Self::ApiKey {
                key: "***REDACTED***".to_string(),
                header_name: header_name.clone(),
            },
            Self::Custom { auth_type, parameters } => Self::Custom {
                auth_type: auth_type.clone(),
                parameters: parameters.iter()
                    .map(|(k, _)| (k.clone(), "***REDACTED***".to_string()))
                    .collect(),
            },
        }
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl Drop for Credentials {
    fn drop(&mut self) {
        // Secure cleanup of sensitive data
        match self {
            Self::Basic { password, .. } => {
                // Overwrite password with zeros
                password.clear();
                unsafe {
                    password.as_mut_vec().fill(0);
                }
            }
            Self::Bearer { token } => {
                token.clear();
                unsafe {
                    token.as_mut_vec().fill(0);
                }
            }
            Self::ApiKey { key, .. } => {
                key.clear();
                unsafe {
                    key.as_mut_vec().fill(0);
                }
            }
            Self::Custom { parameters, .. } => {
                for value in parameters.values_mut() {
                    value.clear();
                    unsafe {
                        value.as_mut_vec().fill(0);
                    }
                }
            }
            Self::None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_constructors() {
        let basic = Credentials::basic("user", "pass");
        assert!(matches!(basic, Credentials::Basic { .. }));

        let bearer = Credentials::bearer("token123");
        assert!(matches!(bearer, Credentials::Bearer { .. }));

        let api_key = Credentials::api_key("key456", "X-API-Key");
        assert!(matches!(api_key, Credentials::ApiKey { .. }));

        let mut params = std::collections::HashMap::new();
        params.insert("param1".to_string(), "value1".to_string());
        let custom = Credentials::custom("oauth2", params);
        assert!(matches!(custom, Credentials::Custom { .. }));
    }

    #[test]
    fn test_credentials_validation() {
        // Valid credentials
        assert!(Credentials::basic("user", "pass").validate().is_ok());
        assert!(Credentials::bearer("token").validate().is_ok());
        assert!(Credentials::api_key("key", "header").validate().is_ok());

        let mut params = std::collections::HashMap::new();
        params.insert("param".to_string(), "value".to_string());
        assert!(Credentials::custom("type", params).validate().is_ok());

        // Invalid credentials
        assert!(Credentials::basic("", "pass").validate().is_err());
        assert!(Credentials::basic("user", "").validate().is_err());
        assert!(Credentials::bearer("").validate().is_err());
        assert!(Credentials::api_key("", "header").validate().is_err());
        assert!(Credentials::api_key("key", "").validate().is_err());
        assert!(Credentials::custom("", std::collections::HashMap::new()).validate().is_err());
    }

    #[test]
    fn test_credentials_security() {
        assert!(!Credentials::None.is_secure());
        assert!(Credentials::basic("user", "pass").is_secure());
        assert!(Credentials::bearer("token").is_secure());
        assert!(Credentials::api_key("key", "header").is_secure());
    }

    #[test]
    fn test_credentials_summary() {
        let basic = Credentials::basic("john", "secret");
        assert_eq!(basic.summary(), "Basic auth for user 'john'");

        let bearer = Credentials::bearer("abc123");
        assert_eq!(bearer.summary(), "Bearer token authentication");

        let api_key = Credentials::api_key("key", "X-API-Key");
        assert_eq!(api_key.summary(), "API key authentication (header: X-API-Key)");
    }

    #[test]
    fn test_authorization_header() {
        let basic = Credentials::basic("user", "pass");
        let header = basic.authorization_header().unwrap();
        assert!(header.starts_with("Basic "));

        let bearer = Credentials::bearer("token123");
        let header = bearer.authorization_header().unwrap();
        assert_eq!(header, "Bearer token123");

        let api_key = Credentials::api_key("key456", "X-API-Key");
        let header = api_key.authorization_header().unwrap();
        assert_eq!(header, "X-API-Key key456");

        assert!(Credentials::None.authorization_header().is_none());
    }

    #[test]
    fn test_protocol_support() {
        let basic = Credentials::basic("user", "pass");
        assert!(basic.supports_protocol("http"));
        assert!(basic.supports_protocol("https"));
        assert!(!basic.supports_protocol("ws"));

        let bearer = Credentials::bearer("token");
        assert!(bearer.supports_protocol("ws"));
        assert!(bearer.supports_protocol("wss"));
    }

    #[test]
    fn test_credentials_redaction() {
        let basic = Credentials::basic("user", "secret123");
        let redacted = basic.redact();

        if let Credentials::Basic { username, password } = redacted {
            assert_eq!(username, "user");
            assert_eq!(password, "***REDACTED***");
        } else {
            panic!("Expected Basic credentials");
        }
    }
}
