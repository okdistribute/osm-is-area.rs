# osm-is-area

```rust
use osmisarea;

let end = 1252234;
let refs = vec![end, 23452234, 28373423, end];
let tags = vec![
    (r"waterway", r"riverbank")
];

osmisarea::way(&tags, &refs);
```

## license

MIT
