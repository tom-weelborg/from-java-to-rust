# String

::left::

## [Java]{style="color:#ED8B00"}

<div class="flex gap-2">

  ````md magic-move {at:1}
  ```java
  // String class
  // that is an object-based
  // reference type
  public final class String {
    ...
  }
  ```
  ````

  <v-click at="2">

  ````md magic-move {at:3}
  <<< @/snippets/string/java/StringExample.java#a
  
  <<< @/snippets/string/java/StringExample.java#b
  
  <<< @/snippets/string/java/StringExample.java#concatenation
  ````

  </v-click>

</div>

::right::

## [Rust]{style="color:#D34516"}

<div class="flex gap-2">

  ````md magic-move {at:1}
  ```rust
  // String struct
  // that is an
  // instance-based type
  pub struct String {
    ...
  }
  ```
  
  ```rust
  // String struct
  // that is an
  // instance-based type
  pub struct String {
    ...
  }

  // String slice
  // that is a reference
  // to a valid UTF-8
  // character sequence
  &str
  ```
  ````

  <v-click at="2">

  ````md magic-move {at:3}
  <<< @/snippets/string/rust/string_example.rs#a
  
  <<< @/snippets/string/rust/string_example.rs#b
  
  <<< @/snippets/string/rust/string_example.rs#concatenation
  ````

  </v-click>

</div>
