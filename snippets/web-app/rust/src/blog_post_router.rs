use crate::{
    blog_post::BlogPost,
    blog_post_repository::BlogPostRepository,
    state::AppState
};

use axum::{
    body::Body,
    extract::{
        Path,
        State
    },
    http::{
        header::CONTENT_TYPE,
        StatusCode
    },
    Json,
    response::Response,
    Router,
    routing::{
        delete as del,
        get,
        post,
        put
    }
};
use serde::Serialize;
use std::sync::{
    Arc,
    MutexGuard
};

// #region content
// #region method
pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/blog-post", post(create))
        .route("/blog-post", put(update))
        .route("/blog-post/{title}", get(read))
        .route("/blog-post/{title}", del(delete))
        .with_state(state)
}
// #endregion method

fn extract_repository_from_state<'a>(state: &'a Arc<AppState>) -> Result<MutexGuard<'a, BlogPostRepository>, (StatusCode, String)> {
    state
        .blog_post_repository
        .lock()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Repository lock poisoned"),
            )
        })
}

fn json_response<T: Serialize>(
    status: StatusCode,
    value: &T,
) -> Result<Response, (StatusCode, String)> {
    let body = serde_json::to_vec(value).map_err(|error| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            error.to_string(),
        )
    })?;

    Ok(Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .expect("Valid response"))
}

// #region post
pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(blog_post): Json<BlogPost>,
) -> Result<Response, (StatusCode, String)> {
// #endregion post
    let mut repository = extract_repository_from_state(&state)?;

    let saved_blog_post = repository.save(blog_post);

    json_response(StatusCode::CREATED, saved_blog_post)
}

// #region get
pub async fn read(
    State(state): State<Arc<AppState>>,
    Path(title): Path<String>,
) -> Result<Response, (StatusCode, String)> {
// #endregion get
    let repository = extract_repository_from_state(&state)?;

    let found_blog_post = repository
        .find_by_title(&title)
        .ok_or((StatusCode::NOT_FOUND, title))?;

    json_response(StatusCode::OK, found_blog_post)

}

// #region put
pub async fn update(
    State(state): State<Arc<AppState>>,
    Json(blog_post): Json<BlogPost>,
) -> Result<Response, (StatusCode, String)> {
// #endregion put
    let mut repository = extract_repository_from_state(&state)?;

    let saved_blog_post = repository.save(blog_post);

    json_response(StatusCode::OK, saved_blog_post)
}

// #region delete
pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(title): Path<String>,
) -> Result<Response, (StatusCode, String)> {
// #endregion delete
    let mut repository = extract_repository_from_state(&state)?;

    let deleted_blog_post = repository
        .delete_by_title(&title)
        .ok_or((StatusCode::NOT_FOUND, title))?;

    json_response(StatusCode::OK, &deleted_blog_post)
}
// #endregion content
