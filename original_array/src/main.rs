struct OArray {
    content: [i32; 3]
}

impl OArray {
    fn new(arr: [i32; 3]) -> OArray {
        OArray { content: arr }
    }

    fn linear_search(&self, target: i32) -> Option<usize> {
        for (idx, elm) in self.content.iter().enumerate() {
            if *elm == target {
                return Some(idx)
            }
        }
        None
    }

    fn binary_search(&self, target: i32) -> Option<usize> {
        let mut upper_bound = self.content.len() - 1;
        let mut lower_bound = 0;

        while lower_bound <= upper_bound {
            let midpoint = (upper_bound + lower_bound) / 2;
            let value_at_midpoint = self.content[midpoint];
            if target == value_at_midpoint {
                return Some(midpoint)
            } else if target < value_at_midpoint {
                upper_bound = midpoint -1
            } else if target > value_at_midpoint {
                lower_bound = midpoint + 1
            }
        }
        None
    }
}

fn main() {
    let arr = OArray::new([1, 3, 5]);
    print_search_result(arr.linear_search(3));
    print_search_result(arr.linear_search(10));
    print_search_result(arr.binary_search(1));
    print_search_result(arr.binary_search(3));
    print_search_result(arr.binary_search(5));
    print_search_result(arr.binary_search(10));
}

fn print_search_result(result: Option<usize>) {
    match result {
        None => println!("Target does not exist"),
        Some(x) => {
            println!("target is at {}", x)
        }
    }
}
