#!/bin/bash

cross build --release --target x86_64-unknown-linux-musl
cross build --release --target i586-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-musl

rm aoit-x86_64-unknown-linux-musl.7z
rm aoit-aarch64-unknown-linux-musl.7z
rm aoit-i586-unknown-linux-musl.7z

7z a -mx9 aoit-x86_64-unknown-linux-musl.7z ./target/x86_64-unknown-linux-musl/release/aoit
7z a -mx9 aoit-aarch64-unknown-linux-musl.7z ./target/aarch64-unknown-linux-musl/release/aoit
7z a -mx9 aoit-i586-unknown-linux-musl.7z ./target/i586-unknown-linux-musl/release/aoit