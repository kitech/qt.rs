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
use super::qbytearray::QByteArray;
use super::qfontinfo::QFontInfo;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
  fn _ZN13QFontDatabase10pointSizesERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QString QFontDatabase::styleString(const QFont & font);
  fn _ZN13QFontDatabase11styleStringERK5QFont(arg0: *const c_void) -> i32;
  // proto: QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
  fn _ZN13QFontDatabase11smoothSizesERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QStringList QFontDatabase::styles(const QString & family);
  fn _ZNK13QFontDatabase6stylesERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QFontDatabase::italic(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase6italicERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QFontDatabase::NewQFontDatabase();
  fn _ZN13QFontDatabaseC1Ev(qthis: *mut c_void) -> i32;
  // proto: QStringList QFontDatabase::applicationFontFamilies(int id);
  fn _ZN13QFontDatabase23applicationFontFamiliesEi(arg0: c_int) -> i32;
  // proto: bool QFontDatabase::hasFamily(const QString & family);
  fn _ZNK13QFontDatabase9hasFamilyERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QFontDatabase::isFixedPitch(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase12isFixedPitchERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QFont QFontDatabase::font(const QString & family, const QString & style, int pointSize);
  fn _ZNK13QFontDatabase4fontERK7QStringS2_i(arg0: *const c_void, arg1: *const c_void, arg2: c_int) -> i32;
  // proto: int QFontDatabase::weight(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase6weightERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QFontDatabase::removeAllApplicationFonts();
  fn _ZN13QFontDatabase25removeAllApplicationFontsEv() -> i32;
  // proto: int QFontDatabase::addApplicationFontFromData(const QByteArray & fontData);
  fn _ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: bool QFontDatabase::supportsThreadedFontRendering();
  fn _ZN13QFontDatabase29supportsThreadedFontRenderingEv() -> i32;
  // proto: bool QFontDatabase::isPrivateFamily(const QString & family);
  fn _ZNK13QFontDatabase15isPrivateFamilyERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QFontDatabase::isScalable(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase10isScalableERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QFontDatabase::removeApplicationFont(int id);
  fn _ZN13QFontDatabase21removeApplicationFontEi(arg0: c_int) -> i32;
  // proto: QString QFontDatabase::styleString(const QFontInfo & fontInfo);
  fn _ZN13QFontDatabase11styleStringERK9QFontInfo(arg0: *const c_void) -> i32;
  // proto: bool QFontDatabase::isBitmapScalable(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QFontDatabase::isSmoothlyScalable(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QFontDatabase::bold(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase4boldERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: int QFontDatabase::addApplicationFont(const QString & fileName);
  fn _ZN13QFontDatabase18addApplicationFontERK7QString(arg0: *const c_void) -> i32;
  // proto: QList<int> QFontDatabase::standardSizes();
  fn _ZN13QFontDatabase13standardSizesEv() -> i32;
}

// body block begin
// class sizeof(QFontDatabase)=8
pub struct QFontDatabase {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontDatabase {
  pub fn pointSizes<T: QFontDatabase_pointSizes>(&mut self, value: T) -> i32 {
    value.pointSizes(self);
    return 1;
  }
}

pub trait QFontDatabase_pointSizes {
  fn pointSizes(self, this: &mut QFontDatabase) -> i32;
}

// proto: QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_pointSizes for (&'a  QString, &'a  QString) {
  fn pointSizes(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase10pointSizesERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QFontDatabase10pointSizesERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn styleString<T: QFontDatabase_styleString>(&mut self, value: T) -> i32 {
    value.styleString(self);
    return 1;
  }
}

pub trait QFontDatabase_styleString {
  fn styleString(self, this: &mut QFontDatabase) -> i32;
}

// proto: QString QFontDatabase::styleString(const QFont & font);
impl<'a> /*trait*/ QFontDatabase_styleString for (&'a  QFont) {
  fn styleString(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11styleStringERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontDatabase11styleStringERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn smoothSizes<T: QFontDatabase_smoothSizes>(&mut self, value: T) -> i32 {
    value.smoothSizes(self);
    return 1;
  }
}

pub trait QFontDatabase_smoothSizes {
  fn smoothSizes(self, this: &mut QFontDatabase) -> i32;
}

// proto: QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_smoothSizes for (&'a  QString, &'a  QString) {
  fn smoothSizes(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11smoothSizesERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QFontDatabase11smoothSizesERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn styles<T: QFontDatabase_styles>(&mut self, value: T) -> i32 {
    value.styles(self);
    return 1;
  }
}

pub trait QFontDatabase_styles {
  fn styles(self, this: &mut QFontDatabase) -> i32;
}

// proto: QStringList QFontDatabase::styles(const QString & family);
impl<'a> /*trait*/ QFontDatabase_styles for (&'a  QString) {
  fn styles(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6stylesERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase6stylesERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn italic<T: QFontDatabase_italic>(&mut self, value: T) -> i32 {
    value.italic(self);
    return 1;
  }
}

pub trait QFontDatabase_italic {
  fn italic(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::italic(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_italic for (&'a  QString, &'a  QString) {
  fn italic(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6italicERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase6italicERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn NewQFontDatabase<T: QFontDatabase_NewQFontDatabase>(value: T) -> QFontDatabase {
    let rsthis = value.NewQFontDatabase();
    return rsthis;
    // return 1;
  }
}

pub trait QFontDatabase_NewQFontDatabase {
  fn NewQFontDatabase(self) -> QFontDatabase;
}

// proto: void QFontDatabase::NewQFontDatabase();
impl<'a> /*trait*/ QFontDatabase_NewQFontDatabase for () {
  fn NewQFontDatabase(self) -> QFontDatabase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabaseC1Ev()};
    unsafe {_ZN13QFontDatabaseC1Ev(qthis)};
    let rsthis = QFontDatabase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn applicationFontFamilies<T: QFontDatabase_applicationFontFamilies>(&mut self, value: T) -> i32 {
    value.applicationFontFamilies(self);
    return 1;
  }
}

pub trait QFontDatabase_applicationFontFamilies {
  fn applicationFontFamilies(self, this: &mut QFontDatabase) -> i32;
}

// proto: QStringList QFontDatabase::applicationFontFamilies(int id);
impl<'a> /*trait*/ QFontDatabase_applicationFontFamilies for (i32) {
  fn applicationFontFamilies(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase23applicationFontFamiliesEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QFontDatabase23applicationFontFamiliesEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn hasFamily<T: QFontDatabase_hasFamily>(&mut self, value: T) -> i32 {
    value.hasFamily(self);
    return 1;
  }
}

pub trait QFontDatabase_hasFamily {
  fn hasFamily(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::hasFamily(const QString & family);
impl<'a> /*trait*/ QFontDatabase_hasFamily for (&'a  QString) {
  fn hasFamily(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase9hasFamilyERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase9hasFamilyERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isFixedPitch<T: QFontDatabase_isFixedPitch>(&mut self, value: T) -> i32 {
    value.isFixedPitch(self);
    return 1;
  }
}

pub trait QFontDatabase_isFixedPitch {
  fn isFixedPitch(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::isFixedPitch(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isFixedPitch for (&'a  QString, &'a  QString) {
  fn isFixedPitch(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase12isFixedPitchERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase12isFixedPitchERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn font<T: QFontDatabase_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QFontDatabase_font {
  fn font(self, this: &mut QFontDatabase) -> i32;
}

// proto: QFont QFontDatabase::font(const QString & family, const QString & style, int pointSize);
impl<'a> /*trait*/ QFontDatabase_font for (&'a  QString, &'a  QString, i32) {
  fn font(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase4fontERK7QStringS2_i()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK13QFontDatabase4fontERK7QStringS2_i(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn weight<T: QFontDatabase_weight>(&mut self, value: T) -> i32 {
    value.weight(self);
    return 1;
  }
}

pub trait QFontDatabase_weight {
  fn weight(self, this: &mut QFontDatabase) -> i32;
}

// proto: int QFontDatabase::weight(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_weight for (&'a  QString, &'a  QString) {
  fn weight(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6weightERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase6weightERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn removeAllApplicationFonts<T: QFontDatabase_removeAllApplicationFonts>(&mut self, value: T) -> i32 {
    value.removeAllApplicationFonts(self);
    return 1;
  }
}

pub trait QFontDatabase_removeAllApplicationFonts {
  fn removeAllApplicationFonts(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::removeAllApplicationFonts();
impl<'a> /*trait*/ QFontDatabase_removeAllApplicationFonts for () {
  fn removeAllApplicationFonts(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase25removeAllApplicationFontsEv()};
    unsafe {_ZN13QFontDatabase25removeAllApplicationFontsEv()};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn addApplicationFontFromData<T: QFontDatabase_addApplicationFontFromData>(&mut self, value: T) -> i32 {
    value.addApplicationFontFromData(self);
    return 1;
  }
}

pub trait QFontDatabase_addApplicationFontFromData {
  fn addApplicationFontFromData(self, this: &mut QFontDatabase) -> i32;
}

// proto: int QFontDatabase::addApplicationFontFromData(const QByteArray & fontData);
impl<'a> /*trait*/ QFontDatabase_addApplicationFontFromData for (&'a  QByteArray) {
  fn addApplicationFontFromData(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn supportsThreadedFontRendering<T: QFontDatabase_supportsThreadedFontRendering>(&mut self, value: T) -> i32 {
    value.supportsThreadedFontRendering(self);
    return 1;
  }
}

pub trait QFontDatabase_supportsThreadedFontRendering {
  fn supportsThreadedFontRendering(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::supportsThreadedFontRendering();
impl<'a> /*trait*/ QFontDatabase_supportsThreadedFontRendering for () {
  fn supportsThreadedFontRendering(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase29supportsThreadedFontRenderingEv()};
    unsafe {_ZN13QFontDatabase29supportsThreadedFontRenderingEv()};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isPrivateFamily<T: QFontDatabase_isPrivateFamily>(&mut self, value: T) -> i32 {
    value.isPrivateFamily(self);
    return 1;
  }
}

pub trait QFontDatabase_isPrivateFamily {
  fn isPrivateFamily(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::isPrivateFamily(const QString & family);
impl<'a> /*trait*/ QFontDatabase_isPrivateFamily for (&'a  QString) {
  fn isPrivateFamily(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase15isPrivateFamilyERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase15isPrivateFamilyERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isScalable<T: QFontDatabase_isScalable>(&mut self, value: T) -> i32 {
    value.isScalable(self);
    return 1;
  }
}

pub trait QFontDatabase_isScalable {
  fn isScalable(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::isScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isScalable for (&'a  QString, &'a  QString) {
  fn isScalable(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase10isScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase10isScalableERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn removeApplicationFont<T: QFontDatabase_removeApplicationFont>(&mut self, value: T) -> i32 {
    value.removeApplicationFont(self);
    return 1;
  }
}

pub trait QFontDatabase_removeApplicationFont {
  fn removeApplicationFont(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::removeApplicationFont(int id);
impl<'a> /*trait*/ QFontDatabase_removeApplicationFont for (i32) {
  fn removeApplicationFont(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase21removeApplicationFontEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QFontDatabase21removeApplicationFontEi(arg0)};
    return 1;
  }
}

// proto: QString QFontDatabase::styleString(const QFontInfo & fontInfo);
impl<'a> /*trait*/ QFontDatabase_styleString for (&'a  QFontInfo) {
  fn styleString(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11styleStringERK9QFontInfo()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontDatabase11styleStringERK9QFontInfo(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isBitmapScalable<T: QFontDatabase_isBitmapScalable>(&mut self, value: T) -> i32 {
    value.isBitmapScalable(self);
    return 1;
  }
}

pub trait QFontDatabase_isBitmapScalable {
  fn isBitmapScalable(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::isBitmapScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isBitmapScalable for (&'a  QString, &'a  QString) {
  fn isBitmapScalable(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isSmoothlyScalable<T: QFontDatabase_isSmoothlyScalable>(&mut self, value: T) -> i32 {
    value.isSmoothlyScalable(self);
    return 1;
  }
}

pub trait QFontDatabase_isSmoothlyScalable {
  fn isSmoothlyScalable(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::isSmoothlyScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isSmoothlyScalable for (&'a  QString, &'a  QString) {
  fn isSmoothlyScalable(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn bold<T: QFontDatabase_bold>(&mut self, value: T) -> i32 {
    value.bold(self);
    return 1;
  }
}

pub trait QFontDatabase_bold {
  fn bold(self, this: &mut QFontDatabase) -> i32;
}

// proto: bool QFontDatabase::bold(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_bold for (&'a  QString, &'a  QString) {
  fn bold(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase4boldERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontDatabase4boldERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn addApplicationFont<T: QFontDatabase_addApplicationFont>(&mut self, value: T) -> i32 {
    value.addApplicationFont(self);
    return 1;
  }
}

pub trait QFontDatabase_addApplicationFont {
  fn addApplicationFont(self, this: &mut QFontDatabase) -> i32;
}

// proto: int QFontDatabase::addApplicationFont(const QString & fileName);
impl<'a> /*trait*/ QFontDatabase_addApplicationFont for (&'a  QString) {
  fn addApplicationFont(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase18addApplicationFontERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontDatabase18addApplicationFontERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn standardSizes<T: QFontDatabase_standardSizes>(&mut self, value: T) -> i32 {
    value.standardSizes(self);
    return 1;
  }
}

pub trait QFontDatabase_standardSizes {
  fn standardSizes(self, this: &mut QFontDatabase) -> i32;
}

// proto: QList<int> QFontDatabase::standardSizes();
impl<'a> /*trait*/ QFontDatabase_standardSizes for () {
  fn standardSizes(self, this: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase13standardSizesEv()};
    unsafe {_ZN13QFontDatabase13standardSizesEv()};
    return 1;
  }
}

