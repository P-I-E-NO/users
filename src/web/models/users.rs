use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{PgExecutor, Row};
use utoipa::ToSchema;

use crate::web::dto::me::notifications::Notification;

pub struct User {
    pub email: String,
    pub name: String,
    pub surname: String,
    pub id: String,
    pub password: String,
    pub propic_url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserModel {
    pub email: String,
    pub name: String,
    pub surname: String,
    pub id: String,
    pub propic_url: Option<String>,
}

impl User {
    pub async fn from_email(
        e: impl PgExecutor<'_>,
        id: &str,
    ) -> Result<Option<User>, sqlx_core::Error> {
        let query = sqlx::query(
            "
                select * from users where email = $1
            ",
        )
        .bind(&id)
        .fetch_optional(e)
        .await?;

        if let Some(user) = query {
            Ok(Some(User {
                id: user.get("id"),
                name: user.get("name"),
                surname: user.get("surname"),
                email: user.get("email"),
                password: user.get("password"),
                propic_url: user.get("propic_url"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_notifications(
        &self,
        e: impl PgExecutor<'_>,
    ) -> Result<Vec<Notification>, sqlx_core::Error> {
        let results: Vec<Notification> = sqlx::query_as(
            "
            	select
             	id, to_user, data, to_char(created_at, 'dd-mm-yyyy HH24:MI:SS (utc)') created_at
             	from notifications where to_user = $1
             ",
        )
        .bind(&self.id)
        .fetch_all(e)
        .await?;

        Ok(results)
    }

    pub async fn add_fcm_token(
        &self,
        e: impl PgExecutor<'_>,
        token: &str,
    ) -> Result<(), sqlx_core::Error> {
        sqlx::query(
            "
                insert into fcm_tokens values ($1, $2)
            ",
        )
        .bind(&token)
        .bind(&self.id)
        .execute(e)
        .await?;

        Ok(())
    }

    pub async fn from_id(
        e: impl PgExecutor<'_>,
        id: &str,
    ) -> Result<Option<User>, sqlx_core::Error> {
        let query = sqlx::query(
            "
                select * from users where id = $1
            ",
        )
        .bind(&id)
        .fetch_optional(e)
        .await?;

        if let Some(user) = query {
            Ok(Some(User {
                id: user.get("id"),
                name: user.get("name"),
                surname: user.get("surname"),
                email: user.get("email"),
                password: user.get("password"),
                propic_url: user.get("propic_url"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn register(
        e: impl PgExecutor<'_>,
        email: &str,
        name: &str,
        surname: &str,
        password_hash: &str,
    ) -> Result<String, sqlx_core::Error> {
        let user_id = nanoid!(32);
        sqlx::query(
            "
            insert into users (id, name, surname, email, password) values
            (
                $1,
                $2,
                $3,
                $4,
                $5
            )
        ",
        )
        .bind(&user_id)
        .bind(name)
        .bind(surname)
        .bind(email)
        .bind(password_hash)
        .execute(e)
        .await?; // in 0.7, `Transaction` can no longer implement `Executor` directly,
                 // so it must be dereferenced to the internal connection type.

        Ok(user_id.to_string())
    }
}
