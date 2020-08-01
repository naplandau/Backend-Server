table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Varchar,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        phone_number -> Nullable<Varchar>,
        dob -> Timestamp,
        role -> Nullable<Varchar>,
        roles -> Nullable<Text>,
        avatar -> Nullable<Text>,
        time_zone -> Nullable<Text>,
        created_by -> Nullable<Text>,
        created_time_dt -> Timestamp,
        updated_by -> Nullable<Text>,
        updated_time_dt -> Timestamp,
        status -> Int2,
        confirm_code -> Nullable<Varchar>,
        confirm_code_created_time_dt -> Timestamp,
    }
}
