use std::ops::Deref;
use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::{Request, State};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

pub type Pool =r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url:String)->Pool{
    let manager=ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).unwrap()
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a,'r> FromRequest<'a,'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool=request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn)=>Outcome::Success(Conn(conn)),
            Err(_)=>Outcome::Failure((Status::ServiceUnavailable,())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}