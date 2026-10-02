use crate::{
    soa::{IdField, U32IdStruct},
    tests::util::BTest,
    u32_id,
};
use std::rc::Rc;

// A full view adds a row across every column. Removing it hands the values
// back.
#[test]
fn retain_and_release_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();
    let mut name = IdField::<BTest, String>::new();

    // SAFETY: both columns are every column of `ids`, in sync with it.
    let mut view = unsafe { ids.view_mut((&mut health, &mut name)) };

    let goblin = view.retain((30, "goblin".to_owned()));
    let troll = view.retain((80, "troll".to_owned()));

    assert_eq!(view.get(goblin), Some((&30, &"goblin".to_owned())));
    assert_eq!(view.len(), 2);

    let (troll_health, troll_name) = view.get_mut(troll).unwrap();
    *troll_health += 1;
    troll_name.push('!');

    assert_eq!(view.release(troll), Some((81, "troll!".to_owned())));
    assert_eq!(view.release(troll), None);
    assert_eq!(view.release(u32_id!(BTest; 9)), None);
    assert_eq!(view.len(), 1);

    // The released id comes back on the next add.
    let orc = view.retain((50, "orc".to_owned()));

    assert_eq!(orc, troll);
    assert_eq!(view.get(orc), Some((&50, &"orc".to_owned())));

    // A bare field leaks what it still holds, so clear it.
    view.clear();
}

// `release` swaps the last row into the gap, and `release_stable` keeps the
// order.
#[test]
fn release_order_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();

    // SAFETY: `health` is the only column of `ids`, in sync with it.
    let mut view = unsafe { ids.view_mut(&mut health) };

    let id_0 = view.retain(0);
    let id_1 = view.retain(1);
    let id_2 = view.retain(2);
    let id_3 = view.retain(3);

    assert_eq!(view.release(id_0), Some(0));

    let order: Vec<_> = view.iter().map(|(id, _)| id).collect();
    assert_eq!(order, vec![id_3, id_1, id_2]);

    assert_eq!(view.release_stable(id_3), Some(3));

    let order: Vec<_> = view.iter().map(|(id, _)| id).collect();
    assert_eq!(order, vec![id_1, id_2]);
}

// The zeroed removals hand the values back and reorder like `release` and
// `release_stable`. They also leave zeros in the row's slots.
#[test]
fn release_zeroed_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();
    let mut attack = IdField::<BTest, u32>::new();

    // SAFETY: both columns are every column of `ids`, in sync with it.
    let mut view = unsafe { ids.view_mut((&mut health, &mut attack)) };

    let id_0 = view.retain((10, 1));
    let id_1 = view.retain((20, 2));
    let id_2 = view.retain((30, 3));
    let id_3 = view.retain((40, 4));

    assert_eq!(view.release_zeroed(id_0), Some((10, 1)));
    assert_eq!(view.release_zeroed(id_0), None);

    let order: Vec<_> = view.iter().map(|(id, _)| id).collect();
    assert_eq!(order, vec![id_3, id_1, id_2]);

    assert_eq!(view.release_stable_zeroed(id_3), Some((40, 4)));
    assert_eq!(view.release_stable_zeroed(id_3), None);

    let order: Vec<_> = view.iter().map(|(id, _)| id).collect();
    assert_eq!(order, vec![id_1, id_2]);

    // SAFETY: zero bytes are a valid `u32`.
    let slots = unsafe {
        [
            *health.get(id_0),
            *attack.get(id_0),
            *health.get(id_3),
            *attack.get(id_3),
        ]
    };

    assert_eq!(slots, [0; 4]);
}

// `gc` relabels the ids to `0..len` and moves every column's values with
// them.
#[test]
fn gc_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();
    let mut attack = IdField::<BTest, u32>::new();

    // SAFETY: both columns are every column of `ids`, in sync with it.
    let mut view = unsafe { ids.view_mut((&mut health, &mut attack)) };

    let id_0 = view.retain((10, 1));
    let id_1 = view.retain((20, 2));
    let id_2 = view.retain((30, 3));

    view.release_stable(id_0);

    let remap = view.gc();
    let new_1 = remap.new_id(id_1).unwrap();
    let new_2 = remap.new_id(id_2).unwrap();

    assert_eq!((new_1, new_2), (u32_id!(BTest; 0), u32_id!(BTest; 1)));
    assert_eq!(view.get(new_1), Some((&20, &2)));
    assert_eq!(view.get(new_2), Some((&30, &3)));
}

// Removing and clearing drop each value exactly once.
#[test]
fn drops_test() {
    let token = Rc::new(());

    let mut ids = U32IdStruct::<BTest>::new();
    let mut tokens = IdField::<BTest, Rc<()>>::new();

    // SAFETY: `tokens` is the only column of `ids`, in sync with it.
    let mut view = unsafe { ids.view_mut(&mut tokens) };

    let id_0 = view.retain(Rc::clone(&token));
    view.retain(Rc::clone(&token));
    view.retain(Rc::clone(&token));

    assert_eq!(Rc::strong_count(&token), 4);

    drop(view.release(id_0));

    assert_eq!(Rc::strong_count(&token), 3);

    view.clear();

    assert_eq!(Rc::strong_count(&token), 1);
    assert!(view.is_empty());
}

// A full view iterates writable rows, and consuming it hands them on.
#[test]
fn iter_mut_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();
    let mut attack = IdField::<BTest, u32>::new();

    // SAFETY: both columns are every column of `ids`, in sync with it.
    let mut view = unsafe { ids.view_mut((&mut health, &mut attack)) };

    view.retain((1, 10));
    view.retain((2, 20));

    for (_, (health, attack)) in &mut view {
        *health += *attack;
    }

    for (_, (_, attack)) in view {
        *attack = 0;
    }

    // SAFETY: both columns are in sync with `ids`.
    let view = unsafe { ids.view((&health, &attack)) };

    let rows: Vec<_> = view
        .iter()
        .map(|(_, (&health, &attack))| (health, attack))
        .collect();

    assert_eq!(rows, vec![(11, 0), (22, 0)]);
}

// Debug shows the pool, not the rows.
#[test]
fn debug_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut health = IdField::<BTest, u32>::new();

    // SAFETY: `health` is the only column of `ids`, in sync with it.
    let mut view = unsafe { ids.view_mut(&mut health) };

    view.retain(30);

    let expected = format!("IdStructViewMut {{ ids: {:?}, .. }}", view.ids());
    assert_eq!(format!("{:?}", view), expected);
}
