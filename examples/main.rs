use identicon_rs::error::IdenticonError;
use identicon_rs::Identicon;

fn main() -> Result<(), IdenticonError> {
    let conways_glider = String::from("conways-glider");
    let test_string = "identicon_rs";

    // Stored example
    let identicon_conways_glider = Identicon::new(conways_glider);
    identicon_conways_glider.save_image("output_1.png")?;

    // Chained example with no border
    Identicon::new(test_string)
        .set_border(0)
        .set_color((68, 153, 58))
        .save_image("output_2.png")?;
    Ok(())
}
