use rand::Rng;
use rand;
use std::fmt;


pub struct GeneticByte {
    value: u8,
    byte_type: bool
}


impl GeneticByte {
    /// # Purpose
    ///
    /// This function will create a new Genetic byte initialized
    /// to a random value or operator depending on it's type.
    ///
    /// # Parameters
    ///
    /// byte_type: the GeneticByte's type
    ///
    /// # return
    ///
    /// A new Genetic Byte initialized to random values
    pub fn new(byte_type: bool) -> GeneticByte {
        if byte_type {
            let value = rand::thread_rng().gen_range(0, 255);
            GeneticByte { value: value, byte_type: true }
        } else {
            let mut value = rand::thread_rng().gen_range(1, 5);
            value = match value {
                1 => '+',
                2 => '-',
                3 => '*',
                4 => '/',
                _ => panic!("you messed up")
            } as u8;
            GeneticByte { value: value, byte_type: false }
        }
    }

    /// # Purpose
    ///
    /// This function will get the GeneticByte's value and return it to the caller.
    ///
    /// # Parameters
    ///
    /// # Return
    ///
    /// The GeneticByte's value
    pub fn get_value(&self) -> u8 {
        self.value
    }

    /// # Purpose
    ///
    /// This function will get the GeneticByte's type and return it to the caller.
    ///
    /// # Parameters
    ///
    /// # Return
    ///
    /// The genetic byte's type.
    pub fn get_type(&self) -> bool {
        self.byte_type
    }

    /// # Purpose
    /// This function will mutate the Genetic byte in the following manner:
    ///
    /// - A GeneticByte's operator operator may change if GeneticByte type = false
    /// - A GeneticByte's value WILL be changed if GeneticByte type = true
    ///
    /// # Parameters
    /// - byte_type: A Genetic Byte's type -
    ///
    /// # Return
    /// none
    pub fn mutate(&mut self, byte_type: bool) {
        if byte_type {
            let bit_to_flip = rand::thread_rng().gen_range(0, 8);
            self.mutate_value(bit_to_flip as u32);
        } else {
            let value = rand::thread_rng().gen_range(1, 5);
            self.value = match value {
                1 => '+',
                2 => '-',
                3 => '*',
                4 => '/',
                _ => panic!("you messed up")
            } as u8;
        }
    }

    /// # Purpose
    /// This function will take a bit index and flip that bit
    /// (e.g. 1 -> 0, 0 -> 1) in the GeneticByte.
    /// # Parameters
    /// bit_to_flip: u32 - the bit index
    ///
    /// # Return
    /// none
    fn mutate_value(&mut self, bit_to_flip: u32)
    {
        let mut u8_bit_representation = [false; 8];
        let mut u8_value = self.value;
        for bool in &mut u8_bit_representation[..] {
            *bool = match u8_value % 2 {
                0 => false,
                1 => true,
                _ => panic!("This should be impossible!!!!")
            };
            u8_value /= 2
        }
        if u8_bit_representation[bit_to_flip as usize] == true {
            self.value -= 2u8.pow(bit_to_flip)
        } else {
            self.value += 2u8.pow(bit_to_flip)
        }
    }
}

impl Clone for GeneticByte {
    fn clone(&self) -> GeneticByte {
        let b_type = self.byte_type;
        let b_val = self.value;
        GeneticByte { byte_type: b_type, value: b_val }
    }
}

/// # Purpose
/// make the genetic byte displayable.
///
/// # Parameters
/// Formatter - Used to determine the format of the string
///
/// # Returns
/// The formatted string for display
impl fmt::Display for GeneticByte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.byte_type {
            write!(f, "{} ", self.value)
        } else {
            write!(f, "{} ", self.value as char)
        }
    }
}

#[cfg(tests)]
mod tests{
    use super::*;
    #[test]
    fn mutate_chages_1_byte() {
        let mut g_byte = GeneticByte::new(true);
        let current_value = g_byte.get_value();
        g_byte.mutate(true);
        let new_value = g_byte.get_value();
        let change_in_value: i16 = current_value - new_value;

    }

    fn one_bit_changed(original_value: u8, new_value: u8) -> bool {
        let change_in_value: i16 = original_value - new_value;
        let number_bits_changed = 1;
        while change_in_value != 0 {

        }
    }

}
