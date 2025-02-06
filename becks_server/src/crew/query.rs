use crate::prelude::*;
use becks_convey::crew::query::*;
use becks_ops::crew::*;

#[get("/query")]
pub(super) async fn query_by_crew(req: web::Json<QueryByRequest>, db: DbData) -> HttpResponse {
    debug!("Querying crew id by columns");
    let login = extract_login!(db, &req.token);
    let query = QueryBy {
        loc: req.loc.clone(),
        fuzzy: req.fuzzy,
    };
    let ids = query.query(login.as_ref());
    HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .json(QueryByResponse { ids })
}
