use anyhow::{Result, Context};
use std::sync::Arc;

use crate::db::DB;

#[derive(Clone)]
pub struct AppState {
  pub db: Arc<DB>,
}

impl AppState {
  pub async fn init() -> Result<Self> {
    let db = DB::new().await.context("Unable to establish DB connection")?;
    Ok(AppState{db: Arc::new(db)})
  }
  pub fn db(&self) -> Arc<DB> {
    self.db.clone()
  }
}
