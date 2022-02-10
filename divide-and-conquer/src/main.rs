use rand::Rng;

fn create_random_list() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut list: Vec<i32> = Vec::new();

    for _i in 1..2048 {
        list.push(rng.gen_range(0..5000));
    }

    list.sort();

    list
}

fn iterative_binary_search(list: &Vec<i32>, element: i32) -> Option<i32> {
    let mut low: usize = 0;
    let mut high: usize = list.len() - 1;
    let mut iteration: i32 = 0;

    while low <= high {
        iteration += 1;
        let middle = (high + low) / 2;
        println!("{:?}. Iterating with middle value {:?}", iteration, middle);

        if let Some(current) = list.get(middle) {
            if current == &element {
                return Some(*current);
            }

            if current > &element {
                high = middle - 1
            }

            if current < &element {
                low = middle + 1
            }
        }
    }

    None
}

fn main() {
    let mut rng = rand::thread_rng();
    let list = create_random_list();

    println!(
        "{:?} element",
        iterative_binary_search(&list, rng.gen_range(1..1000))
    )
}
