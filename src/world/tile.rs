use std::iter::ExactSizeIterator;
use std::ops::Range;

use strum_macros::EnumIter;

pub(crate) const N_CONTENTS: usize = 10;

/// Represents the properties of a tile type.
/// The `TileTypeProps` struct is used to define the properties of a tile type.
///
/// # Variables
/// - `walk`: A `bool` that indicates whether a robot can walk on the tile type.
/// - `hold`: A `Vec<Content>` that indicates the list of Content that a tile can contain.
/// - `cost`: An `usize` that indicates the cost to pass on the tile type.
///
#[derive(Debug, Clone)]
pub struct TileTypeProps {
    walk: bool,
    hold: &'static [(Content, bool); N_CONTENTS],
    cost: usize,
    // Other properties associated with the tile type
}

impl TileTypeProps {
    pub fn walk(&self) -> bool {
        self.walk
    }
    pub fn hold(&self) -> &[(Content, bool); N_CONTENTS] {
        self.hold
    }
    pub fn can_hold(&self, content: &Content) -> bool {
        self.hold[content.index()].1
    }
    pub fn cost(&self) -> usize {
        self.cost
    }
}

/// Represents the properties of a tile content.
/// The `ContentProps` struct is used to define the properties of a tile content.
///
/// # Variables
/// - `destroy`: A `bool` that indicates whether a robot can destroy the tile content.
/// - `max`: An `usize` that indicates the maximum return of elements when destroyed.
/// - `store`: A `bool` that indicates whether a robot can store the tile content.
/// - `cost`: An `usize` that indicates the cost to pass on the tile content.
///
#[derive(Debug, Clone)]
pub struct ContentProps {
    destroy: bool,
    max: usize,
    store: bool,
    cost: usize,
    // Other properties associated with the tile type
}

impl ContentProps {
    pub fn destroy(&self) -> bool {
        self.destroy
    }
    pub fn max(&self) -> usize {
        self.max
    }
    pub fn store(&self) -> bool {
        self.store
    }
    pub fn cost(&self) -> usize {
        self.cost
    }
}

// TileType

/// Represents the types of tiles in a map.
///
/// This enum defines various tile types that can be used to describe the terrain of individual
/// tiles on a map.
///
/// # Variants
/// - `DeepWater`: Deep water area
/// - `ShallowWater`: Shallow water area
/// - `Sand`: Sand area
/// - `Grass`: Grass area
/// - `Street`: Street area
/// - `Hill`: Hill area
/// - `Mountain`: Mountain area
/// - `Snow`: Snow area
/// - `Lava`: Lava area
///
/// # Usage
/// ```
/// use robotics_lib::world::tile::TileType;
/// let tile = TileType::Grass;
///
/// match tile {
///     TileType::Grass => println!("This tile is covered in grass."),
///     TileType::Street => println!("This tile is part of a street."),
///     _ => {}
/// }
/// ```
///;
///
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
pub enum TileType {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Street,
    Hill,
    Mountain,
    Snow,
    Lava,
}

