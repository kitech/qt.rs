// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
  fn _ZN22QStyleOptionDockWidgetC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(int version);
  fn _ZN22QStyleOptionDockWidgetC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget & other);
  fn _ZN22QStyleOptionDockWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionDockWidget)=1
pub struct QStyleOptionDockWidget {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
impl /*struct*/ QStyleOptionDockWidget {
  pub fn NewQStyleOptionDockWidget<T: QStyleOptionDockWidget_NewQStyleOptionDockWidget>(value: T) -> QStyleOptionDockWidget {
    let rsthis = value.NewQStyleOptionDockWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionDockWidget_NewQStyleOptionDockWidget {
  fn NewQStyleOptionDockWidget(self) -> QStyleOptionDockWidget;
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
impl<'a> /*trait*/ QStyleOptionDockWidget_NewQStyleOptionDockWidget for () {
  fn NewQStyleOptionDockWidget(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1Ev()};
    unsafe {_ZN22QStyleOptionDockWidgetC1Ev(qthis)};
    let rsthis = QStyleOptionDockWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(int version);
impl<'a> /*trait*/ QStyleOptionDockWidget_NewQStyleOptionDockWidget for (i32) {
  fn NewQStyleOptionDockWidget(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionDockWidgetC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionDockWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget & other);
impl<'a> /*trait*/ QStyleOptionDockWidget_NewQStyleOptionDockWidget for (QStyleOptionDockWidget) {
  fn NewQStyleOptionDockWidget(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionDockWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionDockWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

