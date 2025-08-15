use async_std::net::TcpStream;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tiberius::Client;

#[async_trait]
pub trait IConnection {
    async fn fn_repo_get_connection_sqlsever() -> anyhow::Result<Client<TcpStream>>;
    async fn fn_repo_get_connection_postgressql() -> anyhow::Result<Pool<Postgres>>;
}
