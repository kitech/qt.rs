// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtGui/qpagesize.h
// dst-file: /src/gui/qpagesize.rs
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
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qsize::QSizeF; // 771
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPageSize_Class_Size() -> c_int;
  // proto:  void QPageSize::QPageSize();
  fn dector_ZN9QPageSizeC1Ev() -> *mut c_void;
  fn _ZN9QPageSizeC1Ev(qthis: *mut c_void);
  // proto:  void QPageSize::QPageSize(const QString & key, const QSize & pointSize, const QString & name);
  fn dector_ZN9QPageSizeC1ERK7QStringRK5QSizeS2_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN9QPageSizeC1ERK7QStringRK5QSizeS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPageSize::~QPageSize();
  fn _ZN9QPageSizeD0Ev(qthis: *mut c_void);
  // proto:  QString QPageSize::key();
  fn _ZNK9QPageSize3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QPageSize::name();
  fn _ZNK9QPageSize4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QPageSize::definitionSize();
  fn _ZNK9QPageSize14definitionSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPageSize::swap(QPageSize & other);
  fn _ZN9QPageSize4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QPageSize::windowsId();
  fn _ZNK9QPageSize9windowsIdEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QPageSize::sizePixels(int resolution);
  fn _ZNK9QPageSize10sizePixelsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QPageSize::QPageSize(const QPageSize & other);
  fn dector_ZN9QPageSizeC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QPageSizeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPageSize::isValid();
  fn _ZNK9QPageSize7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QRect QPageSize::rectPixels(int resolution);
  fn _ZNK9QPageSize10rectPixelsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QRect QPageSize::rectPoints();
  fn _ZNK9QPageSize10rectPointsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPageSize::QPageSize(int windowsId, const QSize & pointSize, const QString & name);
  fn dector_ZN9QPageSizeC1EiRK5QSizeRK7QString(arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN9QPageSizeC1EiRK5QSizeRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QPageSize::isEquivalentTo(const QPageSize & other);
  fn _ZNK9QPageSize14isEquivalentToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QSize QPageSize::sizePoints();
  fn _ZNK9QPageSize10sizePointsEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QPageSize)=1
