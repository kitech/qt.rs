// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qsize::QSize;
use super::qsizef::QSizeF;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPageSize::NewQPageSize();
  fn _ZN9QPageSizeC1Ev(qthis: *mut c_void) ;
  // proto:  void QPageSize::NewQPageSize(const QString & key, const QSize & pointSize, const QString & name);
  fn _ZN9QPageSizeC1ERK7QStringRK5QSizeS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QPageSize::FreeQPageSize();
  fn _ZN9QPageSizeD0Ev(qthis: *mut c_void) ;
  // proto:  QString QPageSize::key();
  fn _ZNK9QPageSize3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QPageSize::name();
  fn _ZNK9QPageSize4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QPageSize::definitionSize();
  fn _ZNK9QPageSize14definitionSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPageSize::swap(QPageSize & other);
  fn _ZN9QPageSize4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QPageSize::windowsId();
  fn _ZNK9QPageSize9windowsIdEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QPageSize::sizePixels(int resolution);
  fn _ZNK9QPageSize10sizePixelsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QPageSize::NewQPageSize(const QPageSize & other);
  fn _ZN9QPageSizeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPageSize::isValid();
  fn _ZNK9QPageSize7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRect QPageSize::rectPixels(int resolution);
  fn _ZNK9QPageSize10rectPixelsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QRect QPageSize::rectPoints();
  fn _ZNK9QPageSize10rectPointsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPageSize::NewQPageSize(int windowsId, const QSize & pointSize, const QString & name);
  fn _ZN9QPageSizeC1EiRK5QSizeRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  bool QPageSize::isEquivalentTo(const QPageSize & other);
  fn _ZNK9QPageSize14isEquivalentToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QSize QPageSize::sizePoints();
  fn _ZNK9QPageSize10sizePointsEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QPageSize)=1
pub struct QPageSize {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPageSize {
  pub fn NewQPageSize<T: QPageSize_NewQPageSize>(value: T) -> QPageSize {
    let rsthis = value.NewQPageSize();
    return rsthis;
    // return 1;
  }
}

pub trait QPageSize_NewQPageSize {
  fn NewQPageSize(self) -> QPageSize;
}

// proto: void QPageSize::NewQPageSize();
impl<'a> /*trait*/ QPageSize_NewQPageSize for () {
  fn NewQPageSize(self) -> QPageSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1Ev()};
    unsafe {_ZN9QPageSizeC1Ev(qthis)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPageSize::NewQPageSize(const QString & key, const QSize & pointSize, const QString & name);
impl<'a> /*trait*/ QPageSize_NewQPageSize for (&'a  QString, &'a  QSize, &'a  QString) {
  fn NewQPageSize(self) -> QPageSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1ERK7QStringRK5QSizeS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN9QPageSizeC1ERK7QStringRK5QSizeS2_(qthis, arg0, arg1, arg2)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn FreeQPageSize<T: QPageSize_FreeQPageSize>(&mut self, value: T)  {
     value.FreeQPageSize(self);
    // return 1;
  }
}

pub trait QPageSize_FreeQPageSize {
  fn FreeQPageSize(self, rsthis: &mut QPageSize) ;
}

// proto:  void QPageSize::FreeQPageSize();
impl<'a> /*trait*/ QPageSize_FreeQPageSize for () {
  fn FreeQPageSize(self, rsthis: &mut QPageSize)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeD0Ev()};
     unsafe {_ZN9QPageSizeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn key<T: QPageSize_key>(&mut self, value: T) -> QString {
    return value.key(self);
    // return 1;
  }
}

pub trait QPageSize_key {
  fn key(self, rsthis: &mut QPageSize) -> QString;
}

// proto:  QString QPageSize::key();
impl<'a> /*trait*/ QPageSize_key for () {
  fn key(self, rsthis: &mut QPageSize) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize3keyEv()};
    let mut ret = unsafe {_ZNK9QPageSize3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn name<T: QPageSize_name>(&mut self, value: T) -> QString {
    return value.name(self);
    // return 1;
  }
}

pub trait QPageSize_name {
  fn name(self, rsthis: &mut QPageSize) -> QString;
}

// proto:  QString QPageSize::name();
impl<'a> /*trait*/ QPageSize_name for () {
  fn name(self, rsthis: &mut QPageSize) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize4nameEv()};
    let mut ret = unsafe {_ZNK9QPageSize4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn definitionSize<T: QPageSize_definitionSize>(&mut self, value: T) -> QSizeF {
    return value.definitionSize(self);
    // return 1;
  }
}

pub trait QPageSize_definitionSize {
  fn definitionSize(self, rsthis: &mut QPageSize) -> QSizeF;
}

