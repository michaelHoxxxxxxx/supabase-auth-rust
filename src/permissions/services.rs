use diesel::prelude::*;
use uuid::Uuid;

use crate::permissions::models::*;
use crate::errors::ServiceError;

// 创建角色
pub fn create_role(
    db: &mut PgConnection,
    role_data: CreateRoleRequest,
) -> Result<Role, ServiceError> {
    use crate::db::schema::roles::dsl::*;
    
    // 检查角色是否已存在
    let existing_role_count = roles
        .filter(name.eq(&role_data.name))
        .count()
        .get_result::<i64>(db)?;
    
    if existing_role_count > 0 {
        return Err(ServiceError::RoleAlreadyExists);
    }
    
    // 创建新角色
    let new_role = NewRole {
        name: role_data.name,
        description: role_data.description,
    };
    
    let role = diesel::insert_into(roles)
        .values(&new_role)
        .get_result(db)?;
    
    Ok(role)
}

// 创建权限
pub fn create_permission(
    db: &mut PgConnection,
    permission_data: CreatePermissionRequest,
) -> Result<Permission, ServiceError> {
    use crate::db::schema::permissions::dsl::*;
    
    // 检查权限是否已存在
    let existing_permission_count = permissions
        .filter(name.eq(&permission_data.name))
        .count()
        .get_result::<i64>(db)?;
    
    if existing_permission_count > 0 {
        return Err(ServiceError::PermissionAlreadyExists);
    }
    
    // 创建新权限
    let new_permission = NewPermission {
        name: permission_data.name,
        description: permission_data.description,
        resource: permission_data.resource,
        action: permission_data.action,
    };
    
    let permission = diesel::insert_into(permissions)
        .values(&new_permission)
        .get_result(db)?;
    
    Ok(permission)
}

// 分配角色给用户
pub fn assign_role_to_user(
    db: &mut PgConnection,
    user_id: Uuid,
    role_id: Uuid,
) -> Result<UserRole, ServiceError> {
    use crate::db::schema::user_roles;
    use crate::db::schema::users;
    use crate::db::schema::roles;
    // 检查用户角色是否已存在
    let existing_count: i64 = user_roles::table                             // 1. 先指定 table (user_roles)
        .filter(user_roles::user_id.eq(user_id).and(user_roles::role_id.eq(role_id))) // 2. 再 filter
        .select(diesel::dsl::count_star())                                   // 3. 再 select (count(*))
        .first(db)?;                                                       // 4. 执行
    if existing_count > 0 {
        let existing = user_roles::table
            .filter(user_roles::user_id.eq(user_id).and(user_roles::role_id.eq(role_id)))
            .first::<UserRole>(db)?;
        
        return Ok(existing);
    }
    
    // 检查用户是否存在
    let user_exists: i64 = users::table
        .filter(users::id.eq(user_id))
        .select(diesel::dsl::count_star())
        .first(db)?;
    
    if user_exists == 0 {
        return Err(ServiceError::UserNotFound);
    }
    
    // 检查角色是否存在
    let role_exists: i64 = roles::table
        .filter(roles::id.eq(role_id))
        .select(diesel::dsl::count_star())
        .first(db)?;
    
    if role_exists == 0 {
        return Err(ServiceError::RoleNotFound);
    }
    
    // 创建用户角色关联
    let new_user_role = (
        user_roles::user_id.eq(user_id),
        user_roles::role_id.eq(role_id),
        user_roles::created_at.eq(diesel::dsl::now),
    );
    
    let user_role = diesel::insert_into(user_roles::table)
        .values(new_user_role)
        .get_result::<UserRole>(db)?;
    
    Ok(user_role)
}

