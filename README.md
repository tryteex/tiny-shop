# tiny-shop: Async CRM in Rust

`tiny-shop` is an asynchronous Customer Relationship Management (CRM) system, written in the Rust programming language.
And uses `tiny-web` is a tiny async library (backend web server) that allows you to write a Laravel-style or Django-style backend in Rust language.

## Documentation and examples

* Commercial e-shop: https://tiny.com.ua/
* Demo website https://demo.tiny.com.ua/
* Website `tiny-shop` project, `tiny-web` library and documentation https://rust.tiny.com.ua/
* Developers' site https://dev.tiny.com.ua/

> **Note**  
> The sites are under construction, follow the projects, and check out our repository on GitHub.

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

### Configuring nginx

The working configuration for Linux and Windows should look like this:

```nginx
worker_processes 7;

events {
  worker_connections  1024;
}

http {
  include mime.types;
  default_type application/octet-stream;

  sendfile on;
  client_max_body_size 2M;

  gzip on;
  proxy_buffering off;

  upstream fcgi_backend {
    server 127.0.0.1:12500;
    keepalive 32;
  }

  server {
    listen       80;
    server_name  fcgi.domain.ua;
    # The path to the root directory,
    # on a Windows server it might look like this
    # root c:/web/tiny/tiny-shop/www;
    root /home/tiny/tiny-shop/www;

    location ~* ^.+\.(?:css|cur|js|jpg|gif|ico|png|xml|otf|ttf|eot|woff|woff2|svg)$ {
      break;
    }

    # It is forbidden to read ini or html directly
    location ~\.(ini|html)$ {
      rewrite ^(.*)$ //$1/ last;
    }

    # Redirect for page index
    location ~ ^/$ {
      rewrite ^(.*)$ // last;
    }

    # Sending a web request to CRM.
    # All requests that start with two slashes "//" at the beginning 
    # are redirected to tine-web engines.
    location ~ ^// {
      fastcgi_connect_timeout 1;
      fastcgi_next_upstream timeout;
      fastcgi_pass fcgi_backend;
      fastcgi_read_timeout 5d;
      fastcgi_param REDIRECT_URL $request_uri;
      include fastcgi_params;
      fastcgi_keep_conn on;
      fastcgi_buffering off;
      fastcgi_socket_keepalive on;
      fastcgi_ignore_client_abort on;
      break;
    }

    # ReWrite module
    if (!-f $request_filename) {
      rewrite ^(.*)$ //$1 last;
    }
  }
}
```

## Contributing

If you'd like to contribute to tiny-web, check out our repository on GitHub.

## License

Tiny-Shop is licensed under the MIT license. Please see the 'LICENSE' file for more information.

## Community

Our project is in its infancy, if you want to join us, welcome!  
https://discord.gg/g5BbxCWJ