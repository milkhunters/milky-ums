use strum_macros::{Display, EnumIter};

#[derive(Display, EnumIter)]
pub enum UMSPermission {
    GetUserSelf,
    GetUser,
    CreateUser,
    UpdateUser,
    UpdateUserSelf,
    DeleteUser,

    GetSessionSelf,
    GetSession,
    CreateSession,
    UpdateSession,
    DeleteSession,
    DeleteSessionSelf,

    GetRoleSelf,
    GetRole,
    CreateRole,
    UpdateRole,
    DeleteRole,
}
