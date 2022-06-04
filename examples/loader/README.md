```
# This demo's build.rs builds the ferris demo
cargo psx build
mkpsxiso mkpsxiso.xml
pcsx-redux -stdout -run -iso loader.bin
```
