enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}



fn main() {
   let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
    /*let msg = Message::ChangeColor(0, 160, 256);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to restructure.");
        }
        Message::Move {x, y} => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }*/

}



/*fn main() {
    let x= 'c';

    match x {
        'a'..='j'=>println!("early ASCII letter"),
        'k'..='z'=>println!("late ASCII letter"),
        _ => {}
    }

    // let x = Some(5);
    // let y = 10;
    //
    // match x {
    //     Some(50) => println!("got 50"),
    //     Some(y) => println!("Matched, y = {y}"),
    //     _ => println!("Default case, x = {:?}", x),
    // }
    //
    // println!("at the end: x = {:?}, y = {y}", x);
}*/