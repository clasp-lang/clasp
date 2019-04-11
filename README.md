# Clasp 🔗

[![Build Status][build-image]][build-link]
[![Apache 2.0 licensed][license-image]][license-link]

A declarative logic-based programming language for cryptographically
authenticating and verifying structured data.

Inspired by [Datalog]-like authorization languages such as [Binder], policy
languages like [OpenPolicyAgent] and logic-based credential formats like
[Macaroons] and [Vanadium].

## Status

Vaporware

## Requirements

-  Rust 1.31+: https://rustup.rs/ 

## Installation

```
$ cargo install clasp
```

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

Copyright © 2019 Tony Arcieri

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[build-image]: https://secure.travis-ci.org/clasp-lang/clasp.svg?branch=develop
[build-link]: https://travis-ci.org/clasp-lang/clasp
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/clasp-lang/clasp/blob/develop/LICENSE
[Datalog]: https://en.wikipedia.org/wiki/Datalog
[Binder]: https://users.soe.ucsc.edu/~abadi/Papers/lics2003.pdf
[OpenPolicyAgent]: https://www.openpolicyagent.org/
[Macaroons]: https://ai.google/research/pubs/pub41892
[Vanadium]: https://arxiv.org/abs/1607.02192
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/clasp-lang/clasp/blob/develop/CODE_OF_CONDUCT.md
