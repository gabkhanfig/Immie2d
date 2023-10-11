use std::fmt;

use super::type_kinds::TypeKind;
use super::type_kinds::TYPE_COUNT;

/* Type is a bitmask of multiple TypeFlags. */
pub struct Type { 
    type_count: u8,
    types: [TypeKind; TYPE_COUNT as usize] // consumes less memory than vec.
}

impl Type {
    pub fn new(in_types: Vec<TypeKind>) -> Type {
        assert!(in_types.len() > 0, "Cannot create an instance of Type with 0 types. See enum TypeKind");
        let mut type_data = Type {
            type_count: 0,
            types: [TypeKind::Invalid; TYPE_COUNT as usize]
        };
        for t in in_types {
            type_data.add_type(t);
        }
        return type_data;
    }

    pub fn has_type(&self, in_type: TypeKind) -> bool {
        for i in 0..self.type_count as usize {
            if self.types[i] == in_type  { return true; }
        }
        return false;
    }

    pub fn add_type(&mut self, in_type: TypeKind) {
        assert!(!self.has_type(in_type), "Type cannot contain duplicate types. Attempted to add type: {:?}\nThe current types are: {:?}", in_type, self.get_types());
        self.types[self.type_count as usize] = in_type;
        self.type_count += 1;
    }

    pub fn get_types(&self) -> Vec<TypeKind> {
        let mut v: Vec<TypeKind> = Vec::new();
        for t in self.iter() {
            v.push(t);
        }
        return v;
    }

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
    index: usize
}

impl<'a> Iterator for TypeIter<'a> {
    type Item = TypeKind;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.types.type_count as usize {
            return None;
        }
        self.index += 1;
        return Some(self.types.types[self.index - 1]);
    }
}