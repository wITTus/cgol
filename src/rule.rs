pub struct AutomataRule {
    b: [bool; 9],
    s: [bool; 9],
}

impl AutomataRule {
    pub fn cgol() -> Self {
        AutomataRule::from("B3/S23")
    }

    pub fn apply(&self, alive: bool, neighbours: usize) -> bool {
        (self.s[neighbours] & alive) | self.b[neighbours]
    }
}

impl From<&str> for AutomataRule {
    fn from(txt: &str) -> Self {
        let vec: Vec<&str> = txt.split('/').collect();

        if vec.len() != 2 {
            panic!("Unknown rule format");
        }

        let mut b = [false; 9];
        let mut s = [false; 9];

        vec[0].chars().skip(1).map(|c| c.to_digit(10).unwrap() as usize).for_each(|i| b[i] = true);
        vec[1].chars().skip(1).map(|c| c.to_digit(10).unwrap() as usize).for_each(|i| s[i] = true);

        AutomataRule { b, s }
    }
}

#[cfg(test)]
mod tests {
    use crate::field::Field;
    use crate::rule::AutomataRule;

    #[test]
    fn test_from() {
        let r = AutomataRule::from("B3/S23");
        assert_eq!([false, false, false, true, false, false, false, false, false], r.b);
        assert_eq!([false, false, true, true, false, false, false, false, false], r.s);
    }

    #[test]
    fn test_apply() {
        let r = AutomataRule::from("B3/S23");
        let glider = Field::from_rle("x=5,y=5,rule=B3/S23\nbob$2bo$3o!");
        let projection = glider.proj2d();
        let neighbours = glider.calculate_neighbours(&projection);
        let result = Field::new(glider.cells.iter().zip(neighbours).map(|(&alive, n)| r.apply(alive, n)).collect(), 5, 5);
        let expected = Field::from_rle("x=5,y=5,rule=B3/S23\n$obo$b2o$bo2$!");

        assert!(expected == result);
    }
}