use async_trait::async_trait;

#[async_trait]
pub trait IRepository<T> {
    //SQLServer
    async fn fn_repo_get_by_id_sqlserver(id: i32) -> anyhow::Result<T>;
    async fn fn_repo_get_all_sqlserver() -> anyhow::Result<Vec<T>>;
    async fn fn_repo_create_sqlserver(obj: T) -> anyhow::Result<T>;
    async fn fn_repo_update_sqlserver(obj: T) -> anyhow::Result<T>;
    async fn fn_repo_delete_sqlserver(id: i32) -> anyhow::Result<bool>;
    async fn fn_repo_get_by_setting_version_sqlserver(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<T>>;
    //PostgreSql
    async fn fn_repo_get_by_id_postgresql(id: i32) -> anyhow::Result<T>;
    async fn fn_repo_get_all_postgresql() -> anyhow::Result<Vec<T>>;
    async fn fn_repo_create_postgresql(obj: T) -> anyhow::Result<T>;
    async fn fn_repo_update_postgresql(obj: T) -> anyhow::Result<T>;
    async fn fn_repo_delete_postgresql(id: i32) -> anyhow::Result<bool>;
    async fn fn_repo_get_by_setting_version_postgresql(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<T>>;
}
