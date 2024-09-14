# aoit-rs

Apt offline installation tool (support `Debian` family distribution only).

[![Rust](https://github.com/rikonaka/aoit-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rikonaka/translator-rs/actions/workflows/rust.yml)

## Requirements

- [x] Two servers, one can be networked called A, one **can not** be networked called B.
- [x] A server with the same `architecture` and the same `distribution` as the B server, and with a good network.
- [x] Both servers must have sufficient hard disk space.

## Usage

### In A server, pack all the dependencies of a package

We use `vim` package as example.

Create a work folder.

```bash
root@debian:~# mkdir test
root@debian:~# cp aoit test/
root@debian:~# cd test
```

Start packing.

```bash
root@debian:~/test# ./aoit --pack vim
```

These three files will appear in the directory.

```bash
root@debian:~/test# ls
aoit  vim.aoit  vim.aoit.sha256
```

Do not change any files, including `naming` and `content`, and make sure all three files are copied to a `USB` or `CD`.

### In B server, offline installation of vim

Check for the presence of these three files.

```bash
root@debian:~/test# ls
aoit  vim.aoit  vim.aoit.sha256
```

Start offline installation.

```bash
root@debian:~/test# ./aoit --install vim.aoit
```

Fixing apt dependencies

```bash
apt install -f
```

If this process does not have any error messages, the installation is successful and you can now use the offline installed `vim`.
