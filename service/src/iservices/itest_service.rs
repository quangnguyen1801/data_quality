use async_trait::async_trait;
use model::modelviews::test_view::TestView;
#[async_trait]
pub trait ITestService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<TestView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<TestView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<TestView>>;
    async fn fn_ser_create(obj: TestView) -> anyhow::Result<TestView>;
    async fn fn_ser_update(obj: TestView) -> anyhow::Result<TestView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
    async fn fn_ser_get_by_setting_version_id(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<TestView>>;
}
