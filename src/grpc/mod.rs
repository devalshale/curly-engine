// File: src/grpc/mod.rs

use tonic::{transport::Server, Request, Response, Status};

pub mod customer_service {
    tonic::include_proto!("myapp");
}

use customer_service::customer_service_server::{CustomerService, CustomerServiceServer};
use customer_service::{FindByIdRequest, FindByIdResponse};

use customer_service::user_service_server::{UserService, UserServiceServer};
use customer_service::{DeleteUserRequest, DeleteUserResponse};

pub struct MyCustomerService;

#[tonic::async_trait]
impl CustomerService for MyCustomerService {
    async fn find_by_id(
        &self,
        request: Request<FindByIdRequest>,
    ) -> Result<Response<FindByIdResponse>, Status> {
        let id = request.into_inner().id;
        println!("Got a request from {:?}", id);

        let response = FindByIdResponse {
            name: "jane Doe".into(),
            email: "john.doe@example.com".into(),
            // ... set other fields
        };

        Ok(Response::new(response))
    }
}

pub struct MyUserService;

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let id = request.into_inner().id;
        println!("Got a request to delete user {:?}", id);

        let response = DeleteUserResponse {
            success: true,
        };

        Ok(Response::new(response))
    }
}

pub async fn serve(port: u16) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let service = MyCustomerService;
    let user_service = MyUserService;

    println!("📡 gRPC server initiating on port {}", port);

    let server = Server::builder()
        .add_service(CustomerServiceServer::new(service))
        .add_service(UserServiceServer::new(user_service))
        .serve(addr);

    server.await.map_err(|e| {
        eprintln!("Failed to start gRPC server: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })
}
