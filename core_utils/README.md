# core_utils
* `src/count_files` - a fast and parallelised recursive file counter.
```Rust
Usage:
count_files <threads> <directory>
Returns:
directory_path  date-time file_count
```

* `src/dstats` - a fast and parallelised function to return total size (in human-readable bytes) of a directory as well as the number of subdirectories and total file count (using the same implementation as in `count_files`).
```Rust
Usage:
dstats <threads> <directory>
Returns:
directory_size  number_of_subdirs file_count 
```

* `src/dsinit` - a data science project initialiser.
```Rust
Usage:
dsinit <project_name> <author>
```