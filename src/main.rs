use axum::Router;
use tokio:: net::TcpListener;
mod routes;
pub mod utils;



#[tokio::main]
async fn main() {
    // build our application with a multiple routes
    let routes_all: Router = Router::new().merge(routes::routes_handler::routes());
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> SERVER RUNNING ON {:?} \n", listener);
    
    axum::serve(listener, routes_all.into_make_service())
    .await
    .unwrap()
}