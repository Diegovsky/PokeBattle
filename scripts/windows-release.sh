#!/bin/bash
TARGET=x86_64-pc-windows-gnu
TMP_FOLDER=target/usuc-windows

mkdir -p $TMP_FOLDER

cargo build --release --target=$TARGET

cp target/$TARGET/release/usuc.exe $TMP_FOLDER
cp windows-libs/*.dll $TMP_FOLDER
zip -rj -9 $TMP_FOLDER.zip $TMP_FOLDER
