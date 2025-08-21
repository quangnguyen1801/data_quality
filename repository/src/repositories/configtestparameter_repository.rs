use anyhow::Ok;
use async_trait::async_trait;
use model::models::configtestparameter::ConfigTestParameter;
use tiberius::Query;
use tiberius_mappers::TryFromRow;

use crate::shared::{connection::Connection, iconnection::IConnection, irepository::IRepository};

pub struct ConfigTestParameterRepository {}
#[async_trait]
impl IRepository<ConfigTestParameter> for ConfigTestParameterRepository {
    async fn fn_repo_get_by_id_sqlserver(id: i32) -> anyhow::Result<ConfigTestParameter> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut rows = client
            .query("SELECT * FROM configtestparameter WHERE id =@P1", &[&id])
            .await?
            .into_first_result()
            .await?;
        let row = rows.pop().unwrap();
        Ok(ConfigTestParameter::try_from_row(row)?)
    }

    async fn fn_repo_get_all_sqlserver() -> anyhow::Result<Vec<ConfigTestParameter>> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let rows = client
            .query("SELECT * FROM configtestparameter", &[])
            .await?
            .into_first_result()
            .await?;
        let mut result = Vec::new();
        for row in rows {
            result.push(ConfigTestParameter::try_from_row(row)?);
        }
        Ok(result)
    }

    async fn fn_repo_create_sqlserver(
        obj: ConfigTestParameter,
    ) -> anyhow::Result<ConfigTestParameter> {
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut query = Query::new(
            "INSERT INTO [dbo].[configtestparameter]
           ([test_id]
           ,[test_parameter_id]
           ,[inc_excl]
           ,[operator]
           ,[vlow]
           ,[vhigh])
            VALUES
                (@P1
                ,@P2
                ,@P3
                ,@P4
                ,@P5
                ,@P6)",
        );
        query.bind(obj.test_id);
        query.bind(obj.test_parameter_id);
        query.bind(obj.inc_excl);
        query.bind(obj.operator);
        query.bind(obj.vlow);
        query.bind(obj.vhigh);
        let res = query.execute(&mut client).await?;
        if res.rows_affected().len() > 0 {
            let rows = Self::fn_repo_get_all_sqlserver().await?;
            let r = rows.iter().max_by_key(|c| c.id).cloned().unwrap();
            Ok(r)
        } else {
            Err(anyhow::anyhow!("CREATE: Error"))
        }
    }

    async fn fn_repo_update_sqlserver(
        obj: ConfigTestParameter,
    ) -> anyhow::Result<ConfigTestParameter> {
        let objclone = obj.clone();
        let mut client = Connection::fn_repo_get_connection_sqlsever().await?;
        let mut query = Query::new(
            "UPDATE [dbo].[configtestparameter]
            SET [test_id] = @P1
                ,[test_parameter_id] = @P2
                ,[inc_excl] = @P3
                ,[operator] = @P4
                ,[vlow] = @P5
                ,[vhigh] = @P6
            WHERE  id=@P7",
        );
        query.bind(obj.test_id);
        query.bind(obj.test_parameter_id);
        query.bind(obj.inc_excl);
        query.bind(obj.operator);
        query.bind(obj.vlow);
        query.bind(obj.vhigh);
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
        let mut query = Query::new("DELETE FROM [dbo].[configtestparameter] WHERE id = @P1");
        query.bind(id);
        let result = query.execute(&mut client).await?;
        if result.rows_affected().len() > 0 {
            Ok(true)
        } else {
            Err(anyhow::anyhow!("DELETE: Error"))
        }
    }

    async fn fn_repo_get_by_id_postgressql(id: i32) -> anyhow::Result<ConfigTestParameter> {
        // TODO: Implement PostgreSQL get by id logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_get_all_postgressql() -> anyhow::Result<Vec<ConfigTestParameter>> {
        // TODO: Implement PostgreSQL get all logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_create_postgressql(
        obj: ConfigTestParameter,
    ) -> anyhow::Result<ConfigTestParameter> {
        // TODO: Implement PostgreSQL create logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_update_postgressql(
        obj: ConfigTestParameter,
    ) -> anyhow::Result<ConfigTestParameter> {
        // TODO: Implement PostgreSQL update logic
        Err(anyhow::anyhow!("Not implemented"))
    }

    async fn fn_repo_delete_postgressql(id: i32) -> anyhow::Result<bool> {
        // TODO: Implement PostgreSQL delete logic
        Err(anyhow::anyhow!("Not implemented"))
    }
}
