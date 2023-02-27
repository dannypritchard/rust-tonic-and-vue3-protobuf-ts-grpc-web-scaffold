mod protos;
mod tls;

extern crate rcgen;

use dotenvy::dotenv;
use entity::prelude::Vote as VoteEntity;
use migration::{Migrator, MigratorTrait};
use protos::voting::{
    voting_server::{Voting, VotingServer},
    *,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

impl From<Vote> for entity::vote::Model {
    fn from(value: Vote) -> Self {
        let v = serde_json::to_value(&value).unwrap();
        serde_json::from_value::<Self>(v).unwrap()
    }
}

impl From<entity::vote::Model> for Vote {
    fn from(value: entity::vote::Model) -> Self {
        let v = serde_json::to_value(&value).unwrap();
        serde_json::from_value::<Self>(v).unwrap()
    }
}

#[derive(Default)]
pub struct VotingService {
    db: Arc<DatabaseConnection>,
}

#[tonic::async_trait]
impl Voting for VotingService {
    async fn index(
        &self,
        _request: Request<VotingIndexRequest>,
    ) -> Result<Response<VotingIndexResponse>, Status> {
        let votes = VoteEntity::find()
            .all(&*Arc::clone(&self.db))
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|vote| Vote::from(vote))
            .collect();

        Ok(Response::new(VotingIndexResponse { votes }))
    }

    async fn get(
        &self,
        request: Request<VotingGetRequest>,
    ) -> Result<Response<VotingGetResponse>, Status> {
        let vote = VoteEntity::find_by_id(request.into_inner().id)
            .one(&*Arc::clone(&self.db))
            .await
            .unwrap_or_default()
            .ok_or_else(|| Status::permission_denied("go away"))?;

        Ok(Response::new(VotingGetResponse {
            vote: Some(Vote::from(vote)),
        }))
    }

    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        let r = request.into_inner();

        let id = entity::vote::Entity::insert(entity::vote::ActiveModel {
            url: Set(r.url.clone()),
            ..Default::default()
        })
        .exec(&*Arc::clone(&self.db))
        .await
        .map_err(|_| Status::permission_denied("go away"))?
        .last_insert_id;

        Ok(Response::new(VotingResponse {
            vote: Some(Vote {
                id,
                url: r.url,
                count: 1,
            }),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let db = Arc::new(Database::connect(dotenvy::var("DATABASE_URL")?).await?);

    Migrator::up(&*Arc::clone(&db), None).await?;

    let addr = "127.0.0.1:8009".parse()?;

    println!("Listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        // .tls_config(tls::get_tls_config()?)?
        .layer(CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(VotingServer::new(VotingService {
            db: Arc::clone(&db),
        }))
        .serve(addr)
        .await?;

    Ok(())
}
