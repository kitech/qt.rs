// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpixmap::QPixmap;
use super::qsize::QSize;
use super::qmatrix::QMatrix;
use super::qstring::QString;
use super::qtransform::QTransform;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QBitmap::NewQBitmap(const QPixmap & );
  fn _ZN7QBitmapC1ERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBitmap::NewQBitmap(const QSize & );
  fn _ZN7QBitmapC1ERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBitmap::NewQBitmap(int w, int h);
  fn _ZN7QBitmapC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QBitmap::FreeQBitmap();
  fn _ZN7QBitmapD0Ev(qthis: *mut c_void) ;
  // proto:  void QBitmap::swap(QBitmap & other);
  fn _ZN7QBitmap4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QBitmap QBitmap::transformed(const QMatrix & );
  fn _ZNK7QBitmap11transformedERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QBitmap::clear();
  fn _ZN7QBitmap5clearEv(qthis: *mut c_void) ;
  // proto:  void QBitmap::NewQBitmap(const QString & fileName, const char * format);
  fn _ZN7QBitmapC1ERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  void QBitmap::NewQBitmap();
  fn _ZN7QBitmapC1Ev(qthis: *mut c_void) ;
  // proto:  QBitmap QBitmap::transformed(const QTransform & matrix);
  fn _ZNK7QBitmap11transformedERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QBitmap)=1
pub struct QBitmap {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBitmap {
  pub fn NewQBitmap<T: QBitmap_NewQBitmap>(value: T) -> QBitmap {
    let rsthis = value.NewQBitmap();
    return rsthis;
    // return 1;
  }
}

pub trait QBitmap_NewQBitmap {
  fn NewQBitmap(self) -> QBitmap;
}

// proto: void QBitmap::NewQBitmap(const QPixmap & );
impl<'a> /*trait*/ QBitmap_NewQBitmap for (&'a  QPixmap) {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QBitmapC1ERK7QPixmap(qthis, arg0)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QBitmap::NewQBitmap(const QSize & );
impl<'a> /*trait*/ QBitmap_NewQBitmap for (&'a  QSize) {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QBitmapC1ERK5QSize(qthis, arg0)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QBitmap::NewQBitmap(int w, int h);
impl<'a> /*trait*/ QBitmap_NewQBitmap for (i32, i32) {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QBitmapC1Eii(qthis, arg0, arg1)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBitmap {
  pub fn FreeQBitmap<RetType, T: QBitmap_FreeQBitmap<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQBitmap(self);
    // return 1;
  }
}

pub trait QBitmap_FreeQBitmap<RetType> {
  fn FreeQBitmap(self, rsthis: &mut QBitmap) -> RetType;
}

// proto:  void QBitmap::FreeQBitmap();
impl<'a> /*trait*/ QBitmap_FreeQBitmap<()> for () {
  fn FreeQBitmap(self, rsthis: &mut QBitmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapD0Ev()};
     unsafe {_ZN7QBitmapD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBitmap {
  pub fn swap<RetType, T: QBitmap_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QBitmap_swap<RetType> {
  fn swap(self, rsthis: &mut QBitmap) -> RetType;
}

// proto:  void QBitmap::swap(QBitmap & other);
impl<'a> /*trait*/ QBitmap_swap<()> for (&'a mut QBitmap) {
  fn swap(self, rsthis: &mut QBitmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmap4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QBitmap4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBitmap {
  pub fn transformed<RetType, T: QBitmap_transformed<RetType>>(&mut self, value: T) -> RetType {
    return value.transformed(self);
    // return 1;
  }
}

pub trait QBitmap_transformed<RetType> {
  fn transformed(self, rsthis: &mut QBitmap) -> RetType;
}

// proto:  QBitmap QBitmap::transformed(const QMatrix & );
impl<'a> /*trait*/ QBitmap_transformed<QBitmap> for (&'a  QMatrix) {
  fn transformed(self, rsthis: &mut QBitmap) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBitmap11transformedERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QBitmap11transformedERK7QMatrix(rsthis.qclsinst, arg0)};
    let mut ret1 = QBitmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBitmap {
  pub fn clear<RetType, T: QBitmap_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QBitmap_clear<RetType> {
  fn clear(self, rsthis: &mut QBitmap) -> RetType;
}

// proto:  void QBitmap::clear();
impl<'a> /*trait*/ QBitmap_clear<()> for () {
  fn clear(self, rsthis: &mut QBitmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmap5clearEv()};
     unsafe {_ZN7QBitmap5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QBitmap::NewQBitmap(const QString & fileName, const char * format);
impl<'a> /*trait*/ QBitmap_NewQBitmap for (&'a  QString, &'a  String) {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN7QBitmapC1ERK7QStringPKc(qthis, arg0, arg1)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QBitmap::NewQBitmap();
impl<'a> /*trait*/ QBitmap_NewQBitmap for () {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1Ev()};
    unsafe {_ZN7QBitmapC1Ev(qthis)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QBitmap QBitmap::transformed(const QTransform & matrix);
impl<'a> /*trait*/ QBitmap_transformed<QBitmap> for (&'a  QTransform) {
  fn transformed(self, rsthis: &mut QBitmap) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBitmap11transformedERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QBitmap11transformedERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QBitmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

