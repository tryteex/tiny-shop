# tiny-shop: Async CRM in Rust

`tiny-shop` is an asynchronous Customer Relationship Management (CRM) system, written in the Rust programming language.
And uses `tiny-web` is a tiny async library (backend web server) that allows you to write a Laravel-style or Django-style backend in Rust language.

## Documentation and examples

* Commercial e-shop: https://tiny.com.ua/
* Demo website https://demo.tiny.com.ua/
* Website `tiny-shop` project, `tiny-web` library and documentation https://rust.tiny.com.ua/
* Developers' site https://dev.tiny.com.ua/

## Features

- **Asynchrony**: The project utilizes Rust's powerful async/await primitives for handling numerous connections concurrently.

- **Performance**: Leveraging Rust's performance characteristics for fast, reliable execution.

- **Safety**: Rust's strong safety guarantees like memory safety without garbage collection, and concurrency without data races make `tiny-shop` robust and secure.

## Getting Started

To get started with `tiny-shop`, you need to have Rust installed. If you don't have Rust installed, you can visit https://www.rust-lang.org/learn/get-started to learn how to install it.

### Building

Clone the repository and navigate into it:

```bash
git clone https://github.com/tryteex/tiny-shop.git
cd tiny-shop
```

Build the project with cargo:

```bash
cargo build
```
### Installation

You need to prepare a `tiny.conf` file in your web server. To do this, take the sample configuration file `tiny.sample.conf` and place it in the root of your project with the new name `tiny.conf`. And adjust the corresponding values. Be sure to change the `salt` parameter. In the future, the `tiny.conf` file will be created when the `install` command is executed.

### Running

Run the project with cargo:

```bash
cargo run start
```

## Contributing

If you'd like to contribute to tiny-web, check out our repository on GitHub.

## License

Tiny-Shop is licensed under the MIT license. Please see the 'LICENSE' file for more information.

