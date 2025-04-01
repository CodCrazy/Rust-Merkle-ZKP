use std::{collections::HashMap, sync::Arc};

use axum::extract::State;

use crate::{models::ddid_models::CoreIdTree, AppState};


pub async fn get_current_leaves(
    table_name: String,
    State(data): State<Arc<AppState>>
) -> HashMap<String, usize> {
    let query = format!(r#"SELECT * FROM {} LIMIT 1"#, table_name);
    let row = sqlx::query_as::<_, CoreIdTree>(
        &query
    )
    .bind(table_name)
    .fetch_one(&data.db)
    .await;
    let retrieved_map: HashMap<String, usize>;
    match row {
        Ok(row) => {
            let data = row.leaves;
            retrieved_map = serde_json::from_value(data).unwrap();
            println!("core id tree data: {:?}", retrieved_map);

        }
        Err(_) => {
            retrieved_map = HashMap::new();
        }
    }
    return retrieved_map;
}