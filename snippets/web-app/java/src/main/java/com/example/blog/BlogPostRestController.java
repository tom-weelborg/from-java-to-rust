package com.example.blog;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;

// #region content
// #region bean
@RestController
@RequestMapping("blog-post")
public class BlogPostRestController {
// #endregion bean
    private final BlogPostRepository blogRepository;

    public BlogPostRestController(final BlogPostRepository blogRepository) {
        this.blogRepository = blogRepository;
    }

    // #region post
    @PostMapping
    public ResponseEntity<BlogPost> create(
        @RequestBody final BlogPost blogPost
    ) {
    // #endregion post
        return ResponseEntity
            .status(HttpStatus.CREATED)
            .body(this.blogRepository.save(blogPost));
    }
    
    // #region get
    @GetMapping("{title}")
    public ResponseEntity<BlogPost> read(
        @PathVariable final String title
    ) {
    // #endregion get
        return this.blogRepository.findByTitle(title)
            .map(ResponseEntity::ok)
            .orElse(ResponseEntity.notFound().build());
    }

    // #region put
    @PutMapping
    public ResponseEntity<BlogPost> updated(
        @RequestBody final BlogPost blogPost
    ) {
    // #endregion put
        return ResponseEntity
            .status(HttpStatus.OK)
            .body(this.blogRepository.save(blogPost));
    }
    
    // #region delete
    @DeleteMapping("{title}")
    public ResponseEntity<BlogPost> delete(
        @PathVariable final String title
    ) {
    // #endregion delete
        return this.blogRepository.deleteByTitle(title)
            .map(ResponseEntity::ok)
            .orElse(ResponseEntity.notFound().build());
    }
}
// #endregion content
