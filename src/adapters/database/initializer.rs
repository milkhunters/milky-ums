use crate::domain::models;
use sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema};

async fn create_table<E>(db: &DbConn, entity: E)
    where
        E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmt = builder.build(&schema.create_table_from_entity(entity));

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn create_tables(db: &DbConn) {
    create_table(db, models::user::Entity).await;
}