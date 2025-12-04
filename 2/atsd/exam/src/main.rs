fn replace<T: Clone + PartialEq>(list: &mut [T], find: &T, with: &T, all: bool) {
    for element in list.iter_mut() {
        if element == find {
            *element = with.clone();
            if !all {
                return;
            }
        }
    }
}

fn main() {
    let mut list: Vec<_> = (0..10).map(|x| x % 7).collect();
    println!("{list:?}");

    let find = 2;
    let with = 42;

    replace(&mut list, &find, &with, false);
    println!("{list:?}");

    replace(&mut list, &find, &with, true);
    println!("{list:?}");

    let mut list = [1, 2, 3, 4, 1, 3, 1, 0, 4];
    replace(&mut list, &1, &9, true);
    println!("{list:?}");
}
