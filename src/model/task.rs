//! src/model/task.rs

use crate::model::ModelManager;
use crate::model::Result;
use crate::model::Error;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::ctx::Ctx;

// region:      --- Task Types

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

// endregion:   --- Task Types

// region:      --- TaskBmc

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate,
    ) -> Result<i64> {
        let db = mm.db(); // we can call db because we are in the model

        let (id,) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO task (title) values ($1) returning id"
        )
            .bind(task_c.title)
            .fetch_one(db)
            .await?;

        Ok(id)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        let db = mm.db();

        let task: Task = sqlx::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "task", id })?;

        Ok(task)
    }
}
// endregion:   --- TaskBmc

// region:      --- Tests

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
    use crate::_dev_utils;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        // --- Exec
        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };
        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        // -- Check
        let (title,): (String,) = sqlx::query_as("SELECT title from task where id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;

        assert_eq!(title, fx_title);

        // -- Clean
        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();
        assert_eq!(count, 1, "Did not delete 1 row?");

        Ok(())
    }
}

// endregion:   --- Tests