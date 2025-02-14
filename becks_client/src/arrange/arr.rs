use becks_crew::*;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct ArrangeItem {
    pub id: Id,
    pub score: Score,
}

impl PartialEq for ArrangeItem {
    fn eq(&self, other: &Self) -> bool {
        self.score.0 == other.score.0
    }
}

impl Eq for ArrangeItem {}

impl Ord for ArrangeItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.0.cmp(&other.score.0)
    }
}

impl PartialOrd for ArrangeItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Arranger {
    ids: Vec<ArrangeItem>,
    group_size: usize,
    pub groups: Vec<super::Group>,
}

fn mild_sort<T>(list: &mut [T])
where
    T: Ord,
{
    if list.is_empty() {
        return;
    }
    let window = list.len().isqrt().min(3);
    list.sort_unstable();
    for begin in 0..list.len() {
        let end = (begin + window + 1).min(list.len());
        let swap = rand::rng().random_range(begin..end);
        list.swap(begin, swap);
    }
}

impl Arranger {
    pub fn new(ids: impl IntoIterator<Item = ArrangeItem>, group_size: usize) -> Self {
        Self {
            ids: ids.into_iter().collect(),
            group_size: group_size.max(1),
            groups: Default::default(),
        }
    }

    pub fn arrange(&mut self) {
        mild_sort(&mut self.ids);

        let total = self.ids.len();
        let groups_count = total.div_euclid(self.group_size);
        let mut remain_len = total - groups_count * self.group_size;

        self.groups.resize_with(groups_count, Default::default);
        let mut count = 0usize;
        let mut group_index = 0usize;
        for (index, ArrangeItem { id, score: _ }) in self.ids.iter().copied().enumerate() {
            self.groups[group_index].all.push(id);
            let rest = total - index;
            if rand::rng().random_ratio(remain_len as u32, rest as u32) {
                // If this crew is chosen to be one of the additions
                remain_len -= 1;
            } else {
                count += 1;
                if count == self.group_size {
                    count = 0;
                    group_index += 1;
                }
            }
        }

        for group in self.groups.iter_mut() {
            group.arrange();
        }
    }
}
