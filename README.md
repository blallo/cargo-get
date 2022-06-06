# `cargo-get`
## A project to handle rust dependecies go-like

> **DISCLAIMER**
>
> As of now, this project is just this readme, and nothing more. To implement it, a lot
> of work should be put into it, and I am not sure I am willing to do it.
> Feel free to take this idea and implement/criticize/improve/deconstruct/whatever it.

The idea underpinning this project is that the way dependencies are handled in `cargo`
is [flawed][blog]. Looking around for alternatives, I was drawn towards one that sounds
quite robust to me, the way of `go`.

At the beginning, every go package was an url. The `go` tool treats them as
subdirectories of a repository. This, although very clever and much approximating the
ideal decentralization utopia, is incredibly prone to attacks, as any remote source
might be taken over, with all the consequences a by-the-book supply chain attack might
entail.

The second iteration of this is given by go modules: packages are still
identifiable by urls, but the origin of these is enclosed in a single entity, identified
by a `go.sum` file at its root. This is used to compute, per each version, a checksum of
the content and this hash is stored by some sort of registry (`proxy.golang.org` being
the default one). This is less ideal, but far more secure.

The idea of this project is to offer two pieces to reproduce the above structure:

 1. a binary `cargo-get` that should be in charge of adding a dependency to a project
    `Cargo.toml` given a url and a commit hash/tag/version number, also resolving all
    its dependencies in terms of other urls
 2. a server that should be used to cache the checksums of each version

By default, `cargo-get` should not use any server and resolve every time all the
dependency tree. If provided with a server url, it should be used to speed up dependency
resolution (and also resolve cases in which is not possible to find a dependency in
terms of a url/repository).

[blog]: https://troubles.noblogs.org/post/2021/03/29/why-so-much-ado-with-crates-io/
