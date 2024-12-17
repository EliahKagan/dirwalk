# dirwalk - Show how `gix::dirwalk` classifies entries

This program lists entries in a Git repository, found in a gitoxide `gix::dirwalk` traversal.

The main goal of this program is to demonstrate what kinds of filesystem entries are found and how they are classified. This is not very configurable, at least so far: it uses a fixed set of `dirwalk::Options` that refrains from recursing into nested repositories (and that does not allow empty globs to match) but otherwise is fairly maximal. Currently this program supports no command-line options and always treats the current directory as the repository to examine.

Glob patterns may be given to adjust which items should be selected (i.e. not pruned). Note that pruned items are still listed, since the purpose of this tool is to show classification, not to produce limited output.

I'm using this to look into [GitoxideLabs/gitoxide#1629](https://github.com/GitoxideLabs/gitoxide/pull/1629). The options are chosen to help with that and may change. I'm not sure how useful this little program is, in general. `ein` and `gix` (which are provided by `gitoxide`) and the `gitoxide` examples are likely to be more useful.

## Usage

```sh
dirwalk [patterns...]
```

## Example

To investigate [GitoxideLabs/gitoxide#1629](https://github.com/GitoxideLabs/gitoxide/pull/1629) (see [acknowledgements](#acknowledgements) below), I used this on an Arch Linux system, within its own repository, first by adding a regular file `a` and a FIFO (named pipe) `b`:

```sh
touch a
mkfifo b
```

Running `ls -lF` shows the files in the top-level directory on disk after those operations:

```text
$ ls -lF
total 68
-rw-r--r-- 1 ek ek     0 Dec 17 09:32 a
prw-r--r-- 1 ek ek     0 Dec 17 09:32 b|
-rw-r--r-- 1 ek ek 45997 Dec 11 13:37 Cargo.lock
-rw-r--r-- 1 ek ek   127 Dec 11 13:37 Cargo.toml
-rw-r--r-- 1 ek ek   664 Dec 17 09:31 LICENSE
-rw-r--r-- 1 ek ek  3815 Dec 17 09:43 README.md
drwxr-xr-x 2 ek ek  4096 Dec 11 12:59 src/
drwxr-xr-x 4 ek ek  4096 Dec 12 14:07 target/
```

Then, first I ran this program with no arguments:

```sh
cargo run
```

That produced this output, confirming the key element of [GitoxideLabs/gitoxide#1629](https://github.com/GitoxideLabs/gitoxide/pull/1629) that `gitoxide` treats non-`.gitignore`d FIFOs as untracked files, the same as if they were regular files:

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/dirwalk`
          Untracked    a
            Tracked    src/main.rs
            Tracked    .gitignore
            Tracked    README.md
             Pruned    .git
            Tracked    Cargo.toml
            Tracked    Cargo.lock
Ignored(Expendable)    target
          Untracked    b
            Tracked    LICENSE
```

The individual contents of the `.gitignore`d file `target` can be shown by running:

```sh
cargo run -- '*'
```

(The `--` is optional but a good idea to ensure the arguments are not interpreted as being for `cargo run` itself, as well as to make clear that this is not happening.)

That lists a lot of output and is thus not shown here.

We can select only some files and see how the status of others is affected. The associated semantics would depend on the specific operation: in many applications, pruned files would not be listed or otherwise operated on, but in some applications, such as cleaning (see `gix clean`), they would be the files selected.

```text
$ cargo run -- a
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/dirwalk a`
Untracked    a
   Pruned    src
   Pruned    .gitignore
   Pruned    README.md
   Pruned    .git
   Pruned    Cargo.toml
   Pruned    Cargo.lock
   Pruned    target
   Pruned    b
   Pruned    LICENSE

$ cargo run -- b
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/dirwalk b`
   Pruned    a
   Pruned    src
   Pruned    .gitignore
   Pruned    README.md
   Pruned    .git
   Pruned    Cargo.toml
   Pruned    Cargo.lock
   Pruned    target
Untracked    b
   Pruned    LICENSE
```

This program can of course be installed (`cargo install --path .`) and then easily run from anywhere.

## License

[0BSD](LICENSE)

## Acknowledgements

The motivation and interesting example here is how `gitoxide`, as of this writing, does not ignore FIFOs as `git` does. This [was discovered](https://github.com/GitoxideLabs/gitoxide/pull/1629) by [**@krobelus**](https://github.com/krobelus).
