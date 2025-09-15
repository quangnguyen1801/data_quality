use async_trait::async_trait;
use model::modelviews::notification_view::NotificationView;
#[async_trait]
pub trait INotificationService {
    async fn fn_ser_get_by_id(id: i32) -> anyhow::Result<NotificationView>;
    async fn fn_ser_get_all() -> anyhow::Result<Vec<NotificationView>>;
    async fn fn_ser_get_by_pagination(
        mut page_index: usize,
        mut page_size: usize,
    ) -> anyhow::Result<Vec<NotificationView>>;
    async fn fn_ser_create(obj: NotificationView) -> anyhow::Result<NotificationView>;
    async fn fn_ser_update(obj: NotificationView) -> anyhow::Result<NotificationView>;
    async fn fn_ser_delete(id: i32) -> anyhow::Result<bool>;
    async fn fn_ser_get_by_setting_version_id(
        setting_version_id: i32,
    ) -> anyhow::Result<Vec<NotificationView>>;
}
