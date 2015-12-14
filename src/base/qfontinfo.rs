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
  pub fn rawMode<T: QFontInfo_rawMode>(&mut self, value: T) -> i8 {
    return value.rawMode(self);
    // return 1;
  }
}

pub trait QFontInfo_rawMode {
  fn rawMode(self, rsthis: &mut QFontInfo) -> i8;
}

// proto:  bool QFontInfo::rawMode();
impl<'a> /*trait*/ QFontInfo_rawMode for () {
  fn rawMode(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo7rawModeEv()};
    let mut ret = unsafe {_ZNK9QFontInfo7rawModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn exactMatch<T: QFontInfo_exactMatch>(&mut self, value: T) -> i8 {
    return value.exactMatch(self);
    // return 1;
  }
}

pub trait QFontInfo_exactMatch {
  fn exactMatch(self, rsthis: &mut QFontInfo) -> i8;
}

// proto:  bool QFontInfo::exactMatch();
impl<'a> /*trait*/ QFontInfo_exactMatch for () {
  fn exactMatch(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10exactMatchEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10exactMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn pointSize<T: QFontInfo_pointSize>(&mut self, value: T) -> i32 {
    return value.pointSize(self);
    // return 1;
  }
}

pub trait QFontInfo_pointSize {
  fn pointSize(self, rsthis: &mut QFontInfo) -> i32;
}

// proto:  int QFontInfo::pointSize();
impl<'a> /*trait*/ QFontInfo_pointSize for () {
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
  pub fn family<T: QFontInfo_family>(&mut self, value: T) -> QString {
    return value.family(self);
    // return 1;
  }
}

pub trait QFontInfo_family {
  fn family(self, rsthis: &mut QFontInfo) -> QString;
}

// proto:  QString QFontInfo::family();
impl<'a> /*trait*/ QFontInfo_family for () {
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
  pub fn bold<T: QFontInfo_bold>(&mut self, value: T) -> i8 {
    return value.bold(self);
    // return 1;
  }
}

pub trait QFontInfo_bold {
  fn bold(self, rsthis: &mut QFontInfo) -> i8;
}

// proto:  bool QFontInfo::bold();
impl<'a> /*trait*/ QFontInfo_bold for () {
  fn bold(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo4boldEv()};
    let mut ret = unsafe {_ZNK9QFontInfo4boldEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn pointSizeF<T: QFontInfo_pointSizeF>(&mut self, value: T) -> f64 {
    return value.pointSizeF(self);
    // return 1;
  }
}

pub trait QFontInfo_pointSizeF {
  fn pointSizeF(self, rsthis: &mut QFontInfo) -> f64;
}

// proto:  double QFontInfo::pointSizeF();
impl<'a> /*trait*/ QFontInfo_pointSizeF for () {
  fn pointSizeF(self, rsthis: &mut QFontInfo) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10pointSizeFEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10pointSizeFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn fixedPitch<T: QFontInfo_fixedPitch>(&mut self, value: T) -> i8 {
    return value.fixedPitch(self);
    // return 1;
  }
}

pub trait QFontInfo_fixedPitch {
  fn fixedPitch(self, rsthis: &mut QFontInfo) -> i8;
}

// proto:  bool QFontInfo::fixedPitch();
impl<'a> /*trait*/ QFontInfo_fixedPitch for () {
  fn fixedPitch(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10fixedPitchEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10fixedPitchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn overline<T: QFontInfo_overline>(&mut self, value: T) -> i8 {
    return value.overline(self);
    // return 1;
  }
}

pub trait QFontInfo_overline {
  fn overline(self, rsthis: &mut QFontInfo) -> i8;
}

// proto:  bool QFontInfo::overline();
impl<'a> /*trait*/ QFontInfo_overline for () {
  fn overline(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo8overlineEv()};
    let mut ret = unsafe {_ZNK9QFontInfo8overlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn swap<T: QFontInfo_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QFontInfo_swap {
  fn swap(self, rsthis: &mut QFontInfo) ;
}

// proto:  void QFontInfo::swap(QFontInfo & other);
impl<'a> /*trait*/ QFontInfo_swap for (&'a mut QFontInfo) {
  fn swap(self, rsthis: &mut QFontInfo)  {
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
  pub fn pixelSize<T: QFontInfo_pixelSize>(&mut self, value: T) -> i32 {
    return value.pixelSize(self);
    // return 1;
  }
}

pub trait QFontInfo_pixelSize {
  fn pixelSize(self, rsthis: &mut QFontInfo) -> i32;
}

// proto:  int QFontInfo::pixelSize();
impl<'a> /*trait*/ QFontInfo_pixelSize for () {
  fn pixelSize(self, rsthis: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9pixelSizeEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9pixelSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn strikeOut<T: QFontInfo_strikeOut>(&mut self, value: T) -> i8 {
    return value.strikeOut(self);
    // return 1;
  }
}

pub trait QFontInfo_strikeOut {
  fn strikeOut(self, rsthis: &mut QFontInfo) -> i8;
}

// proto:  bool QFontInfo::strikeOut();
impl<'a> /*trait*/ QFontInfo_strikeOut for () {
  fn strikeOut(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9strikeOutEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9strikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn FreeQFontInfo<T: QFontInfo_FreeQFontInfo>(&mut self, value: T)  {
     value.FreeQFontInfo(self);
    // return 1;
  }
}

pub trait QFontInfo_FreeQFontInfo {
  fn FreeQFontInfo(self, rsthis: &mut QFontInfo) ;
}

// proto:  void QFontInfo::FreeQFontInfo();
impl<'a> /*trait*/ QFontInfo_FreeQFontInfo for () {
  fn FreeQFontInfo(self, rsthis: &mut QFontInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoD0Ev()};
     unsafe {_ZN9QFontInfoD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn italic<T: QFontInfo_italic>(&mut self, value: T) -> i8 {
    return value.italic(self);
    // return 1;
  }
}

pub trait QFontInfo_italic {
  fn italic(self, rsthis: &mut QFontInfo) -> i8;
}

// proto:  bool QFontInfo::italic();
impl<'a> /*trait*/ QFontInfo_italic for () {
  fn italic(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6italicEv()};
    let mut ret = unsafe {_ZNK9QFontInfo6italicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn underline<T: QFontInfo_underline>(&mut self, value: T) -> i8 {
    return value.underline(self);
    // return 1;
  }
}

pub trait QFontInfo_underline {
  fn underline(self, rsthis: &mut QFontInfo) -> i8;
}

// proto:  bool QFontInfo::underline();
impl<'a> /*trait*/ QFontInfo_underline for () {
  fn underline(self, rsthis: &mut QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9underlineEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9underlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn styleName<T: QFontInfo_styleName>(&mut self, value: T) -> QString {
    return value.styleName(self);
    // return 1;
  }
}

pub trait QFontInfo_styleName {
  fn styleName(self, rsthis: &mut QFontInfo) -> QString;
}

// proto:  QString QFontInfo::styleName();
impl<'a> /*trait*/ QFontInfo_styleName for () {
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
  pub fn weight<T: QFontInfo_weight>(&mut self, value: T) -> i32 {
    return value.weight(self);
    // return 1;
  }
}

pub trait QFontInfo_weight {
  fn weight(self, rsthis: &mut QFontInfo) -> i32;
}

// proto:  int QFontInfo::weight();
impl<'a> /*trait*/ QFontInfo_weight for () {
  fn weight(self, rsthis: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6weightEv()};
    let mut ret = unsafe {_ZNK9QFontInfo6weightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

