table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        description -> Varchar,
        body -> Text,
        tag_list -> Array<Text>,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(articles -> users (user_id));

allow_tables_to_appear_in_same_query!(articles, users,);
