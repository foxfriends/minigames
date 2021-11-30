use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::postgres::{PgArgumentBuffer, PgConnection, PgTypeInfo, PgValueRef, Postgres};
use sqlx::Type;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct UserId(u64);

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Type<Postgres> for UserId {
    fn type_info() -> PgTypeInfo {
        <i64 as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for UserId {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        (self.0 as i64).encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for UserId {
    fn decode(value: PgValueRef) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        Ok(UserId(i64::decode(value)? as u64))
    }
}

pub struct User {
    pub id: UserId,
}

impl User {
    pub async fn upsert(id: UserId, conn: &mut PgConnection) -> anyhow::Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            INSERT INTO users (id) VALUES ($1) ON CONFLICT DO NOTHING RETURNING id as "id: _"
        "#,
            id as UserId
        )
        .fetch_one(conn)
        .await?)
    }
}
