# bioinformatics_utils
* `src/barcode_files` - Generates a unique 'barcode' of length $barcode-length from a defined $barcode-alphabet for each matching $prefix-pattern from files in a directory. This tool was initially built to generate sets of barcodes for single-cell-genomic bam files.
```Rust
Usage:
barcode_files <directory> <prefix-pattern> <barcode-length> <barcode-alphabet>
Returns:
matching_prefix  barcode
```