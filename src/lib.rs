pub mod action;
pub mod batchneed;
pub mod beer;
pub mod equipment;
pub mod equipment_group;
pub mod factory;
pub mod interval;
pub mod plan;
pub mod recipy;
pub mod step_group;
pub mod steps;
pub mod style;
pub mod system;
pub mod r#type;
pub mod volume;
pub mod work;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
