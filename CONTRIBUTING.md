# Contributing

## Setup

1. Install [rust](https://www.rust-lang.org/learn/get-started)

## Getting up to date

1. rustup update

## Deployment

1. Update the version in `Cargo.toml`
1. Run `build-release.sh`
1. Create a release on github.com/brianzchen/particle with the same version ensuring to copy the artefact created `target/release/particle-mac.tar.gz`
1. Bump the url, sha256, version in the homebrew tap [repo](https://github.com/Brianzchen/homebrew-particle/blob/master/Formula/particle.rb)
