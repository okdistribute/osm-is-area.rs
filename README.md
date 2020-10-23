# osm-is-area

Tell if an OpenStreetMap element is an area or not. 

First, decide if your element is a way or a relation, then pick the function
that is best for you.

## Example

```rust
use osmisarea;

let end = 1252234;
let refs = vec![end, 23452234, 28373423, end];
let tags = vec![
    (r"waterway", r"riverbank")
];

osmisarea::way(&tags, &refs);
// or osmisarea::relation(...)
```

## license

MIT
