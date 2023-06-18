# aoit-rs

Apt offline installation tool

## Requirements

- [x] Two servers, one can be networked called `A`, one **can not** be networked called `B`.
- [x] `A` server with the same `architecture` and the same `distribution` as the `b` server, and with a good network.
- [x] Both servers must have sufficient hard disk space.

## Usage

### In `A` server, pack all the dependencies of a package

We use `vim` as example.

Create a work folder.

```bash
root@debian:~# mkdir test
root@debian:~# cp aoit test/
root@debian:~# cd test
```

Start packing.

```bash
root@debian:~/test# ./aoit --pack vim
Create tmp dir success
Resolving depends: vim-common
Resolving depends: vim-runtime
Resolving depends: libacl1
Resolving depends: libc6
Resolving depends: libgpm2
Resolving depends: libselinux1
Resolving depends: libtinfo6
Saving...
Hashing...
Removing tmp dir...
Done
```

These three files will appear in the directory.

```bash
root@debian:~/test# ls
aoit  vim.aoit  vim.aoit.sha256
```

Do not change any files, including `naming` and `content`, and make sure all three files are copied to a `USB` or `CD`.

## In `B` server, offline installation of vim

Check for the presence of these three files.

```bash
root@debian:~/test# ls
aoit  vim.aoit  vim.aoit.sha256
```

Start offline installation.

```bash
root@debian:~/test# ./aoit --install vim.aoit
Checking...
Check sha256 success
Decompress aoit...
Install: libtinfo6_6.2+20201114-2+deb11u1_amd64.deb
Install: libselinux1_3.1-3_amd64.deb
Install: libgpm2_1.20.7-8_amd64.deb
Install: libc6_2.31-13+deb11u6_amd64.deb
Install: libacl1_2.2.53-10_amd64.deb
Install: vim-runtime_2%3a8.2.2434-3+deb11u1_all.deb
Install: vim-common_2%3a8.2.2434-3+deb11u1_all.deb
Install: vim_2%3a8.2.2434-3+deb11u1_amd64.deb
Removing tmp dir...
Done
```

If this process does not have any error messages, the installation is successful and you can now use the offline installed `vim`.
