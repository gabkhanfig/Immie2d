use super::super::types::type_data::Type;

pub trait Ability {
    fn new() -> Box<dyn Ability>
    where Self: Sized;

    fn get_name(&self) -> &'static str;

    fn static_name() -> &'static str
    where Self: Sized;

    fn get_base_ability_data(&self) -> &BaseAbilityData;
    
    fn get_base_ability_data_mut(&mut self) -> &mut BaseAbilityData;
}

pub enum AbilityCategory {
    Attack,
    Status
}

pub struct BaseAbilityData {
    pub category: AbilityCategory,
    pub types: Type,
    pub power: f32,
    pub speed: f32,
}


