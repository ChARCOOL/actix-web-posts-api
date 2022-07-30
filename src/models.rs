use crate::schema::articles;
use crate::{actix::Addr, actors::db::DbActor};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub db: Addr<DbActor>,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Article {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticle {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleData {
    pub title: String,
    pub body: String,
}
