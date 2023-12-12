use image::Rgb;

/// Black color (black)
pub(crate) const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

pub(crate) mod tile {
    use image::Rgb;

    /// DeepWater color (deep blue)
    pub(crate) const DEEP_WATER: Rgb<u8> = Rgb([5, 25, 90]);
    /// ShallowWater color (Dolce & Gabbana light blue)
    pub(crate) const SHALLOW_WATER: Rgb<u8> = Rgb([45, 100, 160]);
    /// Sand color (plaid yellow)
    pub(crate) const SAND: Rgb<u8> = Rgb([240, 230, 140]);
    /// Grass color (Minecraft plain grass green)
    pub(crate) const GRASS: Rgb<u8> = Rgb([74, 111, 40]);
    /// Street color (dark grey)
    pub(crate) const STREET: Rgb<u8> = Rgb([90, 90, 90]);
    /// Hill color (light soil brown)
    pub(crate) const HILL: Rgb<u8> = Rgb([146, 104, 41]);
    /// Mountain color (Minecraft stone grey)
    pub(crate) const MOUNTAIN: Rgb<u8> = Rgb([160, 160, 160]);
    /// Snow color (off white)
    pub(crate) const SNOW: Rgb<u8> = Rgb([250, 249, 246]);
    /// Lava color (Minecraft lava orange)
    pub(crate) const LAVA: Rgb<u8> = Rgb([255, 129, 0]);
    /// Brick color (brick red)
    pub(crate) const BRICK: Rgb<u8> = Rgb([188, 74, 60]);
}

pub(crate) mod content {
    use image::Rgb;

    /// Minecraft oak wood (brown)
    pub(crate) const TREE: Rgb<u8> = Rgb([99, 73, 43]);
    /// Rock color (light brown)
    pub(crate) const ROCK: Rgb<u8> = Rgb([240, 223, 206]);
    /// Fire color (pastel orange)
    pub(crate) const FIRE: Rgb<u8> = Rgb([255, 0, 0]);
    /// Coin color (Nintendo gold coin)
    pub(crate) const COIN: Rgb<u8> = Rgb([243, 199, 13]);
    /// Bin color (black cast iron)
    pub(crate) const BIN: Rgb<u8> = Rgb([57, 60, 65]);
    /// Bank color (marble white)
    pub(crate) const BANK: Rgb<u8> = Rgb([227, 224, 205]);
    /// Market color (Minecraft oak planks)
    pub(crate) const MARKET: Rgb<u8> = Rgb([145, 117, 77]);
    /// Fish color (red goldfish)
    pub(crate) const FISH: Rgb<u8> = Rgb([240, 79, 40]);
    /// Building color (iron grey)
    pub(crate) const BUILDING: Rgb<u8> = Rgb([203, 205, 205]);
    /// Bush color (dark green)
    pub(crate) const BUSH: Rgb<u8> = Rgb([17, 64, 46]);
    /// Scarecrow color (yellow hay)
    pub(crate) const SCARECROW: Rgb<u8> = Rgb([218, 197, 134]);
    /// Jolly block (Minecraft chance cube light blue)
    pub(crate) const JOLLYBLOCK: Rgb<u8> = Rgb([79, 120, 143]);
    /// Crate color (birch wood light brown)
    pub(crate) const CRATE: Rgb<u8> = Rgb([228, 199, 148]);
}
