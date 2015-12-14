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
  // proto:  void QFont::setWordSpacing(qreal spacing);
  fn _ZN5QFont14setWordSpacingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QString QFont::rawName();
  fn _ZNK5QFont7rawNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFont::setRawMode(bool );
  fn _ZN5QFont10setRawModeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFont::setStyleName(const QString & );
  fn _ZN5QFont12setStyleNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFont::strikeOut();
  fn _ZNK5QFont9strikeOutEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QFont::pixelSize();
  fn _ZNK5QFont9pixelSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFont::setWeight(int );
  fn _ZN5QFont9setWeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QFont::weight();
  fn _ZNK5QFont6weightEv(qthis: *mut c_void) -> c_int;
  // proto: static void QFont::insertSubstitutions(const QString & , const QStringList & );
  fn _ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QFont::kerning();
  fn _ZNK5QFont7kerningEv(qthis: *mut c_void) -> int8_t;
  // proto: static QStringList QFont::substitutions();
  fn _ZN5QFont13substitutionsEv() -> *mut c_void;
  // proto:  bool QFont::italic();
  fn _ZNK5QFont6italicEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFont::setUnderline(bool );
  fn _ZN5QFont12setUnderlineEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  double QFont::letterSpacing();
  fn _ZNK5QFont13letterSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFont::setPointSize(int );
  fn _ZN5QFont12setPointSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QFont::NewQFont(const QString & family, int pointSize, int weight, bool italic);
  fn _ZN5QFontC1ERK7QStringiib(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: int8_t) ;
  // proto:  void QFont::setOverline(bool );
  fn _ZN5QFont11setOverlineEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QFont::family();
  fn _ZNK5QFont6familyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFont::lastResortFamily();
  fn _ZNK5QFont16lastResortFamilyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFont::setItalic(bool b);
  fn _ZN5QFont9setItalicEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFont::setFamily(const QString & );
  fn _ZN5QFont9setFamilyERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFont::NewQFont(const QFont & );
  fn _ZN5QFontC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFont::overline();
  fn _ZNK5QFont8overlineEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFont::FreeQFont();
  fn _ZN5QFontD0Ev(qthis: *mut c_void) ;
  // proto:  void QFont::setBold(bool );
  fn _ZN5QFont7setBoldEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto: static void QFont::cacheStatistics();
  fn _ZN5QFont15cacheStatisticsEv() ;
  // proto:  void QFont::setPointSizeF(qreal );
  fn _ZN5QFont13setPointSizeFEd(qthis: *mut c_void, arg0: c_double) ;
  // proto: static QStringList QFont::substitutes(const QString & );
  fn _ZN5QFont11substitutesERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  double QFont::wordSpacing();
  fn _ZNK5QFont11wordSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QFont::toString();
  fn _ZNK5QFont8toStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QFont::pointSizeF();
  fn _ZNK5QFont10pointSizeFEv(qthis: *mut c_void) -> c_double;
  // proto: static void QFont::insertSubstitution(const QString & , const QString & );
  fn _ZN5QFont18insertSubstitutionERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QFont::setStretch(int );
  fn _ZN5QFont10setStretchEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QFont::styleName();
  fn _ZNK5QFont9styleNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFont::NewQFont();
  fn _ZN5QFontC1Ev(qthis: *mut c_void) ;
  // proto:  bool QFont::rawMode();
  fn _ZNK5QFont7rawModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFont::fromString(const QString & );
  fn _ZN5QFont10fromStringERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QFont::underline();
  fn _ZNK5QFont9underlineEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFont::isCopyOf(const QFont & );
  fn _ZNK5QFont8isCopyOfERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  int QFont::pointSize();
  fn _ZNK5QFont9pointSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFont::setKerning(bool );
  fn _ZN5QFont10setKerningEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QFont::bold();
  fn _ZNK5QFont4boldEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFont::fixedPitch();
  fn _ZNK5QFont10fixedPitchEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFont::NewQFont(const QFont & , QPaintDevice * pd);
  fn _ZN5QFontC1ERKS_P12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto: static QString QFont::substitute(const QString & );
  fn _ZN5QFont10substituteERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFont::setFixedPitch(bool );
  fn _ZN5QFont13setFixedPitchEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto: static void QFont::removeSubstitutions(const QString & );
  fn _ZN5QFont19removeSubstitutionsERK7QString(arg0: *mut c_void) ;
  // proto:  void QFont::setPixelSize(int );
  fn _ZN5QFont12setPixelSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto: static void QFont::initialize();
  fn _ZN5QFont10initializeEv() ;
  // proto:  QString QFont::key();
  fn _ZNK5QFont3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFont::lastResortFont();
  fn _ZNK5QFont14lastResortFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFont::swap(QFont & other);
  fn _ZN5QFont4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QFont::defaultFamily();
  fn _ZNK5QFont13defaultFamilyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFont::setStrikeOut(bool );
  fn _ZN5QFont12setStrikeOutEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto: static void QFont::cleanup();
  fn _ZN5QFont7cleanupEv() ;
  // proto:  bool QFont::exactMatch();
  fn _ZNK5QFont10exactMatchEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QFont::stretch();
  fn _ZNK5QFont7stretchEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFont::setRawName(const QString & );
  fn _ZN5QFont10setRawNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QFont)=1
