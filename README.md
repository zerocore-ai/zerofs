<div align="center">
  <!-- <a href="https://github.com/zerocore-ai/zerofs" target="_blank">
    <img src="https://raw.githubusercontent.com/zerocore-ai/zerofs/main/assets/logo.png" alt="zerofs Logo" width="100"></img>
  </a> -->

  <h1 align="center">zerofs</h1>

  <!-- <p>
    <a href="https://crates.io/crates/zerofs">
      <img src="https://img.shields.io/crates/v/zerofs?label=crates" alt="Crate">
    </a>
    <a href="https://codecov.io/gh/zerocore-ai/zerofs">
      <img src="https://codecov.io/gh/zerocore-ai/zerofs/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/zerocore-ai/zerofs/actions?query=">
      <img src="https://github.com/zerocore-ai/zerofs/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/zerocore-ai/zerofs/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>
    <a href="https://docs.rs/zerofs">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
  </p> -->
</div>

**`zerofs`** is a secure distributed content-addressable file system.

### Key Features

This project shares the same [core philosophies][key-features] as zerocore, and in addition to that, it also has these key features:

#### Content Addressable

Data in zerofs is stored based on its content, using a unique cryptographic hash. This content-addressable storage (CAS) ensures data integrity and immutability, facilitating efficient deduplication and integrity checks.

#### Versioning

zerofs features robust data versioning where each modification creates a new immutable version of the data, linked through hashes. This design allows for full historical traceability and simple rollback capabilities.

</br>

> [!WARNING]
> This project is in early development and is not yet ready for production use.

##

## Outline

- [Testing the Project](#testing-the-project)
- [License](#license)

## Testing the Project

- Run tests

  ```console
  cargo test
  ```

## License

This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].

[apache]: https://www.apache.org/licenses/LICENSE-2.0
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[irust]: https://github.com/sigmaSd/IRust
[pre-commit]: https://pre-commit.com/
[distributed]: https://en.wikipedia.org/wiki/Distributed_computing
[multi_tenant]: https://en.wikipedia.org/wiki/Multitenancy
