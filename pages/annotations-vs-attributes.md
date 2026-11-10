# Annotations vs. Attributes

::left::

## [Java]{style="color:#ED8B00"}

````md magic-move {at:1}
<<< @/snippets/annotations-vs-attributes/java/src/main/java/com/example/App.java#deprecated{*|8|4-6|8}

<<< @/snippets/annotations-vs-attributes/java/src/main/java/com/example/App.java#usage{*|1}

<<< @/snippets/annotations-vs-attributes/java/src/main/java/com/example/App.java#equals-nolombok{*|8-14|8-14}

<<< @/snippets/annotations-vs-attributes/java/src/main/java/com/example/App.java#equals-lombok{*|2}

<<< @/snippets/annotations-vs-attributes/java/src/main/java/com/example/App.java#stringify{*|2,22}

<<< @/snippets/annotations-vs-attributes/java/src/main/java/com/example/App.java#copy{*|none|none}

<<< @/snippets/annotations-vs-attributes/java/src/main/java/com/example/App.java#clone{*|6-12,20|20-23}
````

<div v-click="['4', '6']">

````md magic-move {at:5}
<<< @/snippets/annotations-vs-attributes/java/src/main/java/com/example/App.java#definition{*|2}
````

</div>

<a href="https://projectlombok.org/"
   target="_blank"
   class="text-xs ml-10"
   v-click="['9', '13']">
  Comparable to Project Lombok
</a>

::right::

## [Rust]{style="color:#D34516"}

````md magic-move {at:1}
<<< @/snippets/annotations-vs-attributes/rust/src/main.rs#deprecated{*|4|4-6|8}

<<< @/snippets/annotations-vs-attributes/rust/src/main.rs#usage{*|1}

<<< @/snippets/annotations-vs-attributes/rust/src/main.rs#equals-noderive{*|8-15|6-18}

<<< @/snippets/annotations-vs-attributes/rust/src/main.rs#equals-derive{*|2}

<<< @/snippets/annotations-vs-attributes/rust/src/main.rs#stringify{*|1-2,8,10,22-24}

<<< @/snippets/annotations-vs-attributes/rust/src/main.rs#copy{*|1,20|20-23}

<<< @/snippets/annotations-vs-attributes/rust/src/main.rs#clone{*|1,20|20-23}
````

<div v-click="['4', '6']">

````md magic-move {at:5}
<<< @/snippets/annotations-vs-attributes/rust/src/lib.rs#definition{*|2}
````

</div>
