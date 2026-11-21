use actix_web::{Responder, get, post, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Sqlite, prelude::FromRow};

#[derive(Deserialize, Debug)]
struct CreateInput {
    name: String,
    method: String,
    to_url: String,
    from_url: String,
    query: String,
}

#[post("/mapping/create")]
pub async fn add_new_mapping(
    db: web::Data<Pool<Sqlite>>,
    input: web::Json<CreateInput>,
) -> impl Responder {
    let resp = sqlx::query(
        "INSERT INTO gql_to_rest (name, method, to_url, from_url, query) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&input.name)
    .bind(&input.method)
    .bind(&input.to_url)
    .bind(&input.from_url)
    .bind(&input.query)
    .execute(db.get_ref())
    .await;
    let res_json = match resp {
        Ok(r) => json!({
                "success": true,
                "data":json!({
            "rows_affected": r.rows_affected(),
            "last_insert_rowid": r.last_insert_rowid()
        })
            }),
        Err(e) => {
            println!("{:?}", e);
            json!({
                "success": false,
                "data": []
            })
        }
    };
    web::Json(res_json)
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Mapping {
    id: u64,
    pub name: String,
    pub method: String,
    pub to_url: String,
    pub from_url: String,
    pub query: String,
}

#[get("/mapping")]
pub async fn get_all_mappings(db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let resp = sqlx::query_as::<_, Mapping>("SELECT * FROM gql_to_rest")
        .fetch_all(db.get_ref())
        .await;
    let res_json = match resp {
        Ok(r) => json!({
            "success": true,
            "data": r
        }),
        Err(_) => json!({
            "success": false,
            "data": []
        }),
    };
    web::Json(res_json)
}

#[derive(Deserialize)]
struct QueryParam {
    name: String,
}

#[get("/request")]
pub async fn get_from_gql(
    db: web::Data<Pool<Sqlite>>,
    query: web::Query<QueryParam>,
) -> impl Responder {
    let cfg_request = sqlx::query_as::<_, Mapping>("SELECT * FROM gql_to_rest WHERE name = ?")
        .bind(&query.name)
        .fetch_one(db.get_ref())
        .await;
    if cfg_request.is_err() {
        return web::Json(json!({"success": false, "data":[]}));
    }
    let cfg_request_unwrapped = cfg_request.unwrap();
    let client = reqwest::Client::new();
    let body = json!({
        "query": cfg_request_unwrapped.query,
        "variables":{}
    })
    .to_string();
    let result = client
        .post(cfg_request_unwrapped.from_url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    let s = match result {
        Ok(r) => {
            let res = r.text().await;
            if res.is_err() {
                return web::Json(json!({"success": false, "data":[]}));
            }

            let r = match serde_json::from_str(&res.unwrap()) {
                Ok(json) => json,
                Err(_) => json!({"success": false, "data": []}),
            };
            json!({"success": true, "data": r})
        }
        Err(_) => json!({"success": false, "data": []}),
    };
    web::Json(s)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_new_mapping);
    cfg.service(get_all_mappings);
    cfg.service(get_from_gql);
}
