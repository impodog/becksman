use crate::prelude::*;
use becks_convey::crew::modify::*;
use becks_crew::*;
use becks_ops::crew::Column;

macro_rules! modify_by {
    ($type: ty, $column: ident, $login: ident, $req: ident) => {{
        debug_assert_eq!(
            std::any::type_name::<$type>(),
            std::any::type_name_of_val(&$column)
        );
        debug!(
            "Modifying field {} from login {}",
            <$type>::name(),
            $login.name
        );
        if $column.modify($login.as_ref(), $req.id) {
            info!("Modification of field {} is done", <$type>::name());
            HttpResponse::Ok()
                .content_type(http::header::ContentType::plaintext())
                .body("modification successful")
        } else {
            warn!("Unable to modify field {}", <$type>::name());
            HttpResponse::BadRequest()
                .content_type(http::header::ContentType::plaintext())
                .body("unable to modify the desired field")
        }
    }};
}

#[post("/modify")]
pub(super) async fn modify_crew(req: web::Json<ModifyRequest>, db: DbData) -> HttpResponse {
    use ModifyLocation as Loc;

    let login = extract_login!(db, &req.token);
    match req.loc.to_owned() {
        Loc::Name(name) => modify_by!(String, name, login, req),
        Loc::Social(social) => modify_by!(Social, social, login, req),
        Loc::Gender(gender) => modify_by!(Gender, gender, login, req),
        Loc::Clothes(clothes) => modify_by!(Clothes, clothes, login, req),
        Loc::Hand(hand) => modify_by!(Hand, hand, login, req),
        Loc::Hold(hold) => modify_by!(Hold, hold, login, req),
        Loc::Paddle(paddle) => modify_by!(Paddle, paddle, login, req),
        Loc::Red(red) => modify_by!(RedRubber, red, login, req),
        Loc::Black(black) => modify_by!(BlackRubber, black, login, req),
    }
}
