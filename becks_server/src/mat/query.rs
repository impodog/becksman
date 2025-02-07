use crate::prelude::*;
use becks_convey::mat::query::*;

#[get("/query")]
pub(super) async fn query_mat(req: web::Json<QueryRequest>, db: DbData) -> HttpResponse {
    let login = extract_login!(db, &req.token);
    let ids = becks_ops::mat::query(login.as_ref(), &req);
    HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .json(QueryResponse { ids })
}
