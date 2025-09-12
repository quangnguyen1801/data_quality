use anyhow::Ok;
use async_trait::async_trait;
use model::{modelviews::dataset_view::DatasetView, shared::ultihelper::CONFIGS};
use repository::{
    repositories::dataset_repository::DatasetRepository, shared::irepository::IRepository,
};

use crate::iservices::idataset_service::IDatasetService;

pub struct DatasetService {}
#[async_trait]
impl IDatasetService for DatasetService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<DatasetView> {
        let dbtype = (&CONFIGS.app_setting.dbtype).clone();
        let data = match dbtype.as_str() {
            "SQLServer" => DatasetRepository::fn_repo_get_by_id_sqlserver(id).await?,
            "PostgreSql" => DatasetRepository::fn_repo_get_by_id_postgressql(id).await?,
            _ => DatasetRepository::fn_repo_get_by_id_sqlserver(id).await?,
        };
        let json = serde_json::to_string(&data)?;
        let result = serde_json::from_str(&json)?;
        Ok(result)
    }

    async fn fn_ser_get_all() -> anyhow::Result<Vec<DatasetView>> {
        let dbtype = (&CONFIGS.app_setting.dbtype).clone();
        let data = match dbtype.as_str() {
            "SQLServer" => DatasetRepository::fn_repo_get_all_sqlserver().await?,
            "PostgreSql" => DatasetRepository::fn_repo_get_all_postgressql().await?,
            _ => DatasetRepository::fn_repo_get_all_sqlserver().await?,
        };
        let json = serde_json::to_string(&data)?;
        let result = serde_json::from_str(&json)?;
        Ok(result)
    }

    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<DatasetView>> {
        if page_size <= 0 {
            page_size = 100;
        }
        if page_index <= 0 {
            page_index = 1;
        }
        let start = (page_index - 1) * page_size;
        let end = page_index * page_size;
        let dbtype = (&CONFIGS.app_setting.dbtype).clone();
        let mut data = match dbtype.as_str() {
            "SQLServer" => DatasetRepository::fn_repo_get_all_sqlserver().await?,
            "PostgreSql" => DatasetRepository::fn_repo_get_all_postgressql().await?,
            _ => DatasetRepository::fn_repo_get_all_sqlserver().await?,
        };
        if data.len() > start {
            if data.len() > end {
                data = data.iter().skip(start).take(end).cloned().collect();
            } else {
                data = data.iter().skip(start).cloned().collect();
            }
        } else {
            data = [].to_vec();
        }
        let json = serde_json::to_string(&data)?;
        Ok(serde_json::from_str(&json)?)
    }

    async fn fn_ser_create(obj: DatasetView) -> anyhow::Result<DatasetView> {
        let json = serde_json::to_string(&obj)?;
        let data = serde_json::from_str(&json)?;
        let dbtype = (&CONFIGS.app_setting.dbtype).clone();
        let result = match dbtype.as_str() {
            "SQLServer" => DatasetRepository::fn_repo_create_sqlserver(data).await?,
            "PostgreSql" => DatasetRepository::fn_repo_create_postgressql(data).await?,
            _ => DatasetRepository::fn_repo_create_sqlserver(data).await?,
        };
        let js_result = serde_json::to_string(&result)?;
        Ok(serde_json::from_str(&js_result)?)
    }

    async fn fn_ser_update(obj: DatasetView) -> anyhow::Result<DatasetView> {
        let json = serde_json::to_string(&obj)?;
        let data = serde_json::from_str(&json)?;
        let dbtype = (&CONFIGS.app_setting.dbtype).clone();
        let result = match dbtype.as_str() {
            "SQLServer" => DatasetRepository::fn_repo_update_sqlserver(data).await?,
            "PostgreSql" => DatasetRepository::fn_repo_update_postgressql(data).await?,
            _ => DatasetRepository::fn_repo_update_sqlserver(data).await?,
        };
        let js_result = serde_json::to_string(&result)?;
        Ok(serde_json::from_str(&js_result)?)
    }

    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool> {
        let dbtype = (&CONFIGS.app_setting.dbtype).clone();
        let result = match dbtype.as_str() {
            "SQLServer" => DatasetRepository::fn_repo_delete_sqlserver(id).await?,
            "PostgreSql" => DatasetRepository::fn_repo_delete_postgressql(id).await?,
            _ => DatasetRepository::fn_repo_delete_sqlserver(id).await?,
        };
        Ok(result)
    }

    async fn fn_ser_is_exists(id: i32, datasets: Vec<DatasetView>) -> anyhow::Result<bool> {
        let mut ischeck = false;
        if datasets.iter().any(|f| f.id == id) {
            ischeck = true;
        }
        Ok(ischeck)
    }
}
