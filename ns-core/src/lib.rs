mod base_station;
mod communication;
mod computing;
mod location_distribution;
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
