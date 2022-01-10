use super::GameName;
use openssl::rand::rand_bytes;
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef, Postgres};
use sqlx::{Executor, Type};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

const KEY_LENGTH: usize = 48;

#[derive(Eq, PartialEq)]
pub struct ApiKey([u8; KEY_LENGTH]);

impl Display for ApiKey {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        base64::encode(self.0).fmt(fmt)
    }
}

impl ApiKey {
    pub fn generate() -> anyhow::Result<Self> {
        let mut buf = [0; KEY_LENGTH];
        rand_bytes(&mut buf)?;
        Ok(Self(buf))
    }
}

impl FromStr for ApiKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ApiKey(base64::decode(s)?.try_into().map_err(|_| {
            anyhow::anyhow!("API Key must be 64 characters long")
        })?))
    }
}

impl Type<Postgres> for ApiKey {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <String as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for ApiKey {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        let encoded = base64::encode(self.0);
        encoded.encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for ApiKey {
    fn decode(value: PgValueRef) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let string = String::decode(value)?;
        let buf = base64::decode(string)?;
        let buf: [u8; KEY_LENGTH] = buf.try_into().unwrap();
        Ok(Self(buf))
    }
}

pub struct ApiKeys {
    id: Uuid,
    pub game: GameName,
    pub public_key: ApiKey,
    pub secret_key: ApiKey,
}

impl ApiKeys {
    pub async fn generate<Conn>(game: &GameName, mut conn: Conn) -> anyhow::Result<Self>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let public_key = ApiKey::generate()?;
        let secret_key = ApiKey::generate()?;

        let keys = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO api_keys (game_server_name, public_key, secret_key)
                VALUES ($1, $2, $3)
                RETURNING
                id,
                game_server_name as "game: _",
                public_key as "public_key: _",
                secret_key as "secret_key: _"
            "#,
            game as &GameName,
            public_key as ApiKey,
            secret_key as ApiKey,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(keys)
    }

    pub async fn load<Conn>(game: &GameName, mut conn: Conn) -> anyhow::Result<Self>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let keys = sqlx::query_as!(
            Self,
            r#"
            SELECT id, game_server_name as "game: _", public_key as "public_key: _", secret_key as "secret_key: _"
            FROM api_keys
            WHERE game_server_name = $1
            "#,
            game as &GameName,
        ).fetch_one(&mut *conn).await?;
        Ok(keys)
    }
}
