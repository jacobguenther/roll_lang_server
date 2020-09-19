table! {
    players (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        hash_salt -> Text,
        is_admin -> Bool,
    }
}

table! {
    settings (player_id) {
        player_id -> Int4,
        theme -> Text,
    }
}

joinable!(settings -> players (player_id));

allow_tables_to_appear_in_same_query!(
    players,
    settings,
);
