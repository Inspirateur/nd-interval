use std::{array::from_fn, ops::Range};
use iset::IntervalMap;
use itertools::Itertools;
use crate::{nd_interval::NdInterval, utils::RangeUtil};
// FIXME: this doesn't work because when storing different values for the same interval iset only keeps the last one
// need to change lib to fix this, but for now we'll move on with the Vec<> impl
trait VecMap<K: PartialEq, V> {
    fn find_left(&self, key: &K) -> Option<&(K, V)>;
}

impl<K: PartialEq, V> VecMap<K, V> for Vec<(K, V)> {
    fn find_left(&self, key: &K) -> Option<&(K, V)> {
        self.iter().find(|(k, _)| key == k)
    }
}

pub struct NdIntervalMap<const D: usize, E: Clone + Eq>([IntervalMap<f32, E>; D]);

impl<const D: usize, E: Clone + Eq> NdInterval<D, E> for NdIntervalMap<D, E> {
    fn new() -> Self {
        NdIntervalMap(from_fn(|_| IntervalMap::new()))
    }

    fn insert(&mut self, intervals: [Range<f32>; D], value: E) {
        for (i, interval) in intervals.into_iter().enumerate() {
            self.0[i].insert(interval, value.clone());
        }
    }
    
    fn containing(&self, point: [f32; D]) -> Vec<(&E, f32)> {
        let mut candidates_groups = point
        .into_iter().enumerate()
        .map(|(i, p)| 
            self.0[i].intervals_overlap(p)
                .map(|range| (self.0[i].get(range.clone()).unwrap(), range.sign_dist(p)))
                .collect::<Vec<_>>()
        );
        let mut values = candidates_groups.next().unwrap_or(Vec::new());
        for candidates in candidates_groups {
            values = candidates.into_iter()
                .filter_map(|(candidate, dist2)| if let Some((value, dist1)) = values.find_left(&candidate) {
                    Some((*value, dist1.min(dist2)))
                } else {
                    None
                })
                .collect();
            if values.len() == 0 {
                return Vec::new();
            }
        }
        values
    }

    fn closest(&self, point: [f32; D]) -> Option<(&E, f32)> {
        let mut containing = self.containing(point);
        let mut res = containing.pop()?;
        for (v, sign_dist) in self.containing(point).into_iter() {
            if res.1 < sign_dist {
                res = (v, sign_dist);
            }
        }
        Some(res)
    }

    fn domain(&self) -> [Range<f32>; D] {
        from_fn(|i| self.0[i].range().unwrap_or(0f32..0f32))
    }

    fn values(&self) -> Vec<&E> {
        let Some(imap) = self.0.get(0) else {
            return Vec::new();
        };
        imap.unsorted_values().collect_vec()
    }
}
