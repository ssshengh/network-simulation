pub mod algorithm;
mod communication;
mod computing;
pub mod core;
mod location_distribution;
pub mod settings;
mod terminal;
mod user_equipment;
mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
