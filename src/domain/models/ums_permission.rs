use strum_macros::{Display, EnumIter};

#[derive(Display, EnumIter)]
pub enum UMSPermission {
    GetUserSelf,
    GetUser,
    CreateUser,
    UpdateUser,
    UpdateUserSelf,
    DeleteUser,
    ConfirmUser,
    ResetUserPassword,

    SendConfirmCode,

    GetSessionSelf,
    GetSession,
    CreateSession,
    UpdateSession,
    DeleteSession,
    DeleteSessionSelf,
    GetAccessLogSelf,
    GetAccessLog,
    
    GetRole,
    CreateRole,
    UpdateRole,
    DeleteRole,
    SetDefaultRole,
    LinkUserRole,
    GetUserRole,
    GetSelfRole,
    
    GetPermission,
    UpdatePermission,
    LinkRolePermission,
    
    GetService,
    UpdateService,
}
