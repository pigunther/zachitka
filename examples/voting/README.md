Кааароче
Делаешь по пунктам)
какие то пояснения вписала
у меня этот пример заработал
надо в нем разобраться
че нибудь переписать и в продакшн



# Voting demo

This project demonstrates how to bootstrap own voting service
with [Exonum blockchain](https://github.com/exonum/exonum).

![Voting demo](Screenshot.png)

Exonum blockchain keeps balances of users and handles secure
transactions between them.

It implements most basic operations:

- Create a new user
- Add funds to the user's balance
- Transfer funds between users

## Install and run

### Manually

#### Getting started

Be sure you installed necessary packages:

- [git](https://git-scm.com/downloads)
- [Node.js with npm](https://nodejs.org/en/download/)
- [Rust compiler](https://rustup.rs/)

#### Install and run

Below you will find a step-by-step guide to starting the voting
service on 1 or 4 nodes on the local machine.

Modify voting/Cargo.toml with your paths.

In this Cargo.toml example exonum and voting located in same folder:

[badges]
travis-ci = { repository = "exonum/exonum" }
circle-ci = { repository = "exonum/exonum" }

[dependencies]
exonum = { version = "0.9.0", path = "../../exonum/exonum" }
exonum-configuration = { version = "0.9.0", path = "../../exonum/services/configuration" }

[dev-dependencies]
exonum-testkit = { version = "0.9.0", path = "../../exonum/testkit" }


Build the project:

```sh
cd voting/backend

cargo install
```
У меня тут была ошибка, что exonum-voting уже существует. предлагает сдлеать с --force. типа
```sh
cargo install --force
```
Надеюсь все норм тут

Generate template:

<!-- markdownlint-disable MD013 -->

```sh
mkdir example
```

For 4 nodes:

```sh
exonum-voting generate-template example/common.toml --validators-count 4
```
Вместо 4 вообще можно другое число/ Но тип пусть пока 4. (там ниже автор сам приводит для 1, я пока не пробовала)


Generate public and secrets keys for each node: (запускаешь последовательно)

```sh
exonum-voting generate-config example/common.toml  example/pub_1.toml example/sec_1.toml --peer-address 127.0.0.1:6331

exonum-voting generate-config example/common.toml  example/pub_2.toml example/sec_2.toml --peer-address 127.0.0.1:6332

exonum-voting generate-config example/common.toml  example/pub_3.toml example/sec_3.toml --peer-address 127.0.0.1:6333

exonum-voting generate-config example/common.toml  example/pub_4.toml example/sec_4.toml --peer-address 127.0.0.1:6334
```

Finalize configs: (запускаешь последовательно)

```sh
exonum-voting finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 example/sec_1.toml example/node_1_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

exonum-voting finalize --public-api-address 0.0.0.0:8201 --private-api-address 0.0.0.0:8092 example/sec_2.toml example/node_2_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

exonum-voting finalize --public-api-address 0.0.0.0:8202 --private-api-address 0.0.0.0:8093 example/sec_3.toml example/node_3_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

exonum-voting finalize --public-api-address 0.0.0.0:8203 --private-api-address 0.0.0.0:8094 example/sec_4.toml example/node_4_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml
```

Run nodes: (запускаешь параллельно из 4х терминалов, находясь в той же папке voting/backend)

```sh
exonum-voting run --node-config example/node_1_cfg.toml --db-path example/db1 --public-api-address 0.0.0.0:8200

exonum-voting run --node-config example/node_2_cfg.toml --db-path example/db2 --public-api-address 0.0.0.0:8201

exonum-voting run --node-config example/node_3_cfg.toml --db-path example/db3 --public-api-address 0.0.0.0:8202

exonum-voting run --node-config example/node_4_cfg.toml --db-path example/db4 --public-api-address 0.0.0.0:8203
```


For 1 node: (это пока не пробовала)

```sh
exonum-voting generate-template example/common.toml --validators-count 1

exonum-voting generate-config example/common.toml  example/pub_1.toml example/sec_1.toml --peer-address 127.0.0.1:6331

exonum-voting finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 example/sec_1.toml example/node_1_cfg.toml --public-configs example/pub_1.toml

exonum-voting run --node-config example/node_1_cfg.toml --db-path example/db1 --public-api-address 0.0.0.0:8200
```


<!-- markdownlint-enable MD013 -->
круто, дошли до фронта (в этом месте у тебя уже должны быть запущены 4 терминала с exonum-voting run)
в 5ом терминале делаешь дальше))
Install frontend dependencies:

```sh
cd frontend

npm install
```

Build sources:

```sh
npm run build
```
( install & build делаются один раз)
Run the application:

```sh
npm start -- --port=8280 --api-root=http://127.0.0.1:8200
```

`--port` is a port for Node.JS app.

`--api-root` is a root URL of public API address of one of nodes.

Ready! Find demo at [http://127.0.0.1:8280](http://127.0.0.1:8280).
-- заходишь по ссылке для демо/ радуешься
можно из еще одного терминала сделать еще один 
```sh
npm start -- --port=828x --api-root=http://127.0.0.1:820x
```
вместо x 1 или 2 или 3 
в общем все

## License

Cryptocurrency demo is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
