mod protos;
mod tls;

extern crate rcgen;

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use protos::voting::{
    voting_server::{Voting, VotingServer},
    *,
};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

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
        let votes = vec![];
        Ok(Response::new(VotingIndexResponse { votes }))
    }

    async fn get(
        &self,
        _request: Request<VotingGetRequest>,
    ) -> Result<Response<VotingGetResponse>, Status> {
        Ok(Response::new(VotingGetResponse {
            vote: Some(Vote {
                id: 123,
                url: "http://www.google.com".to_string(),
                count: 666,
            }),
        }))
    }

    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        let r = request.into_inner();
        Ok(Response::new(VotingResponse {
            confirmation: format!("You voted {} for {}", r.vote, r.url),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL")?;

    let db = Database::connect(database_url).await?;
    Migrator::up(&db, None).await?;

    let db = Arc::new(db);

    let addr = "127.0.0.1:8009".parse().unwrap();

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
