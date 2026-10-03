use crate::{
    U32Id,
    soa::{IdField, IdList},
    tests::util::BTest,
    u32_id,
};
use std::rc::Rc;

type Names = IdList<BTest, String>;

#[test]
fn retain_get_and_release_test() {
    let mut names = Names::new();

    let goblin = names.retain("goblin".to_owned());
    let troll = names.retain("troll".to_owned());

    assert_eq!(names.get(goblin).map(String::as_str), Some("goblin"));
    assert_eq!(names.len(), 2);
    assert!(!names.is_empty());

    names.get_mut(troll).unwrap().push('!');

    assert_eq!(names.release(troll), Some("troll!".to_owned()));
    assert_eq!(names.release(troll), None);
    assert_eq!(names.get(troll), None);
    assert_eq!(names.get(u32_id!(BTest; 9)), None);
    assert_eq!(names.len(), 1);
}

#[test]
fn index_of_and_is_retained_test() {
    let mut names = Names::new();

    let goblin = names.retain("goblin".to_owned());
    let troll = names.retain("troll".to_owned());

    assert_eq!(names.index_of(troll), Some(1));
    assert!(names.is_retained(troll));

    names.release(goblin);

    assert_eq!(names.index_of(troll), Some(0));
    assert_eq!(names.index_of(goblin), None);
    assert!(!names.is_retained(goblin));
    assert!(!names.is_retained(u32_id!(BTest; 9)));
}

// The zeroed removals hand the values back and reorder like `release` and
// `release_stable`.
#[test]
fn release_zeroed_test() {
    let mut names: Names = ["goblin", "troll", "orc", "imp"]
        .map(str::to_owned)
        .into_iter()
        .collect();
    let ids: Vec<U32Id<BTest>> = names.ids().iter().collect();

    assert_eq!(names.release_zeroed(ids[0]).as_deref(), Some("goblin"));
    assert_eq!(names.release_zeroed(ids[0]), None);

    let order: Vec<_> = names.iter().map(|(_, name)| name.as_str()).collect();
    assert_eq!(order, ["imp", "troll", "orc"]);

    assert_eq!(names.release_stable_zeroed(ids[3]).as_deref(), Some("imp"));
    assert_eq!(names.release_stable_zeroed(ids[3]), None);

    let order: Vec<_> = names.iter().map(|(_, name)| name.as_str()).collect();
    assert_eq!(order, ["troll", "orc"]);
}

// The order moves and resets through the list, and `release_stable` keeps it.
#[test]
fn order_test() {
    let mut list: IdList<BTest, u32> = (0..4).collect();
    let ids: Vec<U32Id<BTest>> = list.ids().iter().collect();

    list.move_to(ids[3], 0);

    assert_eq!(
        list.iter().map(|(_, &value)| value).collect::<Vec<_>>(),
        [3, 0, 1, 2]
    );

    list.set_order(&[ids[0], ids[1], ids[2], ids[3]]);
    list.release_stable(ids[1]);

    assert_eq!(
        list.iter().map(|(_, &value)| value).collect::<Vec<_>>(),
        [0, 2, 3]
    );
    assert_eq!(list.try_move_to(ids[1], 0), None);
    assert_eq!(list.try_set_order(&[ids[0]]), None);
    assert_eq!(list.try_move_to(ids[3], 0), Some(()));
    assert_eq!(list.index_of(ids[3]), Some(0));
}

#[test]
fn iter_test() {
    let mut list: IdList<BTest, u32> = (1..=3).collect();

    for (_, value) in list.iter_mut() {
        *value *= 10;
    }

    for (_, value) in &mut list {
        *value += 1;
    }

    let values: Vec<_> = (&list).into_iter().map(|(_, &value)| value).collect();

    assert_eq!(values, [11, 21, 31]);
    assert_eq!(list.iter().len(), 3);
}

// `gc` relabels the ids to `0..len` and moves the values with them.
#[test]
fn gc_test() {
    let mut list: IdList<BTest, u32> = (0..3).collect();

    list.release_stable(u32_id!(BTest; 0));

    let remap = list.gc();

    assert_eq!(remap.new_id(u32_id!(BTest; 2)), Some(u32_id!(BTest; 1)));
    assert_eq!(list.get(u32_id!(BTest; 0)), Some(&1));
    assert_eq!(list.get(u32_id!(BTest; 1)), Some(&2));
}

// A row read through the list's view outlives the view.
#[test]
fn view_test() {
    let list: IdList<BTest, u32> = (0..2).collect();

    let value = list.view().get(u32_id!(BTest; 1));

    assert_eq!(value, Some(&1));
}

