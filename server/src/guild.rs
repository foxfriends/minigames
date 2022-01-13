use rocket::form::{self, FromFormField, ValueField};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef, Postgres};
use sqlx::Executor;
use sqlx::Type;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GuildId(u64);

impl Display for GuildId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for GuildId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for GuildId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id_str = String::deserialize(deserializer)?;
        Ok(Self(id_str.parse().map_err(D::Error::custom)?))
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
        Ok(Self(i64::decode(value)? as u64))
    }
}

impl<'r> FromFormField<'r> for GuildId {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match field.value.parse() {
            Ok(id) => Ok(Self(id)),
            Err(error) => Err(form::Error::validation(format!(
                "Failed to parse GuildId: {}",
                error
            )))?,
        }
    }
}

pub struct Guild {
    pub id: GuildId,
}

impl Guild {
    pub async fn upsert<Conn>(id: &GuildId, mut conn: Conn) -> anyhow::Result<()>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        sqlx::query!(
            r#"INSERT INTO guilds (id) VALUES ($1) ON CONFLICT DO NOTHING"#,
            id as &GuildId
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}
