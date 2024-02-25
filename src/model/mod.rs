//! src/model/mod.rs
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - ModelManager are designed to be passed as an `argument` to all Model Controllers functions.

// region:    --- Modules

mod error;
mod store;
pub mod task;

use crate::model::store::{Db, new_db_pool};
pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        // the question mark works because of model.errors implements From on line 16
        let db = new_db_pool().await?;
        Ok(ModelManager { db })
    }

    /// Returns the sqlx db pool reference.
    /// (Only for the model layer)
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
