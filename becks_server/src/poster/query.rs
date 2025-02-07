use crate::prelude::*;
use becks_convey::poster::query::*;

#[get("/query")]
pub(super) async fn query_poster(req: web::Json<QueryRequest>, db: DbData) -> HttpResponse {
    debug!("Attempt to query with token {:?}", req.token);
    let login = extract_login!(db, &req.token);
    let ids = becks_ops::poster::query(login.as_ref(), &req);
    HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .json(QueryResponse { ids })
}
