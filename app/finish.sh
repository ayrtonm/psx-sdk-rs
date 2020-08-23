mkdir -p build
cd build
cp ../target/target/release/deps/app-*.o app.o
../../mips_toolchain/ld --gc-sections -o app.elf --script=../../psx.ld app.o
../../elf2psexe/target/release/elf2psexe NA app.elf app.psexe
