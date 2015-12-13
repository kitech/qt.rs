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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPageSize::NewQPageSize();
  fn _ZN9QPageSizeC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QPageSize::NewQPageSize(const QString & key, const QSize & pointSize, const QString & name);
  fn _ZN9QPageSizeC1ERK7QStringRK5QSizeS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPageSize::FreeQPageSize();
  fn _ZN9QPageSizeD0Ev() -> i32;
  // proto: QString QPageSize::key();
  fn _ZNK9QPageSize3keyEv() -> i32;
  // proto: QString QPageSize::name();
  fn _ZNK9QPageSize4nameEv() -> i32;
  // proto: QSizeF QPageSize::definitionSize();
  fn _ZNK9QPageSize14definitionSizeEv() -> i32;
  // proto: void QPageSize::swap(QPageSize & other);
  fn _ZN9QPageSize4swapERS_(arg0: *mut c_void) -> i32;
  // proto: int QPageSize::windowsId();
  fn _ZNK9QPageSize9windowsIdEv() -> i32;
  // proto: QSize QPageSize::sizePixels(int resolution);
  fn _ZNK9QPageSize10sizePixelsEi(arg0: c_int) -> i32;
  // proto: void QPageSize::NewQPageSize(const QPageSize & other);
  fn _ZN9QPageSizeC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QPageSize::isValid();
  fn _ZNK9QPageSize7isValidEv() -> i32;
  // proto: QRect QPageSize::rectPixels(int resolution);
  fn _ZNK9QPageSize10rectPixelsEi(arg0: c_int) -> i32;
  // proto: QRect QPageSize::rectPoints();
  fn _ZNK9QPageSize10rectPointsEv() -> i32;
  // proto: void QPageSize::NewQPageSize(int windowsId, const QSize & pointSize, const QString & name);
  fn _ZN9QPageSizeC1EiRK5QSizeRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: bool QPageSize::isEquivalentTo(const QPageSize & other);
  fn _ZNK9QPageSize14isEquivalentToERKS_(arg0: *const c_void) -> i32;
  // proto: QSize QPageSize::sizePoints();
  fn _ZNK9QPageSize10sizePointsEv() -> i32;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN9QPageSizeC1ERK7QStringRK5QSizeS2_(qthis, arg0, arg1, arg2)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn FreeQPageSize<T: QPageSize_FreeQPageSize>(&mut self, value: T) -> i32 {
    value.FreeQPageSize(self);
    return 1;
  }
}

pub trait QPageSize_FreeQPageSize {
  fn FreeQPageSize(self, this: &mut QPageSize) -> i32;
}

// proto: void QPageSize::FreeQPageSize();
impl<'a> /*trait*/ QPageSize_FreeQPageSize for () {
  fn FreeQPageSize(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeD0Ev()};
    unsafe {_ZN9QPageSizeD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn key<T: QPageSize_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QPageSize_key {
  fn key(self, this: &mut QPageSize) -> i32;
}

// proto: QString QPageSize::key();
impl<'a> /*trait*/ QPageSize_key for () {
  fn key(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize3keyEv()};
    unsafe {_ZNK9QPageSize3keyEv()};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn name<T: QPageSize_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QPageSize_name {
  fn name(self, this: &mut QPageSize) -> i32;
}

// proto: QString QPageSize::name();
impl<'a> /*trait*/ QPageSize_name for () {
  fn name(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize4nameEv()};
    unsafe {_ZNK9QPageSize4nameEv()};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn definitionSize<T: QPageSize_definitionSize>(&mut self, value: T) -> i32 {
    value.definitionSize(self);
    return 1;
  }
}

pub trait QPageSize_definitionSize {
  fn definitionSize(self, this: &mut QPageSize) -> i32;
}

