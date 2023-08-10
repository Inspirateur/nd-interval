use itertools::Itertools;

pub(crate) trait Counter<E> {
    fn add(&mut self, elem: E)
    where E: PartialEq<E>;

    fn divide(&mut self, value: f32);
}

impl<E> Counter<E> for Vec<(E, f32)> {
    fn add(&mut self, elem: E)
        where E: PartialEq<E> 
    {
        if let Some((i, (_, count))) = self.iter().find_position(|(e, _)| *e == elem) {
            self[i] = (elem, *count+1.);
        } else {
            self.push((elem, 1.));
        }
    }

    fn divide(&mut self, value: f32) {
        for i in 0..self.len() {
            self[i].1 /= value;
        }
    }
}