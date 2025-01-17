use std::{
    hash::Hash, 
    collections::HashMap
};
use postgres::types::ToSql;
use crate::api::lemmy::models::author::Author;
use super::{
    DatabaseSchema, 
    DatabaseType
};

impl DatabaseSchema for Author {

    fn get_table_name(

    ) -> String {
        "authors".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "ap_id".to_string(),
            "avatar".to_string(),
            "name".to_string(),
            "display_name".to_string()
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("ap_id".to_string(), DatabaseType::String(0).not_null()),
            ("avatar".to_string(), DatabaseType::String(0).nullable()),
            ("name".to_string(), DatabaseType::String(0).not_null()),
            ("display_name".to_string(), DatabaseType::String(0).nullable())
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.actor_id,
            &self.avatar,
            &self.name,
            &self.display_name
        ]
    }
}

impl PartialEq for Author {
    fn eq(&self, other: &Self) -> bool {
        self.actor_id == other.actor_id
    }
}

impl Eq for Author {

}

impl Hash for Author {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actor_id.hash(state);
    }
}