use async_trait::async_trait;
use model::modelviews::metric_view::MetricView;
#[async_trait]
pub trait IMetricService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<MetricView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<MetricView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<MetricView>>;
    async fn fn_ser_create(obj: MetricView) -> anyhow::Result<MetricView>;
    async fn fn_ser_update(obj: MetricView) -> anyhow::Result<MetricView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
    async fn fn_ser_is_exists(id: i32, datasets: Vec<MetricView>) -> anyhow::Result<bool>;
    async fn fn_ser_get_by_setting_version_id(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<MetricView>>;
}
