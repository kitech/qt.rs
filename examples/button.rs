extern crate qt5;

use qt5::*;

fn main() {
    widgets::QApplication::new((
        &mut 0,
        &mut "example".into(),
        None
    ));

    let button = widgets::QPushButton::new((
        &core::QString::new(&String::from("Hello Qt.rs!")),
        Some(&widgets::QWidget::inheritFrom(0))
    ));

    button.show(());
    button.pressed().connect(Box::new(|| println!("click!")) as Box<Fn()>);

    widgets::QApplication::exec_s(());
}
