# Example Web App

::left::

## [Java]{style="color:#ED8B00"}

<div class="flex flex-col">

  <div>

````md magic-move {at:1}
<<< @/snippets/web-app/java/pom.xml {*|12|13|15|18-21|18-21|18-21}

<<< @/snippets/web-app/java/src/main/java/com/example/blog/BlogApplication.java#content {*|1,5|1,5|1,5|1,5|1,5|1,5|1,5|1,5|1,5}

<<< @/snippets/web-app/java/run.sh
````

  </div>

  <div>

  <div class="flex gap-2 justify-between">

  <div v-click="['9', '16']">

````md magic-move {at:10}
<<< @/snippets/web-app/java/src/main/java/com/example/blog/BlogPostRepository.java#bean
````

  </div>

  <div v-click="['9', '16']">

````md magic-move {at:10}
<<< @/snippets/web-app/java/src/main/java/com/example/blog/BlogPostRestController.java#bean{*|*|*|*|*|*|2}
````

  </div>

  </div>

  <div v-click="['15', '16']" class="grid grid-cols-2 grid-rows-2 gap-1 min-w-105">

````md magic-move {at:16}
<<< @/snippets/web-app/java/src/main/java/com/example/blog/BlogPostRestController.java#post
````

````md magic-move {at:16}
<<< @/snippets/web-app/java/src/main/java/com/example/blog/BlogPostRestController.java#get
````

````md magic-move {at:16}
<<< @/snippets/web-app/java/src/main/java/com/example/blog/BlogPostRestController.java#put
````

````md magic-move {at:16}
<<< @/snippets/web-app/java/src/main/java/com/example/blog/BlogPostRestController.java#delete
````

  </div>

  </div>

</div>

::right::

## [Rust]{style="color:#D34516"}

<div class="flex flex-col">

  <div>

````md magic-move {at:1}
<<< @/snippets/web-app/rust/Cargo.toml {*|2|3|4|7|8-9|10}

<<< @/snippets/web-app/rust/src/main.rs#content {*|1|3|3|3|4|4|4|4|6-8}

<<< @/snippets/web-app/rust/run.sh
````

  </div>

  <div class="flex gap-2 justify-between">

  <div v-click="['9', '16']">

````md magic-move {at:10}
<<< @/snippets/web-app/rust/src/state.rs#content {*|1,4|1,9,12}

<<< @/snippets/web-app/rust/src/state.rs#struct
````

````md magic-move {at:10}
<<< @/snippets/web-app/rust/src/blog_post_repository.rs#struct
````

  </div>

  <div v-click="['12', '16']">

````md magic-move {at:13}
<<< @/snippets/web-app/rust/src/blog_post_router.rs#method {*|1|1-2,7|3-6}
````

  </div>

  </div>

  <div v-click="['15', '16']" class="grid grid-cols-2 grid-rows-2 gap-1">

````md magic-move {at:16}
<<< @/snippets/web-app/rust/src/blog_post_router.rs#post
````

````md magic-move {at:16}
<<< @/snippets/web-app/rust/src/blog_post_router.rs#get
````

````md magic-move {at:16}
<<< @/snippets/web-app/rust/src/blog_post_router.rs#put
````

````md magic-move {at:16}
<<< @/snippets/web-app/rust/src/blog_post_router.rs#delete
````

  </div>

</div>
