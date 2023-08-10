use std::{ops::Range, array::from_fn};
use itertools::Itertools;

use crate::{nd_interval::NdInterval, utils::RangesUtil};


impl<const D: usize, E: Clone> NdInterval<D, E> for Vec<([Range<f32>; D], E)> {
    fn new() -> Self {
        Vec::new()
    }

    fn insert(&mut self, intervals: [Range<f32>; D], value: E) {
        self.push((intervals, value))
    }

    fn containing(&self, point: [f32; D]) -> Vec<(&E, f32)> {
        self.iter().map(|(ranges, value)| 
            (value, ranges.sign_dist(&point))
        ).filter(|(_, sign_dist)| *sign_dist >= 0f32).collect()
    }

    fn closest(&self, point: [f32; D]) -> Option<(&E, f32)> {
        let mut candidates = self.iter()
            .map(|(ranges, value)| (value, ranges.sign_dist(&point)));
        let mut res = candidates.next()?;
        for (v, sign_dist) in candidates {
            if res.1 < sign_dist {
                res = (v, sign_dist);
            }
        }
        if res.1 < 0. {
            None
        } else {
            Some(res)
        }
    }

    fn domain(&self) -> [Range<f32>; D] {
        let Some(first) = self.get(0) else {
            return from_fn(|_| 0f32..0f32);
        };
        let mut res = first.0.clone();
        for (range, _) in self.iter().skip(1) {
            res.enlarge(range);
        }
        res
    }

    fn values(&self) -> Vec<&E> {
        self.iter().map(|(_, value)| value).collect_vec()
    }
}