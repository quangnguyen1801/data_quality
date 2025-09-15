use anyhow::Ok;
use async_trait::async_trait;
use model::models::settingversion::SettingVersion;
use tiberius::Query;
use tiberius_mappers::TryFromRow;

use crate::{
    irepositories::isettingversion_repository::ISettingVersionRepository,
    shared::{connection::Connection, iconnection::IConnection, irepository::IRepository},
};

pub struct SettingVersionRepository {}
#[async_trait]
impl IRepository<SettingVersion> for SettingVersionRepository {
    async fn fn_repo_get_by_id_sqlserver(id: i32) -> anyhow::Result<SettingVersion> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut rows = client
            .query("SELECT * FROM setting_version WHERE id =@P1", &[&id])
            .await?
            .into_first_result()
            .await?;
        let row = rows.pop().unwrap();
        Ok(SettingVersion::try_from_row(row)?)
    }

    async fn fn_repo_get_all_sqlserver() -> anyhow::Result<Vec<SettingVersion>> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let rows = client
            .query("SELECT * FROM setting_version", &[])
            .await?
            .into_first_result()
            .await?;
        let mut result = Vec::new();
        for row in rows {
            result.push(SettingVersion::try_from_row(row)?);
        }
        Ok(result)
    }

    async fn fn_repo_create_sqlserver(obj: SettingVersion) -> anyhow::Result<SettingVersion> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut query = Query::new(
            "INSERT INTO [dbo].[setting_version]
           ([file_name]
           ,[upddate])
            VALUES
                (@P1
                ,@P2)",
        );
        query.bind(obj.file_name);
        query.bind(obj.upddate);
        let res = query.execute(&mut client).await?;
        if res.rows_affected().len() > 0 {
            let rows = Self::fn_repo_get_all_sqlserver().await?;
            let r = rows.iter().max_by_key(|c| c.id).cloned().unwrap();
            Ok(r)
        } else {
            Err(anyhow::anyhow!("CREATE: Error"))
        }
    }

    async fn fn_repo_update_sqlserver(obj: SettingVersion) -> anyhow::Result<SettingVersion> {
        let objclone = obj.clone();
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut query = Query::new(
            "UPDATE [dbo].[setting_version]
            SET [file_name] = @P1
                ,[upddate] = @P2
            WHERE id = @P3",
        );
        query.bind(obj.file_name);
        query.bind(obj.upddate);
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
        let mut query = Query::new("DELETE FROM [dbo].[setting_version] WHERE id = @P1");
        query.bind(id);
        let result = query.execute(&mut client).await?;
        if result.rows_affected().len() > 0 {
            Ok(true)
        } else {
            Err(anyhow::anyhow!("DELETE: Error"))
        }
    }

    async fn fn_repo_get_by_setting_version_sqlserver(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<SettingVersion>> {
        let mut result = Vec::new();
        result.push(Self::fn_repo_get_by_id_sqlserver(setting_version_id).await?);
        Ok(result)
    }

    async fn fn_repo_get_by_id_postgresql(id: i32) -> anyhow::Result<SettingVersion> {
        // TODO: Implement PostgreSQL get by id logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_get_all_postgresql() -> anyhow::Result<Vec<SettingVersion>> {
        // TODO: Implement PostgreSQL get all logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_create_postgresql(obj: SettingVersion) -> anyhow::Result<SettingVersion> {
        // TODO: Implement PostgreSQL create logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_update_postgresql(obj: SettingVersion) -> anyhow::Result<SettingVersion> {
        // TODO: Implement PostgreSQL update logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_delete_postgresql(id: i32) -> anyhow::Result<bool> {
        // TODO: Implement PostgreSQL delete logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_get_by_setting_version_postgresql(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<SettingVersion>> {
        // TODO: Implement PostgreSQL delete logic
        Err(anyhow::anyhow!("Not implemented"))
    }
}

#[async_trait]
impl ISettingVersionRepository<SettingVersion> for SettingVersionRepository {
    async fn fn_repo_get_current_version_sqlserver() -> anyhow::Result<Option<SettingVersion>> {
        let data = Self::fn_repo_get_all_sqlserver().await?;
        let result = data.iter().cloned().max_by_key(|s| s.upddate);
        Ok(result)
    }

    async fn fn_repo_get_current_version_postgresql() -> anyhow::Result<Option<SettingVersion>> {
        // TODO: Implement PostgreSQL delete logic
        Err(anyhow::anyhow!("Not implemented"))
    }
}
