use std::{path::Path, ops::Range};
use crate::nd_interval::{NdInterval, nd_sign_dist};



impl<const D: usize, E: Clone> NdInterval<D, E> for Vec<([Range<f32>; D], E)> {
    fn new() -> Self {
        todo!()
    }

    fn insert(&mut self, intervals: [Range<f32>; D], value: E) {
        todo!()
    }

    fn containing(&self, point: [f32; D]) -> Vec<(&E, f32)> {
        todo!()
    }

    fn closest(&self, point: [f32; D]) -> Option<(&E, f32)> {
        let mut candidates = self.iter().map(|(ranges, value)| (value, nd_sign_dist(ranges, &point)));
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

    fn from_csv(path: &Path) -> Self {
        todo!()
    }
}