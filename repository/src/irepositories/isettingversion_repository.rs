use async_trait::async_trait;

use crate::shared::irepository::IRepository;

#[async_trait]
pub trait ISettingVersionRepository<SettingVersion>: IRepository<SettingVersion> {
    async fn fn_repo_get_current_version_sqlserver() -> anyhow::Result<Option<SettingVersion>>;
    async fn fn_repo_get_current_version_postgresql() -> anyhow::Result<Option<SettingVersion>>;
}