pub struct QPageSize {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPageSize {
  pub fn inheritFrom(qthis: *mut c_void) -> QPageSize {
    return QPageSize{qclsinst: qthis};
  }
}
  // proto:  void QPageSize::QPageSize();
impl /*struct*/ QPageSize {
  pub fn New<T: QPageSize_New>(value: T) -> QPageSize {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPageSize_New {
  fn New(self) -> QPageSize;
}

  // proto:  void QPageSize::QPageSize();
impl<'a> /*trait*/ QPageSize_New for () {
  fn New(self) -> QPageSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1Ev()};
    let ctysz: c_int = unsafe{QPageSize_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN9QPageSizeC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN9QPageSizeC1Ev()};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPageSize::QPageSize(const QString & key, const QSize & pointSize, const QString & name);
impl<'a> /*trait*/ QPageSize_New for (&'a QString, &'a QSize, &'a QString) {
  fn New(self) -> QPageSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1ERK7QStringRK5QSizeS2_()};
    let ctysz: c_int = unsafe{QPageSize_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN9QPageSizeC1ERK7QStringRK5QSizeS2_(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN9QPageSizeC1ERK7QStringRK5QSizeS2_(arg0, arg1, arg2)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPageSize::~QPageSize();
impl /*struct*/ QPageSize {
  pub fn Free<RetType, T: QPageSize_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPageSize_Free<RetType> {
  fn Free(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  void QPageSize::~QPageSize();
impl<'a> /*trait*/ QPageSize_Free<()> for () {
  fn Free(self , rsthis: & QPageSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeD0Ev()};
     unsafe {_ZN9QPageSizeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QPageSize::key();
impl /*struct*/ QPageSize {
  pub fn key<RetType, T: QPageSize_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QPageSize_key<RetType> {
  fn key(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  QString QPageSize::key();
impl<'a> /*trait*/ QPageSize_key<QString> for () {
  fn key(self , rsthis: & QPageSize) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize3keyEv()};
    let mut ret = unsafe {_ZNK9QPageSize3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QPageSize::name();
impl /*struct*/ QPageSize {
  pub fn name<RetType, T: QPageSize_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QPageSize_name<RetType> {
  fn name(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  QString QPageSize::name();
impl<'a> /*trait*/ QPageSize_name<QString> for () {
  fn name(self , rsthis: & QPageSize) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize4nameEv()};
    let mut ret = unsafe {_ZNK9QPageSize4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizeF QPageSize::definitionSize();
impl /*struct*/ QPageSize {
  pub fn definitionSize<RetType, T: QPageSize_definitionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.definitionSize(self);
    // return 1;
  }
}

pub trait QPageSize_definitionSize<RetType> {
  fn definitionSize(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  QSizeF QPageSize::definitionSize();
impl<'a> /*trait*/ QPageSize_definitionSize<QSizeF> for () {
  fn definitionSize(self , rsthis: & QPageSize) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize14definitionSizeEv()};
    let mut ret = unsafe {_ZNK9QPageSize14definitionSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPageSize::swap(QPageSize & other);
impl /*struct*/ QPageSize {
  pub fn swap<RetType, T: QPageSize_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPageSize_swap<RetType> {
  fn swap(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  void QPageSize::swap(QPageSize & other);
impl<'a> /*trait*/ QPageSize_swap<()> for (&'a QPageSize) {
  fn swap(self , rsthis: & QPageSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSize4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QPageSize4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPageSize::windowsId();
impl /*struct*/ QPageSize {
  pub fn windowsId<RetType, T: QPageSize_windowsId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowsId(self);
    // return 1;
  }
}

pub trait QPageSize_windowsId<RetType> {
  fn windowsId(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  int QPageSize::windowsId();
impl<'a> /*trait*/ QPageSize_windowsId<i32> for () {
  fn windowsId(self , rsthis: & QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize9windowsIdEv()};
    let mut ret = unsafe {_ZNK9QPageSize9windowsIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QPageSize::sizePixels(int resolution);
impl /*struct*/ QPageSize {
  pub fn sizePixels<RetType, T: QPageSize_sizePixels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizePixels(self);
    // return 1;
  }
}

pub trait QPageSize_sizePixels<RetType> {
  fn sizePixels(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  QSize QPageSize::sizePixels(int resolution);
impl<'a> /*trait*/ QPageSize_sizePixels<QSize> for (i32) {
  fn sizePixels(self , rsthis: & QPageSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10sizePixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QPageSize10sizePixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPageSize::QPageSize(const QPageSize & other);
impl<'a> /*trait*/ QPageSize_New for (&'a QPageSize) {
  fn New(self) -> QPageSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1ERKS_()};
    let ctysz: c_int = unsafe{QPageSize_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QPageSizeC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QPageSizeC1ERKS_(arg0)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPageSize::isValid();
impl /*struct*/ QPageSize {
  pub fn isValid<RetType, T: QPageSize_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QPageSize_isValid<RetType> {
  fn isValid(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  bool QPageSize::isValid();
impl<'a> /*trait*/ QPageSize_isValid<i8> for () {
  fn isValid(self , rsthis: & QPageSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize7isValidEv()};
    let mut ret = unsafe {_ZNK9QPageSize7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QPageSize::rectPixels(int resolution);
impl /*struct*/ QPageSize {
  pub fn rectPixels<RetType, T: QPageSize_rectPixels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rectPixels(self);
    // return 1;
  }
}

pub trait QPageSize_rectPixels<RetType> {
  fn rectPixels(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  QRect QPageSize::rectPixels(int resolution);
impl<'a> /*trait*/ QPageSize_rectPixels<QRect> for (i32) {
  fn rectPixels(self , rsthis: & QPageSize) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10rectPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QPageSize10rectPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QPageSize::rectPoints();
impl /*struct*/ QPageSize {
  pub fn rectPoints<RetType, T: QPageSize_rectPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rectPoints(self);
    // return 1;
  }
}

pub trait QPageSize_rectPoints<RetType> {
  fn rectPoints(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  QRect QPageSize::rectPoints();
impl<'a> /*trait*/ QPageSize_rectPoints<QRect> for () {
  fn rectPoints(self , rsthis: & QPageSize) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10rectPointsEv()};
    let mut ret = unsafe {_ZNK9QPageSize10rectPointsEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPageSize::QPageSize(int windowsId, const QSize & pointSize, const QString & name);
impl<'a> /*trait*/ QPageSize_New for (i32, &'a QSize, &'a QString) {
  fn New(self) -> QPageSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1EiRK5QSizeRK7QString()};
    let ctysz: c_int = unsafe{QPageSize_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN9QPageSizeC1EiRK5QSizeRK7QString(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN9QPageSizeC1EiRK5QSizeRK7QString(arg0, arg1, arg2)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPageSize::isEquivalentTo(const QPageSize & other);
impl /*struct*/ QPageSize {
  pub fn isEquivalentTo<RetType, T: QPageSize_isEquivalentTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEquivalentTo(self);
    // return 1;
  }
}

pub trait QPageSize_isEquivalentTo<RetType> {
  fn isEquivalentTo(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  bool QPageSize::isEquivalentTo(const QPageSize & other);
impl<'a> /*trait*/ QPageSize_isEquivalentTo<i8> for (&'a QPageSize) {
  fn isEquivalentTo(self , rsthis: & QPageSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize14isEquivalentToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPageSize14isEquivalentToERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QPageSize::sizePoints();
impl /*struct*/ QPageSize {
  pub fn sizePoints<RetType, T: QPageSize_sizePoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizePoints(self);
    // return 1;
  }
}

pub trait QPageSize_sizePoints<RetType> {
  fn sizePoints(self , rsthis: & QPageSize) -> RetType;
}

  // proto:  QSize QPageSize::sizePoints();
impl<'a> /*trait*/ QPageSize_sizePoints<QSize> for () {
  fn sizePoints(self , rsthis: & QPageSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10sizePointsEv()};
    let mut ret = unsafe {_ZNK9QPageSize10sizePointsEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

