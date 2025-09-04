use async_trait::async_trait;
use model::modelviews::matrixexecute_view::MatrixExecuteView;
#[async_trait]
pub trait IMatrixExecuteService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<MatrixExecuteView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<MatrixExecuteView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<MatrixExecuteView>>;
    async fn fn_ser_create(obj: MatrixExecuteView) -> anyhow::Result<MatrixExecuteView>;
    async fn fn_ser_update(obj: MatrixExecuteView) -> anyhow::Result<MatrixExecuteView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
}
