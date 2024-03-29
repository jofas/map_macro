# CHANGELOG

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html>).


## [0.3.0]

### Added

* `hashbrown::hash_map` macro

* `hashbrown::hash_map_e` macro

* `hashbrown::hash_set` macro

* `hashbrown::hash_set_e` macro

* `binary_heap_e` macro

* `btree_set_e` macro

* `hash_set_e` macro

* `vec_no_clone_e` macro

### Changed

* Explicitly typed map macros: keys also cast now (before only values were 
  casted)

### Removed

* `map` macro

* `map_e` macro

* `set` macro


## [0.2.6]

### Added

* `hash_map` macro

* `hash_map_e` macro

* `hash_set` macro

* `binary_heap` macro

* `vec_deque` macro

* `vec_deque_e` macro

* `linked_list` macro

* `linked_list_e` macro

### Deprecated

* `map` macro

* `map_e` macro

* `set` macro


## [0.2.5]

### Added

* `btree_map_e` macro

* `map_e` macro


## [0.2.4]

### Added

* `btree_map` macro

* `btree_set` macro

### Changed

* Enhanced performance of `vec_no_clone` macro when used with a
  list of elements


## [0.2.3]

### Changed

* Better allocation performace for all collections


## [0.2.2]

### Added

* `vec_no_clone` macro


## [0.2.1]

### Added

* Documentation


## [0.2.0]

### Added

* `set!` macro


## [0.1.0]

### Added

* `map!` macro
