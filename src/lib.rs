mod utils;
mod counter;
mod nd_interval;
mod impl_isets;
mod impl_vec;
use std::fmt::Debug;
use nd_interval::NdInterval;


pub fn print_coverage<const D: usize, E: Clone + PartialEq + Debug>(imap: impl NdInterval<D, E>, step: f32) {
    println!("domain: {:?}", imap.domain());
    let mut coverage = imap.coverage(step);
    // add missing values
    imap.values().into_iter().for_each(|value| if !coverage.iter().any(|(v, _)| *v == value) {
        coverage.push((value, 0.))
    });
    // .sum() refuses to work here for some reason
    let unassigned = 1f32 - coverage.iter().map(|(_, count)| *count).fold(0., |a, b| a + b);
    coverage.sort_by(|(_, c1), (_, c2)| c2.partial_cmp(c1).unwrap());
    for (value, count) in coverage {
        println!("{:?}: {:.1}%", value, count*100.);
    }
    println!("---");
    println!("Empty: {:.1}%", unassigned*100.);
}


#[cfg(test)]
mod tests {
    use std::{ops::Range, path::Path};
    use crate::{nd_interval::NdInterval, print_coverage};

    #[test]
    pub fn my_test() {
        let imap: Vec<([Range<f32>; 4], String)> = Vec::from_csv(Path::new("benches/plants.csv")).unwrap();
        print_coverage(imap, 0.05);
    }
}
