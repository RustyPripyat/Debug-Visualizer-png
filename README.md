# Exclusion Zone world generator

This crate defines the world generator **Exclusion Zone**, your robot will be swept into an arduous map, full of pitfalls, just like the Černobyl exclusion zone, you will feel like you are in the 1986 USSR.

This world generator allows for granular customisation of its content, it allows you to specify the order in which the various Tile types and Tile content are generated, allowing you to define priorities. To ensure the best possible performance, multi-threading is exploited wherever possible.

There are methods to **pre-generate the world**, save it as a binary file and load it later.

We recommend a size of at least **1000**, **a dimension lower than 100 wil throw a panic**
