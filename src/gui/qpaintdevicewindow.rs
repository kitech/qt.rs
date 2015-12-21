// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qpaintdevicewindow.h
// dst-file: /src/gui/qpaintdevicewindow.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qregion::QRegion; // 773
use super::qwindow::QWindow; // 773
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QPaintDeviceWindow::update(const QRegion & region);
  fn _ZN18QPaintDeviceWindow6updateERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintDeviceWindow::update();
  fn _ZN18QPaintDeviceWindow6updateEv(qthis: *mut c_void);
  // proto:  void QPaintDeviceWindow::QPaintDeviceWindow(const QPaintDeviceWindow & );
  fn _ZN18QPaintDeviceWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QPaintDeviceWindow::metaObject();
  fn _ZNK18QPaintDeviceWindow10metaObjectEv(qthis: *mut c_void);
  // proto:  void QPaintDeviceWindow::update(const QRect & rect);
  fn _ZN18QPaintDeviceWindow6updateERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPaintDeviceWindow)=1
pub struct QPaintDeviceWindow {
  pub qclsinst: *mut c_void,
}

  // proto:  void QPaintDeviceWindow::update(const QRegion & region);
impl /*struct*/ QPaintDeviceWindow {
  pub fn update<RetType, T: QPaintDeviceWindow_update<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QPaintDeviceWindow_update<RetType> {
  fn update(self , rsthis: &mut QPaintDeviceWindow) -> RetType;
}

  // proto:  void QPaintDeviceWindow::update(const QRegion & region);
impl<'a> /*trait*/ QPaintDeviceWindow_update<()> for (QRegion) {
  fn update(self , rsthis: &mut QPaintDeviceWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QPaintDeviceWindow6updateERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintDeviceWindow::update();
impl<'a> /*trait*/ QPaintDeviceWindow_update<()> for () {
  fn update(self , rsthis: &mut QPaintDeviceWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateEv()};
     unsafe {_ZN18QPaintDeviceWindow6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPaintDeviceWindow::QPaintDeviceWindow(const QPaintDeviceWindow & );
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

  // proto:  void QPaintDeviceWindow::QPaintDeviceWindow(const QPaintDeviceWindow & );
impl<'a> /*trait*/ QPaintDeviceWindow_NewQPaintDeviceWindow for (QPaintDeviceWindow) {
  fn NewQPaintDeviceWindow(self) -> QPaintDeviceWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QPaintDeviceWindowC1ERKS_(qthis, arg0)};
    let rsthis = QPaintDeviceWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPaintDeviceWindow::metaObject();
impl /*struct*/ QPaintDeviceWindow {
  pub fn metaObject<RetType, T: QPaintDeviceWindow_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPaintDeviceWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QPaintDeviceWindow) -> RetType;
}

  // proto:  const QMetaObject * QPaintDeviceWindow::metaObject();
impl<'a> /*trait*/ QPaintDeviceWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QPaintDeviceWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPaintDeviceWindow10metaObjectEv()};
     unsafe {_ZNK18QPaintDeviceWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPaintDeviceWindow::update(const QRect & rect);
impl<'a> /*trait*/ QPaintDeviceWindow_update<()> for (QRect) {
  fn update(self , rsthis: &mut QPaintDeviceWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QPaintDeviceWindow6updateERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

