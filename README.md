[![crates.io](https://img.shields.io/crates/v/read-url?color=%23227700)](https://crates.io/crates/read-url)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/read-url/latest/read_url/)

read-url
========

Go beyond `http://` with this streamlined URL library for Rust.

read-url gets you an `io::Read` or a `tokio::io::AsyncRead` from a wide variety of URL types, including entries in archives and code repositories using a URL notation inspired by Java's [JarURLConnection](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/net/JarURLConnection.html).

We strongly recommend starting with the [examples](https://github.com/tliron/rust-read-url/tree/main/library/examples), specifically `start_here.rs`. They are designed to give you a tour of the API and its features.

```rust
use read_url::*;

let context = UrlContext::new();
let url = context.url("http://localhost:8000/hello.txt")?;
// or something like: "tar:http://localhost:8000/text.tar.gz!hello.txt"
let mut reader = url.open()?; // io::Read
let mut string = String::default();
reader.read_to_string(&mut string)?;
println!("{}", string);
```

Rationale and Features
----------------------

1) Do you have a program that needs a file as input? If there is no strong reason for the file to be local, then read-url allows the user to provide it as either a URL or a local path. Don't force the user to download or copy the file to their local filesystem first. Indeed, they might not be able to do so in some environments.

2) Does that file reference other files relative to its location? For example, a source code file might need to "import" another file that is located next to it or in a subdirectory. Relative paths can be tricky to resolve if the file's location is remote or inside an archive (or inside a *remote* archive). This complexity is one reason why programs often insist on only supporting local files, where relative paths are handled natively. Read-url provides relative path resolution with optimized access for all its supported URL schemes (even in remote archives), thus removing a major obstacle to accepting URLs as inputs.

3) Read-url supports an `internal:` URL scheme, allowing you to mix externally-provided data with data that you provision. Both data types live under a single, unified URL namespace and are accessible by a single API. Relatedly, read-url allows you to override any URL, such that external data can be overridden to use internally provisioned data, which is useful for testing or as a fallback in constrained environments.

CLI Tool
--------

Also included in this repository is a simple CLI tool for reading from URLs, mainly intended for testing if and how using read-url in your code would work on specific URLs. Both blocking and asynchronous code paths are supported by this tool.

It's not intended as a replacement for `curl` or `wget`, though it can do some things that they can't, e.g. reading entries from remote archives.

To install the CLI tool:

```sh
cargo install read-url-cli
```

Crate Features
--------------

By default all URL types are enabled, however you *must* also explicitly enable `blocking`, `async`, or both. Or set [`default-features = false`](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features) and pick and choose which API and URL types you want supported.

Supported URL Types
-------------------

### `http:` and `https:`

Classic. We internally rely on [reqwest](https://github.com/seanmonstar/reqwest) for these.

### `file:`

Represents an absolute path to the local filesystem.

The URL representation must begin with two slashes. If a host is present before the path it will be stored but not otherwise used by read-url, so this:

    file://localhost/the/path

is equivalent to this path:

    /the/path

Because the path must be absolute, it always begins with a slash. The consequence is that `file:` URLs commonly begin with 3 slashes:

    file:///the/path

When compiled for Windows the URL path will be converted to a Windows path. The convention is that backslashes become slashes and a first slash is added to make it absolute. So this URL:

    file:///C:/Windows/win.ini

is equivalent to this Windows path:

    C:\Windows\win.ini

Note that for security reasons relative file URLs *are not* automatically searched against the current working directory by default. If you do want to support the working directory then call `working_dir_url()` and add it explicitly to your base URLs.

It is often desirable to accept input that is *either* a URL *or* a file path. For this use case we provide the `url_or_file_path()` API. If the argument is not a parsable URL it will be treated as a file path and a `FileUrl` will be returned. With this API, users would usually not see `file:` URLs directly.

Note that the design of this API may trip over a rare edge case for Windows. If there happens to be a drive that has the same name as a supported URL scheme, e.g. "http", then callers would have to provide a full file URL, otherwise it would be parsed as a URL of that scheme. E.g. imagine you have a Windows drive named "http". Thus instead of this Windows file path:

    http:\Dir\file

you *must* use the full `file:` URL:

    file:///http:/Dir/file

### `tar:`

Entries in tarballs, with or without compression. Examples:

    tar:https://mysite.org/cloud.tar.gz!path/to/main.yaml
    tar:file:///local/path/cloud.tar!path/to/main.yaml

The archive URL (before the `!`) can be any other read-url URL. Indeed it's technically possible to nest archive URLs inside each other for a tasty tarball sandwich:

    tar:tar:mysite.org/cloud.tar.gz!inner.tar!path/to/main.yaml

When using `url_or_file_path()` the archive URL can be a local path, so this would work with that API:

    tar:local/relative/path/cloud.tar.gz!path/to/main.yaml

Decompression is automatically detected depending on the path suffix. `.tar.gz`, `.tgz`, `.tar.zstd`, and `.tar.zst` are supported. Sure, there are many other algorithms in use, but we decided to include 1) the most common, and 2) the most wanted. You can also specify the decompression algorithm explicitly using the URL fragment:

    tar:/absolute/path/cloud#gzip!path/to/main.yaml
    tar:relative/path/cloud#zstd!path/to/main.yaml

Note that file paths (with `url_or_file_path()`) do not have fragments, so if you need this feature just use a `file:` URL instead.

Tarballs are serial containers, naturally optimized for streaming, meaning that unwanted entries are skipped until our entry is found, and then subsequent entries are ignored. This means that when accessing tarballs over the network the tarball does not have to be downloaded in its entirety, unlike with ZIP (see below).

### `zip:`

Entries in ZIP files. Example:

    zip:http://mysite.org/cloud.zip!path/to/main.yaml

Note that ZIP files require random file access and thus *must* be fully accessible. Consequently for remote zips the entire archive will be downloaded in order to access a single entry. Read-url will optimize by downloading the archive only once per context. Other URLs referring to the archive will use the existing download.

If you have a choice of compression technologies and want efficient support for remote access then you should prefer tarballs to ZIP files.

### `git:`

Files in git repositories. Note that the repository URL is not a read-url URL, but rather must be a URL representation supported by [gitoxide's gix](https://github.com/GitoxideLabs/gitoxide). Example:

    git:https://github.com/tliron/rust-read-url.git!crates/library/Cargo.toml

The default is to fetch the tip (HEAD) of the default branch. However, you can also specify a reference via the URL fragment, either a git tag, a branch name (will fetch the tip), or a commit hash in hex. Example of a branch name:

    git:https://github.com/tliron/rust-read-url.git#main!crates/library/Cargo.toml

Because we are only interested in reading one file, not otherwise working with the git repository, read-url will do nothing more than the the bare minimum required, i.e. a bare (not checked out) shallow clone of just the specified reference. Read-url will also optimize by cloning the archive only once per context. Other URLs referring to the repository will use the existing clone.

### `internal:`

Internal URL content can be stored either in a context (`register_internal_url()`) or globally (`register_global_internal_url()`). All contexts can access the global content. URLs try to use their context's registry first, so that it can be used to override the globals.

### Mock URLs

These are intended to be used for testing. They must be created explicitly via `mock_url()` and are not created by parsing an input. They can thus use any scheme, indeed any notation, with any content.

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/rust-read-url/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/rust-read-url/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
