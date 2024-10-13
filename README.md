# backup.rs

## Usage

### Create or Update

    backup [-f <checksum-file>] <src-dir> <target-dir>

### Check

    backup [-f <checksum-file>] [--check] <target-dir>

### Restore

    backup [-f <checksum-file>] --restore <src-dir> <target-dir>


## backup format

```
target
+-- checksums.txt
+-- source-file
+-- source-dir
...
```