// 分配权限给角色
pub fn assign_permission_to_role(
    db: &mut PgConnection,
    role_id: Uuid,
    permission_id: Uuid,
) -> Result<RolePermission, ServiceError> {
    use crate::db::schema::role_permissions;
    use crate::db::schema::roles;
    use crate::db::schema::permissions;
    
    // 检查角色权限是否已存在
    let existing_count: i64 = role_permissions::table
        .filter(role_permissions::role_id.eq(role_id).and(role_permissions::permission_id.eq(permission_id)))
        .select(diesel::dsl::count_star())
        .first(db)?;
    
    if existing_count > 0 {
        let existing = role_permissions::table
            .filter(role_permissions::role_id.eq(role_id).and(role_permissions::permission_id.eq(permission_id)))
            .first::<RolePermission>(db)?;
        
        return Ok(existing);
    }
    
    // 检查角色是否存在
    let role_exists: i64 = roles::table
        .filter(roles::id.eq(role_id))
        .select(diesel::dsl::count_star())
        .first(db)?;
    
    if role_exists == 0 {
        return Err(ServiceError::RoleNotFound);
    }
    
    // 检查权限是否存在
    let permission_exists: i64 = permissions::table
        .filter(permissions::id.eq(permission_id))
        .select(diesel::dsl::count_star())
        .first(db)?;
    
    if permission_exists == 0 {
        return Err(ServiceError::PermissionNotFound);
    }
    
    // 创建角色权限关联
    let new_role_permission = (
        role_permissions::role_id.eq(role_id),
        role_permissions::permission_id.eq(permission_id),
        role_permissions::created_at.eq(diesel::dsl::now),
    );
    
    let role_permission = diesel::insert_into(role_permissions::table)
        .values(new_role_permission)
        .get_result::<RolePermission>(db)?;
    
    Ok(role_permission)
}

// 获取用户的所有角色
pub fn get_user_roles(
    db: &mut PgConnection,
    target_user_id: Uuid,
) -> Result<Vec<UserRole>, ServiceError> {
    use crate::db::schema::user_roles;
    
    let user_roles_list = user_roles::table
        .filter(user_roles::user_id.eq(target_user_id))
        .load::<UserRole>(db)?;
    
    Ok(user_roles_list)
}

// 获取角色的所有权限
pub fn get_role_permissions(
    db: &mut PgConnection,
    target_role_id: Uuid,
) -> Result<Vec<RolePermission>, ServiceError> {
    use crate::db::schema::role_permissions;
    
    let role_permissions_list = role_permissions::table
        .filter(role_permissions::role_id.eq(target_role_id))
        .load::<RolePermission>(db)?;
    
    Ok(role_permissions_list)
}

// 检查用户是否有特定权限
pub fn check_user_permission(
    db: &mut PgConnection,
    user_id: Uuid,
    resource: &str,
    action: &str,
) -> Result<bool, ServiceError> {
    use crate::db::schema::permissions;
    use crate::db::schema::role_permissions;
    
    // 获取用户的所有角色
    let user_roles_list = get_user_roles(db, user_id)?;
    
    // 如果用户没有角色，则没有权限
    if user_roles_list.is_empty() {
        return Ok(false);
    }
    
    // 获取权限
    let permission = permissions::table
        .filter(permissions::resource.eq(resource).and(permissions::action.eq(action)))
        .first::<Permission>(db)
        .optional()?;
    
    let permission = match permission {
        Some(p) => p,
        None => return Ok(false),
    };
    
    // 检查用户的角色是否有该权限

    for user_role in user_roles_list {
        // --- 修改后的查询 ---
        let role_permission_count: i64 = role_permissions::table // 1. 先指定 table
            .filter(role_permissions::role_id.eq(user_role.role_id).and(role_permissions::permission_id.eq(permission.id))) // 2. 再 filter
            .select(diesel::dsl::count_star())                   // 3. 再 select (count(*))
            .first(db)?;                                         // 4. 执行
        // --- 修改结束 ---

        if role_permission_count > 0 {
            return Ok(true);
        }
    }
    Ok(false)
}
