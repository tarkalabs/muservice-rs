use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{
    FromRow,
    Executor,
    Postgres,
    migrate,
    query_as,
    query_scalar,
};
use std::fmt::{Display, Formatter};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use crate::settings::SETTINGS;
use tracing::info;

pub struct DB {
  pool: PgPool
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
  pub id: Option<i64>,
  pub name: String,
  pub email: String
}

impl Display for User {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self.id {
      Some(i) => write!(f, "id: {}, name: {}, email: {}", i, self.name, self.email),
      None => write!(f, "id: None, name: {}, email: {}", self.name, self.email),
    }
  }
}

impl User {
  pub async fn all<'a, E>(ex: E) -> Result<Vec<User>>
  where E: 'a + Executor<'a, Database = Postgres>
  {
    let users = query_as::<_, User>("select * from users").fetch_all(ex).await?;
    Ok(users)
  }

  pub async fn insert<'a,  E>(&mut self, ex: E) -> Result<()>
  where E: 'a + Executor<'a, Database = Postgres>
  {
    let id = query_scalar::<_, i64>("insert into users(name, email) values($1, $2) returning id")
    .bind(self.name.clone()).bind(self.email.clone())
    .fetch_one(ex).await.context("Unable to save")?;
    self.id = Some(id);
    Ok(())
  }
}

impl DB {
  pub fn connection(&self) -> PgPool {
    self.pool.clone()
  }
  pub async fn new() -> Result<Self> {
    let pool =PgPoolOptions::new()
      .max_connections(5)
      .connect(&SETTINGS.database.url)
      .await.context("Unable to connect")?;
    let migrator = migrate!();
    migrator.run(&pool).await.context("Unable to run migrations!")?;
    info!("Connected to database: {}", SETTINGS.database.url);
    Ok(DB{pool})
  }
}

#[cfg(test)]
mod tests {
  use sqlx::Acquire;
  use super::{DB, User};
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
