use actix::prelude::*;
use sqlx::PgPool;

#[derive(Debug)]
pub struct DbActor {
    pub pool: PgPool,
}

impl Actor for DbActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<Option<User>, sqlx::Error>")]
pub struct GetUserById(pub i32);

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl Handler<GetUserById> for DbActor {
    type Result = ResponseFuture<Result<Option<User>, sqlx::Error>>;

    fn handle(&mut self, msg: GetUserById, _ctx: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        let user_id = msg.0;

        Box::pin(async move {
            let result = sqlx::query_as::<_, User>(
                "SELECT id, name, email FROM users WHERE id = $1"
            )
            .bind(user_id)
            .fetch_optional(&pool)
            .await;

            result
        })
    }
}
