// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
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
use super::qpaintdevicewindow::QPaintDeviceWindow; // 773
use std::ops::Deref;
use super::qwindow::QWindow; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRasterWindow_Class_Size() -> c_int;
  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
  fn dector_ZN13QRasterWindowC1EP7QWindow(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QRasterWindowC1EP7QWindow(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QRasterWindow::metaObject();
  fn _ZNK13QRasterWindow10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QRasterWindow::QRasterWindow(const QRasterWindow & );
  fn dector_ZN13QRasterWindowC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QRasterWindowC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRasterWindow)=1
#[derive(Default)]
pub struct QRasterWindow {
  qbase: QPaintDeviceWindow,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QRasterWindow {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRasterWindow {
    return QRasterWindow{qbase: QPaintDeviceWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QRasterWindow {
  type Target = QPaintDeviceWindow;

  fn deref(&self) -> &QPaintDeviceWindow {
    return & self.qbase;
  }
}
impl AsRef<QPaintDeviceWindow> for QRasterWindow {
  fn as_ref(& self) -> & QPaintDeviceWindow {
    return & self.qbase;
  }
}
  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
impl /*struct*/ QRasterWindow {
  pub fn new<T: QRasterWindow_new>(value: T) -> QRasterWindow {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRasterWindow_new {
  fn new(self) -> QRasterWindow;
}

  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
impl<'a> /*trait*/ QRasterWindow_new for (&'a QWindow) {
  fn new(self) -> QRasterWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QRasterWindowC1EP7QWindow()};
    let ctysz: c_int = unsafe{QRasterWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QRasterWindowC1EP7QWindow(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QRasterWindowC1EP7QWindow(arg0)} as u64;
    let rsthis = QRasterWindow{qbase: QPaintDeviceWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QRasterWindow::metaObject();
impl /*struct*/ QRasterWindow {
  pub fn metaObject<RetType, T: QRasterWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRasterWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QRasterWindow) -> RetType;
}

  // proto:  const QMetaObject * QRasterWindow::metaObject();
impl<'a> /*trait*/ QRasterWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: & QRasterWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QRasterWindow10metaObjectEv()};
     unsafe {_ZNK13QRasterWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRasterWindow::QRasterWindow(const QRasterWindow & );
impl<'a> /*trait*/ QRasterWindow_new for (&'a QRasterWindow) {
  fn new(self) -> QRasterWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QRasterWindowC1ERKS_()};
    let ctysz: c_int = unsafe{QRasterWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QRasterWindowC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QRasterWindowC1ERKS_(arg0)} as u64;
    let rsthis = QRasterWindow{qbase: QPaintDeviceWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

