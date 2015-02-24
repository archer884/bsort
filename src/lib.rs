pub mod bsort {
    use std::cmp::Ordering;

    pub trait BSortable<T>
        where T: PartialOrd
    {
        fn bsort(&mut self);
        fn bsort_by<F: Fn(&T,&T) -> Ordering>(&mut self, f: F);
        fn sorted<F: Fn(&T,&T) -> Ordering>(&self, f: &F) -> bool;
    }

    impl<T> BSortable<T> for Vec<T>
        where T: PartialOrd
    {
        fn bsort(&mut self) {
            self.bsort_by(|&ref t1, &ref t2| t1.partial_cmp(&t2).unwrap());
        }

        fn bsort_by<F: Fn(&T,&T) -> Ordering>(&mut self, f: F) {
            while !self.sorted(&f) {
                for idx in 0..self.len() - 1 {
                    if f(&self[idx], &self[idx + 1]) == Ordering::Greater {
                        self.swap(idx, idx + 1);
                    }
                }
            }
        }

        fn sorted<F: Fn(&T,&T) -> Ordering>(&self, f: &F) -> bool {
            self.iter()
                .take(self.len() - 1)
                .enumerate()
                .all(|(idx, &ref t)| f(t, &self[idx + 1]) != Ordering::Greater)
        }
    }
}

#[cfg(test)]
pub mod bsort_tests {
    use super::bsort::BSortable;

    #[test]
    fn bsort_works() {
        let vec = {
            let mut vec = vec![4, 1, 3, 2];
            vec.bsort();
            vec
        };
        
        assert!(vec == [1, 2, 3, 4]);
        assert!(vec != [4, 1, 3, 2]);
    }

    #[test]
    fn bsort_by_works() {
        let vec = {
            let mut vec = vec![4, 1, 3, 2];
            vec.bsort_by(|&t1, t2| t2.partial_cmp(&t1).unwrap());
            vec
        };

        assert!(vec == [4, 3, 2, 1]);
        assert!(vec != [4, 1, 3, 2]);
    }
}
