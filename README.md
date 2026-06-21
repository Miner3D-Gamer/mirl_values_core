# Mirl Value Core (0.0.0-alpha)

#### MirValCo - Raw Value Values

<details>
<summary>Flags</summary>

### Default:

**Core**

- ~~`std` (Default)~~ - `std` is required
- `all`
- `c_compatible`

**Codec**

- `all_codecs`
- `serde`
- `bitcode`
- `wincode` (bitcode recommended)

**Enum**

- `all_enum_extensions`
- `strum`
- `enum_ext`

### Custom:

- `preserve_entries` - Inside objects/maps/dictionaries, retain value order and duplicate values

</details>

### Purpose

Hold the raw `SimpleValue` and `ContainerValue` enums

### Disclaimer

This lib is to be used by [`mirl_values`](https://github.com/Miner3D-Gamer/mirl_values) to create a single unified `Value` and does not offer a lot of functionality

### Origin

Again: Dependency cycles.

I wanted to add more dynamic values to [`mirl_values`](https://github.com/Miner3D-Gamer/mirl_values) which required both itself and [`mirl_extensions`](https://github.com/Miner3D-Gamer/mirl_extensions) to use and implement object from one another.

So now both those libs pull from this crate.
