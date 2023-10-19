use std::fmt;

use super::element_kinds::ElementKind;
use super::element_kinds::ELEMENT_COUNT;

/* Elements is a bitmask of multiple ElementsFlags. */
pub struct Elements { 
    elements_count: u8,
    elements: [ElementKind; ELEMENT_COUNT as usize]
}

impl Elements {
    /// We create an instance of Elements using a vector of ElementKind.
    /// ```
    /// use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// 
    /// let Elementss = Elements::new(vec![ElementKind::Fire, ElementKind::Standard]);
    /// let other_Elementss = Elements::new(vec![ElementKind::Water]);
    /// ```
    /// The Elementss will be set in the vec order.
    /// Elements::new() will not accept duplicate entries and will panic.
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// // Will panic
    /// let Elementss = Elements::new(vec![ElementKind::Fire, ElementKind::Standard, ElementKind::Fire]);
    /// ```
    /// You also cannot use ElementKind::Invalid. Doing so will cause a panic.
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// // Will panic
    /// let Elementss = Elements::new(vec![ElementKind::Invalid]);
    /// ```
    pub fn new(in_Elementss: Vec<ElementKind>) -> Elements {
        assert!(in_Elementss.len() > 0, "Cannot create an instance of Elements with 0 Elementss. See enum ElementKind");
        let mut elements_data = Elements {
            elements_count: 0,
            elements: [ElementKind::Invalid; ELEMENT_COUNT as usize]
        };
        for t in in_Elementss {
            assert!(t != ElementKind::Invalid, "Cannot use ElementKind::Invalid as a Elements");
            elements_data.add_Elements(t);
        }
        return elements_data;
    }

    /// Check if the Elements instance has a specific Elements.
    /// ```
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let Elementss = Elements::new(vec![ElementKind::Fire]);
    /// let is_Elements_present = Elementss.has_Elements(ElementKind::Fire);
    /// assert!(is_Elements_present);
    /// ```
    /// It will check through the entire array.
    /// ```
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let Elementss = Elements::new(vec![ElementKind::Water, ElementKind::Metal, ElementKind::Dragon]);
    /// assert!(Elementss.has_Elements(ElementKind::Dragon));
    /// ```
    pub fn has_Elements(&self, in_Elements: ElementKind) -> bool {
        for i in 0..self.elements_count as usize {
            if self.elements[i] == in_Elements  { return true; }
        }
        return false;
    }

    /// Adds a ElementKind to a mutable instance of Elements.
    /// ```
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let mut Elementss = Elements::new(vec![ElementKind::Ground]);
    /// Elementss.add_Elements(ElementKind::Water);
    /// assert!(Elementss.has_Elements(ElementKind::Water));
    /// ```
    /// Will panic if the Elements is already present, as duplicates are not allowed
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let mut Elementss = Elements::new(vec![ElementKind::Air]);
    /// // Will panic
    /// Elementss.add_Elements(ElementKind::Air);
    /// ```
    /// Will also panic if the Elements enum variant used is ElementKind::Invalid
    /// ``` should_panic
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let mut Elementss = Elements::new(vec![ElementKind::Fire]);
    /// // Will panic
    /// Elementss.add_Elements(ElementKind::Invalid);
    /// ```
    pub fn add_Elements(&mut self, in_Elements: ElementKind) {
        assert!(!self.has_Elements(in_Elements), "Elements cannot contain duplicate Elementss. Attempted to add Elements: {:?}\nThe current Elementss are: {:?}", in_Elements, self.get_Elementss());
        assert!(in_Elements != ElementKind::Invalid, "Cannot use ElementKind::Invalid as a Elements");
        debug_assert!(self.elements_count < ELEMENT_COUNT as u8); // This should technically never happen because of preventing duplicate entries
        self.elements[self.elements_count as usize] = in_Elements;
        self.elements_count += 1;
    }

    /// Get the Elementss held within the Elements instance as a new vector.
    /// ```
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let Elementss = Elements::new(vec![ElementKind::Light, ElementKind::Dark]);
    /// let v = Elementss.get_Elementss();
    /// assert!(v[0] == ElementKind::Light);
    /// assert!(v[1] == ElementKind::Dark);
    /// ```
    pub fn get_Elementss(&self) -> Vec<ElementKind> {
        let mut v: Vec<ElementKind> = Vec::new();
        for t in self.iter() {
            v.push(t);
        }
        return v;
    }

    /// Get the number of Elementss held within this Elements instance.
    /// ```
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let Elementss = Elements::new(vec![ElementKind::Electric, ElementKind::Air, ElementKind::Metal, ElementKind::Dragon]);
    /// assert_eq!(Elementss.get_Elements_count(), 4);
    /// ```
    pub fn get_Elements_count(&self) -> u8 {
        return self.elements_count;
    }

    /// Get an iterator to the Elementss held by this Elements instance.
    /// ```
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let Elementss = Elements::new(vec![ElementKind::Fire, ElementKind::Water, ElementKind::Nature]);
    /// for t in Elementss.iter() {
    ///     // Do some stuff
    /// }
    /// ```
    /// Will not exceed the number of Elementss.
    /// ```
    /// # use immie2d_shared::gameplay::Elementss::{elements_data::Elements, element_kinds::ElementKind};
    /// let Elementss = Elements::new(vec![ElementKind::Fire, ElementKind::Water, ElementKind::Nature]);
    /// let mut iterator = Elementss.iter();
    /// assert_eq!(iterator.next().unwrap(), ElementKind::Fire);
    /// assert_eq!(iterator.next().unwrap(), ElementKind::Water);
    /// assert_eq!(iterator.next().unwrap(), ElementKind::Nature);
    /// assert!(iterator.next().is_none());
    /// ```
    pub fn iter(&self) -> ElementIter<'_> {
        return ElementIter { elements: &self, index: 0 };
    }

}

impl fmt::Debug for Elements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Elements {{ Elements_count: {:?}, Elementss: [", self.elements_count)?;
        for i in 0..self.elements_count {
            let t = self.elements[i as usize];
            if i == self.elements_count - 1 { // last iteration
                write!(f, "{}", t)?;
                continue;
            }
            write!(f, "{}, ", t)?; 
        }
        return write!(f, "] }}");
    }
}

impl fmt::Display for Elements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

pub struct ElementIter<'a> {
    elements: &'a Elements,
    index: u8
}

impl<'a> Iterator for ElementIter<'a> {
    type Item = ElementKind;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elements.elements_count {
            return None;
        }
        self.index += 1;
        return Some(self.elements.elements[self.index as usize - 1]);
    }
}