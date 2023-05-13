# bioinformatics_utils
* `src/barcode_cells` - Generates a unique 'barcode' of a given length from a defined dictionary for each unique string from a tsv-file input. This tool was initially built to generate sets of barcodes for single-cell genomic bam files.
```Rust
Usage:
barcode_cells <cell-names> --<dictionary> --<lentgth>

Returns:
cell-name  barcode
```