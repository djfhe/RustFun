pub(crate) mod quicksort {
    use std::fmt::Debug;

    pub trait Quicksort: Ord {
        fn quicksort(&mut self) -> ();
    }

    impl<T> Quicksort for [T] where T: Ord + Copy {
        fn quicksort(&mut self) -> () {
            quicksort(&mut self[0..]);
            fn quicksort<T>(unsorted_array: &mut [T]) -> () where T: Ord + Copy {
                if unsorted_array.len() < 2 {
                    return;
                }
                let center: usize = partition(unsorted_array);
                quicksort(&mut unsorted_array[..center]);
                quicksort(&mut unsorted_array[center + 1..]);
            }

            fn get_pivot<T>(unsorted_array: &mut [T]) -> T where T: Ord + Copy {
                let candidates: (T, T, T) = (
                    unsorted_array[0],
                    unsorted_array[unsorted_array.len() / 2],
                    unsorted_array[unsorted_array.len() - 1]
                );
                return candidates.1
            }

            fn swap<T>(unsorted_array: &mut [T], i: usize, j: usize) -> () where T: Copy{
                let t: T = unsorted_array[i];
                unsorted_array[i] = unsorted_array[j];
                unsorted_array[j] = t;
            }

            fn partition<T>(unsorted_array: &mut [T]) -> usize where T: Ord + Copy {
                let pivot: T = get_pivot(unsorted_array);
                let mut lower_bound: usize = 0;
                let mut higher_bound: usize = unsorted_array.len() - 1;
                loop {
                    loop {
                        if unsorted_array[lower_bound] >= pivot { break }
                        lower_bound = lower_bound + 1;
                    }
                    loop {
                        if unsorted_array[higher_bound] <= pivot { break }
                        higher_bound = higher_bound - 1;
                    }
                    if lower_bound >= higher_bound { return higher_bound }
                    swap(unsorted_array, lower_bound, higher_bound);
                }
            }
        }
    }

}