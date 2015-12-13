// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qregion::QRegion;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPaintDeviceWindow::update(const QRegion & region);
  fn _ZN18QPaintDeviceWindow6updateERK7QRegion(arg0: *const c_void) -> i32;
  // proto: void QPaintDeviceWindow::update();
  fn _ZN18QPaintDeviceWindow6updateEv() -> i32;
  // proto: void QPaintDeviceWindow::NewQPaintDeviceWindow(const QPaintDeviceWindow & );
  fn _ZN18QPaintDeviceWindowC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QPaintDeviceWindow::metaObject();
  fn _ZNK18QPaintDeviceWindow10metaObjectEv() -> i32;
  // proto: void QPaintDeviceWindow::update(const QRect & rect);
  fn _ZN18QPaintDeviceWindow6updateERK5QRect(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QPaintDeviceWindow)=1
pub struct QPaintDeviceWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPaintDeviceWindow {
  pub fn update<T: QPaintDeviceWindow_update>(&mut self, value: T) -> i32 {
    value.update(self);
    return 1;
  }
}

pub trait QPaintDeviceWindow_update {
  fn update(self, this: &mut QPaintDeviceWindow) -> i32;
}

// proto: void QPaintDeviceWindow::update(const QRegion & region);
impl<'a> /*trait*/ QPaintDeviceWindow_update for (&'a  QRegion) {
  fn update(self, this: &mut QPaintDeviceWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QPaintDeviceWindow6updateERK7QRegion(arg0)};
    return 1;
  }
}

// proto: void QPaintDeviceWindow::update();
impl<'a> /*trait*/ QPaintDeviceWindow_update for () {
  fn update(self, this: &mut QPaintDeviceWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateEv()};
    unsafe {_ZN18QPaintDeviceWindow6updateEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDeviceWindow {
  pub fn NewQPaintDeviceWindow<T: QPaintDeviceWindow_NewQPaintDeviceWindow>(value: T) -> QPaintDeviceWindow {
    let rsthis = value.NewQPaintDeviceWindow();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintDeviceWindow_NewQPaintDeviceWindow {
  fn NewQPaintDeviceWindow(self) -> QPaintDeviceWindow;
}

// proto: void QPaintDeviceWindow::NewQPaintDeviceWindow(const QPaintDeviceWindow & );
impl<'a> /*trait*/ QPaintDeviceWindow_NewQPaintDeviceWindow for (&'a  QPaintDeviceWindow) {
  fn NewQPaintDeviceWindow(self) -> QPaintDeviceWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QPaintDeviceWindowC1ERKS_(qthis, arg0)};
    let rsthis = QPaintDeviceWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPaintDeviceWindow {
  pub fn metaObject<T: QPaintDeviceWindow_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPaintDeviceWindow_metaObject {
  fn metaObject(self, this: &mut QPaintDeviceWindow) -> i32;
}

// proto: const QMetaObject * QPaintDeviceWindow::metaObject();
impl<'a> /*trait*/ QPaintDeviceWindow_metaObject for () {
  fn metaObject(self, this: &mut QPaintDeviceWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPaintDeviceWindow10metaObjectEv()};
    unsafe {_ZNK18QPaintDeviceWindow10metaObjectEv()};
    return 1;
  }
}

// proto: void QPaintDeviceWindow::update(const QRect & rect);
impl<'a> /*trait*/ QPaintDeviceWindow_update for (&'a  QRect) {
  fn update(self, this: &mut QPaintDeviceWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QPaintDeviceWindow6updateERK5QRect(arg0)};
    return 1;
  }
}

