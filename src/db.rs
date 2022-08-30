use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{migrate};
use error_stack::{IntoReport, Result, ResultExt};
use thiserror::Error;
use crate::settings::SETTINGS;
use tracing::info;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Unable to run migrations!")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("Unable to connect!")]
    ConnectionError(#[from] sqlx::Error)
}

pub struct DB {
  pool: PgPool
}

impl DB {
  pub fn connection(&self) -> PgPool {
    self.pool.clone()
  }
  pub async fn new() -> Result<Self, DBError> {
    let pool = PgPoolOptions::new()
      .max_connections(5)
      .connect(&SETTINGS.database.url)
      .await
      .map_err(|err| err.into())
      .report()
      .attach_printable_lazy(|| format!("Unable to connect!"))?;
    let migrator = migrate!();
    migrator.run(&pool).await
    .map_err(|err| err.into())
    .report()
    .attach_printable_lazy(|| format!("Unable to run migrations!"))?;
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
