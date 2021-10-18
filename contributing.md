# Contributing to Rusty50

## Bug reports

We can't fix what we don't know about, so please report problems liberally. This
includes problems with understanding the documentation, unhelpful error messages,
and unexpected behavior.

Opening an issue is as easy as following [this link][new-issues] and filling out
the fields. Here's a template that you can use to file an issue, though it's not
necessary to use it exactly:

    <short summary of the problem>

    I tried this: <minimal example that causes the problem>

    I expected to see this happen: <explanation>

    Instead, this happened: <explanation>

    I'm using <output of `Rusty50 --version`>

All three components are important: what you did, what you expected, what
happened instead. Please use https://gist.github.com/ if your examples run long.

## Feature requests

Please feel free to open an issue or to send a pr.

## Working on issues


Feel free to ask for guidelines on how to tackle a problem or open a
[new issue][new-issues]. This is especially important if you want to add new
features to Rusty50 or make large changes to the already existing code-base.
Rusty50's core developers will do their best to provide help.

If you start working on an already-filed issue, post a comment on this issue to
let people know that somebody is working it. Feel free to ask for comments if
you are unsure about the solution you would like to submit.

We use the "fork and pull" model [described here][development-models], where
contributors push changes to their personal fork and create pull requests to
bring those changes into the source repository. This process is partly
automated: Pull requests are made against Rusty50's master-branch, tested and
reviewed. Once a change is approved to be merged, a friendly bot merges the
changes into an internal branch, runs the full test-suite on that branch
and only then merges into master. This ensures that Rusty50's master branch
passes the test-suite at all times.

Your basic steps to get going:

-   Fork Rusty50 and create a branch from master for the issue you are working on.
-   Please adhere to the code style that you see around the location you are
    working on.
-   [Commit as you go][githelp].
-   Include tests that cover all non-trivial code. The existing tests
    in `test/` provide templates on how to test Rusty50's behavior in a
    sandbox-environment. The internal crate `testing` provides a vast amount
    of helpers to minimize boilerplate. Check this link out [`testing`] for an
    introduction to writing tests.
-   Make sure `cargo test` passes.
-   All code changes are expected to comply with the formatting suggested by `rustfmt`.
    You can use `rustup component add --toolchain nightly rustfmt-preview` to install `rustfmt` and use
    `rustfmt +nightly --unstable-features --skip-children` on the changed files to automatically format your code.
-   Push your commits to GitHub and create a pull request against Rusty50's `master` branch.

## Getting your development environment set up

After cloning the project there are a few steps required to get the project running.

1.  Run cargo build.

    ```bash
    cargo build
    ```

2.  Run tests.

    ```bash
    cargo test
    ```

3.  Run the project.

    ```bash
    cargo run
    ```

## Pull requests

After the pull request is made, one of the Rusty50's project developers will review your code.
The review-process will make sure that the proposed changes are sound.
Please give the assigned reviewer sufficient time, especially during weekends.
If you don't get a reply, you may poke the core developers on [discord].

A merge of Rusty50's master-branch and your changes is immediately queued
to be tested after the pull request is made. In case unforeseen
problems are discovered during this step (e.g. a failure on a platform you
originally did not develop on), you may ask for guidance. Push additional
commits to your branch to tackle these problems.

The reviewer might point out changes deemed necessary. Please add them as
extra commits; this ensures that the reviewer can see what has changed since
the code was previously reviewed. Large or tricky changes may require several
passes of review and changes.

Once the reviewer approves your pull request, a friendly bot picks it up
and merges it into Rusty50's `master` branch.

## Contributing to the documentation

TODO

## Issue Triage

TODO

## Contact

Feel free to contact the Rusty50 channel on [discord] regarding any questions or clarifications you might have.
We will get back to you as soon as possible.

TODO

[gist]: https://gist.github.com/
[new-issues]: https://github.com/shashank-07/Rusty50/issues/new
[code of conduct]: https://www.rust-lang.org/conduct.html
[`testing`]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[irlo]: https://internals.rust-lang.org/
[subcommands]: https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands
[discord]: https://dwoc.io/project/616bcadf22fc410012e9a5d9