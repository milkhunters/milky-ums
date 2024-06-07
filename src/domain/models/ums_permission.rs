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
    
    GetRole,
    CreateRole,
    UpdateRole,
    DeleteRole,

    SetDefaultRole,
    GetDefaultRole,
    
    LinkUserRole,
    UnlinkUserRole,
    GetUserRole,
    GetSelfRole,
    
    CreatePermission,
    GetPermission,
    UpdatePermission,
    DeletePermission,
    
    LinkRolePermission,
    UnlinkRolePermission,
    
    GetService,
    UpdateService,
    DeleteService,
    
}
