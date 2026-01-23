//! Value generators for property-based testing

use super::SimpleRng;

/// Generator trait for creating random values
pub trait Generator<T> {
    /// Generate a random value
    fn generate(&self, rng: &mut SimpleRng) -> T;
    
    /// Get the name of this generator
    fn name(&self) -> &'static str;
}

/// Integer generator
pub struct IntGenerator {
    pub min: i64,
    pub max: i64,
}

impl Default for IntGenerator {
    fn default() -> Self {
        Self {
            min: i64::MIN / 2,
            max: i64::MAX / 2,
        }
    }
}

impl IntGenerator {
    pub fn new(min: i64, max: i64) -> Self {
        Self { min, max }
    }
    
    pub fn positive() -> Self {
        Self { min: 0, max: i64::MAX / 2 }
    }
    
    pub fn negative() -> Self {
        Self { min: i64::MIN / 2, max: 0 }
    }
    
    pub fn small() -> Self {
        Self { min: -100, max: 100 }
    }
}

impl Generator<i64> for IntGenerator {
    fn generate(&self, rng: &mut SimpleRng) -> i64 {
        rng.gen_range(self.min, self.max)
    }
    
    fn name(&self) -> &'static str {
        "Int"
    }
}

/// Float generator
pub struct FloatGenerator {
    pub min: f64,
    pub max: f64,
}

impl Default for FloatGenerator {
    fn default() -> Self {
        Self { min: -1e10, max: 1e10 }
    }
}

impl FloatGenerator {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    
    pub fn unit() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}

impl Generator<f64> for FloatGenerator {
    fn generate(&self, rng: &mut SimpleRng) -> f64 {
        let t = (rng.next() as f64) / (u64::MAX as f64);
        self.min + t * (self.max - self.min)
    }
    
    fn name(&self) -> &'static str {
        "Float"
    }
}

/// Boolean generator
pub struct BoolGenerator;

impl Generator<bool> for BoolGenerator {
    fn generate(&self, rng: &mut SimpleRng) -> bool {
        rng.next() % 2 == 0
    }
    
    fn name(&self) -> &'static str {
        "Bool"
    }
}

/// String generator
pub struct StringGenerator {
    pub min_len: usize,
    pub max_len: usize,
    pub charset: Vec<char>,
}

impl Default for StringGenerator {
    fn default() -> Self {
        Self {
            min_len: 0,
            max_len: 100,
            charset: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect(),
        }
    }
}

impl StringGenerator {
    pub fn ascii() -> Self {
        Self::default()
    }
    
    pub fn alphanumeric() -> Self {
        Self::default()
    }
    
    pub fn alphabetic() -> Self {
        Self {
            charset: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            ..Default::default()
        }
    }
    
    pub fn numeric() -> Self {
        Self {
            charset: "0123456789".chars().collect(),
            ..Default::default()
        }
    }
    
    pub fn with_length(min: usize, max: usize) -> Self {
        Self {
            min_len: min,
            max_len: max,
            ..Default::default()
        }
    }
}

impl Generator<String> for StringGenerator {
    fn generate(&self, rng: &mut SimpleRng) -> String {
        let len = rng.gen_range(self.min_len as i64, self.max_len as i64) as usize;
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0, self.charset.len() as i64) as usize;
                self.charset[idx]
            })
            .collect()
    }
    
    fn name(&self) -> &'static str {
        "String"
    }
}

/// List generator
pub struct ListGenerator<T, G: Generator<T>> {
    pub element_gen: G,
    pub min_len: usize,
    pub max_len: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, G: Generator<T>> ListGenerator<T, G> {
    pub fn new(element_gen: G) -> Self {
        Self {
            element_gen,
            min_len: 0,
            max_len: 50,
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn with_length(mut self, min: usize, max: usize) -> Self {
        self.min_len = min;
        self.max_len = max;
        self
    }
}

impl<T, G: Generator<T>> Generator<Vec<T>> for ListGenerator<T, G> {
    fn generate(&self, rng: &mut SimpleRng) -> Vec<T> {
        let len = rng.gen_range(self.min_len as i64, self.max_len as i64) as usize;
        (0..len).map(|_| self.element_gen.generate(rng)).collect()
    }
    
    fn name(&self) -> &'static str {
        "List"
    }
}

/// Option generator
pub struct OptionGenerator<T, G: Generator<T>> {
    pub element_gen: G,
    pub none_probability: f64,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, G: Generator<T>> OptionGenerator<T, G> {
    pub fn new(element_gen: G) -> Self {
        Self {
            element_gen,
            none_probability: 0.1,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, G: Generator<T>> Generator<Option<T>> for OptionGenerator<T, G> {
    fn generate(&self, rng: &mut SimpleRng) -> Option<T> {
        let r = (rng.next() as f64) / (u64::MAX as f64);
        if r < self.none_probability {
            None
        } else {
            Some(self.element_gen.generate(rng))
        }
    }
    
    fn name(&self) -> &'static str {
        "Option"
    }
}

/// Tuple generator for pairs
pub struct TupleGenerator<A, B, GA: Generator<A>, GB: Generator<B>> {
    pub gen_a: GA,
    pub gen_b: GB,
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<A, B, GA: Generator<A>, GB: Generator<B>> TupleGenerator<A, B, GA, GB> {
    pub fn new(gen_a: GA, gen_b: GB) -> Self {
        Self {
            gen_a,
            gen_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<A, B, GA: Generator<A>, GB: Generator<B>> Generator<(A, B)> for TupleGenerator<A, B, GA, GB> {
    fn generate(&self, rng: &mut SimpleRng) -> (A, B) {
        (self.gen_a.generate(rng), self.gen_b.generate(rng))
    }
    
    fn name(&self) -> &'static str {
        "Tuple"
    }
}

/// One-of generator - picks from a list of values
pub struct OneOfGenerator<T: Clone> {
    pub values: Vec<T>,
}

impl<T: Clone> OneOfGenerator<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self { values }
    }
}

impl<T: Clone> Generator<T> for OneOfGenerator<T> {
    fn generate(&self, rng: &mut SimpleRng) -> T {
        let idx = rng.gen_range(0, self.values.len() as i64) as usize;
        self.values[idx].clone()
    }
    
    fn name(&self) -> &'static str {
        "OneOf"
    }
}

// Note: gen_range is defined in mod.rs on SimpleRng

#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_rng() -> SimpleRng {
        SimpleRng::new(42)
    }
    
    #[test]
    fn test_int_generator() {
        let gen = IntGenerator::new(0, 100);
        let mut rng = make_rng();
        
        for _ in 0..100 {
            let v = gen.generate(&mut rng);
            assert!(v >= 0 && v < 100);
        }
    }
    
    #[test]
    fn test_string_generator() {
        let gen = StringGenerator::with_length(5, 10);
        let mut rng = make_rng();
        
        for _ in 0..20 {
            let s = gen.generate(&mut rng);
            assert!(s.len() >= 5 && s.len() <= 10);
        }
    }
    
    #[test]
    fn test_list_generator() {
        let gen = ListGenerator::new(IntGenerator::small()).with_length(1, 5);
        let mut rng = make_rng();
        
        for _ in 0..20 {
            let list = gen.generate(&mut rng);
            assert!(list.len() >= 1 && list.len() <= 5);
        }
    }
}
