use crate::{
    soa::{IdField, U32IdStruct},
    tests::util::BTest,
    u32_id,
};

// A one-column view reads a retained id. It answers `None` for a released id
// and for one the pool never handed out, where reading the bare field would
// be undefined behavior.
#[test]
fn get_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();

    let goblin = ids.retain();
    health.retain(goblin, 30);

    let troll = ids.retain();
    health.retain(troll, 80);

    // SAFETY: the troll's value is released before its id.
    unsafe { health.release(troll) };
    ids.release(troll);

    // SAFETY: `health` is in sync with `ids`.
    let view = unsafe { ids.view(&health) };

    assert_eq!(view.get(goblin), Some(&30));
    assert_eq!(view.get(troll), None);
    assert_eq!(view.get(u32_id!(BTest; 9)), None);
}

// Shared columns read for the view's whole lifetime, so a view built in a
// temporary can hand its rows on.
#[test]
fn shared_rows_outlive_the_view_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();

    let goblin = ids.retain();
    health.retain(goblin, 30);

    // SAFETY: `health` is in sync with `ids`.
    let value = unsafe { ids.view(&health) }.get(goblin);

    assert_eq!(value, Some(&30));
}

// A view mixing a shared and a mutable column writes through the mutable one
// and reads the shared one, by id and by iteration.
#[test]
fn mixed_columns_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();
    let mut attack = IdField::<BTest, u32>::new();

    let goblin = ids.retain();
    health.retain(goblin, 30);
    attack.retain(goblin, 5);

    let troll = ids.retain();
    health.retain(troll, 80);
    attack.retain(troll, 12);

    // SAFETY: both columns are in sync with `ids`.
    let mut view = unsafe { ids.view((&health, &mut attack)) };

    let (goblin_health, goblin_attack) = view.get_mut(goblin).unwrap();
    *goblin_attack += *goblin_health;

    for (_, (health, attack)) in view.iter_mut() {
        *attack += *health;
    }

    let rows: Vec<_> = view.iter().collect();

    assert_eq!(rows, vec![(goblin, (&30, &65)), (troll, (&80, &92))]);
}

// Two mutable columns hand out rows for every id at once without aliasing.
#[test]
fn two_mutable_columns_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();
    let mut attack = IdField::<BTest, u32>::new();

    for value in 1..=3 {
        let id = ids.retain();
        health.retain(id, value);
        attack.retain(id, value * 10);
    }

    // SAFETY: both columns are in sync with `ids`.
    let mut view = unsafe { ids.view((&mut health, &mut attack)) };

    let rows: Vec<_> = view.iter_mut().collect();

    for (_, (health, attack)) in rows {
        std::mem::swap(health, attack);
    }

    let rows: Vec<_> = view
        .iter()
        .map(|(_, (&health, &attack))| (health, attack))
        .collect();

    assert_eq!(rows, vec![(10, 1), (20, 2), (30, 3)]);
}

// A view iterates in the pool's order, which a move rearranges, and counts the
// pool's retained ids. Consuming a view hands its rows on for its lifetime.
#[test]
fn iter_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();

    let id_0 = ids.retain();
    health.retain(id_0, 10);

    let id_1 = ids.retain();
    health.retain(id_1, 20);

    let id_2 = ids.retain();
    health.retain(id_2, 30);

    ids.move_to(id_2, 0);

    // SAFETY: `health` is in sync with `ids`.
    let view = unsafe { ids.view(&health) };

    let expected = vec![(id_2, &30), (id_0, &10), (id_1, &20)];

    assert_eq!(view.iter().collect::<Vec<_>>(), expected);
    assert_eq!((&view).into_iter().collect::<Vec<_>>(), expected);
    assert_eq!(view.iter().next_back(), Some((id_1, &20)));
    assert_eq!(view.iter().len(), 3);
    assert_eq!(view.len(), 3);
    assert!(!view.is_empty());

    // SAFETY: `health` is in sync with `ids`.
    let consumed: Vec<_> = unsafe { ids.view(&health) }.into_iter().collect();

    assert_eq!(consumed, expected);
}

// `into_mut` hands out a row for the view's whole lifetime.
#[test]
fn into_mut_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();

    let goblin = ids.retain();
    health.retain(goblin, 30);

    // SAFETY: `health` is in sync with `ids`.
    let value = unsafe { ids.view(&mut health) }.into_mut(goblin).unwrap();
    *value = 31;

    // SAFETY: `health` is in sync with `ids`.
    let view = unsafe { ids.view(&mut health) };

    assert_eq!(view.get(goblin), Some(&31));
    assert!(view.into_mut(u32_id!(BTest; 9)).is_none());
}

#[test]
fn empty_test() {
    let ids = U32IdStruct::<BTest>::new();
    let health = IdField::<BTest, u32>::new();

    // SAFETY: an empty column is in sync with an empty pool.
    let view = unsafe { ids.view(&health) };

    assert!(view.is_empty());
    assert_eq!(view.len(), 0);
    assert_eq!(view.iter().next(), None);
}

// A view of eight columns reads each one. Eight is the most a tuple takes.
#[test]
fn eight_columns_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut columns: [IdField<BTest, u32>; 8] = Default::default();

    let id = ids.retain();
    for (index, column) in (0..).zip(columns.iter_mut()) {
        column.retain(id, index);
    }

    let [c0, c1, c2, c3, c4, c5, c6, c7] = &mut columns;

    // SAFETY: every column is in sync with `ids`.
    let view = unsafe { ids.view((&*c0, &*c1, &*c2, &*c3, c4, c5, c6, c7)) };

    let (v0, v1, v2, v3, v4, v5, v6, v7) = view.get(id).unwrap();

    assert_eq!(
        [*v0, *v1, *v2, *v3, *v4, *v5, *v6, *v7],
        [0, 1, 2, 3, 4, 5, 6, 7]
    );
}

// Debug shows the pool, not the rows.
#[test]
fn debug_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();

    let goblin = ids.retain();
    health.retain(goblin, 30);

    // SAFETY: `health` is in sync with `ids`.
    let view = unsafe { ids.view(&health) };

    let actual = format!("{:?}", view);
    let expected = format!("IdStructView {{ ids: {:?}, .. }}", ids);
    assert_eq!(actual, expected);
}
