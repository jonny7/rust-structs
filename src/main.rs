// the line below is an opt-in annotation to allow  Rust to debug the struct similar to print_r() / var_dump()
// the corresponding println!("rect is {:#?}", rect); is how you view the output
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// define a method on a struct, using its self as reference
// when defined on emum, struct, trait obj their first parameter is always self
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    /*let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    )
    */

    /*
    let rect = (30, 50);

    println!("The area of the rectangle is {} square pixels",
            area(rect)
    );
    */

    let rect = Rectangle {width: 30, height: 50 };

    /*println!("The area of the rectangle is {} square pixels",
            area(&rect)
    );*/

    // how to show debug info of struct
    // {:?} -> rect is Rectangle { width: 30, height: 50 }
    /* {:#?} -> rect is Rectangle {
                    width: 30,
                    height: 50
                }
    */
    /*println!("rect is {:#?}", rect);*/

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    )
}

