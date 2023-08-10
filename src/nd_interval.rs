use std::{path::Path, ops::Range, str::FromStr, array::from_fn};
use anyhow::{Result, bail};
use itertools::Itertools;
use crate::{utils::range_from_str, counter::Counter};


pub trait NdInterval<const D: usize, E: Clone> {
    fn new() -> Self;

    fn insert(&mut self, intervals: [Range<f32>; D], value: E);

    fn containing(&self, point: [f32; D]) -> Vec<(&E, f32)>;
    
    fn closest(&self, point: [f32; D]) -> Option<(&E, f32)>;

    fn domain(&self) -> [Range<f32>; D];

    fn values(&self) -> Vec<&E>;
    
    fn from_csv(path: &Path) -> Result<Self>
        where Self: Sized, E: FromStr
    {
        let mut res = Self::new();
        let mut reader = csv::Reader::from_path(path)?;
        for record in reader.records() {
            let record = record?;
            let Ok(elem) = E::from_str(&record[0]) else {
                bail!("Failed to deserialize value '{}'", &record[0]);
            };
            let intervals: [Range<f32>; D] = from_fn(|i| range_from_str(&record[i+1]).unwrap());
            res.insert(intervals, elem);
        }
        Ok(res)
    }

    fn coverage(&self, step: f32) -> Vec<(&E, f32)>
        where E: PartialEq<E> 
    {
        let mut res = Vec::new();
        let samples = self.domain().into_iter().map(
            |range| {
                let len = ((range.end-range.start)/step) as u32;
                (0..=len).map(move |i| range.start + i as f32*step)
            }
        ).multi_cartesian_product();
        let mut count = 0;
        for point in samples {
            if let Some((value, _)) = self.closest(point.try_into().unwrap()) {
                res.add(value);
            }
            count += 1;
        }
        res.divide(count as f32);
        res
    }
}