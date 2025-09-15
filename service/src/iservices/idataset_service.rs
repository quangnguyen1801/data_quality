use async_trait::async_trait;
use model::modelviews::dataset_view::DatasetView;
#[async_trait]
pub trait IDatasetService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<DatasetView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<DatasetView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<DatasetView>>;
    async fn fn_ser_create(obj: DatasetView) -> anyhow::Result<DatasetView>;
    async fn fn_ser_update(obj: DatasetView) -> anyhow::Result<DatasetView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
    async fn fn_ser_is_exists(id: i32, datasets: Vec<DatasetView>) -> anyhow::Result<bool>;
    async fn fn_ser_get_by_setting_version_id(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<DatasetView>>;
}
