use diesel::prelude::*;
use uuid::Uuid;

use crate::auth::models::*;
use crate::db::schema::users::dsl::*;
use crate::errors::ServiceError;
use crate::utils::password::{hash_password, verify_password};
use crate::utils::jwt::generate_token;
use crate::utils::supabase::sign_up_user;

pub async fn register_user(
    db: &mut PgConnection,
    register_data: RegisterRequest,
) -> Result<LoginResponse, ServiceError> {
    // 检查用户是否已存在
    let existing_user = users
        .filter(email.eq(&register_data.email))
        .first::<User>(db)
        .optional()?;

    if existing_user.is_some() {
        return Err(ServiceError::UserAlreadyExists);
    }

    // 哈希密码
    let password_hash_str = hash_password(&register_data.password)?;

    // 创建新用户
    let new_user = NewUser {
        email: register_data.email.clone(),
        password_hash: password_hash_str,
        full_name: register_data.full_name,
    };

    // 插入用户到数据库
    let user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(db)?;

    // 同步用户到Supabase
    sign_up_user(&user.email, &register_data.password).await?;

    // 生成JWT令牌
    let token = generate_token(user.id)?;

    Ok(LoginResponse {
        token,
        user_id: user.id,
        email: user.email,
    })
}

pub async fn login_user(
    db: &mut PgConnection,
    login_data: LoginRequest,
) -> Result<LoginResponse, ServiceError> {
    // 查找用户
    let user = users
        .filter(email.eq(&login_data.email))
        .first::<User>(db)
        .optional()?
        .ok_or(ServiceError::InvalidCredentials)?;

    // 验证密码
    if !verify_password(&login_data.password, &user.password_hash)? {
        return Err(ServiceError::InvalidCredentials);
    }

    // 更新最后登录时间
    let user: User = diesel::update(users.find(user.id))
        .set(last_login.eq(diesel::dsl::now))
        .get_result(db)?;

    // 生成JWT令牌
    let token = generate_token(user.id)?;

    Ok(LoginResponse {
        token,
        user_id: user.id,
        email: user.email,
    })
}

pub async fn get_user_by_id(
    db: &mut PgConnection,
    user_id: Uuid,
) -> Result<User, ServiceError> {
    let user = users
        .find(user_id)
        .first::<User>(db)
        .optional()?
        .ok_or(ServiceError::UserNotFound)?;

    Ok(user)
}
