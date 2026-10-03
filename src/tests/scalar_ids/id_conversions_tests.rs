use crate::{
    I8Id, I16Id, I32Id, I64Id, I128Id, Id, IsizeId, U8Id, U16Id, U32Id, U64Id, U128Id, UsizeId,
    i8_id, i32_id, i64_id, internal::IdRepr, tests::util::BTest, u8_id, u32_id,
};
use std::fmt::Debug;

fn assert_converts_as_integers<TSrc, TDst>(src: TSrc)
where
    TSrc: Id,
    TDst: Id + TryFrom<TSrc>,
    TDst::Repr: TryFrom<TSrc::Repr> + Debug + PartialEq,
{
    let actual = TDst::try_from(src).ok().map(IdRepr::to_repr);

    let expected = <TDst::Repr as TryFrom<TSrc::Repr>>::try_from(src.to_repr()).ok();

    assert_eq!(actual, expected);
}

macro_rules! assert_widths_convert_as_integers {
    ( [ $( $src:ident ),+ ] $dsts:tt ) => {
        $( assert_width_converts_as_integers!($src $dsts); )+
    };
}

macro_rules! assert_width_converts_as_integers {
    ( $src:ident [ $( $dst:ident ),+ ] ) => {
        for src in [$src::<BTest>::MIN, $src::from_repr(1), $src::MAX] {
            $( assert_converts_as_integers::<_, $dst<BTest>>(src); )+
        }
    };
}

#[test]
fn from_widens_test() {
    let actual: U32Id<BTest> = U32Id::from(u8_id!(BTest; 255));
    let expected = u32_id!(BTest; 255);
    assert_eq!(actual, expected);

    let actual: I64Id<BTest> = I64Id::from(i8_id!(BTest; -1));
    let expected = i64_id!(BTest; -1);
    assert_eq!(actual, expected);
}

#[test]
fn try_from_narrows_test() {
    let actual: Option<U8Id<BTest>> = U8Id::try_from(u32_id!(BTest; 255)).ok();
    let expected = Some(u8_id!(BTest; 255));
    assert_eq!(actual, expected);

    let actual: Option<U8Id<BTest>> = U8Id::try_from(u32_id!(BTest; 256)).ok();
    let expected = None;
    assert_eq!(actual, expected);

    let actual: Option<U32Id<BTest>> = U32Id::try_from(i32_id!(BTest; -1)).ok();
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn every_width_pair_converts_as_its_integers_test() {
    assert_widths_convert_as_integers!(
        [I8Id, I16Id, I32Id, I64Id, I128Id, IsizeId, U8Id, U16Id, U32Id, U64Id, U128Id, UsizeId]
        [I8Id, I16Id, I32Id, I64Id, I128Id, IsizeId, U8Id, U16Id, U32Id, U64Id, U128Id, UsizeId]
    );
}
