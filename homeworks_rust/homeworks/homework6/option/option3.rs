// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

//// I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    let y = match y {
        Some(p) => {
            println!("Co-ordinates are {},{} ", p.x, p.y);
            Some(p)
        }
        _ => {
            println!("no match");
            None
        }
    };

    y; // Fix without deleting this line.
}
