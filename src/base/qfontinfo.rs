// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qfont::QFont;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QFontInfo::rawMode();
  fn _ZNK9QFontInfo7rawModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFontInfo::exactMatch();
  fn _ZNK9QFontInfo10exactMatchEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QFontInfo::pointSize();
  fn _ZNK9QFontInfo9pointSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFontInfo::NewQFontInfo(const QFontInfo & );
  fn _ZN9QFontInfoC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QFontInfo::family();
  fn _ZNK9QFontInfo6familyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFontInfo::bold();
  fn _ZNK9QFontInfo4boldEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QFontInfo::pointSizeF();
  fn _ZNK9QFontInfo10pointSizeFEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QFontInfo::fixedPitch();
  fn _ZNK9QFontInfo10fixedPitchEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFontInfo::overline();
  fn _ZNK9QFontInfo8overlineEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFontInfo::swap(QFontInfo & other);
  fn _ZN9QFontInfo4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFontInfo::NewQFontInfo(const QFont & );
  fn _ZN9QFontInfoC1ERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QFontInfo::pixelSize();
  fn _ZNK9QFontInfo9pixelSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QFontInfo::strikeOut();
  fn _ZNK9QFontInfo9strikeOutEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFontInfo::FreeQFontInfo();
  fn _ZN9QFontInfoD0Ev(qthis: *mut c_void) ;
  // proto:  bool QFontInfo::italic();
  fn _ZNK9QFontInfo6italicEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFontInfo::underline();
  fn _ZNK9QFontInfo9underlineEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFontInfo::styleName();
  fn _ZNK9QFontInfo9styleNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QFontInfo::weight();
  fn _ZNK9QFontInfo6weightEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QFontInfo)=1
pub struct QFontInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontInfo {
  pub fn rawMode<RetType, T: QFontInfo_rawMode<RetType>>(&mut self, value: T) -> RetType {
    return value.rawMode(self);
    // return 1;
  }
}

