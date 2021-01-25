struct Array {
    content: [i32; 5]
}

fn main() {
    let arr = Array::new([7, 4, 3, 2, 1]);
    println!("arr {:?}", arr.content);
    let sorted_arr = arr.bubble_sort();
    println!("arr {:?}", sorted_arr.content);
}

impl Array {
    fn new(arr: [i32; 5]) -> Array {
        Array { content: arr }
    }

    fn bubble_sort(mut self) -> Self {
        let mut unsorted_untile_index = self.content.len() - 1;
        let mut is_sorted = false;
        let mut steps = 0;

        while !is_sorted {
            is_sorted = true;
            for i in 0..unsorted_untile_index {
                steps += 1;
                if self.content[i] > self.content[i+1] {
                    let tmp = self.content[i];
                    self.content[i] = self.content[i+1];
                    self.content[i+1] = tmp;

                    is_sorted = false;
                }
            }
            println!("index: {}", unsorted_untile_index);
            unsorted_untile_index -= 1;
        }
        println!("steps: {}", steps);

        return self
    }
}

