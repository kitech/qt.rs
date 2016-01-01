// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtGui/qbitmap.h
// dst-file: /src/gui/qbitmap.rs
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
use super::qpixmap::QPixmap; // 773
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
use super::qmatrix::QMatrix; // 773
use super::super::core::qstring::QString; // 771
use super::qimage::QImage; // 773
use super::qtransform::QTransform; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QBitmap_Class_Size() -> c_int;
  // proto:  void QBitmap::QBitmap(const QPixmap & );
  fn dector_ZN7QBitmapC1ERK7QPixmap(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QBitmapC1ERK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBitmap::QBitmap(const QSize & );
  fn dector_ZN7QBitmapC1ERK5QSize(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QBitmapC1ERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBitmap::QBitmap(int w, int h);
  fn dector_ZN7QBitmapC1Eii(arg0: c_int, arg1: c_int) -> *mut c_void;
  fn _ZN7QBitmapC1Eii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QBitmap::~QBitmap();
  fn _ZN7QBitmapD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QBitmap::swap(QBitmap & other);
  fn demth_ZN7QBitmap4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QBitmap QBitmap::transformed(const QMatrix & );
  fn _ZNK7QBitmap11transformedERK7QMatrix(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QBitmap::clear();
  fn demth_ZN7QBitmap5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QBitmap::QBitmap(const QString & fileName, const char * format);
  fn dector_ZN7QBitmapC1ERK7QStringPKc(arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  fn _ZN7QBitmapC1ERK7QStringPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QBitmap::QBitmap();
  fn dector_ZN7QBitmapC1Ev() -> *mut c_void;
  fn _ZN7QBitmapC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QBitmap QBitmap::transformed(const QTransform & matrix);
  fn _ZNK7QBitmap11transformedERK10QTransform(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QBitmap)=1
#[derive(Default)]
pub struct QBitmap {
  qbase: QPixmap,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QBitmap {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QBitmap {
    return QBitmap{qbase: QPixmap::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QBitmap {
  type Target = QPixmap;

  fn deref(&self) -> &QPixmap {
    return & self.qbase;
  }
}
impl AsRef<QPixmap> for QBitmap {
  fn as_ref(& self) -> & QPixmap {
    return & self.qbase;
  }
}
  // proto:  void QBitmap::QBitmap(const QPixmap & );
impl /*struct*/ QBitmap {
  pub fn new<T: QBitmap_new>(value: T) -> QBitmap {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QBitmap_new {
  fn new(self) -> QBitmap;
}

  // proto:  void QBitmap::QBitmap(const QPixmap & );
impl<'a> /*trait*/ QBitmap_new for (&'a QPixmap) {
  fn new(self) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK7QPixmap()};
    let ctysz: c_int = unsafe{QBitmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QBitmapC1ERK7QPixmap(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QBitmapC1ERK7QPixmap(arg0)} as u64;
    let rsthis = QBitmap{qbase: QPixmap::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBitmap::QBitmap(const QSize & );
impl<'a> /*trait*/ QBitmap_new for (&'a QSize) {
  fn new(self) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK5QSize()};
    let ctysz: c_int = unsafe{QBitmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QBitmapC1ERK5QSize(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QBitmapC1ERK5QSize(arg0)} as u64;
    let rsthis = QBitmap{qbase: QPixmap::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBitmap::QBitmap(int w, int h);
impl<'a> /*trait*/ QBitmap_new for (i32, i32) {
  fn new(self) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1Eii()};
    let ctysz: c_int = unsafe{QBitmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN7QBitmapC1Eii(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN7QBitmapC1Eii(arg0, arg1)} as u64;
    let rsthis = QBitmap{qbase: QPixmap::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBitmap::~QBitmap();
impl /*struct*/ QBitmap {
  pub fn free<RetType, T: QBitmap_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QBitmap_free<RetType> {
  fn free(self , rsthis: & QBitmap) -> RetType;
}

  // proto:  void QBitmap::~QBitmap();
impl<'a> /*trait*/ QBitmap_free<()> for () {
  fn free(self , rsthis: & QBitmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapD0Ev()};
     unsafe {_ZN7QBitmapD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBitmap::swap(QBitmap & other);
impl /*struct*/ QBitmap {
  pub fn swap<RetType, T: QBitmap_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QBitmap_swap<RetType> {
  fn swap(self , rsthis: & QBitmap) -> RetType;
}

  // proto:  void QBitmap::swap(QBitmap & other);
impl<'a> /*trait*/ QBitmap_swap<()> for (&'a QBitmap) {
  fn swap(self , rsthis: & QBitmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmap4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN7QBitmap4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QBitmap QBitmap::transformed(const QMatrix & );
impl /*struct*/ QBitmap {
  pub fn transformed<RetType, T: QBitmap_transformed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transformed(self);
    // return 1;
  }
}

pub trait QBitmap_transformed<RetType> {
  fn transformed(self , rsthis: & QBitmap) -> RetType;
}

  // proto:  QBitmap QBitmap::transformed(const QMatrix & );
impl<'a> /*trait*/ QBitmap_transformed<QBitmap> for (&'a QMatrix) {
  fn transformed(self , rsthis: & QBitmap) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBitmap11transformedERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QBitmap11transformedERK7QMatrix(rsthis.qclsinst, arg0)};
    let mut ret1 = QBitmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBitmap::clear();
impl /*struct*/ QBitmap {
  pub fn clear<RetType, T: QBitmap_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QBitmap_clear<RetType> {
  fn clear(self , rsthis: & QBitmap) -> RetType;
}

  // proto:  void QBitmap::clear();
impl<'a> /*trait*/ QBitmap_clear<()> for () {
  fn clear(self , rsthis: & QBitmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmap5clearEv()};
     unsafe {demth_ZN7QBitmap5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBitmap::QBitmap(const QString & fileName, const char * format);
impl<'a> /*trait*/ QBitmap_new for (&'a QString, &'a  String) {
  fn new(self) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK7QStringPKc()};
    let ctysz: c_int = unsafe{QBitmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    // unsafe {_ZN7QBitmapC1ERK7QStringPKc(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN7QBitmapC1ERK7QStringPKc(arg0, arg1)} as u64;
    let rsthis = QBitmap{qbase: QPixmap::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBitmap::QBitmap();
impl<'a> /*trait*/ QBitmap_new for () {
  fn new(self) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1Ev()};
    let ctysz: c_int = unsafe{QBitmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN7QBitmapC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN7QBitmapC1Ev()} as u64;
    let rsthis = QBitmap{qbase: QPixmap::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QBitmap QBitmap::transformed(const QTransform & matrix);
impl<'a> /*trait*/ QBitmap_transformed<QBitmap> for (&'a QTransform) {
  fn transformed(self , rsthis: & QBitmap) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBitmap11transformedERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QBitmap11transformedERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QBitmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

