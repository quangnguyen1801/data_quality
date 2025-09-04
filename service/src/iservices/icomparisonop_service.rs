use async_trait::async_trait;
use model::modelviews::comparisonop_view::ComparisonOpView;
#[async_trait]
pub trait IComparisonOpService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<ComparisonOpView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<ComparisonOpView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<ComparisonOpView>>;
    async fn fn_ser_create(obj: ComparisonOpView) -> anyhow::Result<ComparisonOpView>;
    async fn fn_ser_update(obj: ComparisonOpView) -> anyhow::Result<ComparisonOpView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
}
