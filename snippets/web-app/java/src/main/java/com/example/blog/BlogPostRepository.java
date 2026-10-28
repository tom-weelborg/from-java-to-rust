package com.example.blog;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

import org.springframework.stereotype.Repository;

// #region content
// #region bean
@Repository
public class BlogPostRepository {
// #endregion bean
    private final Map<String, BlogPost> blogPosts = new HashMap<>();

    public Optional<BlogPost> findByTitle(final String title) {
        if (blogPosts.containsKey(title)) {
            return Optional.of(blogPosts.get(title));
        } else {
            return Optional.empty();
        }
    }

    public BlogPost save(final BlogPost blogPost) {
        blogPosts.put(blogPost.title(), blogPost);
        return blogPost;
    }

    public Optional<BlogPost> deleteByTitle(final String title) {
        if (blogPosts.containsKey(title)) {
            return Optional.of(blogPosts.remove(title));
        } else {
            return Optional.empty();
        }
    }
}
// #endregion content
