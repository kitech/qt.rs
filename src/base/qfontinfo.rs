// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QFontInfo::rawMode();
  fn _ZNK9QFontInfo7rawModeEv() -> i32;
  // proto: bool QFontInfo::exactMatch();
  fn _ZNK9QFontInfo10exactMatchEv() -> i32;
  // proto: int QFontInfo::pointSize();
  fn _ZNK9QFontInfo9pointSizeEv() -> i32;
  // proto: void QFontInfo::NewQFontInfo(const QFontInfo & );
  fn _ZN9QFontInfoC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QString QFontInfo::family();
  fn _ZNK9QFontInfo6familyEv() -> i32;
  // proto: bool QFontInfo::bold();
  fn _ZNK9QFontInfo4boldEv() -> i32;
  // proto: double QFontInfo::pointSizeF();
  fn _ZNK9QFontInfo10pointSizeFEv() -> i32;
  // proto: bool QFontInfo::fixedPitch();
  fn _ZNK9QFontInfo10fixedPitchEv() -> i32;
  // proto: bool QFontInfo::overline();
  fn _ZNK9QFontInfo8overlineEv() -> i32;
  // proto: void QFontInfo::swap(QFontInfo & other);
  fn _ZN9QFontInfo4swapERS_(arg0: *mut c_void) -> i32;
  // proto: void QFontInfo::NewQFontInfo(const QFont & );
  fn _ZN9QFontInfoC1ERK5QFont(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QFontInfo::pixelSize();
  fn _ZNK9QFontInfo9pixelSizeEv() -> i32;
  // proto: bool QFontInfo::strikeOut();
  fn _ZNK9QFontInfo9strikeOutEv() -> i32;
  // proto: void QFontInfo::FreeQFontInfo();
  fn _ZN9QFontInfoD0Ev() -> i32;
  // proto: bool QFontInfo::italic();
  fn _ZNK9QFontInfo6italicEv() -> i32;
  // proto: bool QFontInfo::underline();
  fn _ZNK9QFontInfo9underlineEv() -> i32;
  // proto: QString QFontInfo::styleName();
  fn _ZNK9QFontInfo9styleNameEv() -> i32;
  // proto: int QFontInfo::weight();
  fn _ZNK9QFontInfo6weightEv() -> i32;
}

// body block begin
// class sizeof(QFontInfo)=1
pub struct QFontInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontInfo {
  pub fn rawMode<T: QFontInfo_rawMode>(&mut self, value: T) -> i32 {
    value.rawMode(self);
    return 1;
  }
}

pub trait QFontInfo_rawMode {
  fn rawMode(self, this: &mut QFontInfo) -> i32;
}

// proto: bool QFontInfo::rawMode();
impl<'a> /*trait*/ QFontInfo_rawMode for () {
  fn rawMode(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo7rawModeEv()};
    unsafe {_ZNK9QFontInfo7rawModeEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn exactMatch<T: QFontInfo_exactMatch>(&mut self, value: T) -> i32 {
    value.exactMatch(self);
    return 1;
  }
}

pub trait QFontInfo_exactMatch {
  fn exactMatch(self, this: &mut QFontInfo) -> i32;
}

// proto: bool QFontInfo::exactMatch();
impl<'a> /*trait*/ QFontInfo_exactMatch for () {
  fn exactMatch(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10exactMatchEv()};
    unsafe {_ZNK9QFontInfo10exactMatchEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn pointSize<T: QFontInfo_pointSize>(&mut self, value: T) -> i32 {
    value.pointSize(self);
    return 1;
  }
}

pub trait QFontInfo_pointSize {
  fn pointSize(self, this: &mut QFontInfo) -> i32;
}

// proto: int QFontInfo::pointSize();
impl<'a> /*trait*/ QFontInfo_pointSize for () {
  fn pointSize(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9pointSizeEv()};
    unsafe {_ZNK9QFontInfo9pointSizeEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QFontInfoC1ERKS_(qthis, arg0)};
    let rsthis = QFontInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn family<T: QFontInfo_family>(&mut self, value: T) -> i32 {
    value.family(self);
    return 1;
  }
}

pub trait QFontInfo_family {
  fn family(self, this: &mut QFontInfo) -> i32;
}

// proto: QString QFontInfo::family();
impl<'a> /*trait*/ QFontInfo_family for () {
  fn family(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6familyEv()};
    unsafe {_ZNK9QFontInfo6familyEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn bold<T: QFontInfo_bold>(&mut self, value: T) -> i32 {
    value.bold(self);
    return 1;
  }
}

pub trait QFontInfo_bold {
  fn bold(self, this: &mut QFontInfo) -> i32;
}

// proto: bool QFontInfo::bold();
impl<'a> /*trait*/ QFontInfo_bold for () {
  fn bold(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo4boldEv()};
    unsafe {_ZNK9QFontInfo4boldEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn pointSizeF<T: QFontInfo_pointSizeF>(&mut self, value: T) -> i32 {
    value.pointSizeF(self);
    return 1;
  }
}

pub trait QFontInfo_pointSizeF {
  fn pointSizeF(self, this: &mut QFontInfo) -> i32;
}

