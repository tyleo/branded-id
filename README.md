# branded-id

Data structures that are _branded_ so they only interoperate with similarly branded integer types.

Every id and container carries a `TBrand` type parameter. Two ids built for different brands are distinct types, so the compiler rejects using one domain's id to index another domain's storage, even though both are just integers at runtime.

```rust
use branded_id::{UsizeId, usize_id};

struct Apples;

let id: UsizeId<Apples> = usize_id!(Apples; 2);
assert_eq!(id.to_usize(), 2);
```

The crate provides brand-typed integer ids (`UsizeId`, `I32Id`, `U32Id`, `IsizeId`), the containers they index (`IdSlice`, `IdArray`, `IdVec`) and pointers (`IdPtr`, `MutIdPtr`), plus optional `extends` (brand conversions) and `soa` (columnar id pools) modules. The `*_id!`, `id_array!`, `id_vec!`, and `id_slice!` macros build them concisely.
