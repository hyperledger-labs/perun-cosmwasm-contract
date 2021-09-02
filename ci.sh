#!/bin/bash

set -e

cargo make --cwd perun_cosmwasm ci
cargo make --cwd commands ci
