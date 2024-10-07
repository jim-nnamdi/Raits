
mod base_traits;
mod derive_traits;
mod dyn_traits;
mod overloading;

fn main() {
    derive_traits::cmp_vals("metro", "Metr0");
    dyn_traits::Return_Trait(2, "Lucas", 32);
    overloading::add_double_structs();
    println!("Hello, world!");
}
