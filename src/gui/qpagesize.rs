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
  // proto:  void QPageSize::QPageSize();
  fn _ZN9QPageSizeC1Ev(qthis: *mut c_void);
  // proto:  void QPageSize::QPageSize(const QString & key, const QSize & pointSize, const QString & name);
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
  fn _ZN9QPageSizeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPageSize::isValid();
  fn _ZNK9QPageSize7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QRect QPageSize::rectPixels(int resolution);
  fn _ZNK9QPageSize10rectPixelsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QRect QPageSize::rectPoints();
  fn _ZNK9QPageSize10rectPointsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPageSize::QPageSize(int windowsId, const QSize & pointSize, const QString & name);
  fn _ZN9QPageSizeC1EiRK5QSizeRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QPageSize::isEquivalentTo(const QPageSize & other);
  fn _ZNK9QPageSize14isEquivalentToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QSize QPageSize::sizePoints();
  fn _ZNK9QPageSize10sizePointsEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QPageSize)=1
pub struct QPageSize {
  pub qclsinst: *mut c_void,
}

  // proto:  void QPageSize::QPageSize();
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

  // proto:  void QPageSize::QPageSize();
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

  // proto:  void QPageSize::QPageSize(const QString & key, const QSize & pointSize, const QString & name);
impl<'a> /*trait*/ QPageSize_NewQPageSize for (QString, QSize, QString) {
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

  // proto:  void QPageSize::~QPageSize();
impl /*struct*/ QPageSize {
  pub fn FreeQPageSize<RetType, T: QPageSize_FreeQPageSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPageSize(self);
    // return 1;
  }
}

