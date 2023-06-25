use std::collections::HashSet;
use postgres::types::ToSql;
use uuid::Uuid;
use super::get_database_client;
use crate::{
    error::LemmySearchError,
    database::DatabasePool,
    api::{
        search::models::search::SearchPost, 
        lemmy::models::{
            post::Post, 
            comment::Comment
        },
    }
};

#[derive(Clone)]
pub struct SearchDatabase {
    pub pool : DatabasePool
}

impl SearchDatabase {

    pub fn new(pool : DatabasePool) -> Self {
        Self {
            pool
        }
    }

    pub async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError> {
        get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS xref (
                    word_id         UUID NOT NULL,
                    post_ap_id      VARCHAR NOT NULL
                )
            ", &[]
            ).map(|_| {
                ()
            })
        })
    }

    pub async fn drop_table_if_exists(
        &self
    ) -> Result<(), LemmySearchError> {
        get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS xref", &[])
                .map(|_| {
                    ()
                })
        })
    }

    pub async fn upsert_post(
        &self,
        words : HashSet<String>,
        post : Post
    ) -> Result<(), LemmySearchError> {

        get_database_client(&self.pool, move |client| {

            let mut transaction = client.transaction()?;
            transaction.execute("DELETE FROM xref WHERE post_ap_id = $1", &[&post.ap_id])?;

            let words = words.into_iter().collect::<Vec<String>>();
            let rows = transaction.query("SELECT id FROM words WHERE word = any($1)", &[&words])?;
            let ids = rows.into_iter().map(|row| {
                row.get::<&str, Uuid>("id")
            }).collect::<Vec<Uuid>>();

            let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
            for id in &ids {
                params.push(id);
            }

            let mut query = format!("INSERT INTO xref (word_id, post_ap_id) VALUES ");
            for index in 0..ids.len() {
                query += format!("(${} , $1),", index+2).as_str();
            }
            query = query.trim_end_matches(",").to_string();
            params.insert(0, &post.ap_id);
            transaction.execute(&query, &params)?;

            transaction.commit()
        })
    }

    pub async fn upsert_comment(
        &self,
        words : HashSet<String>,
        comment : Comment
    ) -> Result<(), LemmySearchError> {
        Ok(())
    }

    pub async fn search(
        &self,
        query : &str,
        instance : &Option<String>,
        community : &Option<String>,
        author : &Option<String>
    ) -> Result<Vec<SearchPost>, LemmySearchError> {        

        let query = query.to_owned();
        let instance = instance.to_owned();
        let community = community.to_owned();
        let author = author.to_owned();

        get_database_client(&self.pool, move |client| {

            let temp = query.split_whitespace().map(|s| {
                s.trim().to_string()
            }).collect::<Vec<String>>();

            let instance_query = match instance {
                Some(_) => "AND s.actor_id = $2",
                None => ""
            };
            let community_query = match community {
                Some(_) => "AND c.ap_id = $3",
                None => ""
            };
            let author_query = match author {
                Some(_) => "AND p.author_actor_id = $4",
                None => ""
            };

            let query_string = format!("
                SELECT p.name, p.body, p.url, p.score, p.ap_id, c.title FROM (
                    SELECT DISTINCT ON (p.ap_id) p.ap_id FROM xref AS x
                        JOIN words AS w ON w.id = x.word_id 
                        JOIN posts AS p ON p.ap_id = x.post_ap_id
                        JOIN communities AS c ON c.ap_id = p.community_ap_id
                        JOIN sites AS s ON c.ap_id LIKE s.actor_id || '%'
                    WHERE w.word = any($1)
                        {}
                        {}
                        {}
                    ORDER BY p.ap_id
                ) AS t
                    JOIN posts AS p ON p.ap_id = t.ap_id
                    JOIN communities AS c ON c.ap_id = p.community_ap_id
                ORDER BY p.score ASC
            ", instance_query, community_query, author_query);

            client.query(&query_string, &[&temp, &instance, &community, &author]).map(|rows| {
                rows.iter().map(|row| {
                    SearchPost {
                        url : row.get("p.url"),
                        name : row.get("p.name"),
                        body : row.get("p.body"),
                        score : row.get("p.score"),
                        actor_id : row.get("p.ap_id"),
                        community_name : row.get("c.title"),
                        comments : Vec::new()
                    }
                }).collect()
            })
        })
    }
}
