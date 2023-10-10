/* Used as bitshift */
#[repr(u32)]
pub enum TypeKind {
    Standard = 0,
    Fire = 1,
    Water = 2,
    Nature = 3,
    Electric = 4,
    Air = 5,
    Ground = 6,
    Metal = 7,
    Light = 8,
    Dark = 9,
    Dragon = 10
}

/* Type is a bitmask of multiple TypeFlags. */
pub struct Type {
    bitmask: u32
}

impl Type {

    pub fn new(in_types: Vec<TypeKind>) -> Type {
        assert!(in_types.len() > 0, "Cannot create an instance of Type with 0 types. See enum TypeKind");
        let mut t = Type {
            bitmask: 0,
        };
        for in_type in in_types {
            t.bitmask |= 1 << in_type as u32;
        }
        return t;
    }

    pub fn has_type(&self, in_type: TypeKind) -> bool {
        return self.bitmask & (1 << in_type as u32) != 0;
    }
}


