# Build instructions

```
# This demo's build.rs builds the ferris demo as well
cargo psx build
mkpsxiso mkpsxiso.xml
duckstation-qt -fastboot `realpath loader.bin`
```
