use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde::de::{Error as _};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::postgres::{PgArgumentBuffer, PgConnection, PgTypeInfo, PgValueRef, Postgres};
use sqlx::Type;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UserId(u64);

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let id_str = String::deserialize(deserializer)?;
        Ok(Self(id_str.parse().map_err(D::Error::custom)?))
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
    pub async fn upsert(id: UserId, conn: &mut PgConnection) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO users (id) VALUES ($1) ON CONFLICT DO NOTHING"#,
            id as UserId
        )
        .execute(conn)
        .await?;
        Ok(())
    }
}