// proto: double QFontInfo::pointSizeF();
impl<'a> /*trait*/ QFontInfo_pointSizeF for () {
  fn pointSizeF(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10pointSizeFEv()};
    unsafe {_ZNK9QFontInfo10pointSizeFEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn fixedPitch<T: QFontInfo_fixedPitch>(&mut self, value: T) -> i32 {
    value.fixedPitch(self);
    return 1;
  }
}

pub trait QFontInfo_fixedPitch {
  fn fixedPitch(self, this: &mut QFontInfo) -> i32;
}

// proto: bool QFontInfo::fixedPitch();
impl<'a> /*trait*/ QFontInfo_fixedPitch for () {
  fn fixedPitch(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10fixedPitchEv()};
    unsafe {_ZNK9QFontInfo10fixedPitchEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn overline<T: QFontInfo_overline>(&mut self, value: T) -> i32 {
    value.overline(self);
    return 1;
  }
}

pub trait QFontInfo_overline {
  fn overline(self, this: &mut QFontInfo) -> i32;
}

// proto: bool QFontInfo::overline();
impl<'a> /*trait*/ QFontInfo_overline for () {
  fn overline(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo8overlineEv()};
    unsafe {_ZNK9QFontInfo8overlineEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn swap<T: QFontInfo_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QFontInfo_swap {
  fn swap(self, this: &mut QFontInfo) -> i32;
}

// proto: void QFontInfo::swap(QFontInfo & other);
impl<'a> /*trait*/ QFontInfo_swap for (&'a mut QFontInfo) {
  fn swap(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFontInfo4swapERS_(arg0)};
    return 1;
  }
}

// proto: void QFontInfo::NewQFontInfo(const QFont & );
impl<'a> /*trait*/ QFontInfo_NewQFontInfo for (&'a  QFont) {
  fn NewQFontInfo(self) -> QFontInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoC1ERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QFontInfoC1ERK5QFont(qthis, arg0)};
    let rsthis = QFontInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn pixelSize<T: QFontInfo_pixelSize>(&mut self, value: T) -> i32 {
    value.pixelSize(self);
    return 1;
  }
}

pub trait QFontInfo_pixelSize {
  fn pixelSize(self, this: &mut QFontInfo) -> i32;
}

// proto: int QFontInfo::pixelSize();
impl<'a> /*trait*/ QFontInfo_pixelSize for () {
  fn pixelSize(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9pixelSizeEv()};
    unsafe {_ZNK9QFontInfo9pixelSizeEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn strikeOut<T: QFontInfo_strikeOut>(&mut self, value: T) -> i32 {
    value.strikeOut(self);
    return 1;
  }
}

pub trait QFontInfo_strikeOut {
  fn strikeOut(self, this: &mut QFontInfo) -> i32;
}

// proto: bool QFontInfo::strikeOut();
impl<'a> /*trait*/ QFontInfo_strikeOut for () {
  fn strikeOut(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9strikeOutEv()};
    unsafe {_ZNK9QFontInfo9strikeOutEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn FreeQFontInfo<T: QFontInfo_FreeQFontInfo>(&mut self, value: T) -> i32 {
    value.FreeQFontInfo(self);
    return 1;
  }
}

pub trait QFontInfo_FreeQFontInfo {
  fn FreeQFontInfo(self, this: &mut QFontInfo) -> i32;
}

// proto: void QFontInfo::FreeQFontInfo();
impl<'a> /*trait*/ QFontInfo_FreeQFontInfo for () {
  fn FreeQFontInfo(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoD0Ev()};
    unsafe {_ZN9QFontInfoD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn italic<T: QFontInfo_italic>(&mut self, value: T) -> i32 {
    value.italic(self);
    return 1;
  }
}

pub trait QFontInfo_italic {
  fn italic(self, this: &mut QFontInfo) -> i32;
}

// proto: bool QFontInfo::italic();
impl<'a> /*trait*/ QFontInfo_italic for () {
  fn italic(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6italicEv()};
    unsafe {_ZNK9QFontInfo6italicEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn underline<T: QFontInfo_underline>(&mut self, value: T) -> i32 {
    value.underline(self);
    return 1;
  }
}

pub trait QFontInfo_underline {
  fn underline(self, this: &mut QFontInfo) -> i32;
}

// proto: bool QFontInfo::underline();
impl<'a> /*trait*/ QFontInfo_underline for () {
  fn underline(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9underlineEv()};
    unsafe {_ZNK9QFontInfo9underlineEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn styleName<T: QFontInfo_styleName>(&mut self, value: T) -> i32 {
    value.styleName(self);
    return 1;
  }
}

pub trait QFontInfo_styleName {
  fn styleName(self, this: &mut QFontInfo) -> i32;
}

// proto: QString QFontInfo::styleName();
impl<'a> /*trait*/ QFontInfo_styleName for () {
  fn styleName(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9styleNameEv()};
    unsafe {_ZNK9QFontInfo9styleNameEv()};
    return 1;
  }
}

impl /*struct*/ QFontInfo {
  pub fn weight<T: QFontInfo_weight>(&mut self, value: T) -> i32 {
    value.weight(self);
    return 1;
  }
}

pub trait QFontInfo_weight {
  fn weight(self, this: &mut QFontInfo) -> i32;
}

// proto: int QFontInfo::weight();
impl<'a> /*trait*/ QFontInfo_weight for () {
  fn weight(self, this: &mut QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6weightEv()};
    unsafe {_ZNK9QFontInfo6weightEv()};
    return 1;
  }
}

