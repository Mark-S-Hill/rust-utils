# nextflow_utils
* `src/gpwd` - searches a nextflow log for a pattern within process names and returns lines which match the search patten, displaying
the work directories, the status (cached or submitted) and the full process name.
```Rust
Usage:
gpwd <"pattern"> </path/to/.nextflow.log>
Returns:
workDir  status process_name
```

* `src/nferr` - returns processes with non-zero exit statuses from a nextflow log, displays the process name, the returned exit status and the work directory for those processes.
```Rust
Usage:
nferr </path/to/.nextflow.log>
Returns:
process_name  exit_status workDir
```