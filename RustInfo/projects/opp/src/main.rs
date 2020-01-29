use opp::Screen;
use opp::gui::{Button, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 12,
                height: 14,
                label: String::from("b1"),
            }),
            Box::new(SelectBox {
                width: 123,
                height: 12,
                options: vec![String::from("LBJ"), String::from("Jordan")],
            })
        ],
    };

    screen.run();
}
