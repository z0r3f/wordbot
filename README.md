[![Build Status](https://dev.azure.com/ferozar/WordBot/_apis/build/status/z0r3f.wordbot?branchName=master)](https://dev.azure.com/ferozar/WordBot/_build/latest?definitionId=26&branchName=master)

# Rust Dictionary Bot

This is a simple bot written in Rust that retrieves the definition of a word from an online dictionary (Merriam-Webster Dictionary API).

## Getting Started

To run this bot, make sure you have Rust installed on your system. If you don't, you can download it from the official Rust website: https://www.rust-lang.org/tools/install

Once you have Rust installed, clone this repository and run the following command to build the bot:

Define telegram token in environment variable `TELOXIDE_TOKEN`

```shell
$ cargo build
```

After the build process is complete, you can run the bot using the following command:

```shell
$ cargo run
```

## Usage
To use the bot, simply enter a word and the bot will retrieve its definition from the online dictionary.

## Contributing
Contributions are always welcome! If you have any suggestions or improvements for this bot, feel free to open a pull request or submit an issue.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
# Rust Dictionary Bot

This is a simple bot written in Rust that retrieves the definition of a word from an online dictionary.

## Getting Started

To run this bot, make sure you have Rust installed on your system. If you don't, you can download it from the official Rust website: https://www.rust-lang.org/tools/install

Once you have Rust installed, clone this repository and run the following command to build the bot:

Define telegram token in environment variable `TELOXIDE_TOKEN`

```shell
$ cargo build
```

After the build process is complete, you can run the bot using the following command:

```shell
$ cargo run
```

## Usage
To use the bot, simply enter a word and the bot will retrieve its definition from the online dictionary.

## Docker
```shell
docker build -t wordbot .
```

```shell
docker run --env TELOXIDE_TOKEN=123456789:blablabla -it --rm --name wordbot wordbot
```

#### Note
To install the appropriate dependencies in the container, I relied on `ldd`, so that once the binary was built in the container it told me which libraries were needed.
```shell
# ldd /usr/local/bin/wordbot
        linux-vdso.so.1 (0x0000ffffa7944000)
        libssl.so.1.1 => not found
        libcrypto.so.1.1 => not found
        libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x0000ffffa7423000)
        libpthread.so.0 => /lib/aarch64-linux-gnu/libpthread.so.0 (0x0000ffffa73f4000)
        libdl.so.2 => /lib/aarch64-linux-gnu/libdl.so.2 (0x0000ffffa73e0000)
        libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x0000ffffa726f000)
        /lib/ld-linux-aarch64.so.1 (0x0000ffffa7916000)

```

## Publish
```shell
docker build --tag z0r3f/wordbot-docker:latest .
```
```shell
docker tag z0r3f/wordbot-docker:latest z0r3f/wordbot-docker:0.1.0
```
```shell
docker push z0r3f/wordbot-docker:latest
docker push z0r3f/wordbot-docker:0.1.0
```

## Contributing
Contributions are always welcome! If you have any suggestions or improvements for this bot, feel free to open a pull request or submit an issue.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
