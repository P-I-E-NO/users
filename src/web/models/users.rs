use sqlx::{Acquire, Postgres, Transaction};
use nanoid::nanoid;

pub struct User {
    email: String,
    name: String,
    surname: String,
    id: String
}

impl User {
    pub async fn register(
        tx: &mut Transaction<'_, Postgres>,
        email: String,
        name: String,
        surname: String,
        password_hash: String,
    ) ->  Result<(), sqlx_core::Error> {
    
        let user_id = nanoid!(32);
        sqlx::query("
            insert into users values 
            (
                $1, 
                $2, 
                $3, 
                $4,
                $5
            )
        ")
        .bind(user_id)
        .bind(name)
        .bind(surname)
        .bind(email)
        .bind(password_hash)
        .execute(&mut **tx).await?; // in 0.7, `Transaction` can no longer implement `Executor` directly,
                                    // so it must be dereferenced to the internal connection type.
    
        Ok(())
    
    }
}