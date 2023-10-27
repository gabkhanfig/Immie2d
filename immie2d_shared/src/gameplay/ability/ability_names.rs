use crate::engine_types::global_string::GlobalString;

pub const MAX_ABILITIES_COUNT: usize = 5;

/* Container to store the names of abilities */
#[derive(Clone, Copy)]
pub struct AbilityNames {
    names: [GlobalString; MAX_ABILITIES_COUNT],
    count: usize
}

impl AbilityNames {
    /// Creates an instance with no abilities.
    /// ```
    /// use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// let abilities = AbilityNames::default();
    /// assert_eq!(abilities.get_count(), 0);
    /// ```
    pub fn default() -> AbilityNames {
        return AbilityNames {
            names: [GlobalString::default(); MAX_ABILITIES_COUNT],
            count: 0
        }
    }

    /// Creates an instance with some abilities, up to MAX_ABILITIES_COUNT
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// let abilities = AbilityNames::new(vec![GlobalString::new(&"fireball".to_string())]);
    /// assert_eq!(abilities.get_count(), 1);
    /// assert_eq!(abilities.get_names()[0], GlobalString::new(&"fireball".to_string()));
    /// ```
    /// The abilities will be set in the vec order.
    /// AbilityNames::new() will not accept duplicate entries and will panic.
    /// ``` should_panic
    /// # use immie2d_shared::engine_types::global_string::GlobalString;
    /// # use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// // Will panic
    /// let name = GlobalString::new(&"fireball".to_string());
    /// let abilities = AbilityNames::new(vec![name.clone(), name.clone()]);
    /// ```
    /// Will also panic if the vec of ability names is longer than MAX_ABILITIES_COUNT
    /// ``` should_panic
    /// # use immie2d_shared::engine_types::global_string::GlobalString;
    /// # use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// // Will panic
    /// let abilities = AbilityNames::new(vec![GlobalString::new(&"a".to_string()), GlobalString::new(&"b".to_string()), GlobalString::new(&"c".to_string()), GlobalString::new(&"d".to_string()), GlobalString::new(&"e".to_string()), GlobalString::new(&"f".to_string())]);
    /// ```
    pub fn new(in_abilities: Vec<GlobalString>) -> AbilityNames {
        assert!(in_abilities.len() <= MAX_ABILITIES_COUNT, "Cannot create an instance of AbilityNames with more abilities than the max of {}", MAX_ABILITIES_COUNT);
        let mut ability_names = AbilityNames::default();
        for name in in_abilities {
            ability_names.add_ability(name);
        }
        return ability_names;
    }

    /// Checks if the AbilityNames instance has a specific ability name.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// 
    /// let abilities = AbilityNames::new(vec![GlobalString::new(&"fireball".to_string())]);
    /// assert!(abilities.has_ability(GlobalString::new(&"fireball".to_string())));
    /// assert!(!abilities.has_ability(GlobalString::new(&"something else".to_string())));
    /// ```
    pub fn has_ability(&self, in_ability: GlobalString) -> bool {
        for i in 0..self.count {
            if self.names[i] == in_ability {return true;}
        }
        return false;
    }

    /// Adds an ability name.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// 
    /// let mut abilities = AbilityNames::default();
    /// abilities.add_ability(GlobalString::new(&"fireball".to_string()));
    /// assert_eq!(abilities.get_count(), 1);
    /// ```
    /// Will panic if the ability is a duplicate of one already contained.
    /// ``` should_panic
    /// # use immie2d_shared::engine_types::global_string::GlobalString;
    /// # use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// let mut abilities = AbilityNames::new(vec![GlobalString::new(&"fireball".to_string())]);
    /// // Will panic
    /// abilities.add_ability(GlobalString::new(&"fireball".to_string()));
    /// ```
    /// Will also panic if the ability count would exceed the maximum of MAX_ABILITIES_COUNT
    /// ``` should_panic
    /// # use immie2d_shared::engine_types::global_string::GlobalString;
    /// # use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// let mut abilities = AbilityNames::new(vec![GlobalString::new(&"a".to_string()), GlobalString::new(&"b".to_string()), GlobalString::new(&"c".to_string()), GlobalString::new(&"d".to_string()), GlobalString::new(&"e".to_string())]);
    /// // Will panic
    /// abilities.add_ability(GlobalString::new(&"f".to_string()));
    /// ```
    pub fn add_ability(&mut self, in_ability: GlobalString) {
        assert!(!self.has_ability(in_ability), "AbilityNames cannot contain duplicate names. Attempted to add ability name: {}\nThe current names are: {:?}", in_ability, self.get_names());
        assert!(self.count < MAX_ABILITIES_COUNT, "Cannot add another ability. All ability name slots are occupied. Max allowed is {}", MAX_ABILITIES_COUNT);
        self.names[self.count] = in_ability;
        self.count += 1;
    }

    /// Get the number of ability names contained.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// let abilities = AbilityNames::new(vec![GlobalString::new(&"fireball".to_string())]);
    /// assert_eq!(abilities.get_count(), 1);
    /// ```
    pub fn get_count(&self) -> usize { 
        return self.count; 
    }

    /// Get the ability names held as a new vector.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// let abilities = AbilityNames::new(vec![GlobalString::new(&"a".to_string()), GlobalString::new(&"b".to_string())]);
    /// let v = abilities.get_names();
    /// assert_eq!(v[0], GlobalString::new(&"a".to_string()));
    /// assert_eq!(v[1], GlobalString::new(&"b".to_string()));
    /// ```
    pub fn get_names(&self) -> Vec<GlobalString> {
        let mut v: Vec<GlobalString> = Vec::new();
        for i in 0..self.count {
            v.push(self.names[i].clone());
        }
        return v;
    }

    /// Get an iterator to the ability names held by this AbilityNames instance.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// let abilities = AbilityNames::new(vec![GlobalString::new(&"a".to_string()), GlobalString::new(&"b".to_string())]);
    /// for name in abilities.iter() {
    ///     // Do some stuff
    /// }
    /// ```
    /// Will not exceed the number of elements.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// use immie2d_shared::gameplay::ability::ability_names::AbilityNames;
    /// let abilities = AbilityNames::new(vec![GlobalString::new(&"a".to_string()), GlobalString::new(&"b".to_string())]);
    /// let mut iterator = abilities.iter();
    /// assert_eq!(iterator.next().unwrap(), GlobalString::new(&"a".to_string()));
    /// assert_eq!(iterator.next().unwrap(), GlobalString::new(&"b".to_string()));
    /// assert!(iterator.next().is_none());
    /// ```
    pub fn iter(&self) -> AbilityNamesIter<'_> {
        return AbilityNamesIter { ability_names: &self, index: 0 }
    }
}

pub struct AbilityNamesIter<'a> {
    ability_names: &'a AbilityNames,
    index: usize
}

impl<'a> Iterator for AbilityNamesIter<'a> {
    type Item = GlobalString;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.ability_names.count {
            return None;
        }
        self.index += 1;
        return Some(self.ability_names.names[self.index - 1]);
    }
}