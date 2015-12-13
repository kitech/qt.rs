// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qstringlist::QStringList;
use super::qpaintdevice::QPaintDevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QFont::setWordSpacing(qreal spacing);
  fn _ZN5QFont14setWordSpacingEd(arg0: c_double) -> i32;
  // proto: QString QFont::rawName();
  fn _ZNK5QFont7rawNameEv() -> i32;
  // proto: void QFont::setRawMode(bool );
  fn _ZN5QFont10setRawModeEb(arg0: int8_t) -> i32;
  // proto: void QFont::setStyleName(const QString & );
  fn _ZN5QFont12setStyleNameERK7QString(arg0: *const c_void) -> i32;
  // proto: QFont QFont::resolve(const QFont & );
  fn _ZNK5QFont7resolveERKS_(arg0: *const c_void) -> i32;
  // proto: bool QFont::strikeOut();
  fn _ZNK5QFont9strikeOutEv() -> i32;
  // proto: int QFont::pixelSize();
  fn _ZNK5QFont9pixelSizeEv() -> i32;
  // proto: void QFont::setWeight(int );
  fn _ZN5QFont9setWeightEi(arg0: c_int) -> i32;
  // proto: int QFont::weight();
  fn _ZNK5QFont6weightEv() -> i32;
  // proto: void QFont::insertSubstitutions(const QString & , const QStringList & );
  fn _ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QFont::kerning();
  fn _ZNK5QFont7kerningEv() -> i32;
  // proto: QStringList QFont::substitutions();
  fn _ZN5QFont13substitutionsEv() -> i32;
  // proto: bool QFont::italic();
  fn _ZNK5QFont6italicEv() -> i32;
  // proto: void QFont::setUnderline(bool );
  fn _ZN5QFont12setUnderlineEb(arg0: int8_t) -> i32;
  // proto: double QFont::letterSpacing();
  fn _ZNK5QFont13letterSpacingEv() -> i32;
  // proto: void QFont::setPointSize(int );
  fn _ZN5QFont12setPointSizeEi(arg0: c_int) -> i32;
  // proto: void QFont::NewQFont(const QString & family, int pointSize, int weight, bool italic);
  fn _ZN5QFontC1ERK7QStringiib(qthis: *mut c_void, arg0: *const c_void, arg1: c_int, arg2: c_int, arg3: int8_t) -> i32;
  // proto: void QFont::setOverline(bool );
  fn _ZN5QFont11setOverlineEb(arg0: int8_t) -> i32;
  // proto: QString QFont::family();
  fn _ZNK5QFont6familyEv() -> i32;
  // proto: QString QFont::lastResortFamily();
  fn _ZNK5QFont16lastResortFamilyEv() -> i32;
  // proto: void QFont::setItalic(bool b);
  fn _ZN5QFont9setItalicEb(arg0: int8_t) -> i32;
  // proto: void QFont::setFamily(const QString & );
  fn _ZN5QFont9setFamilyERK7QString(arg0: *const c_void) -> i32;
  // proto: void QFont::NewQFont(const QFont & );
  fn _ZN5QFontC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QFont::overline();
  fn _ZNK5QFont8overlineEv() -> i32;
  // proto: void QFont::FreeQFont();
  fn _ZN5QFontD0Ev() -> i32;
  // proto: void QFont::resolve(uint mask);
  fn _ZN5QFont7resolveEj(arg0: c_uint) -> i32;
  // proto: void QFont::setBold(bool );
  fn _ZN5QFont7setBoldEb(arg0: int8_t) -> i32;
  // proto: void QFont::cacheStatistics();
  fn _ZN5QFont15cacheStatisticsEv() -> i32;
  // proto: void QFont::setPointSizeF(qreal );
  fn _ZN5QFont13setPointSizeFEd(arg0: c_double) -> i32;
  // proto: QStringList QFont::substitutes(const QString & );
  fn _ZN5QFont11substitutesERK7QString(arg0: *const c_void) -> i32;
  // proto: double QFont::wordSpacing();
  fn _ZNK5QFont11wordSpacingEv() -> i32;
  // proto: QString QFont::toString();
  fn _ZNK5QFont8toStringEv() -> i32;
  // proto: double QFont::pointSizeF();
  fn _ZNK5QFont10pointSizeFEv() -> i32;
  // proto: void QFont::insertSubstitution(const QString & , const QString & );
  fn _ZN5QFont18insertSubstitutionERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QFont::setStretch(int );
  fn _ZN5QFont10setStretchEi(arg0: c_int) -> i32;
  // proto: QString QFont::styleName();
  fn _ZNK5QFont9styleNameEv() -> i32;
  // proto: void QFont::NewQFont();
  fn _ZN5QFontC1Ev(qthis: *mut c_void) -> i32;
  // proto: bool QFont::rawMode();
  fn _ZNK5QFont7rawModeEv() -> i32;
  // proto: bool QFont::fromString(const QString & );
  fn _ZN5QFont10fromStringERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QFont::underline();
  fn _ZNK5QFont9underlineEv() -> i32;
  // proto: bool QFont::isCopyOf(const QFont & );
  fn _ZNK5QFont8isCopyOfERKS_(arg0: *const c_void) -> i32;
  // proto: int QFont::pointSize();
  fn _ZNK5QFont9pointSizeEv() -> i32;
  // proto: void QFont::setKerning(bool );
  fn _ZN5QFont10setKerningEb(arg0: int8_t) -> i32;
  // proto: bool QFont::bold();
  fn _ZNK5QFont4boldEv() -> i32;
  // proto: bool QFont::fixedPitch();
  fn _ZNK5QFont10fixedPitchEv() -> i32;
  // proto: void QFont::NewQFont(const QFont & , QPaintDevice * pd);
  fn _ZN5QFontC1ERKS_P12QPaintDevice(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QString QFont::substitute(const QString & );
  fn _ZN5QFont10substituteERK7QString(arg0: *const c_void) -> i32;
  // proto: void QFont::setFixedPitch(bool );
  fn _ZN5QFont13setFixedPitchEb(arg0: int8_t) -> i32;
  // proto: void QFont::removeSubstitutions(const QString & );
  fn _ZN5QFont19removeSubstitutionsERK7QString(arg0: *const c_void) -> i32;
  // proto: void QFont::setPixelSize(int );
  fn _ZN5QFont12setPixelSizeEi(arg0: c_int) -> i32;
  // proto: void QFont::initialize();
  fn _ZN5QFont10initializeEv() -> i32;
  // proto: QString QFont::key();
  fn _ZNK5QFont3keyEv() -> i32;
  // proto: QString QFont::lastResortFont();
  fn _ZNK5QFont14lastResortFontEv() -> i32;
  // proto: void QFont::swap(QFont & other);
  fn _ZN5QFont4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QString QFont::defaultFamily();
  fn _ZNK5QFont13defaultFamilyEv() -> i32;
  // proto: void QFont::setStrikeOut(bool );
  fn _ZN5QFont12setStrikeOutEb(arg0: int8_t) -> i32;
  // proto: unsigned int QFont::resolve();
  fn _ZNK5QFont7resolveEv() -> i32;
  // proto: void QFont::cleanup();
  fn _ZN5QFont7cleanupEv() -> i32;
  // proto: bool QFont::exactMatch();
  fn _ZNK5QFont10exactMatchEv() -> i32;
  // proto: int QFont::stretch();
  fn _ZNK5QFont7stretchEv() -> i32;
  // proto: void QFont::setRawName(const QString & );
  fn _ZN5QFont10setRawNameERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QFont)=1
