use crate::blog_post_repository::BlogPostRepository;

use std::sync::Mutex;

// #region content
type BprMutex = Mutex<BlogPostRepository>;

// #region struct
pub struct AppState {
    pub blog_post_repository: BprMutex,
}
// #endregion struct

impl AppState {
    pub fn new(
        blog_post_repository: BprMutex
    ) -> Self {
        Self {
            blog_post_repository
        }
    }
}
// #endregion content
