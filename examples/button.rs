extern crate qt5;

use qt5::*;

fn main() {
    widgets::QApplication::New((
        &mut 0,
        &mut "example".into(),
        0
    ));

    let button = widgets::QPushButton::New((
        &core::QString::New(&String::from("Hello Qt.rs!")),
        &widgets::QWidget::inheritFrom(0)
    ));

    button.show(());
//    button.pressed_1().connect(Box::new(|| println!("click!")));

    widgets::QApplication::exec_s(());
}
