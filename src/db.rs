use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::migrate;
use color_eyre::{eyre::WrapErr, Result};
use crate::settings::SETTINGS;
use tracing::info;

pub struct DB {
  pool: PgPool
}

impl DB {
  pub fn connection(&self) -> PgPool {
    self.pool.clone()
  }
  pub async fn new() -> Result<Self> {
    let pool = PgPoolOptions::new()
      .max_connections(5)
      .connect(&SETTINGS.database.url)
      .await.context("Unable to connect")?;
    let migrator = migrate!();
    migrator.run(&pool).await.context("Unable to run migrations!")?;
    info!("Connected to database: {}", SETTINGS.database.url);
    Ok(DB{pool})
  }
  pub fn new_with_pool(pool: PgPool) -> Self {
    DB{pool}
  }
}

#[cfg(test)]
mod tests {
  use sqlx::Acquire;
  use super::DB;
  use crate::model::User;
  #[tokio::test]
  async fn test_should_connect() {
    let db = DB::new().await.unwrap();
    let mut u = User{
      id: None, 
      name: "Vagmi".into(), 
      email: "vagmi@example.com".into()
    };
    let mut t = db.pool.begin().await.unwrap();

    let mut t1 = t.begin().await.unwrap();
    u.insert(&mut t1).await.unwrap();
    t1.commit().await.unwrap();
    match u.id {
      Some(_) => println!("{}", u),
      None => panic!()
    }
    t.rollback().await.unwrap();
  }
}
