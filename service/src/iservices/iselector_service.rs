use async_trait::async_trait;
use model::modelviews::selector_view::SelectorView;
#[async_trait]
pub trait ISelectorService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<SelectorView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<SelectorView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<SelectorView>>;
    async fn fn_ser_create(obj: SelectorView) -> anyhow::Result<SelectorView>;
    async fn fn_ser_update(obj: SelectorView) -> anyhow::Result<SelectorView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
    async fn fn_ser_is_exists(id: i32, selectors: Vec<SelectorView>) -> anyhow::Result<bool>;
    async fn fn_ser_get_by_setting_version_id(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<SelectorView>>;
}
