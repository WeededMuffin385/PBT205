use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "order_action", rename_all = "lowercase")]
pub enum OrderAction {
    Buy,
    Sell,
}