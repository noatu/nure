struct List {
    val: i32,
    next: Option<Box<List>>,
}

impl List {
    pub const fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn print(&self) {
        let mut ptr = self;

        print!("[{}", ptr.val);
        while let Some(ref next) = ptr.next {
            ptr = next;
            print!(", {}", ptr.val);
        }
        println!("]");
    }

    pub fn add(&mut self, val: i32) {
        let mut ptr = self;

        while let Some(ref mut next) = ptr.next {
            ptr = next;
        }
        ptr.next = Some(Box::new(Self::new(val)));
    }

    pub fn bubblesort(&mut self) {
        let mut len = 1; // there is already 1 element in the list
        let mut sorted = false;
        let mut ptr = &mut *self;

        // find the length
        while let Some(next) = &mut ptr.next {
            if ptr.val > next.val {
                std::mem::swap(&mut ptr.val, &mut next.val);
            }

            len += 1;
            ptr = next;
        }

        while !sorted {
            sorted = true;

            ptr = &mut *self;
            // len-2 because looped 1 time while getting len, and checking next element from current
            for _ in 0..len - 2 {
                let next = ptr.next.as_mut().unwrap();
                if ptr.val > next.val {
                    std::mem::swap(&mut ptr.val, &mut next.val);
                    sorted = false;
                }

                ptr = next;
            }
            len -= 1;
        }
    }
}

fn main() {
    let mut list = List::new(42);
    list.add(1);
    list.add(37);
    list.add(69);
    list.add(9);
    list.add(96);
    list.add(12);
    list.add(36);
    print!("List:   ");
    list.print();

    list.bubblesort();
    print!("Sorted: ");
    list.print();

    list.add(36);
    print!("Add 36: ");
    list.print();

    list.bubblesort();
    print!("Sorted: ");
    list.print();
}
