// @generated automatically by Diesel CLI.

diesel::table! {
    employee_departments (id) {
        id -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        created_at -> Timestamp,
        created_by -> Uuid,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Uuid>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    employee_directorates (id) {
        id -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        created_at -> Timestamp,
        created_by -> Uuid,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Uuid>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    employee_divisions (id) {
        id -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        created_at -> Timestamp,
        created_by -> Uuid,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Uuid>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    employee_positions (id) {
        id -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        created_at -> Timestamp,
        created_by -> Uuid,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Uuid>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    employee_projects (id) {
        id -> Uuid,
        employee_id -> Nullable<Uuid>,
        project_id -> Uuid,
        created_at -> Timestamp,
        created_by -> Uuid,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Uuid>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    employees (id) {
        id -> Uuid,
        #[max_length = 16]
        nik -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        position_id -> Nullable<Uuid>,
        department_id -> Nullable<Uuid>,
        division_id -> Nullable<Uuid>,
        directorate_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        created_by -> Uuid,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Uuid>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::joinable!(employee_projects -> employees (employee_id));
diesel::joinable!(employees -> employee_departments (department_id));
diesel::joinable!(employees -> employee_directorates (directorate_id));
diesel::joinable!(employees -> employee_divisions (division_id));
diesel::joinable!(employees -> employee_positions (position_id));

diesel::allow_tables_to_appear_in_same_query!(
    employee_departments,
    employee_directorates,
    employee_divisions,
    employee_positions,
    employee_projects,
    employees,
);
