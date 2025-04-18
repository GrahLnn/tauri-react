use super::enums::table::Table;

pub struct QueryTemplate {
    pub query: &'static str,
}

pub struct Queries {
    pub find_user: QueryTemplate,
    pub create_post: QueryTemplate,
    pub update_status: QueryTemplate,
}

impl Queries {
    pub fn new() -> Self {
        Self {
            find_user: QueryTemplate {
                query: "SELECT * FROM users WHERE id = $1",
            },
            create_post: QueryTemplate {
                query: "INSERT INTO posts (title, content, user_id) VALUES ($1, $2, $3)",
            },
            update_status: QueryTemplate {
                query: "UPDATE users SET status = $1 WHERE id = $2",
            },
        }
    }
    pub fn range_query(table: Table, start: i64, end: i64) -> String {
        let table_name = table.as_str();
        format!("SELECT * FROM {table_name}:{start}..={end};")
    }
}
