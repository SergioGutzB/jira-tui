use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de configuración: {0}")]
    ConfigError(String),

    #[error("Error de red/API: {0}")]
    ApiError(String),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Operación no autorizada. Revisa tus credenciales.")]
    Unauthorized,

    #[error("Error interno desconocido: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
