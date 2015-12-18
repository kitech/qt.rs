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
  // proto:  QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
  fn _ZN13QFontDatabase10pointSizesERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QFontDatabase::styleString(const QFont & font);
  fn _ZN13QFontDatabase11styleStringERK5QFont(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
  fn _ZN13QFontDatabase11smoothSizesERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QStringList QFontDatabase::styles(const QString & family);
  fn _ZNK13QFontDatabase6stylesERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFontDatabase::italic(const QString & family, const QString & style);
  fn _ZNK13QFontDatabase6italicERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  void QFontDatabase::NewQFontDatabase();
  fn _ZN13QFontDatabaseC1Ev(qthis: *mut c_void) ;
  // proto: static QStringList QFontDatabase::applicationFontFamilies(int id);
  fn _ZN13QFontDatabase23applicationFontFamiliesEi(arg0: c_int) ;
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
  pub fn pointSizes<RetType, T: QFontDatabase_pointSizes<RetType>>(&mut self, value: T) -> RetType {
    return value.pointSizes(self);
    // return 1;
  }
}

pub trait QFontDatabase_pointSizes<RetType> {
  fn pointSizes(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_pointSizes<()> for (&'a  QString, &'a  QString) {
  fn pointSizes(self, rsthis: &mut QFontDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase10pointSizesERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontDatabase10pointSizesERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn styleString<RetType, T: QFontDatabase_styleString<RetType>>(&mut self, value: T) -> RetType {
    return value.styleString(self);
    // return 1;
  }
}

pub trait QFontDatabase_styleString<RetType> {
  fn styleString(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  QString QFontDatabase::styleString(const QFont & font);
impl<'a> /*trait*/ QFontDatabase_styleString<QString> for (&'a  QFont) {
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
  pub fn smoothSizes<RetType, T: QFontDatabase_smoothSizes<RetType>>(&mut self, value: T) -> RetType {
    return value.smoothSizes(self);
    // return 1;
  }
}

pub trait QFontDatabase_smoothSizes<RetType> {
  fn smoothSizes(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_smoothSizes<()> for (&'a  QString, &'a  QString) {
  fn smoothSizes(self, rsthis: &mut QFontDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11smoothSizesERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontDatabase11smoothSizesERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn styles<RetType, T: QFontDatabase_styles<RetType>>(&mut self, value: T) -> RetType {
    return value.styles(self);
    // return 1;
  }
}

pub trait QFontDatabase_styles<RetType> {
  fn styles(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  QStringList QFontDatabase::styles(const QString & family);
impl<'a> /*trait*/ QFontDatabase_styles<()> for (&'a  QString) {
  fn styles(self, rsthis: &mut QFontDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6stylesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QFontDatabase6stylesERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn italic<RetType, T: QFontDatabase_italic<RetType>>(&mut self, value: T) -> RetType {
    return value.italic(self);
    // return 1;
  }
}

pub trait QFontDatabase_italic<RetType> {
  fn italic(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  bool QFontDatabase::italic(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_italic<i8> for (&'a  QString, &'a  QString) {
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
  pub fn applicationFontFamilies<RetType, T: QFontDatabase_applicationFontFamilies<RetType>>(&mut self, value: T) -> RetType {
    return value.applicationFontFamilies(self);
    // return 1;
  }
}

pub trait QFontDatabase_applicationFontFamilies<RetType> {
  fn applicationFontFamilies(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto: static QStringList QFontDatabase::applicationFontFamilies(int id);
impl<'a> /*trait*/ QFontDatabase_applicationFontFamilies<()> for (i32) {
  fn applicationFontFamilies(self, rsthis: &mut QFontDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase23applicationFontFamiliesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QFontDatabase23applicationFontFamiliesEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn hasFamily<RetType, T: QFontDatabase_hasFamily<RetType>>(&mut self, value: T) -> RetType {
    return value.hasFamily(self);
    // return 1;
  }
}

pub trait QFontDatabase_hasFamily<RetType> {
  fn hasFamily(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  bool QFontDatabase::hasFamily(const QString & family);
impl<'a> /*trait*/ QFontDatabase_hasFamily<i8> for (&'a  QString) {
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
  pub fn isFixedPitch<RetType, T: QFontDatabase_isFixedPitch<RetType>>(&mut self, value: T) -> RetType {
    return value.isFixedPitch(self);
    // return 1;
  }
}

pub trait QFontDatabase_isFixedPitch<RetType> {
  fn isFixedPitch(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  bool QFontDatabase::isFixedPitch(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isFixedPitch<i8> for (&'a  QString, &'a  QString) {
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
  pub fn font<RetType, T: QFontDatabase_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QFontDatabase_font<RetType> {
  fn font(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  QFont QFontDatabase::font(const QString & family, const QString & style, int pointSize);
impl<'a> /*trait*/ QFontDatabase_font<QFont> for (&'a  QString, &'a  QString, i32) {
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
  pub fn weight<RetType, T: QFontDatabase_weight<RetType>>(&mut self, value: T) -> RetType {
    return value.weight(self);
    // return 1;
  }
}

pub trait QFontDatabase_weight<RetType> {
  fn weight(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  int QFontDatabase::weight(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_weight<i32> for (&'a  QString, &'a  QString) {
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
  pub fn removeAllApplicationFonts<RetType, T: QFontDatabase_removeAllApplicationFonts<RetType>>(&mut self, value: T) -> RetType {
    return value.removeAllApplicationFonts(self);
    // return 1;
  }
}

pub trait QFontDatabase_removeAllApplicationFonts<RetType> {
  fn removeAllApplicationFonts(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto: static bool QFontDatabase::removeAllApplicationFonts();
impl<'a> /*trait*/ QFontDatabase_removeAllApplicationFonts<i8> for () {
  fn removeAllApplicationFonts(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase25removeAllApplicationFontsEv()};
    let mut ret = unsafe {_ZN13QFontDatabase25removeAllApplicationFontsEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn addApplicationFontFromData<RetType, T: QFontDatabase_addApplicationFontFromData<RetType>>(&mut self, value: T) -> RetType {
    return value.addApplicationFontFromData(self);
    // return 1;
  }
}

pub trait QFontDatabase_addApplicationFontFromData<RetType> {
  fn addApplicationFontFromData(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto: static int QFontDatabase::addApplicationFontFromData(const QByteArray & fontData);
impl<'a> /*trait*/ QFontDatabase_addApplicationFontFromData<i32> for (&'a  QByteArray) {
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
  pub fn supportsThreadedFontRendering<RetType, T: QFontDatabase_supportsThreadedFontRendering<RetType>>(&mut self, value: T) -> RetType {
    return value.supportsThreadedFontRendering(self);
    // return 1;
  }
}

pub trait QFontDatabase_supportsThreadedFontRendering<RetType> {
  fn supportsThreadedFontRendering(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto: static bool QFontDatabase::supportsThreadedFontRendering();
impl<'a> /*trait*/ QFontDatabase_supportsThreadedFontRendering<i8> for () {
  fn supportsThreadedFontRendering(self, rsthis: &mut QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase29supportsThreadedFontRenderingEv()};
    let mut ret = unsafe {_ZN13QFontDatabase29supportsThreadedFontRenderingEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontDatabase {
  pub fn isPrivateFamily<RetType, T: QFontDatabase_isPrivateFamily<RetType>>(&mut self, value: T) -> RetType {
    return value.isPrivateFamily(self);
    // return 1;
  }
}

pub trait QFontDatabase_isPrivateFamily<RetType> {
  fn isPrivateFamily(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  bool QFontDatabase::isPrivateFamily(const QString & family);
impl<'a> /*trait*/ QFontDatabase_isPrivateFamily<i8> for (&'a  QString) {
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
  pub fn isScalable<RetType, T: QFontDatabase_isScalable<RetType>>(&mut self, value: T) -> RetType {
    return value.isScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isScalable<RetType> {
  fn isScalable(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  bool QFontDatabase::isScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isScalable<i8> for (&'a  QString, &'a  QString) {
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
  pub fn removeApplicationFont<RetType, T: QFontDatabase_removeApplicationFont<RetType>>(&mut self, value: T) -> RetType {
    return value.removeApplicationFont(self);
    // return 1;
  }
}

pub trait QFontDatabase_removeApplicationFont<RetType> {
  fn removeApplicationFont(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto: static bool QFontDatabase::removeApplicationFont(int id);
impl<'a> /*trait*/ QFontDatabase_removeApplicationFont<i8> for (i32) {
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
impl<'a> /*trait*/ QFontDatabase_styleString<QString> for (&'a  QFontInfo) {
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
  pub fn isBitmapScalable<RetType, T: QFontDatabase_isBitmapScalable<RetType>>(&mut self, value: T) -> RetType {
    return value.isBitmapScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isBitmapScalable<RetType> {
  fn isBitmapScalable(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  bool QFontDatabase::isBitmapScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isBitmapScalable<i8> for (&'a  QString, &'a  QString) {
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
  pub fn isSmoothlyScalable<RetType, T: QFontDatabase_isSmoothlyScalable<RetType>>(&mut self, value: T) -> RetType {
    return value.isSmoothlyScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isSmoothlyScalable<RetType> {
  fn isSmoothlyScalable(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  bool QFontDatabase::isSmoothlyScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isSmoothlyScalable<i8> for (&'a  QString, &'a  QString) {
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
  pub fn bold<RetType, T: QFontDatabase_bold<RetType>>(&mut self, value: T) -> RetType {
    return value.bold(self);
    // return 1;
  }
}

pub trait QFontDatabase_bold<RetType> {
  fn bold(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto:  bool QFontDatabase::bold(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_bold<i8> for (&'a  QString, &'a  QString) {
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
  pub fn addApplicationFont<RetType, T: QFontDatabase_addApplicationFont<RetType>>(&mut self, value: T) -> RetType {
    return value.addApplicationFont(self);
    // return 1;
  }
}

pub trait QFontDatabase_addApplicationFont<RetType> {
  fn addApplicationFont(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto: static int QFontDatabase::addApplicationFont(const QString & fileName);
impl<'a> /*trait*/ QFontDatabase_addApplicationFont<i32> for (&'a  QString) {
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
  pub fn standardSizes<RetType, T: QFontDatabase_standardSizes<RetType>>(&mut self, value: T) -> RetType {
    return value.standardSizes(self);
    // return 1;
  }
}

pub trait QFontDatabase_standardSizes<RetType> {
  fn standardSizes(self, rsthis: &mut QFontDatabase) -> RetType;
}

// proto: static QList<int> QFontDatabase::standardSizes();
impl<'a> /*trait*/ QFontDatabase_standardSizes<()> for () {
  fn standardSizes(self, rsthis: &mut QFontDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase13standardSizesEv()};
     unsafe {_ZN13QFontDatabase13standardSizesEv()};
    // return 1;
  }
}

