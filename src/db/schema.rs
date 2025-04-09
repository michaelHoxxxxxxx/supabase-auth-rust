use diesel::table;
use diesel::joinable;
use diesel::allow_tables_to_appear_in_same_query;

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_login -> Nullable<Timestamptz>,
        is_active -> Bool,
    }
}

table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    permissions (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        resource -> Varchar,
        action -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_roles (id) {
        id -> Uuid,
        user_id -> Uuid,
        role_id -> Uuid,
        created_at -> Timestamptz,
    }
}

table! {
    role_permissions (id) {
        id -> Uuid,
        role_id -> Uuid,
        permission_id -> Uuid,
        created_at -> Timestamptz,
    }
}

joinable!(user_roles -> users (user_id));
joinable!(user_roles -> roles (role_id));
joinable!(role_permissions -> roles (role_id));
joinable!(role_permissions -> permissions (permission_id));

allow_tables_to_appear_in_same_query!(
    users,
    roles,
    permissions,
    user_roles,
    role_permissions,
);
