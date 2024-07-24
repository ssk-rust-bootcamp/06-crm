use std::{ops::Deref, pin::Pin, sync::Arc};

pub use config::AppConfig;
use futures::Stream;
use pb::{
    user_stats_server::{UserStats, UserStatsServer},
    QueryRequest, RowQueryRequest, User,
};
use sqlx::PgPool;
use tonic::{async_trait, Request, Response, Status};
mod abi;
mod config;
mod pb;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send + Sync>>;

#[derive(Clone)]
pub struct UserStatsService {
    inner: Arc<UserStatsServiceInner>,
}

#[allow(unused)]
pub struct UserStatsServiceInner {
    config: AppConfig,
    pool: PgPool,
}

#[async_trait]
impl UserStats for UserStatsService {
    type QueryStream = ResponseStream;
    type RowQueryStream = ResponseStream;

    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        let query = request.into_inner();
        self.query(query).await
    }

    async fn row_query(
        &self,
        request: Request<RowQueryRequest>,
    ) -> ServiceResult<Self::RowQueryStream> {
        let query = request.into_inner();
        self.raw_query(query).await
    }
}

impl UserStatsService {
    pub async fn new(config: AppConfig) -> Self {
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .expect("Failed to connect to database");
        let inner = UserStatsServiceInner { config, pool };

        Self {
            inner: Arc::new(inner),
        }
    }
    pub fn into_server(self) -> UserStatsServer<Self> {
        UserStatsServer::new(self)
    }
}
impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
