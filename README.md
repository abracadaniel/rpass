# rpass
Generates alphanumeric random passwords in Rust

## Building
`$ cargo build --release`

## Running
`$ rpass`

## Arguments
`$ rpass <Strength> <Length>`

### Strength
Strength level. Adds special characters.
Default: 0
Values: 1 / 0

### Length
Length of the password.
Default: 32
Values: Integer