// A view with a column kept elsewhere adds, removes and compacts rows across
// both.
#[test]
fn view_mut_with_test() {
    let mut names = Names::new();
    let mut health = IdField::<BTest, u32>::new();

    // SAFETY: `health` is the only other column of `names` and is in sync
    // with it.
    let mut view = unsafe { names.view_mut_with(&mut health) };

    let goblin = view.retain(("goblin".to_owned(), 30));
    let troll = view.retain(("troll".to_owned(), 80));

    assert_eq!(view.release_stable(goblin), Some(("goblin".to_owned(), 30)));

    let (name, health) = view.get_mut(troll).unwrap();
    name.push('!');
    *health += 1;

    let troll = view.gc().new_id(troll).unwrap();

    assert_eq!(troll, u32_id!(BTest; 0));
    assert_eq!(view.get(troll), Some((&"troll!".to_owned(), &81)));

    view.clear();

    assert!(names.is_empty());
}

// A shared view with a column kept elsewhere reads the list's values and
// writes that column.
#[test]
fn view_with_test() {
    let mut names = Names::new();
    let mut health = IdField::<BTest, u32>::new();

    // SAFETY: `health` is the only other column of `names` and is in sync
    // with it.
    let mut view = unsafe { names.view_mut_with(&mut health) };

    let goblin = view.retain(("goblin".to_owned(), 30));
    let troll = view.retain(("troll".to_owned(), 80));

    // SAFETY: `health` is in sync with `names`.
    let mut view = unsafe { names.view_with(&mut health) };

    for (_, (name, health)) in view.iter_mut() {
        if name == "troll" {
            *health += 1;
        }
    }

    assert_eq!(view.get(goblin), Some((&"goblin".to_owned(), &30)));
    assert_eq!(view.get(troll), Some((&"troll".to_owned(), &81)));
}

// A clone copies the values deeply and matches the original's state. Each list
// drops its values.
#[test]
fn clone_eq_and_drop_test() {
    let token = Rc::new(());

    let mut list = IdList::<BTest, Rc<()>>::new();
    list.retain(Rc::clone(&token));
    list.retain(Rc::clone(&token));

    let copy = list.clone();

    assert_eq!(Rc::strong_count(&token), 5);
    assert!(list.eq_state(&copy));

    drop(copy);

    assert_eq!(Rc::strong_count(&token), 3);

    list.clear();

    assert_eq!(Rc::strong_count(&token), 1);
    assert!(list.is_empty());
}

// `eq_entries` compares each id and its value in order. Released ids do not
// count.
#[test]
fn eq_entries_test() {
    let first: IdList<BTest, u32> = (0..3).collect();
    let mut second = first.clone();

    let released = second.retain(3);
    second.release(released);

    assert!(first.eq_entries(&second));

    second.move_to(u32_id!(BTest; 2), 0);

    assert!(!first.eq_entries(&second));

    let mut third = first.clone();
    *third.get_mut(u32_id!(BTest; 1)).unwrap() = 9;

    assert!(!first.eq_entries(&third));
}

// `eq_state` also compares the released ids.
#[test]
fn eq_state_test() {
    let first: IdList<BTest, u32> = (0..3).collect();
    let mut second = first.clone();

    assert!(first.eq_state(&second));

    let released = second.retain(3);
    second.release(released);

    assert!(first.eq_entries(&second));
    assert!(!first.eq_state(&second));

    let mut third = first.clone();
    *third.get_mut(u32_id!(BTest; 1)).unwrap() = 9;

    assert!(!first.eq_state(&third));
}

// `eq_values` compares the values in order, whatever their ids.
#[test]
fn eq_values_test() {
    let first: IdList<BTest, u32> = (0..3).collect();

    let mut second = IdList::<BTest, u32>::new();
    let extra = second.retain(9);
    second.extend(0..3);
    second.release_stable(extra);

    assert!(first.eq_values(&second));
    assert!(!first.eq_entries(&second));

    second.move_to(u32_id!(BTest; 3), 0);

    assert!(!first.eq_values(&second));
}

// Debug shows the values as a map keyed by id, in the list's order.
#[test]
fn debug_test() {
    let mut list = IdList::<BTest, u32>::default();

    let id_0 = list.retain(7);
    let id_1 = list.retain(9);

    list.move_to(id_1, 0);

    let expected = format!("{{{:?}: 9, {:?}: 7}}", id_1, id_0);
    assert_eq!(format!("{:?}", list), expected);
}
