# Kythi File Host
> ShareX-centered uploader

<!-- [![NPM Version][npm-image]][npm-url]
[![Build Status][travis-image]][travis-url]
[![Downloads Stats][npm-downloads]][npm-url] -->

The backend for Kythi's file host, a ShareX-centered uploader. I am currently in school doing my finals so development will be delayed in favour of schoolwork, but it should be completed sometime in the next year-ish.

## Development setup

- Rename Config.example.toml -> Config.toml
- Create a PostgreSQL database and provide it in the config
- Create a Redis server and provide it in the config
- Create a 64-char secret and provide it for the identity secret

### Building
```sh
cargo build --release 
```

### Running
```sh
cargo run
```

## Meta

Ian Fogarty - [@catforgor](https://discord.com/users/1176802609416392745) – eeviefogarty@gmail.com

Distributed under the “Commons Clause” License Condition v1.0 license. See ``LICENSE`` for more information.

[https://github.com/fem-rs](https://github.com/fem-rs/)

## Contributing

1. Fork it (<https://github.com/Kythi-solutions/imagehost-api/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request