use anyhow::Ok;
use async_trait::async_trait;
use model::modelviews::comparisonop_view::ComparisonOpView;
use repository::{
    repositories::comparisonop_repository::ComparisonOpRepository, shared::irepository::IRepository,
};

use crate::iservices::icomparisonop_service::IComparisonOpService;

pub struct ComparisonOpService {}
#[async_trait]
impl IComparisonOpService for ComparisonOpService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<ComparisonOpView> {
        let data = ComparisonOpRepository::fn_repo_get_by_id_sqlserver(id).await?;
        let json = serde_json::to_string(&data)?;
        let result = serde_json::from_str(&json)?;
        Ok(result)
    }

    async fn fn_ser_get_all() -> anyhow::Result<Vec<ComparisonOpView>> {
        let data = ComparisonOpRepository::fn_repo_get_all_sqlserver().await?;
        let json = serde_json::to_string(&data)?;
        let result = serde_json::from_str(&json)?;
        Ok(result)
    }

    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<ComparisonOpView>> {
        if page_size <= 0 {
            page_size = 100;
        }
        if page_index <= 0 {
            page_index = 1;
        }
        let start = (page_index - 1) * page_size;
        let end = page_index * page_size;
        let mut data = ComparisonOpRepository::fn_repo_get_all_sqlserver().await?;
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

    async fn fn_ser_create(obj: ComparisonOpView) -> anyhow::Result<ComparisonOpView> {
        let json = serde_json::to_string(&obj)?;
        let data = serde_json::from_str(&json)?;
        let result = ComparisonOpRepository::fn_repo_create_sqlserver(data).await?;
        let js_result = serde_json::to_string(&result)?;
        Ok(serde_json::from_str(&js_result)?)
    }

    async fn fn_ser_update(obj: ComparisonOpView) -> anyhow::Result<ComparisonOpView> {
        let json = serde_json::to_string(&obj)?;
        let data = serde_json::from_str(&json)?;
        let result = ComparisonOpRepository::fn_repo_update_sqlserver(data).await?;
        let js_result = serde_json::to_string(&result)?;
        Ok(serde_json::from_str(&js_result)?)
    }

    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool> {
        let result = ComparisonOpRepository::fn_repo_delete_sqlserver(id).await?;
        Ok(result)
    }
}
