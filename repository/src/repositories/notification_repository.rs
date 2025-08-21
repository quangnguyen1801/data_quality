use async_trait::async_trait;
use model::models::notification::Notification;
use tiberius::Query;
use tiberius_mappers::TryFromRow;

use crate::shared::{connection::Connection, iconnection::IConnection, irepository::IRepository};

pub struct NotificationRepository {}
#[async_trait]
impl IRepository<Notification> for NotificationRepository {
    async fn fn_repo_get_by_id_sqlserver(id: i32) -> anyhow::Result<Notification> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut rows = client
            .query("SELECT * FROM notification WHERE id =@P1", &[&id])
            .await?
            .into_first_result()
            .await?;
        let row = rows.pop().unwrap();
        Ok(Notification::try_from_row(row)?)
    }

    async fn fn_repo_get_all_sqlserver() -> anyhow::Result<Vec<Notification>> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let rows = client
            .query("SELECT * FROM notification", &[])
            .await?
            .into_first_result()
            .await?;
        let mut result = Vec::new();
        for row in rows {
            result.push(Notification::try_from_row(row)?);
        }
        Ok(result)
    }

    async fn fn_repo_create_sqlserver(obj: Notification) -> anyhow::Result<Notification> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut query = Query::new(
            "INSERT INTO [dbo].[notification]
           ([to]
           ,[cc]
           ,[bcc])
            VALUES
                (@P1
                ,@P2
                ,@P3)",
        );
        query.bind(obj.to);
        query.bind(obj.cc);
        query.bind(obj.bcc);
        let res = query.execute(&mut client).await?;
        if res.rows_affected().len() > 0 {
            let rows = Self::fn_repo_get_all_sqlserver().await?;
            let r = rows.iter().max_by_key(|c| c.id).cloned().unwrap();
            Ok(r)
        } else {
            Err(anyhow::anyhow!("CREATE: Error"))
        }
    }

    async fn fn_repo_update_sqlserver(obj: Notification) -> anyhow::Result<Notification> {
        let objclone = obj.clone();
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut query = Query::new(
            "UPDATE [dbo].[notification]
            SET [to] = @P1
                ,[cc] = @P2
                ,[bcc] = @P3
            WHERE id=@P4",
        );
        query.bind(obj.to);
        query.bind(obj.cc);
        query.bind(obj.bcc);
        query.bind(obj.id);
        let result = query.execute(&mut client).await?;
        if result.rows_affected().len() > 0 {
            Ok(objclone)
        } else {
            Err(anyhow::anyhow!("UPDATE: Error"))
        }
    }

    async fn fn_repo_delete_sqlserver(id: i32) -> anyhow::Result<bool> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut query = Query::new("DELETE FROM [dbo].[notification] WHERE id = @P1");
        query.bind(id);
        let result = query.execute(&mut client).await?;
        if result.rows_affected().len() > 0 {
            Ok(true)
        } else {
            Err(anyhow::anyhow!("DELETE: Error"))
        }
    }

    async fn fn_repo_get_by_id_postgressql(id: i32) -> anyhow::Result<Notification> {
        // TODO: Implement PostgreSQL get by id logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_get_all_postgressql() -> anyhow::Result<Vec<Notification>> {
        // TODO: Implement PostgreSQL get all logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_create_postgressql(obj: Notification) -> anyhow::Result<Notification> {
        // TODO: Implement PostgreSQL create logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_update_postgressql(obj: Notification) -> anyhow::Result<Notification> {
        // TODO: Implement PostgreSQL update logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_delete_postgressql(id: i32) -> anyhow::Result<bool> {
        // TODO: Implement PostgreSQL delete logic
        Err(anyhow::anyhow!("Not implemented"))
    }
}
