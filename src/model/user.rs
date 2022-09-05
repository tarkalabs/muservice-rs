use sqlx::{
    FromRow,
    Executor,
    Postgres,
    query_as,
    query_scalar,
};
use tracing::instrument;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use color_eyre::{eyre::WrapErr, Result};


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
  #[instrument(skip(ex))]
  pub async fn all<'a, E>(ex: E) -> Result<Vec<User>>
  where E: 'a + Executor<'a, Database = Postgres>
  {
    let users = query_as::<_, User>("select * from users").fetch_all(ex).await?;
    Ok(users)
  }

  #[instrument(skip(ex))]
  pub async fn insert<'a,  E>(&mut self, ex: E) -> Result<()>
  where E: 'a + Executor<'a, Database = Postgres>
  {
    let id = query_scalar::<_, i64>("insert into users(name, email) values($1, $2) returning id")
    .bind(self.name.clone()).bind(self.email.clone())
    .fetch_one(ex).await.context("Unable to save user")?;
    self.id = Some(id);
    Ok(())
  }
}