// proto:  QSizeF QPageSize::definitionSize();
impl<'a> /*trait*/ QPageSize_definitionSize for () {
  fn definitionSize(self, rsthis: &mut QPageSize) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize14definitionSizeEv()};
    let mut ret = unsafe {_ZNK9QPageSize14definitionSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn swap<T: QPageSize_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QPageSize_swap {
  fn swap(self, rsthis: &mut QPageSize) ;
}

// proto:  void QPageSize::swap(QPageSize & other);
impl<'a> /*trait*/ QPageSize_swap for (&'a mut QPageSize) {
  fn swap(self, rsthis: &mut QPageSize)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSize4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QPageSize4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn windowsId<T: QPageSize_windowsId>(&mut self, value: T) -> i32 {
    return value.windowsId(self);
    // return 1;
  }
}

pub trait QPageSize_windowsId {
  fn windowsId(self, rsthis: &mut QPageSize) -> i32;
}

// proto:  int QPageSize::windowsId();
impl<'a> /*trait*/ QPageSize_windowsId for () {
  fn windowsId(self, rsthis: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize9windowsIdEv()};
    let mut ret = unsafe {_ZNK9QPageSize9windowsIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn sizePixels<T: QPageSize_sizePixels>(&mut self, value: T) -> QSize {
    return value.sizePixels(self);
    // return 1;
  }
}

pub trait QPageSize_sizePixels {
  fn sizePixels(self, rsthis: &mut QPageSize) -> QSize;
}

// proto:  QSize QPageSize::sizePixels(int resolution);
impl<'a> /*trait*/ QPageSize_sizePixels for (i32) {
  fn sizePixels(self, rsthis: &mut QPageSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10sizePixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QPageSize10sizePixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPageSize::NewQPageSize(const QPageSize & other);
impl<'a> /*trait*/ QPageSize_NewQPageSize for (&'a  QPageSize) {
  fn NewQPageSize(self) -> QPageSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QPageSizeC1ERKS_(qthis, arg0)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn isValid<T: QPageSize_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QPageSize_isValid {
  fn isValid(self, rsthis: &mut QPageSize) -> i8;
}

// proto:  bool QPageSize::isValid();
impl<'a> /*trait*/ QPageSize_isValid for () {
  fn isValid(self, rsthis: &mut QPageSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize7isValidEv()};
    let mut ret = unsafe {_ZNK9QPageSize7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn rectPixels<T: QPageSize_rectPixels>(&mut self, value: T) -> QRect {
    return value.rectPixels(self);
    // return 1;
  }
}

pub trait QPageSize_rectPixels {
  fn rectPixels(self, rsthis: &mut QPageSize) -> QRect;
}

// proto:  QRect QPageSize::rectPixels(int resolution);
impl<'a> /*trait*/ QPageSize_rectPixels for (i32) {
  fn rectPixels(self, rsthis: &mut QPageSize) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10rectPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QPageSize10rectPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn rectPoints<T: QPageSize_rectPoints>(&mut self, value: T) -> QRect {
    return value.rectPoints(self);
    // return 1;
  }
}

pub trait QPageSize_rectPoints {
  fn rectPoints(self, rsthis: &mut QPageSize) -> QRect;
}

// proto:  QRect QPageSize::rectPoints();
impl<'a> /*trait*/ QPageSize_rectPoints for () {
  fn rectPoints(self, rsthis: &mut QPageSize) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10rectPointsEv()};
    let mut ret = unsafe {_ZNK9QPageSize10rectPointsEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPageSize::NewQPageSize(int windowsId, const QSize & pointSize, const QString & name);
impl<'a> /*trait*/ QPageSize_NewQPageSize for (i32, &'a  QSize, &'a  QString) {
  fn NewQPageSize(self) -> QPageSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1EiRK5QSizeRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN9QPageSizeC1EiRK5QSizeRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn isEquivalentTo<T: QPageSize_isEquivalentTo>(&mut self, value: T) -> i8 {
    return value.isEquivalentTo(self);
    // return 1;
  }
}

pub trait QPageSize_isEquivalentTo {
  fn isEquivalentTo(self, rsthis: &mut QPageSize) -> i8;
}

// proto:  bool QPageSize::isEquivalentTo(const QPageSize & other);
impl<'a> /*trait*/ QPageSize_isEquivalentTo for (&'a  QPageSize) {
  fn isEquivalentTo(self, rsthis: &mut QPageSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize14isEquivalentToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPageSize14isEquivalentToERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn sizePoints<T: QPageSize_sizePoints>(&mut self, value: T) -> QSize {
    return value.sizePoints(self);
    // return 1;
  }
}

pub trait QPageSize_sizePoints {
  fn sizePoints(self, rsthis: &mut QPageSize) -> QSize;
}

// proto:  QSize QPageSize::sizePoints();
impl<'a> /*trait*/ QPageSize_sizePoints for () {
  fn sizePoints(self, rsthis: &mut QPageSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10sizePointsEv()};
    let mut ret = unsafe {_ZNK9QPageSize10sizePointsEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

