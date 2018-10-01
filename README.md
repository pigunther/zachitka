https://exonum.com/doc/get-started/install/
Для фронта у них своя js библиотека, займусь этим потом. Пока смотрю/смотри пример
1. Делаешь как в ссылке
2. Устанавливаешь карго (https://www.rust-lang.org/en-US/install.html)
3. скачиваешь проект
4. Заходишь в examples/cryptocurrency или типа того
5. https://exonum.com/doc/get-started/create-service/ посмотри, почитай
6. cargo run --example demo
7. Далее в ссылке выше есть curl … типа отправить пут запрос на создания кошелька 
8. Потом curl post (типа того) смотришь что вышло
9. Очень сложно. Мда


Про Rust пока читала тут https://habr.com/company/bitfury/blog/342208/



# Exonum

**Status:**
[![Travis Build Status](https://img.shields.io/travis/exonum/exonum/master.svg?label=Linux)](https://travis-ci.com/exonum/exonum)
[![CircleCI Build Status](https://img.shields.io/circleci/project/github/exonum/exonum/master.svg?label=MacOS)](https://circleci.com/gh/exonum/exonum/tree/master)
[![Appveyor Build Status](https://img.shields.io/appveyor/ci/exonum-org/exonum/master.svg?label=Windows)](https://ci.appveyor.com/project/exonum-org/exonum)
[![dependency status](https://deps.rs/repo/github/exonum/exonum/status.svg)](https://deps.rs/repo/github/exonum/exonum)

**Project info:**
[![Docs.rs](https://docs.rs/exonum/badge.svg)](https://docs.rs/exonum)
[![License: Apache-2.0](https://img.shields.io/github/license/exonum/exonum.svg)](LICENSE.md)
[![LoC](https://tokei.rs/b1/github/exonum/exonum)](https://github.com/exonum/exonum)
![rust 1.27+ required](https://img.shields.io/badge/rust-1.27+-blue.svg?label=Required%20Rust)

**Community:**
[![Join the chat at https://gitter.im/exonum/exonum](https://img.shields.io/gitter/room/exonum/exonum.svg?label=Chat)](https://gitter.im/exonum/exonum)
[![Join the chat at https://t.me/exonum_blockchain](https://img.shields.io/badge/Chat-on%20telegram-brightgreen.svg)](https://t.me/exonum_blockchain)
[![Join the chat at https://gitter.im/exonum/ruExonum](https://img.shields.io/gitter/room/exonum/ruExonum.svg?label=Russian%20chat)](https://gitter.im/exonum/ruExonum)
[![Join the chat at https://t.me/ExonumRU](https://img.shields.io/badge/Russian%20chat-on%20telegram-brightgreen.svg)](https://t.me/ExonumRU)
[![Website](https://img.shields.io/website/http/exonum.com.svg?label=Website)](https://exonum.com)

[Exonum](https://exonum.com/) is an extensible open-source framework for
creating blockchain applications. Exonum can be used to create cryptographically
powered distributed ledgers in virtually any problem domain, including FinTech,
GovTech, and LegalTech. The Exonum framework is oriented towards creating
permissioned blockchains, that is, blockchains with the known set of blockchain
infrastructure providers.

This is the main Exonum repository that includes

* [Exonum core library](exonum/README.md).
* [Exonum testing framework](testkit/README.md).
* Services:
  * [Configuration service](services/configuration/README.md).
  * [Time service](services/time/README.md).
* Examples
  * [Cryptocurrency](examples/cryptocurrency/README.md).
  * [Cryptocurrency-advanced](examples/cryptocurrency-advanced/README.md).
  * [Timestamping](examples/timestamping/README.md).

See individual projects readme for the details.

If you are using Exonum in your project and want to be listed on our website &
GitHub list — write us a line to <exonum@bitfury.com>.

## Other languages support

* [Java](https://github.com/exonum/exonum-java-binding)
