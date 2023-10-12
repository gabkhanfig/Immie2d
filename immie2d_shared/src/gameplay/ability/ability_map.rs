use std::collections::HashMap;

use super::ability::Ability;

pub struct AbilityMap {
    map: HashMap<&'static str, fn() -> Box<dyn Ability>>
}

impl AbilityMap {
    pub fn new() -> Self {
        return AbilityMap { map: HashMap::new() };
    }

    /// Dependency inject ability.
    /// ```
    /// use immie2d_shared::gameplay::ability::{ability::AbilityMap, abilities::fireball::Fireball};
    /// let mut map = AbilityMap::new();
    /// map.add_ability::<Fireball>();
    /// ```
    pub fn add_ability<T: Ability>(&mut self) {
        self.map.insert(T::static_name(), T::new);
    }

    /// Create a new instance of Ability.
    /// ```
    /// # use immie2d_shared::gameplay::ability::{ability::AbilityMap, abilities::fireball::Fireball};
    /// let mut map = AbilityMap::new();
    /// map.add_ability::<Fireball>();
    /// let ability = map.new_ability("fireball");
    /// ```
    /// Will panic if ability name doesn't exist. See AbilityMap::is_ability_name()
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::ability::{ability::AbilityMap, abilities::fireball::Fireball};
    /// # let mut map = AbilityMap::new();
    /// # map.add_ability::<Fireball>();
    /// // Will panic
    /// let ability2 = map.new_ability("aksdaiuhsdpiauhsd");
    /// ```
    pub fn new_ability(&self, name: &str) -> Box<dyn Ability> {
        let entry = self.map.get(name).expect(format!("Ability name [{}] is not valid", name).as_str());
        return entry();
    }

    /// Check if an ability name is valid.
    /// ```
    /// # use immie2d_shared::gameplay::ability::{ability::AbilityMap, abilities::fireball::Fireball};
    /// let mut map = AbilityMap::new();
    /// map.add_ability::<Fireball>();
    /// assert!(map.is_ability_name("fireball") == true);
    /// assert!(map.is_ability_name("wuhafjnb") == false);
    /// ```
    pub fn is_ability_name(&self, name: &str) -> bool {
        return self.map.contains_key(name);
    }

}