impl TileType {
    pub fn properties(&self) -> &'static TileTypeProps {
        const fn gen_props(tile_type: TileType) -> TileTypeProps {
            match tile_type {
                | TileType::DeepWater => TileTypeProps {
                    cost: 0,
                    walk: false,
                    hold: &[
                        (Content::Rock(0), false),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), false),
                        (Content::Fire, false),
                        (Content::Coin(0), false),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), true),
                        (Content::None, false),
                    ],
                },
                | TileType::ShallowWater => TileTypeProps {
                    cost: 5,
                    walk: true,
                    hold: &[
                        (Content::Rock(0), false),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), false),
                        (Content::Fire, false),
                        (Content::Coin(0), false),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), true),
                        (Content::None, false),
                    ],
                },
                | TileType::Sand => TileTypeProps {
                    cost: 3,
                    walk: true,
                    hold: &[
                        (Content::Rock(0), true),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), true),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                    ],
                },
                | TileType::Grass => TileTypeProps {
                    cost: 1,
                    walk: true,
                    hold: &[
                        (Content::Rock(0), true),
                        (Content::Tree(0), true),
                        (Content::Garbage(0), true),
                        (Content::Fire, true),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), true),
                        (Content::Water(0), false),
                        (Content::None, true),
                    ],
                },
                | TileType::Street => TileTypeProps {
                    cost: 0,
                    walk: true,
                    hold: &[
                        (Content::Rock(0), true),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), true),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), true),
                        (Content::Water(0), false),
                        (Content::None, true),
                    ],
                },
                | TileType::Hill => TileTypeProps {
                    cost: 5,
                    walk: true,
                    hold: &[
                        (Content::Rock(0), true),
                        (Content::Tree(0), true),
                        (Content::Garbage(0), true),
                        (Content::Fire, true),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                    ],
                },
                | TileType::Mountain => TileTypeProps {
                    cost: 10,
                    walk: true,
                    hold: &[
                        (Content::Rock(0), true),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), true),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                    ],
                },
                | TileType::Snow => TileTypeProps {
                    cost: 3,
                    walk: true,
                    hold: &[
                        (Content::Rock(0), true),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), true),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                    ],
                },
                | TileType::Lava => TileTypeProps {
                    cost: 0,
                    walk: false,
                    hold: &[
                        (Content::Rock(0), false),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), false),
                        (Content::Fire, false),
                        (Content::Coin(0), false),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                    ],
                },
            }
        }
        match self {
            | TileType::DeepWater => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::DeepWater);
                &TILETYPE_PROPS
            }
            | TileType::ShallowWater => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::ShallowWater);
                &TILETYPE_PROPS
            }
            | TileType::Sand => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Sand);
                &TILETYPE_PROPS
            }
            | TileType::Grass => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Grass);
                &TILETYPE_PROPS
            }
            | TileType::Street => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Street);
                &TILETYPE_PROPS
            }
            | TileType::Hill => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Hill);
                &TILETYPE_PROPS
            }
            | TileType::Mountain => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Mountain);
                &TILETYPE_PROPS
            }
            | TileType::Snow => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Snow);
                &TILETYPE_PROPS
            }
            | TileType::Lava => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Lava);
                &TILETYPE_PROPS
            }
        }
    }
}

// ----------------------------------------------------
// Content

/// Represents the content of a tile. The (usize) is the max amount of the content that will be given when destroyed or the
/// max amount that can be stored inside it. (Decided by the generator)
///
/// This enum defines various entity types that can be placed in the contents of a tile.
///
/// # Variants
/// - `Rock`: Rock entity
/// - `Tree`: Tree entity
/// - `Garbage`: Garbage entity
/// - `Fire`: Fire entity
/// - `Coin`: Coin entity
/// - `Bin`: Bin entity
/// - `Crate`: Crate entity
/// - `Water`: Water entity
/// - `Bank`: Bank entity
/// - `None`: None entity
/// - `...`
///
/// # Usage
/// ```
/// use robotics_lib::world::tile::Content;
/// let entity = Content::Rock(20);
///
/// match entity {
///     Content::Rock(_) => println!("This is a solid rock in the game world."),
///     Content::Tree(_) => println!("A tall tree stands here."),
///     Content::Garbage(_) => println!("You find a pile of garbage."),
///     _ => {}
/// }
/// ```
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum Content {
    Rock(usize),
    Tree(usize),
    Garbage(usize),
    Fire,
    Coin(usize),
    Bin(Range<usize>),
    Crate(Range<usize>),
    Bank(Range<usize>),
    Water(usize),
    // polls for having at most 10 types, and what are they
    // going to be
    None,
}

