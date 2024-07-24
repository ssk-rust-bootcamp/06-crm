use anyhow::Result;
use crm::pb::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, GetUserRequest, User,
};
use tonic::{async_trait, transport::Server, Response, Status};

#[derive(Default)]
pub struct UserServer {}
#[async_trait]
impl UserService for UserServer {
    async fn get_user(
        &self,
        request: tonic::Request<GetUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("get_user: {:?}", input);
        Ok(Response::new(User::default()))
    }
    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("create_user: {:?}", input);
        Ok(Response::new(User::default()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let svc = UserServer::default();

    println!("UserService listening on: {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
