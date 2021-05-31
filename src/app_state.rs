use std::sync::{Arc, Mutex};
use crate::db::DB;
use anyhow::{Result, Error, Context};

pub struct AppState {
  db: Arc<DB>,
}

impl AppState {
  pub async fn init() -> Result<Self> {
    let db = DB::new().await.context("unable to establish db connection")?;
    Ok(AppState{db: Arc::new(db)})
  }
  pub fn db(&self) -> Arc<DB> {
    self.db.clone()
  }
}