impl Content {
    pub fn index(&self) -> usize {
        match self {
            | Content::Rock(_) => 0,
            | Content::Tree(_) => 1,
            | Content::Garbage(_) => 2,
            | Content::Fire => 3,
            | Content::Coin(_) => 4,
            | Content::Bin(_) => 5,
            | Content::Crate(_) => 6,
            | Content::Bank(_) => 7,
            | Content::Water(_) => 8,
            | Content::None => 9,
        }
    }
    pub fn to_default(&self) -> Self {
        match self {
            | Content::Coin(_) => Content::Coin(0),
            | Content::Garbage(_) => Content::Garbage(0),
            | Content::Water(_) => Content::Water(0),
            | Content::Rock(_) => Content::Rock(0),
            | Content::Tree(_) => Content::Tree(0),
            | Content::Bin(_) => Content::Bin(0..0),
            | Content::Bank(_) => Content::Bank(0..0),
            | Content::Crate(_) => Content::Crate(0..0),
            | other => other.clone(),
        }
    }
    pub fn properties(&self) -> &'static ContentProps {
        const fn gen_props(content: Content) -> ContentProps {
            match content {
                | Content::Rock(_) => ContentProps {
                    destroy: true,
                    max: 4,
                    store: false,
                    cost: 1,
                },
                | Content::Tree(_) => ContentProps {
                    destroy: true,
                    max: 5,
                    store: false,
                    cost: 3,
                },
                | Content::Garbage(_) => ContentProps {
                    destroy: true,
                    max: 3,
                    store: false,
                    cost: 4,
                },
                | Content::Fire => ContentProps {
                    destroy: true,
                    max: 0,
                    store: false,
                    cost: 5,
                },
                | Content::Coin(_) => ContentProps {
                    destroy: true,
                    max: 10,
                    store: true,
                    cost: 0,
                },
                | Content::Bin(_) => ContentProps {
                    destroy: false,
                    max: 10,
                    store: true,
                    cost: 0,
                },
                | Content::Crate(_) => ContentProps {
                    destroy: false,
                    max: 20,
                    store: true,
                    cost: 0,
                },
                | Content::Bank(_) => ContentProps {
                    destroy: false,
                    max: 50,
                    store: true,
                    cost: 0,
                },
                | Content::Water(_) => ContentProps {
                    destroy: true,
                    max: 20,
                    store: true,
                    cost: 3,
                },
                | Content::None => ContentProps {
                    destroy: false,
                    max: 0,
                    store: false,
                    cost: 0,
                },
            }
        }
        match self {
            | Content::Rock(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Rock(0));
                &CONTENT_PROPS
            }
            | Content::Tree(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Tree(0));
                &CONTENT_PROPS
            }
            | Content::Garbage(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Garbage(0));
                &CONTENT_PROPS
            }
            | Content::Fire => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Fire);
                &CONTENT_PROPS
            }
            | Content::Coin(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Coin(0));
                &CONTENT_PROPS
            }
            | Content::Bin(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Bin(0..0));
                &CONTENT_PROPS
            }
            | Content::Crate(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Crate(0..0));
                &CONTENT_PROPS
            }
            | Content::Bank(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Bank(0..0));
                &CONTENT_PROPS
            }
            | Content::Water(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Water(0));
                &CONTENT_PROPS
            }
            | Content::None => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::None);
                &CONTENT_PROPS
            }
        }
    }
}

// ----------------------------------------------------
// Tile

/// Represents a tile in the game world.
///
/// The `Tile` struct is used to define individual tiles within the game world. Each tile can have
/// a specific `TileType` that describes the terrain or content of the tile and may contain a
/// `Content` representing a static object on that tile.
///
///
/// # Fields
///
/// - `tile_type`: A `TileType`.
/// - `content`: A `Content`
///
///
/// # Usage
///
/// ```
/// // Create a new tile with a grassy type and no content.
/// use robotics_lib::world::tile::{Content, Tile, TileType};
///
/// // Create a tile representing a street with a garbage pile.
/// let street_tile = Tile {
///     tile_type: TileType::Grass,
///     content: Content::Garbage(2),
/// };
/// ```
#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub content: Content,
}
