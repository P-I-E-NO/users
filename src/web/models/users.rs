use sqlx::{PgExecutor, Row};
use nanoid::nanoid;

pub struct User {
    pub email: String,
    pub name: String,
    pub surname: String,
    pub id: String,
    pub password: String,
}

impl User {

    pub async fn from_email(
        e: impl PgExecutor<'_>, 
        id: &str
    ) -> Result<Option<User>, sqlx_core::Error> {
        let query = sqlx
            ::query("
                select * from users where email = $1
            ")
            .bind(&id)
            .fetch_optional(e).await?;

        if let Some(user) = query {
            Ok(
                Some(User {
                    id: user.get("id"),
                    name: user.get("name"),
                    surname: user.get("surname"),
                    email: user.get("email"),
                    password: user.get("password")
                })
            )
        }else{
            Ok(None)
        }

    }

    pub async fn from_id(
        e: impl PgExecutor<'_>, 
        id: &str
    ) -> Result<Option<User>, sqlx_core::Error> {
        let query = sqlx
            ::query("
                select * from users where id = $1
            ")
            .bind(&id)
            .fetch_optional(e).await?;

        if let Some(user) = query {
            Ok(
                Some(User {
                    id: user.get("id"),
                    name: user.get("name"),
                    surname: user.get("surname"),
                    email: user.get("email"),
                    password: user.get("password")
                })
            )
        }else{
            Ok(None)
        }

    }

    pub async fn register(
        e: impl PgExecutor<'_>,
        email: &str,
        name: &str,
        surname: &str,
        password_hash: &str,
    ) ->  Result<String, sqlx_core::Error> {
    
        let user_id = nanoid!(32);
        sqlx::query("
            insert into users (id, name, surname, email, password) values 
            (
                $1, 
                $2, 
                $3, 
                $4,
                $5
            )
        ")
        .bind(&user_id)
        .bind(name)
        .bind(surname)
        .bind(email)
        .bind(password_hash)
        .execute(e).await?; // in 0.7, `Transaction` can no longer implement `Executor` directly,
                                    // so it must be dereferenced to the internal connection type.
    
        Ok(user_id.to_string())
    
    }
}