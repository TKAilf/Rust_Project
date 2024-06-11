use publish_crates::PrimaryColor;
use publish_crates::SecondaryColor;
use publish_crates::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let mix = mix(red, yellow);
    match mix {
        SecondaryColor::Orange => println!("Orange"),
        SecondaryColor::Green => println!("Green"),
        SecondaryColor::Purple => println!("Purple"),
    }
}