// proto: QSizeF QPageSize::definitionSize();
impl<'a> /*trait*/ QPageSize_definitionSize for () {
  fn definitionSize(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize14definitionSizeEv()};
    unsafe {_ZNK9QPageSize14definitionSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn swap<T: QPageSize_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPageSize_swap {
  fn swap(self, this: &mut QPageSize) -> i32;
}

// proto: void QPageSize::swap(QPageSize & other);
impl<'a> /*trait*/ QPageSize_swap for (&'a mut QPageSize) {
  fn swap(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSize4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QPageSize4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn windowsId<T: QPageSize_windowsId>(&mut self, value: T) -> i32 {
    value.windowsId(self);
    return 1;
  }
}

pub trait QPageSize_windowsId {
  fn windowsId(self, this: &mut QPageSize) -> i32;
}

// proto: int QPageSize::windowsId();
impl<'a> /*trait*/ QPageSize_windowsId for () {
  fn windowsId(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize9windowsIdEv()};
    unsafe {_ZNK9QPageSize9windowsIdEv()};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn sizePixels<T: QPageSize_sizePixels>(&mut self, value: T) -> i32 {
    value.sizePixels(self);
    return 1;
  }
}

pub trait QPageSize_sizePixels {
  fn sizePixels(self, this: &mut QPageSize) -> i32;
}

// proto: QSize QPageSize::sizePixels(int resolution);
impl<'a> /*trait*/ QPageSize_sizePixels for (i32) {
  fn sizePixels(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10sizePixelsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QPageSize10sizePixelsEi(arg0)};
    return 1;
  }
}

// proto: void QPageSize::NewQPageSize(const QPageSize & other);
impl<'a> /*trait*/ QPageSize_NewQPageSize for (&'a  QPageSize) {
  fn NewQPageSize(self) -> QPageSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QPageSizeC1ERKS_(qthis, arg0)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn isValid<T: QPageSize_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QPageSize_isValid {
  fn isValid(self, this: &mut QPageSize) -> i32;
}

// proto: bool QPageSize::isValid();
impl<'a> /*trait*/ QPageSize_isValid for () {
  fn isValid(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize7isValidEv()};
    unsafe {_ZNK9QPageSize7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn rectPixels<T: QPageSize_rectPixels>(&mut self, value: T) -> i32 {
    value.rectPixels(self);
    return 1;
  }
}

pub trait QPageSize_rectPixels {
  fn rectPixels(self, this: &mut QPageSize) -> i32;
}

// proto: QRect QPageSize::rectPixels(int resolution);
impl<'a> /*trait*/ QPageSize_rectPixels for (i32) {
  fn rectPixels(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10rectPixelsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QPageSize10rectPixelsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn rectPoints<T: QPageSize_rectPoints>(&mut self, value: T) -> i32 {
    value.rectPoints(self);
    return 1;
  }
}

pub trait QPageSize_rectPoints {
  fn rectPoints(self, this: &mut QPageSize) -> i32;
}

// proto: QRect QPageSize::rectPoints();
impl<'a> /*trait*/ QPageSize_rectPoints for () {
  fn rectPoints(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10rectPointsEv()};
    unsafe {_ZNK9QPageSize10rectPointsEv()};
    return 1;
  }
}

// proto: void QPageSize::NewQPageSize(int windowsId, const QSize & pointSize, const QString & name);
impl<'a> /*trait*/ QPageSize_NewQPageSize for (i32, &'a  QSize, &'a  QString) {
  fn NewQPageSize(self) -> QPageSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPageSizeC1EiRK5QSizeRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN9QPageSizeC1EiRK5QSizeRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QPageSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn isEquivalentTo<T: QPageSize_isEquivalentTo>(&mut self, value: T) -> i32 {
    value.isEquivalentTo(self);
    return 1;
  }
}

pub trait QPageSize_isEquivalentTo {
  fn isEquivalentTo(self, this: &mut QPageSize) -> i32;
}

// proto: bool QPageSize::isEquivalentTo(const QPageSize & other);
impl<'a> /*trait*/ QPageSize_isEquivalentTo for (&'a  QPageSize) {
  fn isEquivalentTo(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize14isEquivalentToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QPageSize14isEquivalentToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageSize {
  pub fn sizePoints<T: QPageSize_sizePoints>(&mut self, value: T) -> i32 {
    value.sizePoints(self);
    return 1;
  }
}

pub trait QPageSize_sizePoints {
  fn sizePoints(self, this: &mut QPageSize) -> i32;
}

// proto: QSize QPageSize::sizePoints();
impl<'a> /*trait*/ QPageSize_sizePoints for () {
  fn sizePoints(self, this: &mut QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPageSize10sizePointsEv()};
    unsafe {_ZNK9QPageSize10sizePointsEv()};
    return 1;
  }
}

