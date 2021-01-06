# `napi-gif-encoder`

![https://github.com/tyrone-sudeium/napi-gif-encoder/actions](https://github.com/tyrone-sudeium/napi-gif-encoder/workflows/CI/badge.svg)

> GIF Encoder for Node JS that tries to have reasonable performance.

## Install

```
yarn add @tyrone-sudeium/napi-gif-encoder
```

## Support matrix

### Operating Systems

| Linux x64/aarch64 | macOS x64/aarch64 | Windows x64 |
| ----------------- | ----------------- | ----------- |
| ✓                 | ✓                 | ✓           |

### NodeJS

Theoretically, any version of Node.js that supports N-API should work. The CI
is validated against LTS versions of Node:

| Node 12 | Node14 |
| ------- | ------ |
| ✓       | ✓      |

### Building

If you are using this as a dependency, since we use N-API, you don't
need to build anything! However, if you want to tinker with this code
or submit a PR, read below.

## Developing

- Install latest `Rust`. Suggest using [rustup](https://rustup.rs/).
- Install `NodeJS@10+`. LTS versions suggested. Any version supporting `N-API` should work.
- Install `yarn@1.x`.

You can then compile the rust code with:

    yarn build

After `yarn build/npm run build` command, you can see
`napi-gif-encoder.[darwin|win32|linux].node` file in project root.
This is the native addon built from [lib.rs](./src/lib.rs).

## Try out using sample project

- `yarn`
- `yarn build`
- `cd sample`
- `yarn`
- `node index.js`

You'll then see `output.gif`, which was encoded using the rust encoder.

### Performance

Using a Ryzen 3950X to encode the sample project:

| Encoder                                                         | Time  |
| --------------------------------------------------------------- | ----- |
| [`gif-encoder-2`](https://github.com/benjaminadk/gif-encoder-2) | 787ms |
| `napi-gif-encoder`                                              | 217ms |