pub struct QFont {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFont {
  pub fn setWordSpacing<T: QFont_setWordSpacing>(&mut self, value: T) -> i32 {
    value.setWordSpacing(self);
    return 1;
  }
}

pub trait QFont_setWordSpacing {
  fn setWordSpacing(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setWordSpacing(qreal spacing);
impl<'a> /*trait*/ QFont_setWordSpacing for (f64) {
  fn setWordSpacing(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont14setWordSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN5QFont14setWordSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn rawName<T: QFont_rawName>(&mut self, value: T) -> i32 {
    value.rawName(self);
    return 1;
  }
}

pub trait QFont_rawName {
  fn rawName(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::rawName();
impl<'a> /*trait*/ QFont_rawName for () {
  fn rawName(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7rawNameEv()};
    unsafe {_ZNK5QFont7rawNameEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setRawMode<T: QFont_setRawMode>(&mut self, value: T) -> i32 {
    value.setRawMode(self);
    return 1;
  }
}

pub trait QFont_setRawMode {
  fn setRawMode(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setRawMode(bool );
impl<'a> /*trait*/ QFont_setRawMode for (i8) {
  fn setRawMode(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setRawModeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QFont10setRawModeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setStyleName<T: QFont_setStyleName>(&mut self, value: T) -> i32 {
    value.setStyleName(self);
    return 1;
  }
}

pub trait QFont_setStyleName {
  fn setStyleName(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setStyleName(const QString & );
impl<'a> /*trait*/ QFont_setStyleName for (&'a  QString) {
  fn setStyleName(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setStyleNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFont12setStyleNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn resolve<T: QFont_resolve>(&mut self, value: T) -> i32 {
    value.resolve(self);
    return 1;
  }
}

pub trait QFont_resolve {
  fn resolve(self, this: &mut QFont) -> i32;
}

// proto: QFont QFont::resolve(const QFont & );
impl<'a> /*trait*/ QFont_resolve for (&'a  QFont) {
  fn resolve(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7resolveERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QFont7resolveERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn strikeOut<T: QFont_strikeOut>(&mut self, value: T) -> i32 {
    value.strikeOut(self);
    return 1;
  }
}

pub trait QFont_strikeOut {
  fn strikeOut(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::strikeOut();
impl<'a> /*trait*/ QFont_strikeOut for () {
  fn strikeOut(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9strikeOutEv()};
    unsafe {_ZNK5QFont9strikeOutEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn pixelSize<T: QFont_pixelSize>(&mut self, value: T) -> i32 {
    value.pixelSize(self);
    return 1;
  }
}

pub trait QFont_pixelSize {
  fn pixelSize(self, this: &mut QFont) -> i32;
}

// proto: int QFont::pixelSize();
impl<'a> /*trait*/ QFont_pixelSize for () {
  fn pixelSize(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9pixelSizeEv()};
    unsafe {_ZNK5QFont9pixelSizeEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setWeight<T: QFont_setWeight>(&mut self, value: T) -> i32 {
    value.setWeight(self);
    return 1;
  }
}

pub trait QFont_setWeight {
  fn setWeight(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setWeight(int );
impl<'a> /*trait*/ QFont_setWeight for (i32) {
  fn setWeight(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont9setWeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QFont9setWeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn weight<T: QFont_weight>(&mut self, value: T) -> i32 {
    value.weight(self);
    return 1;
  }
}

pub trait QFont_weight {
  fn weight(self, this: &mut QFont) -> i32;
}

// proto: int QFont::weight();
impl<'a> /*trait*/ QFont_weight for () {
  fn weight(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont6weightEv()};
    unsafe {_ZNK5QFont6weightEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn insertSubstitutions<T: QFont_insertSubstitutions>(&mut self, value: T) -> i32 {
    value.insertSubstitutions(self);
    return 1;
  }
}

pub trait QFont_insertSubstitutions {
  fn insertSubstitutions(self, this: &mut QFont) -> i32;
}

// proto: void QFont::insertSubstitutions(const QString & , const QStringList & );
impl<'a> /*trait*/ QFont_insertSubstitutions for (&'a  QString, &'a  QStringList) {
  fn insertSubstitutions(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn kerning<T: QFont_kerning>(&mut self, value: T) -> i32 {
    value.kerning(self);
    return 1;
  }
}

pub trait QFont_kerning {
  fn kerning(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::kerning();
impl<'a> /*trait*/ QFont_kerning for () {
  fn kerning(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7kerningEv()};
    unsafe {_ZNK5QFont7kerningEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn substitutions<T: QFont_substitutions>(&mut self, value: T) -> i32 {
    value.substitutions(self);
    return 1;
  }
}

pub trait QFont_substitutions {
  fn substitutions(self, this: &mut QFont) -> i32;
}

// proto: QStringList QFont::substitutions();
impl<'a> /*trait*/ QFont_substitutions for () {
  fn substitutions(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13substitutionsEv()};
    unsafe {_ZN5QFont13substitutionsEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn italic<T: QFont_italic>(&mut self, value: T) -> i32 {
    value.italic(self);
    return 1;
  }
}

pub trait QFont_italic {
  fn italic(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::italic();
impl<'a> /*trait*/ QFont_italic for () {
  fn italic(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont6italicEv()};
    unsafe {_ZNK5QFont6italicEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setUnderline<T: QFont_setUnderline>(&mut self, value: T) -> i32 {
    value.setUnderline(self);
    return 1;
  }
}

pub trait QFont_setUnderline {
  fn setUnderline(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setUnderline(bool );
impl<'a> /*trait*/ QFont_setUnderline for (i8) {
  fn setUnderline(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setUnderlineEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QFont12setUnderlineEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn letterSpacing<T: QFont_letterSpacing>(&mut self, value: T) -> i32 {
    value.letterSpacing(self);
    return 1;
  }
}

pub trait QFont_letterSpacing {
  fn letterSpacing(self, this: &mut QFont) -> i32;
}

// proto: double QFont::letterSpacing();
impl<'a> /*trait*/ QFont_letterSpacing for () {
  fn letterSpacing(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont13letterSpacingEv()};
    unsafe {_ZNK5QFont13letterSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPointSize<T: QFont_setPointSize>(&mut self, value: T) -> i32 {
    value.setPointSize(self);
    return 1;
  }
}

pub trait QFont_setPointSize {
  fn setPointSize(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setPointSize(int );
impl<'a> /*trait*/ QFont_setPointSize for (i32) {
  fn setPointSize(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setPointSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QFont12setPointSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn NewQFont<T: QFont_NewQFont>(value: T) -> QFont {
    let rsthis = value.NewQFont();
    return rsthis;
    // return 1;
  }
}

pub trait QFont_NewQFont {
  fn NewQFont(self) -> QFont;
}

// proto: void QFont::NewQFont(const QString & family, int pointSize, int weight, bool italic);
impl<'a> /*trait*/ QFont_NewQFont for (&'a  QString, i32, i32, i8) {
  fn NewQFont(self) -> QFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontC1ERK7QStringiib()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as int8_t;
    unsafe {_ZN5QFontC1ERK7QStringiib(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setOverline<T: QFont_setOverline>(&mut self, value: T) -> i32 {
    value.setOverline(self);
    return 1;
  }
}

pub trait QFont_setOverline {
  fn setOverline(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setOverline(bool );
impl<'a> /*trait*/ QFont_setOverline for (i8) {
  fn setOverline(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont11setOverlineEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QFont11setOverlineEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn family<T: QFont_family>(&mut self, value: T) -> i32 {
    value.family(self);
    return 1;
  }
}

pub trait QFont_family {
  fn family(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::family();
impl<'a> /*trait*/ QFont_family for () {
  fn family(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont6familyEv()};
    unsafe {_ZNK5QFont6familyEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn lastResortFamily<T: QFont_lastResortFamily>(&mut self, value: T) -> i32 {
    value.lastResortFamily(self);
    return 1;
  }
}

pub trait QFont_lastResortFamily {
  fn lastResortFamily(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::lastResortFamily();
impl<'a> /*trait*/ QFont_lastResortFamily for () {
  fn lastResortFamily(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont16lastResortFamilyEv()};
    unsafe {_ZNK5QFont16lastResortFamilyEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setItalic<T: QFont_setItalic>(&mut self, value: T) -> i32 {
    value.setItalic(self);
    return 1;
  }
}

pub trait QFont_setItalic {
  fn setItalic(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setItalic(bool b);
impl<'a> /*trait*/ QFont_setItalic for (i8) {
  fn setItalic(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont9setItalicEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QFont9setItalicEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setFamily<T: QFont_setFamily>(&mut self, value: T) -> i32 {
    value.setFamily(self);
    return 1;
  }
}

pub trait QFont_setFamily {
  fn setFamily(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setFamily(const QString & );
impl<'a> /*trait*/ QFont_setFamily for (&'a  QString) {
  fn setFamily(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont9setFamilyERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFont9setFamilyERK7QString(arg0)};
    return 1;
  }
}

// proto: void QFont::NewQFont(const QFont & );
impl<'a> /*trait*/ QFont_NewQFont for (&'a  QFont) {
  fn NewQFont(self) -> QFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFontC1ERKS_(qthis, arg0)};
    let rsthis = QFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn overline<T: QFont_overline>(&mut self, value: T) -> i32 {
    value.overline(self);
    return 1;
  }
}

pub trait QFont_overline {
  fn overline(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::overline();
impl<'a> /*trait*/ QFont_overline for () {
  fn overline(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont8overlineEv()};
    unsafe {_ZNK5QFont8overlineEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn FreeQFont<T: QFont_FreeQFont>(&mut self, value: T) -> i32 {
    value.FreeQFont(self);
    return 1;
  }
}

pub trait QFont_FreeQFont {
  fn FreeQFont(self, this: &mut QFont) -> i32;
}

// proto: void QFont::FreeQFont();
impl<'a> /*trait*/ QFont_FreeQFont for () {
  fn FreeQFont(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontD0Ev()};
    unsafe {_ZN5QFontD0Ev()};
    return 1;
  }
}

// proto: void QFont::resolve(uint mask);
impl<'a> /*trait*/ QFont_resolve for (u32) {
  fn resolve(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont7resolveEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QFont7resolveEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setBold<T: QFont_setBold>(&mut self, value: T) -> i32 {
    value.setBold(self);
    return 1;
  }
}

pub trait QFont_setBold {
  fn setBold(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setBold(bool );
impl<'a> /*trait*/ QFont_setBold for (i8) {
  fn setBold(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont7setBoldEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QFont7setBoldEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn cacheStatistics<T: QFont_cacheStatistics>(&mut self, value: T) -> i32 {
    value.cacheStatistics(self);
    return 1;
  }
}

pub trait QFont_cacheStatistics {
  fn cacheStatistics(self, this: &mut QFont) -> i32;
}

// proto: void QFont::cacheStatistics();
impl<'a> /*trait*/ QFont_cacheStatistics for () {
  fn cacheStatistics(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont15cacheStatisticsEv()};
    unsafe {_ZN5QFont15cacheStatisticsEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPointSizeF<T: QFont_setPointSizeF>(&mut self, value: T) -> i32 {
    value.setPointSizeF(self);
    return 1;
  }
}

pub trait QFont_setPointSizeF {
  fn setPointSizeF(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setPointSizeF(qreal );
impl<'a> /*trait*/ QFont_setPointSizeF for (f64) {
  fn setPointSizeF(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13setPointSizeFEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN5QFont13setPointSizeFEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn substitutes<T: QFont_substitutes>(&mut self, value: T) -> i32 {
    value.substitutes(self);
    return 1;
  }
}

pub trait QFont_substitutes {
  fn substitutes(self, this: &mut QFont) -> i32;
}

// proto: QStringList QFont::substitutes(const QString & );
impl<'a> /*trait*/ QFont_substitutes for (&'a  QString) {
  fn substitutes(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont11substitutesERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFont11substitutesERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn wordSpacing<T: QFont_wordSpacing>(&mut self, value: T) -> i32 {
    value.wordSpacing(self);
    return 1;
  }
}

pub trait QFont_wordSpacing {
  fn wordSpacing(self, this: &mut QFont) -> i32;
}

// proto: double QFont::wordSpacing();
impl<'a> /*trait*/ QFont_wordSpacing for () {
  fn wordSpacing(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont11wordSpacingEv()};
    unsafe {_ZNK5QFont11wordSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn toString<T: QFont_toString>(&mut self, value: T) -> i32 {
    value.toString(self);
    return 1;
  }
}

pub trait QFont_toString {
  fn toString(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::toString();
impl<'a> /*trait*/ QFont_toString for () {
  fn toString(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont8toStringEv()};
    unsafe {_ZNK5QFont8toStringEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn pointSizeF<T: QFont_pointSizeF>(&mut self, value: T) -> i32 {
    value.pointSizeF(self);
    return 1;
  }
}

pub trait QFont_pointSizeF {
  fn pointSizeF(self, this: &mut QFont) -> i32;
}

// proto: double QFont::pointSizeF();
impl<'a> /*trait*/ QFont_pointSizeF for () {
  fn pointSizeF(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont10pointSizeFEv()};
    unsafe {_ZNK5QFont10pointSizeFEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn insertSubstitution<T: QFont_insertSubstitution>(&mut self, value: T) -> i32 {
    value.insertSubstitution(self);
    return 1;
  }
}

pub trait QFont_insertSubstitution {
  fn insertSubstitution(self, this: &mut QFont) -> i32;
}

// proto: void QFont::insertSubstitution(const QString & , const QString & );
impl<'a> /*trait*/ QFont_insertSubstitution for (&'a  QString, &'a  QString) {
  fn insertSubstitution(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont18insertSubstitutionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QFont18insertSubstitutionERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setStretch<T: QFont_setStretch>(&mut self, value: T) -> i32 {
    value.setStretch(self);
    return 1;
  }
}

pub trait QFont_setStretch {
  fn setStretch(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setStretch(int );
impl<'a> /*trait*/ QFont_setStretch for (i32) {
  fn setStretch(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setStretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QFont10setStretchEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn styleName<T: QFont_styleName>(&mut self, value: T) -> i32 {
    value.styleName(self);
    return 1;
  }
}

pub trait QFont_styleName {
  fn styleName(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::styleName();
impl<'a> /*trait*/ QFont_styleName for () {
  fn styleName(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9styleNameEv()};
    unsafe {_ZNK5QFont9styleNameEv()};
    return 1;
  }
}

// proto: void QFont::NewQFont();
impl<'a> /*trait*/ QFont_NewQFont for () {
  fn NewQFont(self) -> QFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontC1Ev()};
    unsafe {_ZN5QFontC1Ev(qthis)};
    let rsthis = QFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn rawMode<T: QFont_rawMode>(&mut self, value: T) -> i32 {
    value.rawMode(self);
    return 1;
  }
}

pub trait QFont_rawMode {
  fn rawMode(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::rawMode();
impl<'a> /*trait*/ QFont_rawMode for () {
  fn rawMode(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7rawModeEv()};
    unsafe {_ZNK5QFont7rawModeEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn fromString<T: QFont_fromString>(&mut self, value: T) -> i32 {
    value.fromString(self);
    return 1;
  }
}

pub trait QFont_fromString {
  fn fromString(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::fromString(const QString & );
impl<'a> /*trait*/ QFont_fromString for (&'a  QString) {
  fn fromString(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10fromStringERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFont10fromStringERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn underline<T: QFont_underline>(&mut self, value: T) -> i32 {
    value.underline(self);
    return 1;
  }
}

pub trait QFont_underline {
  fn underline(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::underline();
impl<'a> /*trait*/ QFont_underline for () {
  fn underline(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9underlineEv()};
    unsafe {_ZNK5QFont9underlineEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn isCopyOf<T: QFont_isCopyOf>(&mut self, value: T) -> i32 {
    value.isCopyOf(self);
    return 1;
  }
}

pub trait QFont_isCopyOf {
  fn isCopyOf(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::isCopyOf(const QFont & );
impl<'a> /*trait*/ QFont_isCopyOf for (&'a  QFont) {
  fn isCopyOf(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont8isCopyOfERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QFont8isCopyOfERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn pointSize<T: QFont_pointSize>(&mut self, value: T) -> i32 {
    value.pointSize(self);
    return 1;
  }
}

pub trait QFont_pointSize {
  fn pointSize(self, this: &mut QFont) -> i32;
}

// proto: int QFont::pointSize();
impl<'a> /*trait*/ QFont_pointSize for () {
  fn pointSize(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9pointSizeEv()};
    unsafe {_ZNK5QFont9pointSizeEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setKerning<T: QFont_setKerning>(&mut self, value: T) -> i32 {
    value.setKerning(self);
    return 1;
  }
}

pub trait QFont_setKerning {
  fn setKerning(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setKerning(bool );
impl<'a> /*trait*/ QFont_setKerning for (i8) {
  fn setKerning(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setKerningEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QFont10setKerningEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn bold<T: QFont_bold>(&mut self, value: T) -> i32 {
    value.bold(self);
    return 1;
  }
}

pub trait QFont_bold {
  fn bold(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::bold();
impl<'a> /*trait*/ QFont_bold for () {
  fn bold(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont4boldEv()};
    unsafe {_ZNK5QFont4boldEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn fixedPitch<T: QFont_fixedPitch>(&mut self, value: T) -> i32 {
    value.fixedPitch(self);
    return 1;
  }
}

pub trait QFont_fixedPitch {
  fn fixedPitch(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::fixedPitch();
impl<'a> /*trait*/ QFont_fixedPitch for () {
  fn fixedPitch(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont10fixedPitchEv()};
    unsafe {_ZNK5QFont10fixedPitchEv()};
    return 1;
  }
}

// proto: void QFont::NewQFont(const QFont & , QPaintDevice * pd);
impl<'a> /*trait*/ QFont_NewQFont for (&'a  QFont, &'a mut QPaintDevice) {
  fn NewQFont(self) -> QFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontC1ERKS_P12QPaintDevice()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QFontC1ERKS_P12QPaintDevice(qthis, arg0, arg1)};
    let rsthis = QFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn substitute<T: QFont_substitute>(&mut self, value: T) -> i32 {
    value.substitute(self);
    return 1;
  }
}

pub trait QFont_substitute {
  fn substitute(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::substitute(const QString & );
impl<'a> /*trait*/ QFont_substitute for (&'a  QString) {
  fn substitute(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10substituteERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFont10substituteERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setFixedPitch<T: QFont_setFixedPitch>(&mut self, value: T) -> i32 {
    value.setFixedPitch(self);
    return 1;
  }
}

pub trait QFont_setFixedPitch {
  fn setFixedPitch(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setFixedPitch(bool );
impl<'a> /*trait*/ QFont_setFixedPitch for (i8) {
  fn setFixedPitch(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13setFixedPitchEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QFont13setFixedPitchEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn removeSubstitutions<T: QFont_removeSubstitutions>(&mut self, value: T) -> i32 {
    value.removeSubstitutions(self);
    return 1;
  }
}

pub trait QFont_removeSubstitutions {
  fn removeSubstitutions(self, this: &mut QFont) -> i32;
}

// proto: void QFont::removeSubstitutions(const QString & );
impl<'a> /*trait*/ QFont_removeSubstitutions for (&'a  QString) {
  fn removeSubstitutions(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont19removeSubstitutionsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFont19removeSubstitutionsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPixelSize<T: QFont_setPixelSize>(&mut self, value: T) -> i32 {
    value.setPixelSize(self);
    return 1;
  }
}

pub trait QFont_setPixelSize {
  fn setPixelSize(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setPixelSize(int );
impl<'a> /*trait*/ QFont_setPixelSize for (i32) {
  fn setPixelSize(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setPixelSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QFont12setPixelSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn initialize<T: QFont_initialize>(&mut self, value: T) -> i32 {
    value.initialize(self);
    return 1;
  }
}

pub trait QFont_initialize {
  fn initialize(self, this: &mut QFont) -> i32;
}

// proto: void QFont::initialize();
impl<'a> /*trait*/ QFont_initialize for () {
  fn initialize(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10initializeEv()};
    unsafe {_ZN5QFont10initializeEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn key<T: QFont_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QFont_key {
  fn key(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::key();
impl<'a> /*trait*/ QFont_key for () {
  fn key(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont3keyEv()};
    unsafe {_ZNK5QFont3keyEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn lastResortFont<T: QFont_lastResortFont>(&mut self, value: T) -> i32 {
    value.lastResortFont(self);
    return 1;
  }
}

pub trait QFont_lastResortFont {
  fn lastResortFont(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::lastResortFont();
impl<'a> /*trait*/ QFont_lastResortFont for () {
  fn lastResortFont(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont14lastResortFontEv()};
    unsafe {_ZNK5QFont14lastResortFontEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn swap<T: QFont_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QFont_swap {
  fn swap(self, this: &mut QFont) -> i32;
}

// proto: void QFont::swap(QFont & other);
impl<'a> /*trait*/ QFont_swap for (&'a mut QFont) {
  fn swap(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFont4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn defaultFamily<T: QFont_defaultFamily>(&mut self, value: T) -> i32 {
    value.defaultFamily(self);
    return 1;
  }
}

pub trait QFont_defaultFamily {
  fn defaultFamily(self, this: &mut QFont) -> i32;
}

// proto: QString QFont::defaultFamily();
impl<'a> /*trait*/ QFont_defaultFamily for () {
  fn defaultFamily(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont13defaultFamilyEv()};
    unsafe {_ZNK5QFont13defaultFamilyEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setStrikeOut<T: QFont_setStrikeOut>(&mut self, value: T) -> i32 {
    value.setStrikeOut(self);
    return 1;
  }
}

pub trait QFont_setStrikeOut {
  fn setStrikeOut(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setStrikeOut(bool );
impl<'a> /*trait*/ QFont_setStrikeOut for (i8) {
  fn setStrikeOut(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setStrikeOutEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QFont12setStrikeOutEb(arg0)};
    return 1;
  }
}

// proto: unsigned int QFont::resolve();
impl<'a> /*trait*/ QFont_resolve for () {
  fn resolve(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7resolveEv()};
    unsafe {_ZNK5QFont7resolveEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn cleanup<T: QFont_cleanup>(&mut self, value: T) -> i32 {
    value.cleanup(self);
    return 1;
  }
}

pub trait QFont_cleanup {
  fn cleanup(self, this: &mut QFont) -> i32;
}

// proto: void QFont::cleanup();
impl<'a> /*trait*/ QFont_cleanup for () {
  fn cleanup(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont7cleanupEv()};
    unsafe {_ZN5QFont7cleanupEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn exactMatch<T: QFont_exactMatch>(&mut self, value: T) -> i32 {
    value.exactMatch(self);
    return 1;
  }
}

pub trait QFont_exactMatch {
  fn exactMatch(self, this: &mut QFont) -> i32;
}

// proto: bool QFont::exactMatch();
impl<'a> /*trait*/ QFont_exactMatch for () {
  fn exactMatch(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont10exactMatchEv()};
    unsafe {_ZNK5QFont10exactMatchEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn stretch<T: QFont_stretch>(&mut self, value: T) -> i32 {
    value.stretch(self);
    return 1;
  }
}

pub trait QFont_stretch {
  fn stretch(self, this: &mut QFont) -> i32;
}

// proto: int QFont::stretch();
impl<'a> /*trait*/ QFont_stretch for () {
  fn stretch(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7stretchEv()};
    unsafe {_ZNK5QFont7stretchEv()};
    return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setRawName<T: QFont_setRawName>(&mut self, value: T) -> i32 {
    value.setRawName(self);
    return 1;
  }
}

pub trait QFont_setRawName {
  fn setRawName(self, this: &mut QFont) -> i32;
}

// proto: void QFont::setRawName(const QString & );
impl<'a> /*trait*/ QFont_setRawName for (&'a  QString) {
  fn setRawName(self, this: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setRawNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFont10setRawNameERK7QString(arg0)};
    return 1;
  }
}

