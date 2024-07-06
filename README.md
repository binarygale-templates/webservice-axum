# webservice-axum

This is a template project for a webservice using `axum` with some bells and whistles.

## Features

- `anyhow` and `thiserror` for a solid error handling motivation.
- `axum` as the core, with a modular router setup.
- `clap` to allow configuration via CLI flags and environment variables.
- `tracing`, with configurable log verbosity levels, and log formats (plain text, colored plain text, JSON).

## Routes

- `/livez` and `/readyz` as always-passing implementations for further implementation.
- `/versionz` that shows the package version and a `git describe` info.

## Container image

The `Dockerfile` is Alpine based and builds the app and runs it as a non-root user. The Group ID and User ID are fixed to 10001. It contains a healthcheck with default parameters that automatically queries `/readyz`.

## GitHub Actions

Three workflows are included:

- The `build` workflow builds all pushes to the `main` branch. It outputs a static binary build, and it pushes a build of the container to the `:develop` tag to GHCR.
- The `test` workflow runs on all pushes to the `main` branch, and on several PR actions. It runs a build, and then follows that with `cargo test`, `cargo clippy`, and `cargo fmt`.
- The `release` workflow runs when a new GitHub Release is pushed. It builds the container image and pushes the result to GHCR under the `latest` tag, and semver-hierarchical version tags.

The `build` and `release` workflows can also be triggered manually, in case a previous release needs to be build again to catch up with OS-level updates.

## Linting

The template contains a very basic `.editorconfig`, and a config file for [`pre-commit`](https://pre-commit.com/) that runs Clippy and Rustfmt in a `pre-commit` hook. Be sure to run `pre-commit install` in new clones to set up the git hook.
