use crate::blog_post::BlogPost;

use std::collections::HashMap;

// #region content
// #region struct
pub struct BlogPostRepository {
    blog_posts: HashMap<String, BlogPost>,
}
// #endregion struct

impl BlogPostRepository {
    pub fn new() -> Self {
        Self {
            blog_posts: HashMap::new()
        }
    }

    pub fn find_by_title(&self, title: &str) -> Option<&BlogPost> {
        self.blog_posts.get(title)
    }

    pub fn save(&mut self, blog_post: BlogPost) -> &BlogPost {
        let title = blog_post.title().to_owned();
        self.blog_posts.insert(title.clone(), blog_post);
        self.blog_posts.get(&title).expect("A newly added blog post should always exist.")
    }

    pub fn delete_by_title(&mut self, title: &str) -> Option<BlogPost> {
        self.blog_posts.remove(title)
    }
}
// #endregion content
