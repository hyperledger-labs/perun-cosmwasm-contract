<h1 align="center"><br>
    <a href="https://perun.network/"><img src=".assets/go_perun.png" alt="Perun" width="30%"></a>
<br></h1>

<h4 align="center">Perun CosmWASM</h4>

<p align="center">
  <a href="https://www.apache.org/licenses/LICENSE-2.0.txt"><img src="https://img.shields.io/badge/license-Apache%202-blue" alt="License: Apache 2.0"></a>
  </a>
</p>


# Project structure


## Development Workflow
This project uses a `cargo-make` which wraps `cargo` to run dev related tasks. All checks can be run with:  
```sh
cargo make ci
```

### Schema
All `json` schema files can be found in the `schema` directory.

The reproducible and optimized production build can be started with:
```sh
make optimize
```
This will create a `artefacts/perun_cosmwasm.wasm` binary with checksum file.

## Examples
It is possible to generate binary encodings of the off-chain structs.  
This is useful for testing the *go-perun* connector. Try:  
```sh
make serde
```
will create a `serde/*` directory which contains `.bin` files which should be deserializeable from the *go-perun* connector.

## TODO (remove before submission)
- What happens when a function panics? CosmWASM specifically advertises that overflows panic, which makes it sound like a good thing.
- Unfunded channels can be disputed.
- move the random generation functions into a package that is accessible for the tests and examples.
- Add sensible constants for maximum values. Eg: `MAX_NUM_PARTS` and update the tests
- Release builds in Release CIs
- Input validation?
- See if the typedef approach for `ExecuteMsg`s work out or if it have to be transformed back into structs.
- Codecov for public repo
