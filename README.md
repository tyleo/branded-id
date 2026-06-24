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

Where the integer ids are indices, the crate also provides brand-typed string ids that act as opaque keys: a borrowed/owned pair (like `StrId`/`StringId`) for each standard string type. The owned id derefs and borrows to its borrowed half, so a map keyed by the owned id can be looked up by a borrowed id without allocating.

```rust
use branded_id::{string_id, str_id, StringId};
use std::collections::HashMap;

struct BUser;

let mut users: HashMap<StringId<BUser>, u32> = HashMap::new();
users.insert(string_id!(BUser; "alice"), 1);

// Look up with a borrowed branded id, no allocation.
assert_eq!(users.get(str_id!(BUser; "alice")), Some(&1));
```
