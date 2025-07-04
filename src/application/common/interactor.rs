use crate::application::common::error::AppError;

pub trait Interactor<I, O> {
    async fn execute(&self, data: I) -> Result<O, AppError>;
}