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
use super::qstringlist::QStringList;
use super::qbytearray::QByteArray;
use super::qfontinfo::QFontInfo;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
  fn _ZN13QFontDatabase10pointSizesERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QFontDatabase::styleString(const QFont & font);
  fn _ZN13QFontDatabase11styleStringERK5QFont(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
  fn _ZN13QFontDatabase11smoothSizesERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QStringList QFontDatabase::styles(const QString & family);
  fn _ZNK13QFontDatabase6stylesERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFontDatabase::italic(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase6italicERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  void QFontDatabase::NewQFontDatabase();
  fn _ZN13QFontDatabaseC1Ev(qthis: *mut c_void) ;
  // proto: static QStringList QFontDatabase::applicationFontFamilies(int id);
  fn _ZN13QFontDatabase23applicationFontFamiliesEi(arg0: c_int) -> *mut c_void;
  // proto:  bool QFontDatabase::hasFamily(const QString & family);
  fn _ZNK13QFontDatabase9hasFamilyERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QFontDatabase::isFixedPitch(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase12isFixedPitchERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  QFont QFontDatabase::font(const QString & family, const QString & style, int pointSize);
  fn _ZNK13QFontDatabase4fontERK7QStringS2_i(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  // proto:  int QFontDatabase::weight(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase6weightERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto: static bool QFontDatabase::removeAllApplicationFonts();
  fn _ZN13QFontDatabase25removeAllApplicationFontsEv() -> int8_t;
  // proto: static int QFontDatabase::addApplicationFontFromData(const QByteArray & fontData);
  fn _ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray(arg0: *mut c_void) -> c_int;
  // proto: static bool QFontDatabase::supportsThreadedFontRendering();
  fn _ZN13QFontDatabase29supportsThreadedFontRenderingEv() -> int8_t;
  // proto:  bool QFontDatabase::isPrivateFamily(const QString & family);
  fn _ZNK13QFontDatabase15isPrivateFamilyERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QFontDatabase::isScalable(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase10isScalableERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static bool QFontDatabase::removeApplicationFont(int id);
  fn _ZN13QFontDatabase21removeApplicationFontEi(arg0: c_int) -> int8_t;
  // proto:  QString QFontDatabase::styleString(const QFontInfo & fontInfo);
  fn _ZN13QFontDatabase11styleStringERK9QFontInfo(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFontDatabase::isBitmapScalable(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  bool QFontDatabase::isSmoothlyScalable(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  bool QFontDatabase::bold(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase4boldERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static int QFontDatabase::addApplicationFont(const QString & fileName);
  fn _ZN13QFontDatabase18addApplicationFontERK7QString(arg0: *mut c_void) -> c_int;
  // proto: static QList<int> QFontDatabase::standardSizes();
  fn _ZN13QFontDatabase13standardSizesEv() ;
}

// body block begin
// class sizeof(QFontDatabase)=8
pub struct QFontDatabase {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontDatabase {
  pub fn pointSizes<T: QFontDatabase_pointSizes>(&mut self, value: T)  {
     value.pointSizes(self);
    // return 1;
  }
}

pub trait QFontDatabase_pointSizes {
  fn pointSizes(self, rsthis: &mut QFontDatabase) ;
}

// proto:  QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_pointSizes for (&'a  QString, &'a  QString) {
  fn pointSizes(self, rsthis: &mut QFontDatabase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase10pointSizesERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontDatabase10pointSizesERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn styleString<T: QFontDatabase_styleString>(&mut self, value: T) -> QString {
    return value.styleString(self);
    // return 1;
  }
}

pub trait QFontDatabase_styleString {
  fn styleString(self, rsthis: &mut QFontDatabase) -> QString;
}

// proto:  QString QFontDatabase::styleString(const QFont & font);
impl<'a> /*trait*/ QFontDatabase_styleString for (&'a  QFont) {
  fn styleString(self, rsthis: &mut QFontDatabase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11styleStringERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QFontDatabase11styleStringERK5QFont(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn smoothSizes<T: QFontDatabase_smoothSizes>(&mut self, value: T)  {
     value.smoothSizes(self);
    // return 1;
  }
}

pub trait QFontDatabase_smoothSizes {
  fn smoothSizes(self, rsthis: &mut QFontDatabase) ;
}

// proto:  QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_smoothSizes for (&'a  QString, &'a  QString) {
  fn smoothSizes(self, rsthis: &mut QFontDatabase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11smoothSizesERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontDatabase11smoothSizesERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn styles<T: QFontDatabase_styles>(&mut self, value: T) -> QStringList {
    return value.styles(self);
    // return 1;
  }
}

pub trait QFontDatabase_styles {
  fn styles(self, rsthis: &mut QFontDatabase) -> QStringList;
}

// proto:  QStringList QFontDatabase::styles(const QString & family);
impl<'a> /*trait*/ QFontDatabase_styles for (&'a  QString) {
  fn styles(self, rsthis: &mut QFontDatabase) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6stylesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase6stylesERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn italic<T: QFontDatabase_italic>(&mut self, value: T) -> i8 {
    return value.italic(self);
    // return 1;
  }
}

pub trait QFontDatabase_italic {
  fn italic(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto:  bool QFontDatabase::italic(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_italic for (&'a  QString, &'a  QString) {
  fn italic(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6italicERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase6italicERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
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
  pub fn applicationFontFamilies<T: QFontDatabase_applicationFontFamilies>(&mut self, value: T) -> QStringList {
    return value.applicationFontFamilies(self);
    // return 1;
  }
}

pub trait QFontDatabase_applicationFontFamilies {
  fn applicationFontFamilies(self, rsthis: &mut QFontDatabase) -> QStringList;
}

// proto: static QStringList QFontDatabase::applicationFontFamilies(int id);
impl<'a> /*trait*/ QFontDatabase_applicationFontFamilies for (i32) {
  fn applicationFontFamilies(self, rsthis: &mut QFontDatabase) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase23applicationFontFamiliesEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN13QFontDatabase23applicationFontFamiliesEi(arg0)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn hasFamily<T: QFontDatabase_hasFamily>(&mut self, value: T) -> i8 {
    return value.hasFamily(self);
    // return 1;
  }
}

pub trait QFontDatabase_hasFamily {
  fn hasFamily(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto:  bool QFontDatabase::hasFamily(const QString & family);
impl<'a> /*trait*/ QFontDatabase_hasFamily for (&'a  QString) {
  fn hasFamily(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase9hasFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase9hasFamilyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isFixedPitch<T: QFontDatabase_isFixedPitch>(&mut self, value: T) -> i8 {
    return value.isFixedPitch(self);
    // return 1;
  }
}

pub trait QFontDatabase_isFixedPitch {
  fn isFixedPitch(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto:  bool QFontDatabase::isFixedPitch(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isFixedPitch for (&'a  QString, &'a  QString) {
  fn isFixedPitch(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase12isFixedPitchERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase12isFixedPitchERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn font<T: QFontDatabase_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QFontDatabase_font {
  fn font(self, rsthis: &mut QFontDatabase) -> QFont;
}

// proto:  QFont QFontDatabase::font(const QString & family, const QString & style, int pointSize);
impl<'a> /*trait*/ QFontDatabase_font for (&'a  QString, &'a  QString, i32) {
  fn font(self, rsthis: &mut QFontDatabase) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase4fontERK7QStringS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK13QFontDatabase4fontERK7QStringS2_i(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn weight<T: QFontDatabase_weight>(&mut self, value: T) -> i32 {
    return value.weight(self);
    // return 1;
  }
}

pub trait QFontDatabase_weight {
  fn weight(self, rsthis: &mut QFontDatabase) -> i32;
}

// proto:  int QFontDatabase::weight(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_weight for (&'a  QString, &'a  QString) {
  fn weight(self, rsthis: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6weightERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase6weightERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn removeAllApplicationFonts<T: QFontDatabase_removeAllApplicationFonts>(&mut self, value: T) -> i8 {
    return value.removeAllApplicationFonts(self);
    // return 1;
  }
}

pub trait QFontDatabase_removeAllApplicationFonts {
  fn removeAllApplicationFonts(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto: static bool QFontDatabase::removeAllApplicationFonts();
impl<'a> /*trait*/ QFontDatabase_removeAllApplicationFonts for () {
  fn removeAllApplicationFonts(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase25removeAllApplicationFontsEv()};
    let mut ret = unsafe {_ZN13QFontDatabase25removeAllApplicationFontsEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn addApplicationFontFromData<T: QFontDatabase_addApplicationFontFromData>(&mut self, value: T) -> i32 {
    return value.addApplicationFontFromData(self);
    // return 1;
  }
}

pub trait QFontDatabase_addApplicationFontFromData {
  fn addApplicationFontFromData(self, rsthis: &mut QFontDatabase) -> i32;
}

// proto: static int QFontDatabase::addApplicationFontFromData(const QByteArray & fontData);
impl<'a> /*trait*/ QFontDatabase_addApplicationFontFromData for (&'a  QByteArray) {
  fn addApplicationFontFromData(self, rsthis: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray(arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn supportsThreadedFontRendering<T: QFontDatabase_supportsThreadedFontRendering>(&mut self, value: T) -> i8 {
    return value.supportsThreadedFontRendering(self);
    // return 1;
  }
}

pub trait QFontDatabase_supportsThreadedFontRendering {
  fn supportsThreadedFontRendering(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto: static bool QFontDatabase::supportsThreadedFontRendering();
impl<'a> /*trait*/ QFontDatabase_supportsThreadedFontRendering for () {
  fn supportsThreadedFontRendering(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase29supportsThreadedFontRenderingEv()};
    let mut ret = unsafe {_ZN13QFontDatabase29supportsThreadedFontRenderingEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isPrivateFamily<T: QFontDatabase_isPrivateFamily>(&mut self, value: T) -> i8 {
    return value.isPrivateFamily(self);
    // return 1;
  }
}

pub trait QFontDatabase_isPrivateFamily {
  fn isPrivateFamily(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto:  bool QFontDatabase::isPrivateFamily(const QString & family);
impl<'a> /*trait*/ QFontDatabase_isPrivateFamily for (&'a  QString) {
  fn isPrivateFamily(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase15isPrivateFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase15isPrivateFamilyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isScalable<T: QFontDatabase_isScalable>(&mut self, value: T) -> i8 {
    return value.isScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isScalable {
  fn isScalable(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto:  bool QFontDatabase::isScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isScalable for (&'a  QString, &'a  QString) {
  fn isScalable(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase10isScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase10isScalableERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn removeApplicationFont<T: QFontDatabase_removeApplicationFont>(&mut self, value: T) -> i8 {
    return value.removeApplicationFont(self);
    // return 1;
  }
}

pub trait QFontDatabase_removeApplicationFont {
  fn removeApplicationFont(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto: static bool QFontDatabase::removeApplicationFont(int id);
impl<'a> /*trait*/ QFontDatabase_removeApplicationFont for (i32) {
  fn removeApplicationFont(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase21removeApplicationFontEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN13QFontDatabase21removeApplicationFontEi(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QFontDatabase::styleString(const QFontInfo & fontInfo);
impl<'a> /*trait*/ QFontDatabase_styleString for (&'a  QFontInfo) {
  fn styleString(self, rsthis: &mut QFontDatabase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11styleStringERK9QFontInfo()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QFontDatabase11styleStringERK9QFontInfo(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isBitmapScalable<T: QFontDatabase_isBitmapScalable>(&mut self, value: T) -> i8 {
    return value.isBitmapScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isBitmapScalable {
  fn isBitmapScalable(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto:  bool QFontDatabase::isBitmapScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isBitmapScalable for (&'a  QString, &'a  QString) {
  fn isBitmapScalable(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isSmoothlyScalable<T: QFontDatabase_isSmoothlyScalable>(&mut self, value: T) -> i8 {
    return value.isSmoothlyScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isSmoothlyScalable {
  fn isSmoothlyScalable(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto:  bool QFontDatabase::isSmoothlyScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isSmoothlyScalable for (&'a  QString, &'a  QString) {
  fn isSmoothlyScalable(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn bold<T: QFontDatabase_bold>(&mut self, value: T) -> i8 {
    return value.bold(self);
    // return 1;
  }
}

pub trait QFontDatabase_bold {
  fn bold(self, rsthis: &mut QFontDatabase) -> i8;
}

// proto:  bool QFontDatabase::bold(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_bold for (&'a  QString, &'a  QString) {
  fn bold(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase4boldERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontDatabase4boldERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn addApplicationFont<T: QFontDatabase_addApplicationFont>(&mut self, value: T) -> i32 {
    return value.addApplicationFont(self);
    // return 1;
  }
}

pub trait QFontDatabase_addApplicationFont {
  fn addApplicationFont(self, rsthis: &mut QFontDatabase) -> i32;
}

// proto: static int QFontDatabase::addApplicationFont(const QString & fileName);
impl<'a> /*trait*/ QFontDatabase_addApplicationFont for (&'a  QString) {
  fn addApplicationFont(self, rsthis: &mut QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase18addApplicationFontERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QFontDatabase18addApplicationFontERK7QString(arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn standardSizes<T: QFontDatabase_standardSizes>(&mut self, value: T)  {
     value.standardSizes(self);
    // return 1;
  }
}

pub trait QFontDatabase_standardSizes {
  fn standardSizes(self, rsthis: &mut QFontDatabase) ;
}

// proto: static QList<int> QFontDatabase::standardSizes();
impl<'a> /*trait*/ QFontDatabase_standardSizes for () {
  fn standardSizes(self, rsthis: &mut QFontDatabase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase13standardSizesEv()};
     unsafe {_ZN13QFontDatabase13standardSizesEv()};
    // return 1;
  }
}

