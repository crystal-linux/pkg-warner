# **THIS PROJECT HAS BEEN MIGRATED TO GITLAB**
Please make any PRs here, rather than to the GitHub: https://git.getcryst.al/crystal

# REPOSITORIES HERE ARE **OUTDATED**, GO TO THE LINK  BELOW
# ---- > https://git.getcryst.al/crystal < ----

<p align="center">
  <a href="https://github.com/crystal-linux/Malachite">
    <img src="https://getcryst.al/site/assets/other/logo.png" alt="Logo" width="150" height="150">
  </a>
</p>

<h2 align="center">Package Warner</h2>

<p align="center">
    <a href="https://github.com/crystal-linux/.github/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-blue.svg" alt="License">
    <a href="https://github/crystal-linux/malachite"><img alt="GitHub isses" src="https://img.shields.io/github/issues-raw/crystal-linux/pkg-warner"></a>
    <a href="https://github/crystal-linux/malachite"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr-raw/crystal-linux/pkg-warner"></a><br>
    <a href="https://discord.gg/hYJgu8K5aA"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"> </a>
    <a href="https://github.com/not-my-segfault"> <img src="https://img.shields.io/badge/Maintainer-@not%2D-my%2D-segfault-brightgreen" alt="The maintainer of this repository" href="https://github.com/not-my-segfault"></a><br>
    <a href="https://fosstodon.org/@crystal_linux"><img alt="Mastodon Follow" src="https://img.shields.io/mastodon/follow/108618426259408142?domain=https%3A%2F%2Ffosstodon.org">
    <a href="https://twitter.com/crystal_linux"><img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/crystal_linux"></a>
</p>



<p align="center"> <code>pkg-warner</code> is a simple and configurable package warner tool for distribution packagers.</p>


## Configuration

`pkg-warner` is configured at build time using three environment variables:
- `PKG_WARNER_PACKAGES`: a comma-separated list of **incorrect** package managers to warn about.
- `PKG_WARNER_DISTRO`: the distribution name to use in the warning.
- `PKG_WARNER_PMAN`: the **correct** package manager name to use in the warning.

Then, in the packaging process you can run `pkg-warner -id "${dest_dir}"` to copy itself to `${dest_dir}/<each value in PKG_WARNER_PACKAGES>` under the destination directory. (e.g., `./pkg/usr/bin/apt`)

If no `-d/--dest-dir` id provided, it'll install to `/usr/bin` by default.

If you want an example of how to adapt this to your own distribution/packaging process, see the provided [PKGBUILD](PKGBUILD)

## How to build:

Tested on latest Cargo (1.60.0-nightly)

### Debug/development builds

- `cargo build`

### Optimised/release builds

- `cargo build --release`
