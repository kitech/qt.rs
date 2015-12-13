// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QSize QWidgetItemV2::sizeHint();
  fn _ZNK13QWidgetItemV28sizeHintEv() -> i32;
  // proto: QSize QWidgetItemV2::minimumSize();
  fn _ZNK13QWidgetItemV211minimumSizeEv() -> i32;
  // proto: int QWidgetItemV2::heightForWidth(int width);
  fn _ZNK13QWidgetItemV214heightForWidthEi(arg0: c_int) -> i32;
  // proto: void QWidgetItemV2::FreeQWidgetItemV2();
  fn _ZN13QWidgetItemV2D0Ev() -> i32;
  // proto: void QWidgetItemV2::NewQWidgetItemV2(QWidget * widget);
  fn _ZN13QWidgetItemV2C1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QSize QWidgetItemV2::maximumSize();
  fn _ZNK13QWidgetItemV211maximumSizeEv() -> i32;
  // proto: void QWidgetItemV2::NewQWidgetItemV2(const QWidgetItemV2 & );
  fn _ZN13QWidgetItemV2C1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QWidgetItemV2)=1
pub struct QWidgetItemV2 {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWidgetItemV2 {
  pub fn sizeHint<T: QWidgetItemV2_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QWidgetItemV2_sizeHint {
  fn sizeHint(self, this: &mut QWidgetItemV2) -> i32;
}

// proto: QSize QWidgetItemV2::sizeHint();
impl<'a> /*trait*/ QWidgetItemV2_sizeHint for () {
  fn sizeHint(self, this: &mut QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV28sizeHintEv()};
    unsafe {_ZNK13QWidgetItemV28sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QWidgetItemV2 {
  pub fn minimumSize<T: QWidgetItemV2_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QWidgetItemV2_minimumSize {
  fn minimumSize(self, this: &mut QWidgetItemV2) -> i32;
}

// proto: QSize QWidgetItemV2::minimumSize();
impl<'a> /*trait*/ QWidgetItemV2_minimumSize for () {
  fn minimumSize(self, this: &mut QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV211minimumSizeEv()};
    unsafe {_ZNK13QWidgetItemV211minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWidgetItemV2 {
  pub fn heightForWidth<T: QWidgetItemV2_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QWidgetItemV2_heightForWidth {
  fn heightForWidth(self, this: &mut QWidgetItemV2) -> i32;
}

// proto: int QWidgetItemV2::heightForWidth(int width);
impl<'a> /*trait*/ QWidgetItemV2_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV214heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QWidgetItemV214heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidgetItemV2 {
  pub fn FreeQWidgetItemV2<T: QWidgetItemV2_FreeQWidgetItemV2>(&mut self, value: T) -> i32 {
    value.FreeQWidgetItemV2(self);
    return 1;
  }
}

pub trait QWidgetItemV2_FreeQWidgetItemV2 {
  fn FreeQWidgetItemV2(self, this: &mut QWidgetItemV2) -> i32;
}

// proto: void QWidgetItemV2::FreeQWidgetItemV2();
impl<'a> /*trait*/ QWidgetItemV2_FreeQWidgetItemV2 for () {
  fn FreeQWidgetItemV2(self, this: &mut QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2D0Ev()};
    unsafe {_ZN13QWidgetItemV2D0Ev()};
    return 1;
  }
}

impl /*struct*/ QWidgetItemV2 {
  pub fn NewQWidgetItemV2<T: QWidgetItemV2_NewQWidgetItemV2>(value: T) -> QWidgetItemV2 {
    let rsthis = value.NewQWidgetItemV2();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItemV2_NewQWidgetItemV2 {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2;
}

// proto: void QWidgetItemV2::NewQWidgetItemV2(QWidget * widget);
impl<'a> /*trait*/ QWidgetItemV2_NewQWidgetItemV2 for (&'a mut QWidget) {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2 {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2C1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetItemV2C1EP7QWidget(qthis, arg0)};
    let rsthis = QWidgetItemV2{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWidgetItemV2 {
  pub fn maximumSize<T: QWidgetItemV2_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QWidgetItemV2_maximumSize {
  fn maximumSize(self, this: &mut QWidgetItemV2) -> i32;
}

// proto: QSize QWidgetItemV2::maximumSize();
impl<'a> /*trait*/ QWidgetItemV2_maximumSize for () {
  fn maximumSize(self, this: &mut QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV211maximumSizeEv()};
    unsafe {_ZNK13QWidgetItemV211maximumSizeEv()};
    return 1;
  }
}

// proto: void QWidgetItemV2::NewQWidgetItemV2(const QWidgetItemV2 & );
impl<'a> /*trait*/ QWidgetItemV2_NewQWidgetItemV2 for (&'a  QWidgetItemV2) {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2 {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2C1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QWidgetItemV2C1ERKS_(qthis, arg0)};
    let rsthis = QWidgetItemV2{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

