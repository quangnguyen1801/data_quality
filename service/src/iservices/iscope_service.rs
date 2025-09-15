use async_trait::async_trait;
use model::modelviews::scope_view::ScopeView;
#[async_trait]
pub trait IScopeService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<ScopeView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<ScopeView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<ScopeView>>;
    async fn fn_ser_create(obj: ScopeView) -> anyhow::Result<ScopeView>;
    async fn fn_ser_update(obj: ScopeView) -> anyhow::Result<ScopeView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
    async fn fn_ser_get_by_setting_version_id(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<ScopeView>>;
}
