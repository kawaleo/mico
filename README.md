# mico

A simple math interpreter made in Rust

## Usage

```shell
git clone https://github.com/kawaleo/mico.git
cd mico
cargo run
```

And that's it!

## Syntax

`+` - Addition<br>
`-` - Subtraction<br>
`*` - Multiplication<br>
`/` - Division<br>

`(` - LParen<br>
`)` - RParen<br>

## Examples

```shell
12 ( 2 ( 2 ) )
Result: 48.0
```

```shell
4 + 3 / 2
Result: 5.5
```

```shell
9 + ( 1 * 3 / 2 )
Result: 10.5
```

```shell
(5 - ( 2 - 5 ) ) * ( 4 - 2 )
Result: 16.0
```

```shell
( 7 + 8 * 3 ) - 7 + 8
Result: 32.0
```

```shell
2 ( ( ( ( 2 ) ) ) )
Result: 4.0
```

## Configuration

To go into debug mode, run:

```shell
cargo run +dbg
```

## TODO

- [x] Addition/Subtraction/Multiplication/Division
- [x] Order of Operations
- [x] Parentheses
- [x] Nested Parentheses
- [ ] Unary Operations
- [ ] Special Operators
