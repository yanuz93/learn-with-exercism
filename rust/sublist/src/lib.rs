use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug, hash::Hash, usize};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison
where
    T: Eq + Hash + Ord + Debug,
{
    // todo!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    fn count<T>(items: &[T]) -> BTreeMap<&T, usize>
    where
        T: Eq + Hash + Ord + Debug,
    {
        let mut maps = BTreeMap::new();
        for item in items {
            *maps.entry(item).or_insert(0) += 1
        }
        maps
    }

    fn compare<T>(source: BTreeMap<&T, usize>, target: BTreeMap<&T, usize>) -> Comparison
    where
        T: Eq + Hash + Ord + Debug,
    {
        dbg!(source.clone());
        dbg!(target.clone());
        let size_cmp = source.len().cmp(&target.len());
        //
        let (lower, greater) = if size_cmp.is_lt() {
            (source, target)
        } else {
            (target, source)
        };

        let mut is_sublist = false;
        for (item, val) in lower.iter() {
            let found = greater.get(item);
            if found.is_some() {
                match found.unwrap().cmp(val) {
                    Ordering::Less => is_sublist = true,
                    Ordering::Equal => is_sublist = true,
                    Ordering::Greater => is_sublist = false,
                }
            }
        }

        if size_cmp.is_gt() && is_sublist {
            return Comparison::Superlist;
        }

        if size_cmp.is_lt() && is_sublist {
            return Comparison::Sublist;
        }

        Comparison::Unequal
    }

    if _first_list == _second_list {
        return Comparison::Equal;
    }

    compare(count(_first_list), count(_second_list))
}