pub struct QFont {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFont {
  pub fn setWordSpacing<T: QFont_setWordSpacing>(&mut self, value: T)  {
     value.setWordSpacing(self);
    // return 1;
  }
}

pub trait QFont_setWordSpacing {
  fn setWordSpacing(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setWordSpacing(qreal spacing);
impl<'a> /*trait*/ QFont_setWordSpacing for (f64) {
  fn setWordSpacing(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont14setWordSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN5QFont14setWordSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn rawName<T: QFont_rawName>(&mut self, value: T) -> QString {
    return value.rawName(self);
    // return 1;
  }
}

pub trait QFont_rawName {
  fn rawName(self, rsthis: &mut QFont) -> QString;
}

// proto:  QString QFont::rawName();
impl<'a> /*trait*/ QFont_rawName for () {
  fn rawName(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7rawNameEv()};
    let mut ret = unsafe {_ZNK5QFont7rawNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setRawMode<T: QFont_setRawMode>(&mut self, value: T)  {
     value.setRawMode(self);
    // return 1;
  }
}

pub trait QFont_setRawMode {
  fn setRawMode(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setRawMode(bool );
impl<'a> /*trait*/ QFont_setRawMode for (i8) {
  fn setRawMode(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setRawModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont10setRawModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setStyleName<T: QFont_setStyleName>(&mut self, value: T)  {
     value.setStyleName(self);
    // return 1;
  }
}

pub trait QFont_setStyleName {
  fn setStyleName(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setStyleName(const QString & );
impl<'a> /*trait*/ QFont_setStyleName for (&'a  QString) {
  fn setStyleName(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setStyleNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont12setStyleNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn strikeOut<T: QFont_strikeOut>(&mut self, value: T) -> i8 {
    return value.strikeOut(self);
    // return 1;
  }
}

pub trait QFont_strikeOut {
  fn strikeOut(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::strikeOut();
impl<'a> /*trait*/ QFont_strikeOut for () {
  fn strikeOut(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9strikeOutEv()};
    let mut ret = unsafe {_ZNK5QFont9strikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn pixelSize<T: QFont_pixelSize>(&mut self, value: T) -> i32 {
    return value.pixelSize(self);
    // return 1;
  }
}

pub trait QFont_pixelSize {
  fn pixelSize(self, rsthis: &mut QFont) -> i32;
}

// proto:  int QFont::pixelSize();
impl<'a> /*trait*/ QFont_pixelSize for () {
  fn pixelSize(self, rsthis: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9pixelSizeEv()};
    let mut ret = unsafe {_ZNK5QFont9pixelSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setWeight<T: QFont_setWeight>(&mut self, value: T)  {
     value.setWeight(self);
    // return 1;
  }
}

pub trait QFont_setWeight {
  fn setWeight(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setWeight(int );
impl<'a> /*trait*/ QFont_setWeight for (i32) {
  fn setWeight(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont9setWeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QFont9setWeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn weight<T: QFont_weight>(&mut self, value: T) -> i32 {
    return value.weight(self);
    // return 1;
  }
}

pub trait QFont_weight {
  fn weight(self, rsthis: &mut QFont) -> i32;
}

// proto:  int QFont::weight();
impl<'a> /*trait*/ QFont_weight for () {
  fn weight(self, rsthis: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont6weightEv()};
    let mut ret = unsafe {_ZNK5QFont6weightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn insertSubstitutions<T: QFont_insertSubstitutions>(&mut self, value: T)  {
     value.insertSubstitutions(self);
    // return 1;
  }
}

pub trait QFont_insertSubstitutions {
  fn insertSubstitutions(self, rsthis: &mut QFont) ;
}

// proto: static void QFont::insertSubstitutions(const QString & , const QStringList & );
impl<'a> /*trait*/ QFont_insertSubstitutions for (&'a  QString, &'a  QStringList) {
  fn insertSubstitutions(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn kerning<T: QFont_kerning>(&mut self, value: T) -> i8 {
    return value.kerning(self);
    // return 1;
  }
}

pub trait QFont_kerning {
  fn kerning(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::kerning();
impl<'a> /*trait*/ QFont_kerning for () {
  fn kerning(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7kerningEv()};
    let mut ret = unsafe {_ZNK5QFont7kerningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn substitutions<T: QFont_substitutions>(&mut self, value: T) -> QStringList {
    return value.substitutions(self);
    // return 1;
  }
}

pub trait QFont_substitutions {
  fn substitutions(self, rsthis: &mut QFont) -> QStringList;
}

// proto: static QStringList QFont::substitutions();
impl<'a> /*trait*/ QFont_substitutions for () {
  fn substitutions(self, rsthis: &mut QFont) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13substitutionsEv()};
    let mut ret = unsafe {_ZN5QFont13substitutionsEv()};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn italic<T: QFont_italic>(&mut self, value: T) -> i8 {
    return value.italic(self);
    // return 1;
  }
}

pub trait QFont_italic {
  fn italic(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::italic();
impl<'a> /*trait*/ QFont_italic for () {
  fn italic(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont6italicEv()};
    let mut ret = unsafe {_ZNK5QFont6italicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setUnderline<T: QFont_setUnderline>(&mut self, value: T)  {
     value.setUnderline(self);
    // return 1;
  }
}

pub trait QFont_setUnderline {
  fn setUnderline(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setUnderline(bool );
impl<'a> /*trait*/ QFont_setUnderline for (i8) {
  fn setUnderline(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setUnderlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont12setUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn letterSpacing<T: QFont_letterSpacing>(&mut self, value: T) -> f64 {
    return value.letterSpacing(self);
    // return 1;
  }
}

pub trait QFont_letterSpacing {
  fn letterSpacing(self, rsthis: &mut QFont) -> f64;
}

// proto:  double QFont::letterSpacing();
impl<'a> /*trait*/ QFont_letterSpacing for () {
  fn letterSpacing(self, rsthis: &mut QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont13letterSpacingEv()};
    let mut ret = unsafe {_ZNK5QFont13letterSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPointSize<T: QFont_setPointSize>(&mut self, value: T)  {
     value.setPointSize(self);
    // return 1;
  }
}

pub trait QFont_setPointSize {
  fn setPointSize(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setPointSize(int );
impl<'a> /*trait*/ QFont_setPointSize for (i32) {
  fn setPointSize(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setPointSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QFont12setPointSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
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
  pub fn setOverline<T: QFont_setOverline>(&mut self, value: T)  {
     value.setOverline(self);
    // return 1;
  }
}

pub trait QFont_setOverline {
  fn setOverline(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setOverline(bool );
impl<'a> /*trait*/ QFont_setOverline for (i8) {
  fn setOverline(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont11setOverlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont11setOverlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn family<T: QFont_family>(&mut self, value: T) -> QString {
    return value.family(self);
    // return 1;
  }
}

pub trait QFont_family {
  fn family(self, rsthis: &mut QFont) -> QString;
}

// proto:  QString QFont::family();
impl<'a> /*trait*/ QFont_family for () {
  fn family(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont6familyEv()};
    let mut ret = unsafe {_ZNK5QFont6familyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn lastResortFamily<T: QFont_lastResortFamily>(&mut self, value: T) -> QString {
    return value.lastResortFamily(self);
    // return 1;
  }
}

pub trait QFont_lastResortFamily {
  fn lastResortFamily(self, rsthis: &mut QFont) -> QString;
}

// proto:  QString QFont::lastResortFamily();
impl<'a> /*trait*/ QFont_lastResortFamily for () {
  fn lastResortFamily(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont16lastResortFamilyEv()};
    let mut ret = unsafe {_ZNK5QFont16lastResortFamilyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setItalic<T: QFont_setItalic>(&mut self, value: T)  {
     value.setItalic(self);
    // return 1;
  }
}

pub trait QFont_setItalic {
  fn setItalic(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setItalic(bool b);
impl<'a> /*trait*/ QFont_setItalic for (i8) {
  fn setItalic(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont9setItalicEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont9setItalicEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setFamily<T: QFont_setFamily>(&mut self, value: T)  {
     value.setFamily(self);
    // return 1;
  }
}

pub trait QFont_setFamily {
  fn setFamily(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setFamily(const QString & );
impl<'a> /*trait*/ QFont_setFamily for (&'a  QString) {
  fn setFamily(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont9setFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont9setFamilyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QFont::NewQFont(const QFont & );
impl<'a> /*trait*/ QFont_NewQFont for (&'a  QFont) {
  fn NewQFont(self) -> QFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFontC1ERKS_(qthis, arg0)};
    let rsthis = QFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn overline<T: QFont_overline>(&mut self, value: T) -> i8 {
    return value.overline(self);
    // return 1;
  }
}

pub trait QFont_overline {
  fn overline(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::overline();
impl<'a> /*trait*/ QFont_overline for () {
  fn overline(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont8overlineEv()};
    let mut ret = unsafe {_ZNK5QFont8overlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn FreeQFont<T: QFont_FreeQFont>(&mut self, value: T)  {
     value.FreeQFont(self);
    // return 1;
  }
}

pub trait QFont_FreeQFont {
  fn FreeQFont(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::FreeQFont();
impl<'a> /*trait*/ QFont_FreeQFont for () {
  fn FreeQFont(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontD0Ev()};
     unsafe {_ZN5QFontD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setBold<T: QFont_setBold>(&mut self, value: T)  {
     value.setBold(self);
    // return 1;
  }
}

pub trait QFont_setBold {
  fn setBold(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setBold(bool );
impl<'a> /*trait*/ QFont_setBold for (i8) {
  fn setBold(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont7setBoldEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont7setBoldEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn cacheStatistics<T: QFont_cacheStatistics>(&mut self, value: T)  {
     value.cacheStatistics(self);
    // return 1;
  }
}

pub trait QFont_cacheStatistics {
  fn cacheStatistics(self, rsthis: &mut QFont) ;
}

// proto: static void QFont::cacheStatistics();
impl<'a> /*trait*/ QFont_cacheStatistics for () {
  fn cacheStatistics(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont15cacheStatisticsEv()};
     unsafe {_ZN5QFont15cacheStatisticsEv()};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPointSizeF<T: QFont_setPointSizeF>(&mut self, value: T)  {
     value.setPointSizeF(self);
    // return 1;
  }
}

pub trait QFont_setPointSizeF {
  fn setPointSizeF(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setPointSizeF(qreal );
impl<'a> /*trait*/ QFont_setPointSizeF for (f64) {
  fn setPointSizeF(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13setPointSizeFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN5QFont13setPointSizeFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn substitutes<T: QFont_substitutes>(&mut self, value: T) -> QStringList {
    return value.substitutes(self);
    // return 1;
  }
}

pub trait QFont_substitutes {
  fn substitutes(self, rsthis: &mut QFont) -> QStringList;
}

// proto: static QStringList QFont::substitutes(const QString & );
impl<'a> /*trait*/ QFont_substitutes for (&'a  QString) {
  fn substitutes(self, rsthis: &mut QFont) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont11substitutesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFont11substitutesERK7QString(arg0)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn wordSpacing<T: QFont_wordSpacing>(&mut self, value: T) -> f64 {
    return value.wordSpacing(self);
    // return 1;
  }
}

pub trait QFont_wordSpacing {
  fn wordSpacing(self, rsthis: &mut QFont) -> f64;
}

// proto:  double QFont::wordSpacing();
impl<'a> /*trait*/ QFont_wordSpacing for () {
  fn wordSpacing(self, rsthis: &mut QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont11wordSpacingEv()};
    let mut ret = unsafe {_ZNK5QFont11wordSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn toString<T: QFont_toString>(&mut self, value: T) -> QString {
    return value.toString(self);
    // return 1;
  }
}

pub trait QFont_toString {
  fn toString(self, rsthis: &mut QFont) -> QString;
}

// proto:  QString QFont::toString();
impl<'a> /*trait*/ QFont_toString for () {
  fn toString(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont8toStringEv()};
    let mut ret = unsafe {_ZNK5QFont8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn pointSizeF<T: QFont_pointSizeF>(&mut self, value: T) -> f64 {
    return value.pointSizeF(self);
    // return 1;
  }
}

pub trait QFont_pointSizeF {
  fn pointSizeF(self, rsthis: &mut QFont) -> f64;
}

// proto:  double QFont::pointSizeF();
impl<'a> /*trait*/ QFont_pointSizeF for () {
  fn pointSizeF(self, rsthis: &mut QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont10pointSizeFEv()};
    let mut ret = unsafe {_ZNK5QFont10pointSizeFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn insertSubstitution<T: QFont_insertSubstitution>(&mut self, value: T)  {
     value.insertSubstitution(self);
    // return 1;
  }
}

pub trait QFont_insertSubstitution {
  fn insertSubstitution(self, rsthis: &mut QFont) ;
}

// proto: static void QFont::insertSubstitution(const QString & , const QString & );
impl<'a> /*trait*/ QFont_insertSubstitution for (&'a  QString, &'a  QString) {
  fn insertSubstitution(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont18insertSubstitutionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont18insertSubstitutionERK7QStringS2_(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setStretch<T: QFont_setStretch>(&mut self, value: T)  {
     value.setStretch(self);
    // return 1;
  }
}

pub trait QFont_setStretch {
  fn setStretch(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setStretch(int );
impl<'a> /*trait*/ QFont_setStretch for (i32) {
  fn setStretch(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QFont10setStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn styleName<T: QFont_styleName>(&mut self, value: T) -> QString {
    return value.styleName(self);
    // return 1;
  }
}

pub trait QFont_styleName {
  fn styleName(self, rsthis: &mut QFont) -> QString;
}

// proto:  QString QFont::styleName();
impl<'a> /*trait*/ QFont_styleName for () {
  fn styleName(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9styleNameEv()};
    let mut ret = unsafe {_ZNK5QFont9styleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn rawMode<T: QFont_rawMode>(&mut self, value: T) -> i8 {
    return value.rawMode(self);
    // return 1;
  }
}

pub trait QFont_rawMode {
  fn rawMode(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::rawMode();
impl<'a> /*trait*/ QFont_rawMode for () {
  fn rawMode(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7rawModeEv()};
    let mut ret = unsafe {_ZNK5QFont7rawModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn fromString<T: QFont_fromString>(&mut self, value: T) -> i8 {
    return value.fromString(self);
    // return 1;
  }
}

pub trait QFont_fromString {
  fn fromString(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::fromString(const QString & );
impl<'a> /*trait*/ QFont_fromString for (&'a  QString) {
  fn fromString(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10fromStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFont10fromStringERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn underline<T: QFont_underline>(&mut self, value: T) -> i8 {
    return value.underline(self);
    // return 1;
  }
}

pub trait QFont_underline {
  fn underline(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::underline();
impl<'a> /*trait*/ QFont_underline for () {
  fn underline(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9underlineEv()};
    let mut ret = unsafe {_ZNK5QFont9underlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn isCopyOf<T: QFont_isCopyOf>(&mut self, value: T) -> i8 {
    return value.isCopyOf(self);
    // return 1;
  }
}

pub trait QFont_isCopyOf {
  fn isCopyOf(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::isCopyOf(const QFont & );
impl<'a> /*trait*/ QFont_isCopyOf for (&'a  QFont) {
  fn isCopyOf(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont8isCopyOfERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QFont8isCopyOfERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn pointSize<T: QFont_pointSize>(&mut self, value: T) -> i32 {
    return value.pointSize(self);
    // return 1;
  }
}

pub trait QFont_pointSize {
  fn pointSize(self, rsthis: &mut QFont) -> i32;
}

// proto:  int QFont::pointSize();
impl<'a> /*trait*/ QFont_pointSize for () {
  fn pointSize(self, rsthis: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9pointSizeEv()};
    let mut ret = unsafe {_ZNK5QFont9pointSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setKerning<T: QFont_setKerning>(&mut self, value: T)  {
     value.setKerning(self);
    // return 1;
  }
}

pub trait QFont_setKerning {
  fn setKerning(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setKerning(bool );
impl<'a> /*trait*/ QFont_setKerning for (i8) {
  fn setKerning(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setKerningEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont10setKerningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn bold<T: QFont_bold>(&mut self, value: T) -> i8 {
    return value.bold(self);
    // return 1;
  }
}

pub trait QFont_bold {
  fn bold(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::bold();
impl<'a> /*trait*/ QFont_bold for () {
  fn bold(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont4boldEv()};
    let mut ret = unsafe {_ZNK5QFont4boldEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn fixedPitch<T: QFont_fixedPitch>(&mut self, value: T) -> i8 {
    return value.fixedPitch(self);
    // return 1;
  }
}

pub trait QFont_fixedPitch {
  fn fixedPitch(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::fixedPitch();
impl<'a> /*trait*/ QFont_fixedPitch for () {
  fn fixedPitch(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont10fixedPitchEv()};
    let mut ret = unsafe {_ZNK5QFont10fixedPitchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QFont::NewQFont(const QFont & , QPaintDevice * pd);
impl<'a> /*trait*/ QFont_NewQFont for (&'a  QFont, &'a mut QPaintDevice) {
  fn NewQFont(self) -> QFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontC1ERKS_P12QPaintDevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QFontC1ERKS_P12QPaintDevice(qthis, arg0, arg1)};
    let rsthis = QFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn substitute<T: QFont_substitute>(&mut self, value: T) -> QString {
    return value.substitute(self);
    // return 1;
  }
}

pub trait QFont_substitute {
  fn substitute(self, rsthis: &mut QFont) -> QString;
}

// proto: static QString QFont::substitute(const QString & );
impl<'a> /*trait*/ QFont_substitute for (&'a  QString) {
  fn substitute(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10substituteERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFont10substituteERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setFixedPitch<T: QFont_setFixedPitch>(&mut self, value: T)  {
     value.setFixedPitch(self);
    // return 1;
  }
}

pub trait QFont_setFixedPitch {
  fn setFixedPitch(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setFixedPitch(bool );
impl<'a> /*trait*/ QFont_setFixedPitch for (i8) {
  fn setFixedPitch(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13setFixedPitchEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont13setFixedPitchEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn removeSubstitutions<T: QFont_removeSubstitutions>(&mut self, value: T)  {
     value.removeSubstitutions(self);
    // return 1;
  }
}

pub trait QFont_removeSubstitutions {
  fn removeSubstitutions(self, rsthis: &mut QFont) ;
}

// proto: static void QFont::removeSubstitutions(const QString & );
impl<'a> /*trait*/ QFont_removeSubstitutions for (&'a  QString) {
  fn removeSubstitutions(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont19removeSubstitutionsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont19removeSubstitutionsERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPixelSize<T: QFont_setPixelSize>(&mut self, value: T)  {
     value.setPixelSize(self);
    // return 1;
  }
}

pub trait QFont_setPixelSize {
  fn setPixelSize(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setPixelSize(int );
impl<'a> /*trait*/ QFont_setPixelSize for (i32) {
  fn setPixelSize(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setPixelSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QFont12setPixelSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn initialize<T: QFont_initialize>(&mut self, value: T)  {
     value.initialize(self);
    // return 1;
  }
}

pub trait QFont_initialize {
  fn initialize(self, rsthis: &mut QFont) ;
}

// proto: static void QFont::initialize();
impl<'a> /*trait*/ QFont_initialize for () {
  fn initialize(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10initializeEv()};
     unsafe {_ZN5QFont10initializeEv()};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn key<T: QFont_key>(&mut self, value: T) -> QString {
    return value.key(self);
    // return 1;
  }
}

pub trait QFont_key {
  fn key(self, rsthis: &mut QFont) -> QString;
}

// proto:  QString QFont::key();
impl<'a> /*trait*/ QFont_key for () {
  fn key(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont3keyEv()};
    let mut ret = unsafe {_ZNK5QFont3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn lastResortFont<T: QFont_lastResortFont>(&mut self, value: T) -> QString {
    return value.lastResortFont(self);
    // return 1;
  }
}

pub trait QFont_lastResortFont {
  fn lastResortFont(self, rsthis: &mut QFont) -> QString;
}

// proto:  QString QFont::lastResortFont();
impl<'a> /*trait*/ QFont_lastResortFont for () {
  fn lastResortFont(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont14lastResortFontEv()};
    let mut ret = unsafe {_ZNK5QFont14lastResortFontEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn swap<T: QFont_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QFont_swap {
  fn swap(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::swap(QFont & other);
impl<'a> /*trait*/ QFont_swap for (&'a mut QFont) {
  fn swap(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn defaultFamily<T: QFont_defaultFamily>(&mut self, value: T) -> QString {
    return value.defaultFamily(self);
    // return 1;
  }
}

pub trait QFont_defaultFamily {
  fn defaultFamily(self, rsthis: &mut QFont) -> QString;
}

// proto:  QString QFont::defaultFamily();
impl<'a> /*trait*/ QFont_defaultFamily for () {
  fn defaultFamily(self, rsthis: &mut QFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont13defaultFamilyEv()};
    let mut ret = unsafe {_ZNK5QFont13defaultFamilyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setStrikeOut<T: QFont_setStrikeOut>(&mut self, value: T)  {
     value.setStrikeOut(self);
    // return 1;
  }
}

pub trait QFont_setStrikeOut {
  fn setStrikeOut(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setStrikeOut(bool );
impl<'a> /*trait*/ QFont_setStrikeOut for (i8) {
  fn setStrikeOut(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setStrikeOutEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont12setStrikeOutEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn cleanup<T: QFont_cleanup>(&mut self, value: T)  {
     value.cleanup(self);
    // return 1;
  }
}

pub trait QFont_cleanup {
  fn cleanup(self, rsthis: &mut QFont) ;
}

// proto: static void QFont::cleanup();
impl<'a> /*trait*/ QFont_cleanup for () {
  fn cleanup(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont7cleanupEv()};
     unsafe {_ZN5QFont7cleanupEv()};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn exactMatch<T: QFont_exactMatch>(&mut self, value: T) -> i8 {
    return value.exactMatch(self);
    // return 1;
  }
}

pub trait QFont_exactMatch {
  fn exactMatch(self, rsthis: &mut QFont) -> i8;
}

// proto:  bool QFont::exactMatch();
impl<'a> /*trait*/ QFont_exactMatch for () {
  fn exactMatch(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont10exactMatchEv()};
    let mut ret = unsafe {_ZNK5QFont10exactMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn stretch<T: QFont_stretch>(&mut self, value: T) -> i32 {
    return value.stretch(self);
    // return 1;
  }
}

pub trait QFont_stretch {
  fn stretch(self, rsthis: &mut QFont) -> i32;
}

// proto:  int QFont::stretch();
impl<'a> /*trait*/ QFont_stretch for () {
  fn stretch(self, rsthis: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7stretchEv()};
    let mut ret = unsafe {_ZNK5QFont7stretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setRawName<T: QFont_setRawName>(&mut self, value: T)  {
     value.setRawName(self);
    // return 1;
  }
}

pub trait QFont_setRawName {
  fn setRawName(self, rsthis: &mut QFont) ;
}

// proto:  void QFont::setRawName(const QString & );
impl<'a> /*trait*/ QFont_setRawName for (&'a  QString) {
  fn setRawName(self, rsthis: &mut QFont)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setRawNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont10setRawNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

