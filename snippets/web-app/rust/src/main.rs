mod blog_post;
mod blog_post_repository;
mod blog_post_router;
mod state;

use crate::{
    blog_post_repository::BlogPostRepository,
    blog_post_router::router,
    state::AppState
};

use std::{
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr
    },
    sync::{
        Arc,
        Mutex
    }
};

use tokio::net::TcpListener;

// #region content
#[tokio::main]
async fn main() {
    let app_state = Arc::from(AppState::new(Mutex::new(BlogPostRepository::new())));
    let router = router(app_state);

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8080);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
// #endregion content
