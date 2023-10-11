use std::fmt;

use super::type_kinds::TypeKind;
use super::type_kinds::TYPE_COUNT;

/* Type is a bitmask of multiple TypeFlags. */
pub struct Type { 
    type_count: u8,
    types: [TypeKind; TYPE_COUNT as usize]
}

impl Type {
    /// We create an instance of Type using a vector of TypeKind.
    /// ```
    /// use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// 
    /// let types = Type::new(vec![TypeKind::Fire, TypeKind::Standard]);
    /// let other_types = Type::new(vec![TypeKind::Water]);
    /// ```
    /// The types will be set in the vec order.
    /// Type::new() will not accept duplicate entries and will panic.
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// // Will panic
    /// let types = Type::new(vec![TypeKind::Fire, TypeKind::Standard, TypeKind::Fire]);
    /// ```
    /// You also cannot use TypeKind::Invalid. Doing so will cause a panic.
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// // Will panic
    /// let types = Type::new(vec![TypeKind::Invalid]);
    /// ```
    pub fn new(in_types: Vec<TypeKind>) -> Type {
        assert!(in_types.len() > 0, "Cannot create an instance of Type with 0 types. See enum TypeKind");
        let mut type_data = Type {
            type_count: 0,
            types: [TypeKind::Invalid; TYPE_COUNT as usize]
        };
        for t in in_types {
            assert!(t != TypeKind::Invalid, "Cannot use TypeKind::Invalid as a type");
            type_data.add_type(t);
        }
        return type_data;
    }

    /// Check if the Type instance has a specific type.
    /// ```
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let types = Type::new(vec![TypeKind::Fire]);
    /// let is_type_present = types.has_type(TypeKind::Fire);
    /// assert!(is_type_present);
    /// ```
    /// It will check through the entire array.
    /// ```
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let types = Type::new(vec![TypeKind::Water, TypeKind::Metal, TypeKind::Dragon]);
    /// assert!(types.has_type(TypeKind::Dragon));
    /// ```
    pub fn has_type(&self, in_type: TypeKind) -> bool {
        for i in 0..self.type_count as usize {
            if self.types[i] == in_type  { return true; }
        }
        return false;
    }

    /// Adds a TypeKind to a mutable instance of Type.
    /// ```
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let mut types = Type::new(vec![TypeKind::Ground]);
    /// types.add_type(TypeKind::Water);
    /// assert!(types.has_type(TypeKind::Water));
    /// ```
    /// Will panic if the type is already present, as duplicates are not allowed
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let mut types = Type::new(vec![TypeKind::Air]);
    /// // Will panic
    /// types.add_type(TypeKind::Air);
    /// ```
    /// Will also panic if the type enum variant used is TypeKind::Invalid
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let mut types = Type::new(vec![TypeKind::Fire]);
    /// // Will panic
    /// types.add_type(TypeKind::Invalid);
    /// ```
    pub fn add_type(&mut self, in_type: TypeKind) {
        assert!(!self.has_type(in_type), "Type cannot contain duplicate types. Attempted to add type: {:?}\nThe current types are: {:?}", in_type, self.get_types());
        assert!(in_type != TypeKind::Invalid, "Cannot use TypeKind::Invalid as a type");
        debug_assert!(self.type_count < TYPE_COUNT as u8); // This should technically never happen because of preventing duplicate entries
        self.types[self.type_count as usize] = in_type;
        self.type_count += 1;
    }

    /// Get the types held within the Type instance as a new vector.
    /// ```
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let types = Type::new(vec![TypeKind::Light, TypeKind::Dark]);
    /// let v = types.get_types();
    /// assert!(v[0] == TypeKind::Light);
    /// assert!(v[1] == TypeKind::Dark);
    /// ```
    pub fn get_types(&self) -> Vec<TypeKind> {
        let mut v: Vec<TypeKind> = Vec::new();
        for t in self.iter() {
            v.push(t);
        }
        return v;
    }

    /// Get the number of types held within this Type instance.
    /// ```
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let types = Type::new(vec![TypeKind::Electric, TypeKind::Air, TypeKind::Metal, TypeKind::Dragon]);
    /// assert_eq!(types.get_type_count(), 4);
    /// ```
    pub fn get_type_count(&self) -> u8 {
        return self.type_count;
    }

    /// Get an iterator to the types held by this Type instance.
    /// ```
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let types = Type::new(vec![TypeKind::Fire, TypeKind::Water, TypeKind::Nature]);
    /// for t in types.iter() {
    ///     // Do some stuff
    /// }
    /// ```
    /// Will not exceed the number of types.
    /// ```
    /// # use immie2d_shared::gameplay::types::{type_data::Type, type_kinds::TypeKind};
    /// let types = Type::new(vec![TypeKind::Fire, TypeKind::Water, TypeKind::Nature]);
    /// let mut iterator = types.iter();
    /// assert_eq!(iterator.next().unwrap(), TypeKind::Fire);
    /// assert_eq!(iterator.next().unwrap(), TypeKind::Water);
    /// assert_eq!(iterator.next().unwrap(), TypeKind::Nature);
    /// assert!(iterator.next().is_none());
    /// ```
    pub fn iter(&self) -> TypeIter<'_> {
        return TypeIter { types: &self, index: 0 };
    }

}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type {{ type_count: {:?}, types: [", self.type_count)?;
        for i in 0..self.type_count {
            let t = self.types[i as usize];
            if i == self.type_count - 1 { // last iteration
                write!(f, "{}", t)?;
                continue;
            }
            write!(f, "{}, ", t)?; 
        }
        return write!(f, "] }}");
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

pub struct TypeIter<'a> {
    types: &'a Type,
    index: u8
}

impl<'a> Iterator for TypeIter<'a> {
    type Item = TypeKind;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.types.type_count {
            return None;
        }
        self.index += 1;
        return Some(self.types.types[self.index as usize - 1]);
    }
}