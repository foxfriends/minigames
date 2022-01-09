use super::GameName;
use openssl::rand::rand_bytes;
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef, Postgres};
use sqlx::{Executor, Type};
use std::error::Error;
use uuid::Uuid;

pub struct ApiKey([u8; 96]);

impl ApiKey {
    pub fn generate() -> anyhow::Result<Self> {
        let mut buf = [0; 96];
        rand_bytes(&mut buf)?;
        Ok(Self(buf))
    }
}

impl Type<Postgres> for ApiKey {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
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
        let buf: [u8; 96] = buf.try_into().unwrap();
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
    pub async fn generate<Conn>(game: GameName, mut conn: Conn) -> anyhow::Result<Self>
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
            game as GameName,
            public_key as ApiKey,
            secret_key as ApiKey,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(keys)
    }
}
