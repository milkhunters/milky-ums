


pub trait Interactor<I, O> {
    async fn execute(&self, data: I) -> Result<O, String>;
}