pub enum BinaryHeapType {
    Min,
    Max,
}

pub struct BinaryHeap<T>
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    data: Vec<T>,
    tree_type: BinaryHeapType,
}

impl<T> BinaryHeap<T>
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    pub fn new(tree_type: BinaryHeapType) -> Self {
        BinaryHeap {
            data: vec![],
            tree_type,
        }
    }

    pub fn hipify(tree_type: BinaryHeapType, vec: Vec<T>) -> Self {
        let mut tree = BinaryHeap {
            data: vec,
            tree_type,
        };
        tree.hipify_int();
        tree
    }

    fn hipify_int(&mut self) {
        let len = self.data.len();
        if len == 0 {
            return;
        }

        //find all elements from the lower level and bubble them up
        let levels = self.get_level_number(len);
        let start_index = (1 << (levels - 1)) - 1;

        for i in start_index..len {
            self.bubble_up(i, true);
        }
    }

    fn get_level_number(&self, element_number: usize) -> usize {
        let mut level = 0;
        while (1 << level) <= element_number {
            level += 1;
        }
        level
    }

    fn cmp(&self, value1: &T, value2: &T) -> bool {
        match &self.tree_type {
            BinaryHeapType::Min => value1 < value2,
            BinaryHeapType::Max => value1 > value2,
        }
    }

    fn bubble_up(&mut self, i: usize, to_top: bool) {
        let mut i: usize = i;

        while i > 0 {
            let parent: usize = (i - 1) / 2;

            if self.cmp(&self.data[i], &self.data[parent]) {
                self.data.swap(parent, i);
            } else if !to_top {
                break;
            }

            i = parent;
        }
    }

    fn bubble_down(&mut self, i: usize) {
        let len = self.data.len();

        let mut i = i;

        loop {
            let mut minmax = i;

            for j in 1..=2 {
                let pos = 2 * i + j; //j - means left (1) and right (2) child
                if pos < len && self.cmp(&self.data[pos], &self.data[minmax]) {
                    minmax = pos;
                }
            }

            if minmax == i {
                break;
            }

            self.data.swap(i, minmax);
            i = minmax;
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.bubble_up(self.data.len() - 1, false);
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.data.len();

        let result = match len > 1 {
            true => {
                self.data.swap(0, len - 1);
                let last = self.data.pop()?;
                self.bubble_down(0);
                last
            }
            false => self.data.pop()?,
        };

        Some(result)
    }

    pub fn display(&self) {
        let max_elem_len = self.data.iter().map(|v| format!("{:?}", v).len()).max();
        let max_elem_len = match max_elem_len {
            Some(v) => v + 1,
            None => return,
        };

        let len = self.data.len();
        let levels = self.get_level_number(len);
        let last_level_length = max_elem_len * 2usize.pow(levels as u32 - 1);

        let mut j = 1;
        let mut local = 0;

        for i in 0..len {
            let value_str = format!("{:?}", &self.data[i]);
            let value_lead = format!("{:fill$}", value_str, fill = max_elem_len);

            let multiplier = if local == 0 { 0.5f32 } else { 1f32 }; //local == 0 for new line
            let size_for_value = last_level_length / j;
            let first_spaces = multiplier * (size_for_value - max_elem_len) as f32;

            print!(
                "{}{value_lead}",
                format_args!("{:fill$}", "", fill = (first_spaces as usize))
            );

            local += 1;
            if local >= j {
                local = 0;
                j <<= 1;
                println!();
            }
        }

        if local < j {
            println!(); //missing additional line
        }
        println!("{}", "*".repeat(last_level_length));
    }
}
