

use rocket_contrib::json::Json;
use crate::db::Conn;
use super::models::Parent;
use serde_json::{json, Value};
use crate::models::NewParent;

#[get("/parents" ,format="application/json")]
pub fn parents(conn:Conn)->Json<Value> {
    let result=Parent::get_all(&conn);
    let value=json!(
        {
            "status":200,
            "result":result
        }
    );
    Json(value)
}

#[post("/parents" ,format="application/json", data="<parent>")]
pub fn insert_parent(parent:Json<NewParent>,conn:Conn)->Json<Value> {
    let result=Parent::insert(parent.into_inner(),&conn);
    let value=json!(
        {
            "status":200,
            "result":result
        }
    );
    Json(value)
}

#[get("/parents/<id>" ,format="application/json")]
pub fn get_parent(id:i32,conn:Conn)->Json<Value> {
    let result=Parent::get(id,&conn);
    let value=json!(
        {
            "status":200,
            "result":result
        }
    );
    Json(value)
}
#[put("/parents/<id>" ,format="application/json",data="<parent>")]
pub fn update_parent(id:i32,parent:Json<NewParent>,conn:Conn)->Json<Value> {
    let result=Parent::update_by_id(id,parent.into_inner(),&conn);
    let value=json!(
        {
            "status":200,
            "result":result
        }
    );
    Json(value)
}

#[delete("/parents/<id>" ,format="application/json")]
pub fn delete_parent(id:i32,conn:Conn)->Json<Value> {
    let result=Parent::delete_by_id(id,&conn);
    let value=json!(
        {
            "status":200,
            "result":result
        }
    );
    Json(value)
}
#[delete("/parents" )]
pub fn delete_all_parents(conn:Conn)->Json<Value> {
    let result=Parent::delete_all(&conn);
    let value=json!(
        {
            "status":200,
            "result":result
        }
    );
    Json(value)
}

/*#[catch(404)]
fn not_found()->Json<Value>{

}*/

