struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl oops::Draw for SelectBox {
    fn draw(&self) {
        unimplemented!()
    }
}
fn main() {
    let screen = oops::Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(oops::Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
