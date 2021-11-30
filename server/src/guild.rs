use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::postgres::{PgArgumentBuffer, PgConnection, PgTypeInfo, PgValueRef, Postgres};
use sqlx::Type;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct GuildId(u64);

impl Display for GuildId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Type<Postgres> for GuildId {
    fn type_info() -> PgTypeInfo {
        <i64 as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for GuildId {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        (self.0 as i64).encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for GuildId {
    fn decode(value: PgValueRef) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        Ok(GuildId(i64::decode(value)? as u64))
    }
}

pub struct Guild {
    pub id: GuildId,
}

impl Guild {
    pub async fn upsert(id: GuildId, conn: &mut PgConnection) -> anyhow::Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            INSERT INTO guilds (id) VALUES ($1) ON CONFLICT DO NOTHING RETURNING id as "id: _"
        "#,
            id as GuildId
        )
        .fetch_one(conn)
        .await?)
    }
}
