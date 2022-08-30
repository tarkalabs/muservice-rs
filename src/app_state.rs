use error_stack::Result;
use std::sync::Arc;

use crate::db::{DB, DBError};

#[derive(Clone)]
pub struct AppState {
  db: Arc<DB>
}

impl AppState {
  pub async fn init() -> Result<Self, DBError> {
    let db = DB::new().await?;
    Ok(AppState{db: Arc::new(db)})
  }
  pub fn db(&self) -> Arc<DB> {
    self.db.clone()
  }
  pub fn init_with_db(db: DB) -> Self {
    AppState {db: Arc::new(db)}
  }
}