pub trait QFontInfo_rawMode<RetType> {
  fn rawMode(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  bool QFontInfo::rawMode();
impl<'a> /*trait*/ QFontInfo_rawMode<i8> for () {
  fn rawMode(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo7rawModeEv()};
    let mut ret = unsafe {_ZNK9QFontInfo7rawModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn exactMatch<RetType, T: QFontInfo_exactMatch<RetType>>(&mut self, value: T) -> RetType {
    return value.exactMatch(self);
    // return 1;
  }
}

pub trait QFontInfo_exactMatch<RetType> {
  fn exactMatch(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  bool QFontInfo::exactMatch();
impl<'a> /*trait*/ QFontInfo_exactMatch<i8> for () {
  fn exactMatch(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10exactMatchEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10exactMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn pointSize<RetType, T: QFontInfo_pointSize<RetType>>(&mut self, value: T) -> RetType {
    return value.pointSize(self);
    // return 1;
  }
}

pub trait QFontInfo_pointSize<RetType> {
  fn pointSize(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  int QFontInfo::pointSize();
impl<'a> /*trait*/ QFontInfo_pointSize<i32> for () {
  fn pointSize(self, rsthis: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9pointSizeEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9pointSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn NewQFontInfo<T: QFontInfo_NewQFontInfo>(value: T) -> QFontInfo {
    let rsthis = value.NewQFontInfo();
    return rsthis;
    // return 1;
  }
}

pub trait QFontInfo_NewQFontInfo {
  fn NewQFontInfo(self) -> QFontInfo;
}

// proto: void QFontInfo::NewQFontInfo(const QFontInfo & );
impl<'a> /*trait*/ QFontInfo_NewQFontInfo for (&'a  QFontInfo) {
  fn NewQFontInfo(self) -> QFontInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFontInfoC1ERKS_(qthis, arg0)};
    let rsthis = QFontInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn family<RetType, T: QFontInfo_family<RetType>>(&mut self, value: T) -> RetType {
    return value.family(self);
    // return 1;
  }
}

pub trait QFontInfo_family<RetType> {
  fn family(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  QString QFontInfo::family();
impl<'a> /*trait*/ QFontInfo_family<QString> for () {
  fn family(self, rsthis: &mut QFontInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6familyEv()};
    let mut ret = unsafe {_ZNK9QFontInfo6familyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn bold<RetType, T: QFontInfo_bold<RetType>>(&mut self, value: T) -> RetType {
    return value.bold(self);
    // return 1;
  }
}

pub trait QFontInfo_bold<RetType> {
  fn bold(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  bool QFontInfo::bold();
impl<'a> /*trait*/ QFontInfo_bold<i8> for () {
  fn bold(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo4boldEv()};
    let mut ret = unsafe {_ZNK9QFontInfo4boldEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn pointSizeF<RetType, T: QFontInfo_pointSizeF<RetType>>(&mut self, value: T) -> RetType {
    return value.pointSizeF(self);
    // return 1;
  }
}

pub trait QFontInfo_pointSizeF<RetType> {
  fn pointSizeF(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  double QFontInfo::pointSizeF();
impl<'a> /*trait*/ QFontInfo_pointSizeF<f64> for () {
  fn pointSizeF(self, rsthis: &mut QFontInfo) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10pointSizeFEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10pointSizeFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn fixedPitch<RetType, T: QFontInfo_fixedPitch<RetType>>(&mut self, value: T) -> RetType {
    return value.fixedPitch(self);
    // return 1;
  }
}

pub trait QFontInfo_fixedPitch<RetType> {
  fn fixedPitch(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  bool QFontInfo::fixedPitch();
impl<'a> /*trait*/ QFontInfo_fixedPitch<i8> for () {
  fn fixedPitch(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10fixedPitchEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10fixedPitchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn overline<RetType, T: QFontInfo_overline<RetType>>(&mut self, value: T) -> RetType {
    return value.overline(self);
    // return 1;
  }
}

pub trait QFontInfo_overline<RetType> {
  fn overline(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  bool QFontInfo::overline();
impl<'a> /*trait*/ QFontInfo_overline<i8> for () {
  fn overline(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo8overlineEv()};
    let mut ret = unsafe {_ZNK9QFontInfo8overlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn swap<RetType, T: QFontInfo_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QFontInfo_swap<RetType> {
  fn swap(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  void QFontInfo::swap(QFontInfo & other);
impl<'a> /*trait*/ QFontInfo_swap<()> for (&'a mut QFontInfo) {
  fn swap(self, rsthis: &mut QFontInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFontInfo4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QFontInfo::NewQFontInfo(const QFont & );
impl<'a> /*trait*/ QFontInfo_NewQFontInfo for (&'a  QFont) {
  fn NewQFontInfo(self) -> QFontInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoC1ERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFontInfoC1ERK5QFont(qthis, arg0)};
    let rsthis = QFontInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn pixelSize<RetType, T: QFontInfo_pixelSize<RetType>>(&mut self, value: T) -> RetType {
    return value.pixelSize(self);
    // return 1;
  }
}

pub trait QFontInfo_pixelSize<RetType> {
  fn pixelSize(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  int QFontInfo::pixelSize();
impl<'a> /*trait*/ QFontInfo_pixelSize<i32> for () {
  fn pixelSize(self, rsthis: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9pixelSizeEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9pixelSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn strikeOut<RetType, T: QFontInfo_strikeOut<RetType>>(&mut self, value: T) -> RetType {
    return value.strikeOut(self);
    // return 1;
  }
}

pub trait QFontInfo_strikeOut<RetType> {
  fn strikeOut(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  bool QFontInfo::strikeOut();
impl<'a> /*trait*/ QFontInfo_strikeOut<i8> for () {
  fn strikeOut(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9strikeOutEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9strikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn FreeQFontInfo<RetType, T: QFontInfo_FreeQFontInfo<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQFontInfo(self);
    // return 1;
  }
}

pub trait QFontInfo_FreeQFontInfo<RetType> {
  fn FreeQFontInfo(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  void QFontInfo::FreeQFontInfo();
impl<'a> /*trait*/ QFontInfo_FreeQFontInfo<()> for () {
  fn FreeQFontInfo(self, rsthis: &mut QFontInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoD0Ev()};
     unsafe {_ZN9QFontInfoD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn italic<RetType, T: QFontInfo_italic<RetType>>(&mut self, value: T) -> RetType {
    return value.italic(self);
    // return 1;
  }
}

pub trait QFontInfo_italic<RetType> {
  fn italic(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  bool QFontInfo::italic();
impl<'a> /*trait*/ QFontInfo_italic<i8> for () {
  fn italic(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6italicEv()};
    let mut ret = unsafe {_ZNK9QFontInfo6italicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn underline<RetType, T: QFontInfo_underline<RetType>>(&mut self, value: T) -> RetType {
    return value.underline(self);
    // return 1;
  }
}

pub trait QFontInfo_underline<RetType> {
  fn underline(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  bool QFontInfo::underline();
impl<'a> /*trait*/ QFontInfo_underline<i8> for () {
  fn underline(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9underlineEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9underlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn styleName<RetType, T: QFontInfo_styleName<RetType>>(&mut self, value: T) -> RetType {
    return value.styleName(self);
    // return 1;
  }
}

pub trait QFontInfo_styleName<RetType> {
  fn styleName(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  QString QFontInfo::styleName();
impl<'a> /*trait*/ QFontInfo_styleName<QString> for () {
  fn styleName(self, rsthis: &mut QFontInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9styleNameEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9styleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn weight<RetType, T: QFontInfo_weight<RetType>>(&mut self, value: T) -> RetType {
    return value.weight(self);
    // return 1;
  }
}

pub trait QFontInfo_weight<RetType> {
  fn weight(self, rsthis: &mut QFontInfo) -> RetType;
}

// proto:  int QFontInfo::weight();
impl<'a> /*trait*/ QFontInfo_weight<i32> for () {
  fn weight(self, rsthis: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6weightEv()};
    let mut ret = unsafe {_ZNK9QFontInfo6weightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

