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
  // proto:  QFont QFont::resolve(const QFont & );
  fn _ZNK5QFont7resolveERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
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
  fn _ZN5QFont13substitutionsEv() ;
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
  // proto:  void QFont::resolve(uint mask);
  fn _ZN5QFont7resolveEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QFont::setBold(bool );
  fn _ZN5QFont7setBoldEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto: static void QFont::cacheStatistics();
  fn _ZN5QFont15cacheStatisticsEv() ;
  // proto:  void QFont::setPointSizeF(qreal );
  fn _ZN5QFont13setPointSizeFEd(qthis: *mut c_void, arg0: c_double) ;
  // proto: static QStringList QFont::substitutes(const QString & );
  fn _ZN5QFont11substitutesERK7QString(arg0: *mut c_void) ;
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
  // proto:  unsigned int QFont::resolve();
  fn _ZNK5QFont7resolveEv(qthis: *mut c_void) -> c_uint;
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
  pub fn setWordSpacing<RetType, T: QFont_setWordSpacing<RetType>>(&mut self, value: T) -> RetType {
    return value.setWordSpacing(self);
    // return 1;
  }
}

pub trait QFont_setWordSpacing<RetType> {
  fn setWordSpacing(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setWordSpacing(qreal spacing);
impl<'a> /*trait*/ QFont_setWordSpacing<()> for (f64) {
  fn setWordSpacing(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont14setWordSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN5QFont14setWordSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn rawName<RetType, T: QFont_rawName<RetType>>(&mut self, value: T) -> RetType {
    return value.rawName(self);
    // return 1;
  }
}

pub trait QFont_rawName<RetType> {
  fn rawName(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QString QFont::rawName();
impl<'a> /*trait*/ QFont_rawName<QString> for () {
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
  pub fn setRawMode<RetType, T: QFont_setRawMode<RetType>>(&mut self, value: T) -> RetType {
    return value.setRawMode(self);
    // return 1;
  }
}

pub trait QFont_setRawMode<RetType> {
  fn setRawMode(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setRawMode(bool );
impl<'a> /*trait*/ QFont_setRawMode<()> for (i8) {
  fn setRawMode(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setRawModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont10setRawModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setStyleName<RetType, T: QFont_setStyleName<RetType>>(&mut self, value: T) -> RetType {
    return value.setStyleName(self);
    // return 1;
  }
}

pub trait QFont_setStyleName<RetType> {
  fn setStyleName(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setStyleName(const QString & );
impl<'a> /*trait*/ QFont_setStyleName<()> for (&'a  QString) {
  fn setStyleName(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setStyleNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont12setStyleNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn resolve<RetType, T: QFont_resolve<RetType>>(&mut self, value: T) -> RetType {
    return value.resolve(self);
    // return 1;
  }
}

pub trait QFont_resolve<RetType> {
  fn resolve(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QFont QFont::resolve(const QFont & );
impl<'a> /*trait*/ QFont_resolve<QFont> for (&'a  QFont) {
  fn resolve(self, rsthis: &mut QFont) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7resolveERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QFont7resolveERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn strikeOut<RetType, T: QFont_strikeOut<RetType>>(&mut self, value: T) -> RetType {
    return value.strikeOut(self);
    // return 1;
  }
}

pub trait QFont_strikeOut<RetType> {
  fn strikeOut(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::strikeOut();
impl<'a> /*trait*/ QFont_strikeOut<i8> for () {
  fn strikeOut(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9strikeOutEv()};
    let mut ret = unsafe {_ZNK5QFont9strikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn pixelSize<RetType, T: QFont_pixelSize<RetType>>(&mut self, value: T) -> RetType {
    return value.pixelSize(self);
    // return 1;
  }
}

pub trait QFont_pixelSize<RetType> {
  fn pixelSize(self, rsthis: &mut QFont) -> RetType;
}

// proto:  int QFont::pixelSize();
impl<'a> /*trait*/ QFont_pixelSize<i32> for () {
  fn pixelSize(self, rsthis: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9pixelSizeEv()};
    let mut ret = unsafe {_ZNK5QFont9pixelSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setWeight<RetType, T: QFont_setWeight<RetType>>(&mut self, value: T) -> RetType {
    return value.setWeight(self);
    // return 1;
  }
}

pub trait QFont_setWeight<RetType> {
  fn setWeight(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setWeight(int );
impl<'a> /*trait*/ QFont_setWeight<()> for (i32) {
  fn setWeight(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont9setWeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QFont9setWeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn weight<RetType, T: QFont_weight<RetType>>(&mut self, value: T) -> RetType {
    return value.weight(self);
    // return 1;
  }
}

pub trait QFont_weight<RetType> {
  fn weight(self, rsthis: &mut QFont) -> RetType;
}

// proto:  int QFont::weight();
impl<'a> /*trait*/ QFont_weight<i32> for () {
  fn weight(self, rsthis: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont6weightEv()};
    let mut ret = unsafe {_ZNK5QFont6weightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn insertSubstitutions<RetType, T: QFont_insertSubstitutions<RetType>>(&mut self, value: T) -> RetType {
    return value.insertSubstitutions(self);
    // return 1;
  }
}

pub trait QFont_insertSubstitutions<RetType> {
  fn insertSubstitutions(self, rsthis: &mut QFont) -> RetType;
}

// proto: static void QFont::insertSubstitutions(const QString & , const QStringList & );
impl<'a> /*trait*/ QFont_insertSubstitutions<()> for (&'a  QString, &'a  QStringList) {
  fn insertSubstitutions(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn kerning<RetType, T: QFont_kerning<RetType>>(&mut self, value: T) -> RetType {
    return value.kerning(self);
    // return 1;
  }
}

pub trait QFont_kerning<RetType> {
  fn kerning(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::kerning();
impl<'a> /*trait*/ QFont_kerning<i8> for () {
  fn kerning(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7kerningEv()};
    let mut ret = unsafe {_ZNK5QFont7kerningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn substitutions<RetType, T: QFont_substitutions<RetType>>(&mut self, value: T) -> RetType {
    return value.substitutions(self);
    // return 1;
  }
}

pub trait QFont_substitutions<RetType> {
  fn substitutions(self, rsthis: &mut QFont) -> RetType;
}

// proto: static QStringList QFont::substitutions();
impl<'a> /*trait*/ QFont_substitutions<()> for () {
  fn substitutions(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13substitutionsEv()};
     unsafe {_ZN5QFont13substitutionsEv()};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn italic<RetType, T: QFont_italic<RetType>>(&mut self, value: T) -> RetType {
    return value.italic(self);
    // return 1;
  }
}

pub trait QFont_italic<RetType> {
  fn italic(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::italic();
impl<'a> /*trait*/ QFont_italic<i8> for () {
  fn italic(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont6italicEv()};
    let mut ret = unsafe {_ZNK5QFont6italicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setUnderline<RetType, T: QFont_setUnderline<RetType>>(&mut self, value: T) -> RetType {
    return value.setUnderline(self);
    // return 1;
  }
}

pub trait QFont_setUnderline<RetType> {
  fn setUnderline(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setUnderline(bool );
impl<'a> /*trait*/ QFont_setUnderline<()> for (i8) {
  fn setUnderline(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setUnderlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont12setUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn letterSpacing<RetType, T: QFont_letterSpacing<RetType>>(&mut self, value: T) -> RetType {
    return value.letterSpacing(self);
    // return 1;
  }
}

pub trait QFont_letterSpacing<RetType> {
  fn letterSpacing(self, rsthis: &mut QFont) -> RetType;
}

// proto:  double QFont::letterSpacing();
impl<'a> /*trait*/ QFont_letterSpacing<f64> for () {
  fn letterSpacing(self, rsthis: &mut QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont13letterSpacingEv()};
    let mut ret = unsafe {_ZNK5QFont13letterSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPointSize<RetType, T: QFont_setPointSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setPointSize(self);
    // return 1;
  }
}

pub trait QFont_setPointSize<RetType> {
  fn setPointSize(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setPointSize(int );
impl<'a> /*trait*/ QFont_setPointSize<()> for (i32) {
  fn setPointSize(self, rsthis: &mut QFont) -> () {
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
  pub fn setOverline<RetType, T: QFont_setOverline<RetType>>(&mut self, value: T) -> RetType {
    return value.setOverline(self);
    // return 1;
  }
}

pub trait QFont_setOverline<RetType> {
  fn setOverline(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setOverline(bool );
impl<'a> /*trait*/ QFont_setOverline<()> for (i8) {
  fn setOverline(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont11setOverlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont11setOverlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn family<RetType, T: QFont_family<RetType>>(&mut self, value: T) -> RetType {
    return value.family(self);
    // return 1;
  }
}

pub trait QFont_family<RetType> {
  fn family(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QString QFont::family();
impl<'a> /*trait*/ QFont_family<QString> for () {
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
  pub fn lastResortFamily<RetType, T: QFont_lastResortFamily<RetType>>(&mut self, value: T) -> RetType {
    return value.lastResortFamily(self);
    // return 1;
  }
}

pub trait QFont_lastResortFamily<RetType> {
  fn lastResortFamily(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QString QFont::lastResortFamily();
impl<'a> /*trait*/ QFont_lastResortFamily<QString> for () {
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
  pub fn setItalic<RetType, T: QFont_setItalic<RetType>>(&mut self, value: T) -> RetType {
    return value.setItalic(self);
    // return 1;
  }
}

pub trait QFont_setItalic<RetType> {
  fn setItalic(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setItalic(bool b);
impl<'a> /*trait*/ QFont_setItalic<()> for (i8) {
  fn setItalic(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont9setItalicEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont9setItalicEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setFamily<RetType, T: QFont_setFamily<RetType>>(&mut self, value: T) -> RetType {
    return value.setFamily(self);
    // return 1;
  }
}

pub trait QFont_setFamily<RetType> {
  fn setFamily(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setFamily(const QString & );
impl<'a> /*trait*/ QFont_setFamily<()> for (&'a  QString) {
  fn setFamily(self, rsthis: &mut QFont) -> () {
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
  pub fn overline<RetType, T: QFont_overline<RetType>>(&mut self, value: T) -> RetType {
    return value.overline(self);
    // return 1;
  }
}

pub trait QFont_overline<RetType> {
  fn overline(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::overline();
impl<'a> /*trait*/ QFont_overline<i8> for () {
  fn overline(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont8overlineEv()};
    let mut ret = unsafe {_ZNK5QFont8overlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn FreeQFont<RetType, T: QFont_FreeQFont<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQFont(self);
    // return 1;
  }
}

pub trait QFont_FreeQFont<RetType> {
  fn FreeQFont(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::FreeQFont();
impl<'a> /*trait*/ QFont_FreeQFont<()> for () {
  fn FreeQFont(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFontD0Ev()};
     unsafe {_ZN5QFontD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QFont::resolve(uint mask);
impl<'a> /*trait*/ QFont_resolve<()> for (u32) {
  fn resolve(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont7resolveEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN5QFont7resolveEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setBold<RetType, T: QFont_setBold<RetType>>(&mut self, value: T) -> RetType {
    return value.setBold(self);
    // return 1;
  }
}

pub trait QFont_setBold<RetType> {
  fn setBold(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setBold(bool );
impl<'a> /*trait*/ QFont_setBold<()> for (i8) {
  fn setBold(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont7setBoldEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont7setBoldEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn cacheStatistics<RetType, T: QFont_cacheStatistics<RetType>>(&mut self, value: T) -> RetType {
    return value.cacheStatistics(self);
    // return 1;
  }
}

pub trait QFont_cacheStatistics<RetType> {
  fn cacheStatistics(self, rsthis: &mut QFont) -> RetType;
}

// proto: static void QFont::cacheStatistics();
impl<'a> /*trait*/ QFont_cacheStatistics<()> for () {
  fn cacheStatistics(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont15cacheStatisticsEv()};
     unsafe {_ZN5QFont15cacheStatisticsEv()};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPointSizeF<RetType, T: QFont_setPointSizeF<RetType>>(&mut self, value: T) -> RetType {
    return value.setPointSizeF(self);
    // return 1;
  }
}

pub trait QFont_setPointSizeF<RetType> {
  fn setPointSizeF(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setPointSizeF(qreal );
impl<'a> /*trait*/ QFont_setPointSizeF<()> for (f64) {
  fn setPointSizeF(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13setPointSizeFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN5QFont13setPointSizeFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn substitutes<RetType, T: QFont_substitutes<RetType>>(&mut self, value: T) -> RetType {
    return value.substitutes(self);
    // return 1;
  }
}

pub trait QFont_substitutes<RetType> {
  fn substitutes(self, rsthis: &mut QFont) -> RetType;
}

// proto: static QStringList QFont::substitutes(const QString & );
impl<'a> /*trait*/ QFont_substitutes<()> for (&'a  QString) {
  fn substitutes(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont11substitutesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont11substitutesERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn wordSpacing<RetType, T: QFont_wordSpacing<RetType>>(&mut self, value: T) -> RetType {
    return value.wordSpacing(self);
    // return 1;
  }
}

pub trait QFont_wordSpacing<RetType> {
  fn wordSpacing(self, rsthis: &mut QFont) -> RetType;
}

// proto:  double QFont::wordSpacing();
impl<'a> /*trait*/ QFont_wordSpacing<f64> for () {
  fn wordSpacing(self, rsthis: &mut QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont11wordSpacingEv()};
    let mut ret = unsafe {_ZNK5QFont11wordSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn toString<RetType, T: QFont_toString<RetType>>(&mut self, value: T) -> RetType {
    return value.toString(self);
    // return 1;
  }
}

pub trait QFont_toString<RetType> {
  fn toString(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QString QFont::toString();
impl<'a> /*trait*/ QFont_toString<QString> for () {
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
  pub fn pointSizeF<RetType, T: QFont_pointSizeF<RetType>>(&mut self, value: T) -> RetType {
    return value.pointSizeF(self);
    // return 1;
  }
}

pub trait QFont_pointSizeF<RetType> {
  fn pointSizeF(self, rsthis: &mut QFont) -> RetType;
}

// proto:  double QFont::pointSizeF();
impl<'a> /*trait*/ QFont_pointSizeF<f64> for () {
  fn pointSizeF(self, rsthis: &mut QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont10pointSizeFEv()};
    let mut ret = unsafe {_ZNK5QFont10pointSizeFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn insertSubstitution<RetType, T: QFont_insertSubstitution<RetType>>(&mut self, value: T) -> RetType {
    return value.insertSubstitution(self);
    // return 1;
  }
}

pub trait QFont_insertSubstitution<RetType> {
  fn insertSubstitution(self, rsthis: &mut QFont) -> RetType;
}

// proto: static void QFont::insertSubstitution(const QString & , const QString & );
impl<'a> /*trait*/ QFont_insertSubstitution<()> for (&'a  QString, &'a  QString) {
  fn insertSubstitution(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont18insertSubstitutionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont18insertSubstitutionERK7QStringS2_(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setStretch<RetType, T: QFont_setStretch<RetType>>(&mut self, value: T) -> RetType {
    return value.setStretch(self);
    // return 1;
  }
}

pub trait QFont_setStretch<RetType> {
  fn setStretch(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setStretch(int );
impl<'a> /*trait*/ QFont_setStretch<()> for (i32) {
  fn setStretch(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QFont10setStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn styleName<RetType, T: QFont_styleName<RetType>>(&mut self, value: T) -> RetType {
    return value.styleName(self);
    // return 1;
  }
}

pub trait QFont_styleName<RetType> {
  fn styleName(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QString QFont::styleName();
impl<'a> /*trait*/ QFont_styleName<QString> for () {
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
  pub fn rawMode<RetType, T: QFont_rawMode<RetType>>(&mut self, value: T) -> RetType {
    return value.rawMode(self);
    // return 1;
  }
}

pub trait QFont_rawMode<RetType> {
  fn rawMode(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::rawMode();
impl<'a> /*trait*/ QFont_rawMode<i8> for () {
  fn rawMode(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7rawModeEv()};
    let mut ret = unsafe {_ZNK5QFont7rawModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn fromString<RetType, T: QFont_fromString<RetType>>(&mut self, value: T) -> RetType {
    return value.fromString(self);
    // return 1;
  }
}

pub trait QFont_fromString<RetType> {
  fn fromString(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::fromString(const QString & );
impl<'a> /*trait*/ QFont_fromString<i8> for (&'a  QString) {
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
  pub fn underline<RetType, T: QFont_underline<RetType>>(&mut self, value: T) -> RetType {
    return value.underline(self);
    // return 1;
  }
}

pub trait QFont_underline<RetType> {
  fn underline(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::underline();
impl<'a> /*trait*/ QFont_underline<i8> for () {
  fn underline(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9underlineEv()};
    let mut ret = unsafe {_ZNK5QFont9underlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn isCopyOf<RetType, T: QFont_isCopyOf<RetType>>(&mut self, value: T) -> RetType {
    return value.isCopyOf(self);
    // return 1;
  }
}

pub trait QFont_isCopyOf<RetType> {
  fn isCopyOf(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::isCopyOf(const QFont & );
impl<'a> /*trait*/ QFont_isCopyOf<i8> for (&'a  QFont) {
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
  pub fn pointSize<RetType, T: QFont_pointSize<RetType>>(&mut self, value: T) -> RetType {
    return value.pointSize(self);
    // return 1;
  }
}

pub trait QFont_pointSize<RetType> {
  fn pointSize(self, rsthis: &mut QFont) -> RetType;
}

// proto:  int QFont::pointSize();
impl<'a> /*trait*/ QFont_pointSize<i32> for () {
  fn pointSize(self, rsthis: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont9pointSizeEv()};
    let mut ret = unsafe {_ZNK5QFont9pointSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setKerning<RetType, T: QFont_setKerning<RetType>>(&mut self, value: T) -> RetType {
    return value.setKerning(self);
    // return 1;
  }
}

pub trait QFont_setKerning<RetType> {
  fn setKerning(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setKerning(bool );
impl<'a> /*trait*/ QFont_setKerning<()> for (i8) {
  fn setKerning(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setKerningEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont10setKerningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn bold<RetType, T: QFont_bold<RetType>>(&mut self, value: T) -> RetType {
    return value.bold(self);
    // return 1;
  }
}

pub trait QFont_bold<RetType> {
  fn bold(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::bold();
impl<'a> /*trait*/ QFont_bold<i8> for () {
  fn bold(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont4boldEv()};
    let mut ret = unsafe {_ZNK5QFont4boldEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn fixedPitch<RetType, T: QFont_fixedPitch<RetType>>(&mut self, value: T) -> RetType {
    return value.fixedPitch(self);
    // return 1;
  }
}

pub trait QFont_fixedPitch<RetType> {
  fn fixedPitch(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::fixedPitch();
impl<'a> /*trait*/ QFont_fixedPitch<i8> for () {
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
  pub fn substitute<RetType, T: QFont_substitute<RetType>>(&mut self, value: T) -> RetType {
    return value.substitute(self);
    // return 1;
  }
}

pub trait QFont_substitute<RetType> {
  fn substitute(self, rsthis: &mut QFont) -> RetType;
}

// proto: static QString QFont::substitute(const QString & );
impl<'a> /*trait*/ QFont_substitute<QString> for (&'a  QString) {
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
  pub fn setFixedPitch<RetType, T: QFont_setFixedPitch<RetType>>(&mut self, value: T) -> RetType {
    return value.setFixedPitch(self);
    // return 1;
  }
}

pub trait QFont_setFixedPitch<RetType> {
  fn setFixedPitch(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setFixedPitch(bool );
impl<'a> /*trait*/ QFont_setFixedPitch<()> for (i8) {
  fn setFixedPitch(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont13setFixedPitchEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont13setFixedPitchEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn removeSubstitutions<RetType, T: QFont_removeSubstitutions<RetType>>(&mut self, value: T) -> RetType {
    return value.removeSubstitutions(self);
    // return 1;
  }
}

pub trait QFont_removeSubstitutions<RetType> {
  fn removeSubstitutions(self, rsthis: &mut QFont) -> RetType;
}

// proto: static void QFont::removeSubstitutions(const QString & );
impl<'a> /*trait*/ QFont_removeSubstitutions<()> for (&'a  QString) {
  fn removeSubstitutions(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont19removeSubstitutionsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont19removeSubstitutionsERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setPixelSize<RetType, T: QFont_setPixelSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setPixelSize(self);
    // return 1;
  }
}

pub trait QFont_setPixelSize<RetType> {
  fn setPixelSize(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setPixelSize(int );
impl<'a> /*trait*/ QFont_setPixelSize<()> for (i32) {
  fn setPixelSize(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setPixelSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QFont12setPixelSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn initialize<RetType, T: QFont_initialize<RetType>>(&mut self, value: T) -> RetType {
    return value.initialize(self);
    // return 1;
  }
}

pub trait QFont_initialize<RetType> {
  fn initialize(self, rsthis: &mut QFont) -> RetType;
}

// proto: static void QFont::initialize();
impl<'a> /*trait*/ QFont_initialize<()> for () {
  fn initialize(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10initializeEv()};
     unsafe {_ZN5QFont10initializeEv()};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn key<RetType, T: QFont_key<RetType>>(&mut self, value: T) -> RetType {
    return value.key(self);
    // return 1;
  }
}

pub trait QFont_key<RetType> {
  fn key(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QString QFont::key();
impl<'a> /*trait*/ QFont_key<QString> for () {
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
  pub fn lastResortFont<RetType, T: QFont_lastResortFont<RetType>>(&mut self, value: T) -> RetType {
    return value.lastResortFont(self);
    // return 1;
  }
}

pub trait QFont_lastResortFont<RetType> {
  fn lastResortFont(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QString QFont::lastResortFont();
impl<'a> /*trait*/ QFont_lastResortFont<QString> for () {
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
  pub fn swap<RetType, T: QFont_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QFont_swap<RetType> {
  fn swap(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::swap(QFont & other);
impl<'a> /*trait*/ QFont_swap<()> for (&'a mut QFont) {
  fn swap(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn defaultFamily<RetType, T: QFont_defaultFamily<RetType>>(&mut self, value: T) -> RetType {
    return value.defaultFamily(self);
    // return 1;
  }
}

pub trait QFont_defaultFamily<RetType> {
  fn defaultFamily(self, rsthis: &mut QFont) -> RetType;
}

// proto:  QString QFont::defaultFamily();
impl<'a> /*trait*/ QFont_defaultFamily<QString> for () {
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
  pub fn setStrikeOut<RetType, T: QFont_setStrikeOut<RetType>>(&mut self, value: T) -> RetType {
    return value.setStrikeOut(self);
    // return 1;
  }
}

pub trait QFont_setStrikeOut<RetType> {
  fn setStrikeOut(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setStrikeOut(bool );
impl<'a> /*trait*/ QFont_setStrikeOut<()> for (i8) {
  fn setStrikeOut(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont12setStrikeOutEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QFont12setStrikeOutEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  unsigned int QFont::resolve();
impl<'a> /*trait*/ QFont_resolve<u32> for () {
  fn resolve(self, rsthis: &mut QFont) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7resolveEv()};
    let mut ret = unsafe {_ZNK5QFont7resolveEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn cleanup<RetType, T: QFont_cleanup<RetType>>(&mut self, value: T) -> RetType {
    return value.cleanup(self);
    // return 1;
  }
}

pub trait QFont_cleanup<RetType> {
  fn cleanup(self, rsthis: &mut QFont) -> RetType;
}

// proto: static void QFont::cleanup();
impl<'a> /*trait*/ QFont_cleanup<()> for () {
  fn cleanup(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont7cleanupEv()};
     unsafe {_ZN5QFont7cleanupEv()};
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn exactMatch<RetType, T: QFont_exactMatch<RetType>>(&mut self, value: T) -> RetType {
    return value.exactMatch(self);
    // return 1;
  }
}

pub trait QFont_exactMatch<RetType> {
  fn exactMatch(self, rsthis: &mut QFont) -> RetType;
}

// proto:  bool QFont::exactMatch();
impl<'a> /*trait*/ QFont_exactMatch<i8> for () {
  fn exactMatch(self, rsthis: &mut QFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont10exactMatchEv()};
    let mut ret = unsafe {_ZNK5QFont10exactMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn stretch<RetType, T: QFont_stretch<RetType>>(&mut self, value: T) -> RetType {
    return value.stretch(self);
    // return 1;
  }
}

pub trait QFont_stretch<RetType> {
  fn stretch(self, rsthis: &mut QFont) -> RetType;
}

// proto:  int QFont::stretch();
impl<'a> /*trait*/ QFont_stretch<i32> for () {
  fn stretch(self, rsthis: &mut QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFont7stretchEv()};
    let mut ret = unsafe {_ZNK5QFont7stretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFont {
  pub fn setRawName<RetType, T: QFont_setRawName<RetType>>(&mut self, value: T) -> RetType {
    return value.setRawName(self);
    // return 1;
  }
}

pub trait QFont_setRawName<RetType> {
  fn setRawName(self, rsthis: &mut QFont) -> RetType;
}

// proto:  void QFont::setRawName(const QString & );
impl<'a> /*trait*/ QFont_setRawName<()> for (&'a  QString) {
  fn setRawName(self, rsthis: &mut QFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFont10setRawNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFont10setRawNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

