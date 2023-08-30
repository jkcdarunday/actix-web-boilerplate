use actix_web::error::{ErrorInternalServerError, InternalError};
use actix_web::{Error, HttpRequest, HttpResponse};
use serde_json::{json, Value};
use validator::ValidationErrors;

fn handle_validation_errors(validation_errors: ValidationErrors) -> Error {
    let json_errors = validation_errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            errors
                .iter()
                .map(|error| {
                    json!({
                        "field": field.to_string(),
                        "message": error.code.as_ref().to_string(),
                    })
                })
                .collect::<Value>()
        })
        .collect::<Value>();

    let payload = json!({
        "errors": json_errors,
    });

    InternalError::from_response("Validation error", HttpResponse::BadRequest().json(payload))
        .into()
}

pub fn handle_error(error: actix_web_validator::Error, _request: &HttpRequest) -> Error {
    match error {
        actix_web_validator::Error::Validate(errors) => handle_validation_errors(errors),
        _ => ErrorInternalServerError("Internal Server Error"),
    }
}
