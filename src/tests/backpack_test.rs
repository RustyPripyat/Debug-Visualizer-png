use std::collections::HashMap;
use std::ops::Range;

use crate::runner::backpack::BackPack;
use crate::world::tile::*;

#[test]
fn backpack_unit_test_default() {
    const DEFAULT_VAL: usize = 0;
    const DEFAULT_RANGE: Range<usize> = 0..0;
    const TEST_SIZE: usize = 10;
    let backpack = BackPack::new(TEST_SIZE);
    let content = vec![
        Content::Rock(DEFAULT_VAL),
        Content::Tree(DEFAULT_VAL),
        Content::Garbage(DEFAULT_VAL),
        Content::Fire,
        Content::Coin(DEFAULT_VAL),
        Content::Bin(DEFAULT_RANGE),
        Content::Crate(DEFAULT_RANGE),
        Content::Bank(DEFAULT_RANGE),
        Content::Water(DEFAULT_VAL),
        Content::None,
    ]
    .into_iter()
    .map(|c| (c, 0_usize))
    .collect::<HashMap<_, _>>();

    assert_eq!(backpack.get_size(), TEST_SIZE);

    assert_eq!(backpack.get_contents().len(), content.len());

    backpack.get_contents().iter().for_each(|(key, value)| {
        assert_eq!(Some(value), content.get(key));
    })
}
