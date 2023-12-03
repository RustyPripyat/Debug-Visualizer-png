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
    /// Gray color (straight grey)
    pub(crate) const GRAY: Rgb<u8> = Rgb([128, 128, 128]);
    /// Black color (black)
    pub(crate) const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
    /// Brick color (brick red)
    pub(crate) const BRICK: Rgb<u8> = Rgb([188, 74, 60]);
}

pub(crate) mod content {
    use image::Rgb;

    /// Garbage color (solid yellow)
    pub(crate) const GARBAGE: Rgb<u8> = Rgb([255, 232, 28]);
}