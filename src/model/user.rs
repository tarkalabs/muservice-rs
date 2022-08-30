use sqlx::{
    FromRow,
    Executor,
    Postgres,
    query_as,
    query_scalar,
};
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use error_stack::{IntoReport, Result, ResultExt};

use crate::db::DBError;

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
  pub async fn all<'a, E>(ex: E) -> Result<Vec<User>, DBError>
  where E: 'a + Executor<'a, Database = Postgres>
  {
    let users = query_as::<_, User>("select * from users").fetch_all(ex)
    .await
    .map_err(|err| err.into())
    .report()
    .attach_printable_lazy(|| format!("Failed to get user list!"))?;
    Ok(users)
  }

  pub async fn insert<'a,  E>(&mut self, ex: E) -> Result<(), DBError>
  where E: 'a + Executor<'a, Database = Postgres>
  {
    let id = query_scalar::<_, i64>("insert into users(name, email) values($1, $2) returning id")
    .bind(self.name.clone()).bind(self.email.clone())
    .fetch_one(ex)
    .await
    .map_err(|err| err.into())
    .report()
    .attach_printable_lazy(|| format!("Failed to insert user!"))?;
    self.id = Some(id);
    Ok(())
  }
}
