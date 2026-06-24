# branded-id

Data structures that are _branded_ so they only interoperate with similarly branded integer types.

Every id and container carries a `TBrand` type parameter. Two ids built for different brands are distinct types, so the compiler rejects using one domain's id to index another domain's storage, even though both are just integers at runtime.

```rust
use branded_id::{UsizeId, usize_id};

struct BApples;

let id: UsizeId<BApples> = usize_id!(BApples; 2);
assert_eq!(id.to_usize(), 2);
```

The crate provides a brand-typed integer id for each primitive integer width (for example `UsizeId` and `I32Id`), the containers they index (`IdSlice`, `IdArray`, `IdVec`) and pointers (`IdPtr`, `MutIdPtr`), plus optional `extends` (brand conversions) and `soa` (columnar id pools) modules. The `*_id!`, `id_array!`, `id_vec!`, and `id_slice!` macros build them concisely.
