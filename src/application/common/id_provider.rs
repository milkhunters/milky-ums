use uuid::Uuid;

pub trait IdProvider {
    fn user_id(&self) -> Uuid;
    fn permission(&self) -> String;
}
