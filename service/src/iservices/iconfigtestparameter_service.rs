use async_trait::async_trait;
use model::modelviews::configtestparameter_view::ConfigTestParameterView;
#[async_trait]
pub trait IConfigTestParameterService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<ConfigTestParameterView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<ConfigTestParameterView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<ConfigTestParameterView>>;
    async fn fn_ser_create(obj: ConfigTestParameterView)
    -> anyhow::Result<ConfigTestParameterView>;
    async fn fn_ser_update(obj: ConfigTestParameterView)
    -> anyhow::Result<ConfigTestParameterView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
}