pub trait QPageSize_FreeQPageSize<RetType> {
  fn FreeQPageSize(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  void QPageSize::~QPageSize();
impl<'a> /*trait*/ QPageSize_FreeQPageSize<()> for () {
  fn FreeQPageSize(self , rsthis: &mut QPageSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeD0Ev()};
     unsafe {_ZN9QPageSizeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QPageSize::key();
impl /*struct*/ QPageSize {
  pub fn key<RetType, T: QPageSize_key<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QPageSize_key<RetType> {
  fn key(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  QString QPageSize::key();
impl<'a> /*trait*/ QPageSize_key<QString> for () {
  fn key(self , rsthis: &mut QPageSize) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize3keyEv()};
    let mut ret = unsafe {_ZNK9QPageSize3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QPageSize::name();
impl /*struct*/ QPageSize {
  pub fn name<RetType, T: QPageSize_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QPageSize_name<RetType> {
  fn name(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  QString QPageSize::name();
impl<'a> /*trait*/ QPageSize_name<QString> for () {
  fn name(self , rsthis: &mut QPageSize) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize4nameEv()};
    let mut ret = unsafe {_ZNK9QPageSize4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSizeF QPageSize::definitionSize();
impl /*struct*/ QPageSize {
  pub fn definitionSize<RetType, T: QPageSize_definitionSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.definitionSize(self);
    // return 1;
  }
}

pub trait QPageSize_definitionSize<RetType> {
  fn definitionSize(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  QSizeF QPageSize::definitionSize();
impl<'a> /*trait*/ QPageSize_definitionSize<QSizeF> for () {
  fn definitionSize(self , rsthis: &mut QPageSize) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize14definitionSizeEv()};
    let mut ret = unsafe {_ZNK9QPageSize14definitionSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPageSize::swap(QPageSize & other);
impl /*struct*/ QPageSize {
  pub fn swap<RetType, T: QPageSize_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPageSize_swap<RetType> {
  fn swap(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  void QPageSize::swap(QPageSize & other);
impl<'a> /*trait*/ QPageSize_swap<()> for (QPageSize) {
  fn swap(self , rsthis: &mut QPageSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSize4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QPageSize4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPageSize::windowsId();
impl /*struct*/ QPageSize {
  pub fn windowsId<RetType, T: QPageSize_windowsId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.windowsId(self);
    // return 1;
  }
}

pub trait QPageSize_windowsId<RetType> {
  fn windowsId(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  int QPageSize::windowsId();
impl<'a> /*trait*/ QPageSize_windowsId<i32> for () {
  fn windowsId(self , rsthis: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize9windowsIdEv()};
    let mut ret = unsafe {_ZNK9QPageSize9windowsIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QPageSize::sizePixels(int resolution);
impl /*struct*/ QPageSize {
  pub fn sizePixels<RetType, T: QPageSize_sizePixels<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizePixels(self);
    // return 1;
  }
}

pub trait QPageSize_sizePixels<RetType> {
  fn sizePixels(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  QSize QPageSize::sizePixels(int resolution);
impl<'a> /*trait*/ QPageSize_sizePixels<QSize> for (i32) {
  fn sizePixels(self , rsthis: &mut QPageSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10sizePixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QPageSize10sizePixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPageSize::QPageSize(const QPageSize & other);
impl<'a> /*trait*/ QPageSize_NewQPageSize for (QPageSize) {
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

  // proto:  bool QPageSize::isValid();
impl /*struct*/ QPageSize {
  pub fn isValid<RetType, T: QPageSize_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QPageSize_isValid<RetType> {
  fn isValid(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  bool QPageSize::isValid();
impl<'a> /*trait*/ QPageSize_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QPageSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize7isValidEv()};
    let mut ret = unsafe {_ZNK9QPageSize7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QPageSize::rectPixels(int resolution);
impl /*struct*/ QPageSize {
  pub fn rectPixels<RetType, T: QPageSize_rectPixels<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rectPixels(self);
    // return 1;
  }
}

pub trait QPageSize_rectPixels<RetType> {
  fn rectPixels(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  QRect QPageSize::rectPixels(int resolution);
impl<'a> /*trait*/ QPageSize_rectPixels<QRect> for (i32) {
  fn rectPixels(self , rsthis: &mut QPageSize) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10rectPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QPageSize10rectPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QPageSize::rectPoints();
impl /*struct*/ QPageSize {
  pub fn rectPoints<RetType, T: QPageSize_rectPoints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rectPoints(self);
    // return 1;
  }
}

pub trait QPageSize_rectPoints<RetType> {
  fn rectPoints(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  QRect QPageSize::rectPoints();
impl<'a> /*trait*/ QPageSize_rectPoints<QRect> for () {
  fn rectPoints(self , rsthis: &mut QPageSize) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10rectPointsEv()};
    let mut ret = unsafe {_ZNK9QPageSize10rectPointsEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPageSize::QPageSize(int windowsId, const QSize & pointSize, const QString & name);
impl<'a> /*trait*/ QPageSize_NewQPageSize for (i32, QSize, QString) {
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

  // proto:  bool QPageSize::isEquivalentTo(const QPageSize & other);
impl /*struct*/ QPageSize {
  pub fn isEquivalentTo<RetType, T: QPageSize_isEquivalentTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEquivalentTo(self);
    // return 1;
  }
}

pub trait QPageSize_isEquivalentTo<RetType> {
  fn isEquivalentTo(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  bool QPageSize::isEquivalentTo(const QPageSize & other);
impl<'a> /*trait*/ QPageSize_isEquivalentTo<i8> for (QPageSize) {
  fn isEquivalentTo(self , rsthis: &mut QPageSize) -> i8 {
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
  pub fn sizePoints<RetType, T: QPageSize_sizePoints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizePoints(self);
    // return 1;
  }
}

pub trait QPageSize_sizePoints<RetType> {
  fn sizePoints(self , rsthis: &mut QPageSize) -> RetType;
}

  // proto:  QSize QPageSize::sizePoints();
impl<'a> /*trait*/ QPageSize_sizePoints<QSize> for () {
  fn sizePoints(self , rsthis: &mut QPageSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10sizePointsEv()};
    let mut ret = unsafe {_ZNK9QPageSize10sizePointsEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

