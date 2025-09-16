use async_trait::async_trait;
use model::modelviews::settingversion_view::SettingVersionView;
#[async_trait]
pub trait ISettingVersionService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<SettingVersionView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<SettingVersionView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<SettingVersionView>>;
    async fn fn_ser_create(obj: SettingVersionView) -> anyhow::Result<SettingVersionView>;
    async fn fn_ser_update(obj: SettingVersionView) -> anyhow::Result<SettingVersionView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
    async fn fn_ser_get_current_version() -> anyhow::Result<SettingVersionView>;
}
