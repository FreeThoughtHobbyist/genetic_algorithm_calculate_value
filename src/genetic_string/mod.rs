mod genetic_byte;
use std::fmt;


///
pub struct GeneticString {
    vector: Vec<genetic_byte::GeneticByte>
}

trait GeneticStringTestUtilities {
    fn new(Vec<genetic_byte::GeneticByte>) -> GeneticString;
}

impl GeneticStringTestUtilities for GeneticString {
    /// # Purpose
    /// Create a genetic strings whose value can be precomputed. Used for testing purposes.
    ///
    /// # Parameters
    /// g_byte_vec - a vector of genetic bytes
    ///
    /// # Returns
    /// a genetic string
    fn new (g_byte_vec: Vec<genetic_byte::GeneticByte>) -> GeneticString {
        GeneticString{vector: g_byte_vec}
    }
}

impl GeneticString {
    /// # Purpose
    ///
    /// This function will create a new genetic string
    ///
    /// # Parameters
    ///
    /// initial_number_of_values, how many values should the genetic string start with.
    ///
    /// # return
    ///
    /// a new genetic string
    ///
    pub fn new(initial_number_of_values: u32) -> GeneticString {
        let mut genetic_string_vec: Vec<genetic_byte::GeneticByte> = vec!();
        for x in 0..initial_number_of_values {
            genetic_string_vec.push(genetic_byte::GeneticByte::new(true));
            if x != initial_number_of_values - 1 {
                genetic_string_vec.push(genetic_byte::GeneticByte::new(false))
            }
        }
        GeneticString { vector: genetic_string_vec }
    }

    /// Returns the vector of genetic bytes contained in the genetic string.
    ///
    /// # Parameters
    ///
    /// # Returns
    /// The vector of genetic bytes
    pub fn get_vector(&self) -> Vec<genetic_byte::GeneticByte> {
        self.vector.clone()
    }


    /// # Purpose
    /// Take a genetic string slice and compute the value of that slice
    ///
    /// # Parameters
    /// gen_str_slice - The genetic string slice
    ///
    /// # returns
    /// The genetic string slices value.
    pub fn calculate_value(gen_str_slice: &[genetic_byte::GeneticByte]) -> i32 {
        if gen_str_slice.len() == 1 {
            return gen_str_slice[0].get_value() as i32
        } else {
            for index in 0..gen_str_slice.len() {
                if gen_str_slice[index].get_type() == false {
                    if GeneticString::order_of_ops(gen_str_slice[index].get_value() as char) {
                        return match gen_str_slice[index].get_value() as char {
                            '+' => GeneticString::calculate_value(&gen_str_slice[0..index]) +
                                GeneticString::calculate_value(&gen_str_slice[index + 1..gen_str_slice.len()]),
                            '-' => GeneticString::calculate_value(&gen_str_slice[0..index]) -
                                GeneticString::calculate_value(&gen_str_slice[index + 1..gen_str_slice.len()]),
                            _ => panic!("Should never get here")
                        };
                    }
                }
            }

            for index in 0..gen_str_slice.len() {
                if gen_str_slice[index].get_type() == false {
                    //print!("value {}, ", gen_str_slice[index].get_value() as char);
                    return match gen_str_slice[index].get_value() as char {
                        '*' => GeneticString::calculate_value(&gen_str_slice[0..index]) *
                            GeneticString::calculate_value(&gen_str_slice[index + 1..gen_str_slice.len()]),
                        '/' => GeneticString::calculate_value(&gen_str_slice[0..index]) /
                            GeneticString::calculate_value(&gen_str_slice[index + 1..gen_str_slice.len()]),
                        _ => panic!("Should never get here")
                    };
                }
            }
            //println!("String slice length {}", gen_str_slice.len());
            panic!("Should never get here")
        }
    }

    /// # Purpose
    /// This method will take a operator from the caller and determine whether the order of
    /// operations is higher e.g. *,/ or lower, e.g. +,-
    ///
    /// # Parameters
    /// operaoter - the operator whose order is to be determned
    ///
    /// # Returns
    /// true - if lower priority
    /// false - if higher priority
    fn order_of_ops(operator: char) -> bool {
        match operator {
            '+' => true,
            '-' => true,
            '*' => false,
            '/' => false,
            _ => panic!("Should never get here")
        }
    }

    //    pub fn breed(&self, partner: &GeneticString, crossover_rate: f32) -> GeneticString {
    //        let range = thread_rng();
    //        let self_split_loc = (rng.gen_range(0.0f32,1.0f32) * self.vector.len()) as i32;
    //        self::partner_split_loc(false);
    //
    //    }
}

impl fmt::Display for GeneticString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut gene_string = String::new();
        for g_byte in &self.vector {
            gene_string.push_str(&g_byte.to_string())
        }
        write!(f, "{}", gene_string)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::genetic_byte;


    #[test]
    fn single_byte_calculate_value_should_equal_23_when_byte_equal_23() {
        let mut g_string: GeneticString = build_single_item_gen_string();
        assert!(23, GeneticString::calculate_value(&(g_string.get_vector())))
    }
    fn build_empty_genetic_string() {

    }

    /// # Purpose
    /// Build a genetic string 1 byte long with a precomputed value
    ///
    /// # Parameters
    ///
    /// # Return
    /// The genetic string
    fn build_single_item_gen_string() -> GeneticString {
        let mut g_byte_1 = genetic_byte::GeneticByteTestUtilities::new(23, true);
        let mut g_vec = vec![g_byte_1];
        GeneticStringTestUtilities::new(g_vec);
    }

    fn build_multi_item_gen_string() {

    }

    fn build_bad_multi_item_genetic_string() {

    }



}
