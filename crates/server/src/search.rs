use actix_web::{
    get, 
    web::{
        Json, 
        Query
    }, 
    Responder, 
    Result
};
use lemmy_search_common::models::{
    SearchQuery, 
    SearchResult,
    LemmyInstance
};

#[get("/search")]
async fn search(
    search_query: Query<SearchQuery>
) -> Result<impl Responder> {
    let search_results = SearchResult {
        original_query : search_query.into_inner(),
        search_results : Vec::new(),
        total_pages : 0

    };
    Ok(Json(search_results))
}

#[get("/instances")]
async fn get_instances(

) -> Result<impl Responder> {
    let instances = Vec::<LemmyInstance>::new();

    Ok(Json(instances))   
}
