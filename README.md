[![](https://user-images.githubusercontent.com/25987204/78205790-10b0c680-74d8-11ea-9767-5bb93e920044.png)](https://dessert.dev/)

# Dessert jsonschema-core
============

[![npm-badge]][npm-url]
[![license-badge]][license]

[npm-badge]: https://img.shields.io/npm/v/dessert-jsonschema-core.svg
[npm-url]: https://www.npmjs.org/package/dessert-jsonschema-core
[license-badge]: https://img.shields.io/github/license/dessert-wasm/dessert-jsonschema-core
[license]: LICENSE_MIT

> Exposes the base API for the [ajv] module, written in Rust for WebAssembly.

>[ajv]: https://github.com/epoberezkin/ajv

## Table of contents
* [Usage](#usage)
* [API](#api)
* [Installation](#installation)
* [Building](#building)
* [Testing](#testing)
* [License](#license)
* [Contributing](#contributing)

## Usage

> This module is **not** supposed to be used directly as a dependence by an application, it is used as a backend for js connector

```js
var wasm  = require('dessert-jsonschema-core');

console.log(wasm.validate(instance, schema));
```

## API

### validate(instance: JsValue, schema: JsValue)

Matching a json object to its schema

## Installation

> dessert-markdown-core is depended on by [dessert-showdown](https://github.com/dessert-wasm/dessert-showdown)
```sh
npm install dessert-jsonschema-core
```

## Building
The project is built using [wasm-pack]  
To build the project, run

[wasm-pack]: https://github.com/rustwasm/wasm-pack
```sh
wasm-pack build
```

## Testing

```sh
wasm-pack test --headless --firefox # or --chrome
```

## License
MIT

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md)