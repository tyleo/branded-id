# branded-id

Data structures that are _branded_ so they only interoperate with similarly branded integer types.

Every id and container carries a `TBrand` type parameter. Two ids built for different brands are distinct types, so the compiler rejects using one domain's id to index another domain's storage, even though both are just integers at runtime.

```rust
use branded_id::{UsizeId, usize_id};

struct BApples;

let id: UsizeId<BApples> = usize_id!(BApples; 2);
assert_eq!(id.to_usize(), 2);
```

## Integer Ids and Containers

A brand-typed integer id for each primitive width (for example `UsizeId` and `I32Id`), built with the `*_id!` macros. `UsizeId` is the canonical width that indexes storage; the others convert through it. `(a..b).into_id_range()` iterates the ids the way `a..b` iterates integers, and `(a..=b).into_id_range()` does the same for `a..=b`. `IdRange::from_len` covers the first `len` ids. Brand-typed containers and pointers only accept ids of their own brand, built with `id_array!`, `id_vec!`, and `id_slice!`.

## String Ids

Opaque brand-typed keys rather than indices: a borrowed/owned pair (like `StrId`/`StringId`) for each standard string type. The owned id derefs to its borrowed half, so a map keyed by the owned id can be looked up by a borrowed id without allocating.

```rust
use branded_id::{string_id, str_id, StringId};
use std::collections::HashMap;

struct BUser;

let mut users: HashMap<StringId<BUser>, u32> = HashMap::new();
users.insert(string_id!(BUser; "alice"), 1);

// Look up with a borrowed branded id, no allocation.
assert_eq!(users.get(str_id!(BUser; "alice")), Some(&1));
```

## Extension Traits (`ext`)

Sealed extension traits that add id-typed views to primitives, slices, arrays, `Vec`, and raw pointers. `IteratorExt::enumerate_ids` pairs each item of an iterator with an id counting from zero.

## Brand Conversions (`extends`, default feature)

When one brand extends another, ids and containers cast between the two with `upcast`/`downcast` methods.

## Struct of Arrays (`soa`, default feature)

A columnar struct-of-arrays id pool. An `IdStruct` hands out and recycles typed ids, and each `IdField` stores one column keyed by those ids, so an entity's data lives across parallel columns rather than in one struct.

```rust
use branded_id::soa::{IdField, IdStruct};

struct BEnemy;

let mut enemies = IdStruct::<BEnemy>::new();
let mut health = IdField::<BEnemy, i32>::new();
let mut attack = IdField::<BEnemy, i32>::new();

// Spawn an enemy by retaining an id, then give it a value in each column.
let goblin = enemies.retain();
health.retain(goblin, 30);
attack.retain(goblin, 5);

let troll = enemies.retain();
health.retain(troll, 80);
attack.retain(troll, 12);

// The pool tracks liveness, so reading a column is unsafe: the caller
// vouches that the id is retained and has a value here.
// SAFETY: goblin and troll are retained with a value in each field.
unsafe {
    // The troll hits the goblin.
    *health.get_mut(goblin) -= *attack.get(troll);
    assert_eq!(*health.get(goblin), 18);
}
```

A view reads columns by row without an `unsafe` per read. `view` borrows the pool beside any mix of shared and mutable columns. Building it is the one unsafe step: the caller vouches that the columns are in sync with the pool. `view_mut` takes every column, all mutable, so rows can be added and removed too.

```rust
// SAFETY: both columns are in sync with `enemies`.
let mut fight = unsafe { enemies.view((&health, &mut attack)) };

// Each enemy hits harder the healthier it is.
for (_, (health, attack)) in fight.iter_mut() {
    *attack += *health / 10;
}

assert_eq!(fight.get(troll), Some((&80, &20)));

// SAFETY: both columns are every column of `enemies`, in sync with it.
let mut roster = unsafe { enemies.view_mut((&mut health, &mut attack)) };

let orc = roster.retain((50, 8));
assert_eq!(roster.release(orc), Some((50, 8)));
```

An `IdList` owns a pool and its one column, so it keeps them in sync itself and needs no `unsafe`.

```rust
use branded_id::soa::IdList;

struct BItem;

let mut items = IdList::<BItem, &str>::new();

let sword = items.retain("sword");
let shield = items.retain("shield");

assert_eq!(items.get(sword), Some(&"sword"));
assert_eq!(items.release(sword), Some("sword"));
assert_eq!(items.len(), 1);
```

`view_with` and `view_mut_with` view the list beside columns kept elsewhere. The caller makes the same promise as for `view` and `view_mut`. A row reads as `(value, row of columns)`.

```rust
use branded_id::soa::IdField;

// The shield is the list's one item, so it needs a weight.
let mut weights = IdField::<BItem, u32>::new();
weights.retain(shield, 8);

// SAFETY: `weights` is the list's only other column and is in sync with it.
let mut inventory = unsafe { items.view_mut_with(&mut weights) };

let bow = inventory.retain(("bow", 3));
assert_eq!(inventory.get(bow), Some((&"bow", &3)));
assert_eq!(inventory.get(shield), Some((&"shield", &8)));
```

## Serde (`serde`, optional feature)

Implements `Serialize` and `Deserialize` for every scalar id as its bare integer, and for `UuidId` as its `Uuid` when `uuid` is on too. Enable it with `branded-id = { version = "...", features = ["serde"] }`. The brand never reaches the wire, so a branded id reads and writes as the unbranded value would, as a value or as a map key.

```rust
use branded_id::{u32_id, U32Id};
use std::collections::BTreeMap;

struct BUser;

let id: U32Id<BUser> = u32_id!(BUser; 7);
assert_eq!(serde_json::to_string(&id).unwrap(), "7");

let by_id = BTreeMap::from([(id, "ada")]);
assert_eq!(serde_json::to_string(&by_id).unwrap(), r#"{"7":"ada"}"#);
```

## UUID Ids (`uuid`, optional feature)

Adds `UuidId`, an opaque brand-typed `Uuid` key. Enable it with `branded-id = { version = "...", features = ["uuid"] }`, then convert to and from a raw `Uuid` with `from_uuid`/`to_uuid` (or the matching `From` impls).

```rust
use branded_id::{uuid_id, UuidId};
use uuid::Uuid;

struct BUser;

let raw = Uuid::from_u128(0x1234);
let id: UuidId<BUser> = uuid_id!(BUser; raw);
assert_eq!(id.to_uuid(), raw);
```
