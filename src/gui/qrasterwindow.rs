// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qrasterwindow.h
// dst-file: /src/gui/qrasterwindow.rs
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
use super::qwindow::QWindow; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
  fn _ZN13QRasterWindowC1EP7QWindow(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QRasterWindow::metaObject();
  fn _ZNK13QRasterWindow10metaObjectEv(qthis: *mut c_void);
  // proto:  void QRasterWindow::QRasterWindow(const QRasterWindow & );
  fn _ZN13QRasterWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRasterWindow)=1
pub struct QRasterWindow {
  pub qclsinst: *mut c_void,
}

  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
impl /*struct*/ QRasterWindow {
  pub fn NewQRasterWindow<T: QRasterWindow_NewQRasterWindow>(value: T) -> QRasterWindow {
    let rsthis = value.NewQRasterWindow();
    return rsthis;
    // return 1;
  }
}

pub trait QRasterWindow_NewQRasterWindow {
  fn NewQRasterWindow(self) -> QRasterWindow;
}

  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
impl<'a> /*trait*/ QRasterWindow_NewQRasterWindow for (QWindow) {
  fn NewQRasterWindow(self) -> QRasterWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QRasterWindowC1EP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QRasterWindowC1EP7QWindow(qthis, arg0)};
    let rsthis = QRasterWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QRasterWindow::metaObject();
impl /*struct*/ QRasterWindow {
  pub fn metaObject<RetType, T: QRasterWindow_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRasterWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QRasterWindow) -> RetType;
}

  // proto:  const QMetaObject * QRasterWindow::metaObject();
impl<'a> /*trait*/ QRasterWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QRasterWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QRasterWindow10metaObjectEv()};
     unsafe {_ZNK13QRasterWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRasterWindow::QRasterWindow(const QRasterWindow & );
impl<'a> /*trait*/ QRasterWindow_NewQRasterWindow for (QRasterWindow) {
  fn NewQRasterWindow(self) -> QRasterWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QRasterWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QRasterWindowC1ERKS_(qthis, arg0)};
    let rsthis = QRasterWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

