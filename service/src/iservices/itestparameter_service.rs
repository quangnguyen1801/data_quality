use async_trait::async_trait;
use model::modelviews::testparameter_view::TestParameterView;
#[async_trait]
pub trait ITestParameterService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<TestParameterView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<TestParameterView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<TestParameterView>>;
    async fn fn_ser_create(obj: TestParameterView) -> anyhow::Result<TestParameterView>;
    async fn fn_ser_update(obj: TestParameterView) -> anyhow::Result<TestParameterView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
    async fn fn_ser_get_by_setting_version_id(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<TestParameterView>>;
}
