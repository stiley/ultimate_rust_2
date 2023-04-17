// 1. Thank you for volunteering to document our pumpkin library! Let's start by grabbing the first
// paragraph from https://en.wikipedia.org/wiki/Pumpkin and pasting it as our module-level
// documentation. Hint: Use inner-documentation comments.
//
// Once you've got the documentation here, run `cargo doc --no-deps --open` and take a look!

//!A pumpkin is a vernacular term for mature winter squash of species and varieties in the genus Cucurbita that has culinary and cultural significance1 2 but no agreed upon botanical or scientific meaning.3
//! The term pumpkin is sometimes used interchangeably with "squash" or "winter squash", and is commonly used for cultivars of Cucurbita argyrosperma, Cucurbita ficifolia, Cucurbita maxima, Cucurbita moschata, and Cucurbita pepo.1
//! Native to North America (northeastern Mexico and the southern United States), C. pepo pumpkins are one of the oldest domesticated plants, having been used as early as 7,000 to 5,500 BC. Today, pumpkins of varied species are widely
//! grown for food, as well as for aesthetic and recreational purposes.4 The pumpkin's thick shell contains edible seeds and pulp. Pumpkin pie, for instance, is a traditional part of Thanksgiving meals in Canada and the United States, and pumpkins are frequently carved as jack-o'-lanterns for decoration around Halloween,
//! although commercially canned pumpkin purée and pumpkin pie fillings are usually made of different pumpkin varieties from those used for jack-o'-lanterns.5
//!
//![Pumpkin image](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg)
//!

// 2. What about an image!? Add an image of a pumpkin to the end of the module-level documentation.
// The markdown format is ![some alt text](https://url-to-the-image.png)
// Here's the image to link to: https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg

// 3. Document the Pumpkin struct.
// - The description on the index page should be "Big orange thing"
// - Make a section header called "Recipes"
// - Explain that recipes will be coming soon.
// - Document the "roundness" field, explaining that it is a percentage
// - Document the "orangeness" field, explaining that it is a number from 8 to 27

/// Big orange thing
///
/// # Recipies
///
/// Recipies coming soon!
pub struct Pumpkin {
    /// `roundness` is a percentage
    pub roundness: f32,
    /// `orangeness` is a number from 8 -> 27
    pub orangeness: i32,
}

// 4. Document the "smash" method. Explain that if you smash the pumpkin, it will be gone. Then it
// can't be used for pie. :'-(

impl Pumpkin {
    /// if you smash the pumpkin, it will be gone.
    /// Then it can't be used for pie. 😭
    pub fn smash(self) {}
}

// 5. Document that BURNT_ORANGE is for the "orangeness" field in the Pumpkin struct.
// - Link to the Pumpkin struct in your description
/// `BURNT_ORANGE` is for the `orangeness` field in the [Pumpkin] struct
pub const BURNT_ORANGE: i32 = 13;

// Challenge: Find the option to pass to `cargo doc` so that documentation for this private item
// gets generated as well.  Hint: `cargo doc -h` will show you all the relevant options.

/// For internal use only. In fact, this documentation is so private that it won't be generated.
/// At least not by default. But if you pass the correct option in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}
