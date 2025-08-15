use async_trait::async_trait;

#[async_trait]
pub trait IRepository<T> {
    //SQLServer
    async fn fn_repo_get_by_id_sqlserver(id: i32) -> anyhow::Result<T>;
    async fn fn_repo_get_all_sqlserver() -> anyhow::Result<Vec<T>>;
    async fn fn_repo_create_sqlserver(obj: T) -> anyhow::Result<T>;
    async fn fn_repo_update_sqlserver(obj: T) -> anyhow::Result<T>;
    async fn fn_repo_delete_sqlserver(id: i32) -> anyhow::Result<bool>;
    //PostgresSql
    async fn fn_repo_get_by_id_postgressql(id: i32) -> anyhow::Result<T>;
    async fn fn_repo_get_all_postgressql() -> anyhow::Result<Vec<T>>;
    async fn fn_repo_create_postgressql(obj: T) -> anyhow::Result<T>;
    async fn fn_repo_update_postgressql(obj: T) -> anyhow::Result<T>;
    async fn fn_repo_delete_postgressql(id: i32) -> anyhow::Result<bool>;
}
