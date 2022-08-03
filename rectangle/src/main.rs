pub struct Point(i32, i32);

pub struct Rectangle{
    pub top_left_corner: Point,
    pub bottom_right_corner: Point
}

impl Rectangle{
    fn width(&self) -> u32 {
        (self.bottom_right_corner.0 - self.top_left_corner.0) as u32
    }

    fn height(&self) -> u32 {
        (self.top_left_corner.1 - self.bottom_right_corner.1) as u32
    }

    fn area(&self) -> u32 {
        self.width()*self.height()
    }

    fn print(&self) {
        let width_size = self.width() as usize;
        let heigth_size = self.height() as usize;
        print!(" ");
        println!();
        vec![" "; heigth_size]
            .iter()
            .for_each(|char| {
                print!("{}",char);
                vec![" . "; width_size]
                    .iter()
                    .for_each(|char| {
                        print!("{}",char);
                    });
                print!("{}",char);
                println!();
            });
        print!(" ");
        vec!["   "; width_size]
            .iter()
            .for_each(|char| {
                print!("{}",char);
            });
        println!();
    }
}

fn get_line(name: &str) -> i32{
    println!("Insert {}", name);
    let mut value = String::new();
    std::io::stdin()
        .read_line(&mut value).expect("Erro reading input");
    value.trim().parse().expect("Not a number")
}

fn main() {
    let x1 = get_line("x1");
    let y1 = get_line("y1");
    let x2 = get_line("x2");
    let y2 = get_line("y2");

    let rect = Rectangle{
        top_left_corner: Point(x1,y1),
        bottom_right_corner: Point(x2, y2)
    };
    println!("Rectangle area is {}",rect.area());
    println!("Here is the rectangle: ");
    rect.print();

}
