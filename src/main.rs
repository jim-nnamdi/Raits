
mod base_traits;
mod derive_traits;
mod dyn_traits;

fn main() {
    derive_traits::cmp_vals("metro", "Metr0");
    dyn_traits::Return_Trait(2, "Lucas", 32);
    println!("Hello, world!");
}
