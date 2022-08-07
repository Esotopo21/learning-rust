use std::fmt::{Display, Formatter};
use inner_functions::MeanMedianModeInput;


#[derive(Debug)]
pub enum ResultElement{
    Mean(f64),
    Median(f64),
    Mode(Vec<i32>)
}

impl Display for ResultElement{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultElement::Mean(mean) => { write!(f, "{}",mean) },
            ResultElement::Median(median) => { write!(f, "{}", median) }
            ResultElement::Mode(mode) => { write!(f, "{:?}", mode) }
        }
    }
}

pub fn get_mean_median_and_mode(numbers: &Vec<i32>) -> Vec<ResultElement> {
    let input = MeanMedianModeInput::new(numbers.clone());
    vec![ResultElement::Mean(inner_functions::mean(&input)),
         ResultElement::Median(inner_functions::median(&input)),
         ResultElement::Mode(inner_functions::mode(&input))]
}

pub fn mean(vector: &Vec<i32>) -> f64{
    let input = MeanMedianModeInput::new(vector.clone());
    inner_functions::mean(&input)
}

pub fn median(vector: &Vec<i32>) -> f64{
    let input = MeanMedianModeInput::new(vector.clone());
    inner_functions::median(&input)

}

pub fn mode(vector: &Vec<i32>) -> Vec<i32>{
    let input = MeanMedianModeInput::new(vector.clone());
    inner_functions::mode(&input)

}

mod inner_functions{
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::slice::Iter;

    pub struct MeanMedianModeInput{
        vec: Vec<i32>
    }

    impl MeanMedianModeInput {
        pub fn new(initialization_vector: Vec<i32>) -> MeanMedianModeInput{
            if initialization_vector.len() == 0 {
                panic!("Initialization vector must have at least 1 element")
            };
            MeanMedianModeInput{
                vec: initialization_vector
            }
        }
    }

    // Wrappers for some collections methods
    impl MeanMedianModeInput {
        pub fn len(&self) -> usize { self.vec.len() }
        pub fn iter(&self) -> Iter<i32> { self.vec.iter() }
    }



    pub fn mean(numbers: &MeanMedianModeInput) -> f64 {
        numbers.iter().sum::<i32>() as f64 / numbers.len() as f64
    }

    pub fn median(numbers: &MeanMedianModeInput) -> f64 {
        let numbers_len = numbers.len();
        let mut ordered_numbers: Vec<i32> = numbers.vec.clone();
        ordered_numbers.sort();
        let middle_index = numbers_len / 2;
        let median;
        if numbers_len % 2 == 0{
            median = (*ordered_numbers.get(middle_index - 1).unwrap() as f64
                + *ordered_numbers.get(middle_index).unwrap() as f64) / 2f64;
        }else{
            median = *ordered_numbers.get(middle_index).unwrap() as f64;
        }
        median
    }

    pub fn mode(numbers: &MeanMedianModeInput) -> Vec<i32> {
        let mut map : HashMap<i32, i32> = HashMap::new();

        numbers.iter().for_each(|n|{
            let count = map.entry(*n).or_insert(0);
            *count += 1;
        });
        let mut max_opt: Option<Vec<i32>> = None;
        let mut max: Option<i32> = None;

        for (key, value) in map {

            match max_opt {
                Some(ref mut vec) => {
                    match value.cmp(&max.unwrap()){
                        Ordering::Less => {},
                        Ordering::Equal => { vec.push(key); }
                        Ordering::Greater => {
                            max_opt = Some(vec![key]);
                            max = Some(value);
                        }
                    };
                },
                None => {
                    max_opt = Some(vec![key]);
                    max = Some(value);
                }
            }
        };
        max_opt.unwrap();
    }
}





