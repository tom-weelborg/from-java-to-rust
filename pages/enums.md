# Enums

::left::

<h2>
  <v-switch>
    <template #0-4>
      <span style="color:#ED8B00">
        Java
      </span>
    </template>
    <template #4-8>
      <span style="color:#3178C6">
        TypeScript
      </span>
    </template>
  </v-switch>
</h2>

<div class="flex gap-1">

  <div>

````md magic-move {at:1}
<<< @/snippets/enums/java/Enums.java#compile-time

<<< @/snippets/enums/java/Enums.java#runtime

<<< @/snippets/enums/java/Enums.java#result-types

<<< @/snippets/enums/java/Enums.java#result-types

<<< @/snippets/enums/typescript/typeUnion.ts#union

<<< @/snippets/enums/typescript/typeUnion.ts#alias

<<< @/snippets/enums/typescript/typeUnion.ts#alias

<<< @/snippets/enums/typescript/typeUnion.ts#alias
````

  </div>

  <div v-click="['2', '+1']">

````md magic-move {at:2}
```
```

<<< @/snippets/enums/java/Enums.java#result-usage

```
```
````

  </div>

</div>

::right::

## [Rust]{style="color:#D34516"}

<div class="flex gap-1">

  <div>

````md magic-move {at:1}
<<< @/snippets/enums/rust/enums.rs#enum

<<< @/snippets/enums/rust/enums.rs#enum

<<< @/snippets/enums/rust/enums.rs#result-types

<<< @/snippets/enums/rust/enums.rs#result-types

<<< @/snippets/enums/rust/type_union.rs#union

<<< @/snippets/enums/rust/type_union.rs#alias-bad

<<< @/snippets/enums/rust/type_union.rs#alias-still-bad

<<< @/snippets/enums/rust/type_union.rs#alias-good
````

  </div>

  <div v-click="['3', '+1']">

````md magic-move {at:3}
```
```

<<< @/snippets/enums/rust/enums.rs#result-usage

```
```
````

  </div>

</div>
