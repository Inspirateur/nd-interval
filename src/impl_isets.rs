use std::{path::Path, array::from_fn, ops::Range};
use iset::IntervalMap;
use crate::nd_interval::{NdInterval, sign_dist};

trait VecMap<K: PartialEq, V> {
    fn find_left(&self, key: &K) -> Option<&(K, V)>;
}

impl<K: PartialEq, V> VecMap<K, V> for Vec<(K, V)> {
    fn find_left(&self, key: &K) -> Option<&(K, V)> {
        self.iter().find(|(k, _)| key == k)
    }
}


impl<const D: usize, E: Clone + Eq> NdInterval<D, E> for [IntervalMap<f32, E>; D] {
    fn new() -> Self {
        from_fn(|_| IntervalMap::new())
    }

    fn insert(&mut self, intervals: [Range<f32>; D], value: E) {
        for (i, interval) in intervals.into_iter().enumerate() {
            self[i].insert(interval, value.clone());
        }
    }
    
    fn containing(&self, point: [f32; D]) -> Vec<(&E, f32)> {
        let mut candidates_groups = point
        .into_iter().enumerate()
        .map(|(i, p)| 
            self[i].intervals_overlap(p)
                .map(move |range| (self[i].get(range.clone()).unwrap(), sign_dist(&range, p)))
                .collect::<Vec<_>>()
        );
        let mut values = candidates_groups.next().unwrap_or(Vec::new());
        for candidates in candidates_groups {
            values = candidates.into_iter()
                .filter_map(|(candidate, dist2)| if let Some((value, dist1)) = values.find_left(&candidate) {
                    Some((*value, dist1 + dist2))
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

    fn from_csv(path: &Path) -> Self {
        todo!()
    }
}
