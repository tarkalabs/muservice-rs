use color_eyre::{Result, Help};
use std::sync::Arc;
use tracing::instrument;
use crate::db::DB;

#[derive(Clone, Debug)]
pub struct AppState {
  db: Arc<DB>,
}

impl AppState {
  #[instrument]
  pub async fn init() -> Result<Self> {
    let db = DB::new().await.suggestion("Ensure that the Database URL environment variable is correct")?;
    Ok(AppState{db: Arc::new(db)})
  }
  pub fn db(&self) -> Arc<DB> {
    self.db.clone()
  }
  pub fn init_with_db(db: DB) -> Self {
    AppState {db: Arc::new(db)}
  }
}
