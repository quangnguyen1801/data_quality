use anyhow::Ok;
use async_std::net::TcpStream;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tiberius::{Client, Config};

use crate::shared::{commonfuntion::CONFIGS, iconnection::IConnection};

pub struct Connection {}
#[async_trait]
impl IConnection for Connection {
    async fn fn_repo_get_connection_sqlsever() -> anyhow::Result<Client<TcpStream>> {
        let config = Config::from_ado_string(&CONFIGS.app_setting.sqlserver_connection)?;
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        let client = Client::connect(config, tcp).await?;
        Ok(client)
    }

    async fn fn_repo_get_connection_postgressql() -> anyhow::Result<Pool<Postgres>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&CONFIGS.app_setting.postgressql_connection)
            .await?;
        Ok(pool)
    }
}
