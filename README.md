# osm-is-area

An OpenSreetMap area must be a way or a relation. But not just any way or relation. 

If you are not so fortunate that all your elements are nodes, look further...

## API


### `osm_is_area::way`

According to [Overpass turbo](https://wiki.openstreetmap.org/wiki/Overpass_turbo/Polygon_Features), a way is considered an area if 
  1. It forms a closed loop
  2. It is not tagged `area=no`
  3. It conforms to one of the conditions for polygon tags.

```rust
use osm_is_area;

let tags = vec![
 (r"waterway", r"riverbank")
];
let refs = vec![1, 3, 2, 1];

let is_area = osm_is_area::way(&tags, &refs);
assert_eq!(true, is_area);
```

### `osm_is_area::relation`

A relation is an area when it has a tag "type" with value "multipolygon".
```rust
use osm_is_area;

let tags = vec![
 (r"type", r"multipolygon")
];
let members = vec![1, 3, 2, 1];

let is_area = osm_is_area::relation(&tags, &members);
assert_eq!(true, is_area);
```

## License

MIT
