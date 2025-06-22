use std::sync::Arc;

use tonic::{Request, Status, metadata::MetadataMap, service::Interceptor};

#[derive(Clone)]
pub struct AuthInterceptor {
    valid_token: Arc<String>,
}

impl AuthInterceptor {
    pub fn new(token: String) -> Self {
        Self {
            valid_token: Arc::new(format!("{} {}", "Bearer", token)),
        }
    }

    fn validate_token(&self, metadata: &MetadataMap) -> Result<(), Status> {
        match metadata.get("authorization") {
            Some(token) if token == self.valid_token.as_str() => Ok(()),
            Some(_) => Err(Status::unauthenticated("Invalid token")),
            None => Err(Status::unauthenticated("Missing token")),
        }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        self.validate_token(request.metadata())?;
        Ok(request)
    }
}
