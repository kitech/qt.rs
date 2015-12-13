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
  // proto: void QHBoxLayout::NewQHBoxLayout(QWidget * parent);
  fn _ZN11QHBoxLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QHBoxLayout::metaObject();
  fn _ZNK11QHBoxLayout10metaObjectEv() -> i32;
  // proto: void QHBoxLayout::FreeQHBoxLayout();
  fn _ZN11QHBoxLayoutD0Ev() -> i32;
  // proto: void QHBoxLayout::NewQHBoxLayout(const QHBoxLayout & );
  fn _ZN11QHBoxLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QHBoxLayout::NewQHBoxLayout();
  fn _ZN11QHBoxLayoutC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QHBoxLayout)=1
pub struct QHBoxLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHBoxLayout {
  pub fn NewQHBoxLayout<T: QHBoxLayout_NewQHBoxLayout>(value: T) -> QHBoxLayout {
    let rsthis = value.NewQHBoxLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QHBoxLayout_NewQHBoxLayout {
  fn NewQHBoxLayout(self) -> QHBoxLayout;
}

// proto: void QHBoxLayout::NewQHBoxLayout(QWidget * parent);
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for (&'a mut QWidget) {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QHBoxLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QHBoxLayout {
  pub fn metaObject<T: QHBoxLayout_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QHBoxLayout_metaObject {
  fn metaObject(self, this: &mut QHBoxLayout) -> i32;
}

// proto: const QMetaObject * QHBoxLayout::metaObject();
impl<'a> /*trait*/ QHBoxLayout_metaObject for () {
  fn metaObject(self, this: &mut QHBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHBoxLayout10metaObjectEv()};
    unsafe {_ZNK11QHBoxLayout10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QHBoxLayout {
  pub fn FreeQHBoxLayout<T: QHBoxLayout_FreeQHBoxLayout>(&mut self, value: T) -> i32 {
    value.FreeQHBoxLayout(self);
    return 1;
  }
}

pub trait QHBoxLayout_FreeQHBoxLayout {
  fn FreeQHBoxLayout(self, this: &mut QHBoxLayout) -> i32;
}

// proto: void QHBoxLayout::FreeQHBoxLayout();
impl<'a> /*trait*/ QHBoxLayout_FreeQHBoxLayout for () {
  fn FreeQHBoxLayout(self, this: &mut QHBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutD0Ev()};
    unsafe {_ZN11QHBoxLayoutD0Ev()};
    return 1;
  }
}

// proto: void QHBoxLayout::NewQHBoxLayout(const QHBoxLayout & );
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for (&'a  QHBoxLayout) {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QHBoxLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QHBoxLayout::NewQHBoxLayout();
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for () {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1Ev()};
    unsafe {_ZN11QHBoxLayoutC1Ev(qthis)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

