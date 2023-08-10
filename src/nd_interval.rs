use std::{path::Path, ops::Range};

pub(crate) fn sign_dist(range: &Range<f32>, p: f32) -> f32 {
    (p-range.start).min(range.end-p)
}

pub(crate) fn nd_sign_dist<const D: usize>(ranges: &[Range<f32>; D], point: &[f32; D]) -> f32 {
    ranges.into_iter().zip(point.into_iter())
        .map(|(range, p)| sign_dist(range, *p)).sum()
}

pub trait NdInterval<const D: usize, E: Clone> {
    fn new() -> Self;

    fn insert(&mut self, intervals: [Range<f32>; D], value: E);

    fn containing(&self, point: [f32; D]) -> Vec<(&E, f32)>;
    
    fn closest(&self, point: [f32; D]) -> Option<(&E, f32)>;

    fn from_csv(path: &Path) -> Self;
}