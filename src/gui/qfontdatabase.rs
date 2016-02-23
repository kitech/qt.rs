// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qfontdatabase.h
// dst-file: /src/gui/qfontdatabase.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qstring::*; // 771
// use super::qlist::*; // 775
use super::qfont::*; // 773
use super::super::core::qstringlist::*; // 771
use super::super::core::qbytearray::*; // 771
use super::qfontinfo::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFontDatabase_Class_Size() -> c_int;
  // proto:  QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
  fn C_ZN13QFontDatabase10pointSizesERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QFontDatabase::styleString(const QFont & font);
  fn C_ZN13QFontDatabase11styleStringERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
  fn C_ZN13QFontDatabase11smoothSizesERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QStringList QFontDatabase::styles(const QString & family);
  fn C_ZNK13QFontDatabase6stylesERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFontDatabase::italic(const QString & family, const QString & style);
  fn C_ZNK13QFontDatabase6italicERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QFontDatabase::QFontDatabase();
  fn C_ZN13QFontDatabaseC2Ev() -> u64;
  // proto: static QStringList QFontDatabase::applicationFontFamilies(int id);
  fn C_ZN13QFontDatabase23applicationFontFamiliesEi(arg0: c_int) -> *mut c_void;
  // proto:  bool QFontDatabase::hasFamily(const QString & family);
  fn C_ZNK13QFontDatabase9hasFamilyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QFontDatabase::isFixedPitch(const QString & family, const QString & style);
  fn C_ZNK13QFontDatabase12isFixedPitchERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  QFont QFontDatabase::font(const QString & family, const QString & style, int pointSize);
  fn C_ZNK13QFontDatabase4fontERK7QStringS2_i(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  // proto:  int QFontDatabase::weight(const QString & family, const QString & style);
  fn C_ZNK13QFontDatabase6weightERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto: static bool QFontDatabase::removeAllApplicationFonts();
  fn C_ZN13QFontDatabase25removeAllApplicationFontsEv() -> c_char;
  // proto: static int QFontDatabase::addApplicationFontFromData(const QByteArray & fontData);
  fn C_ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray(arg0: *mut c_void) -> c_int;
  // proto: static bool QFontDatabase::supportsThreadedFontRendering();
  fn C_ZN13QFontDatabase29supportsThreadedFontRenderingEv() -> c_char;
  // proto:  bool QFontDatabase::isPrivateFamily(const QString & family);
  fn C_ZNK13QFontDatabase15isPrivateFamilyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QFontDatabase::isScalable(const QString & family, const QString & style);
  fn C_ZNK13QFontDatabase10isScalableERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static bool QFontDatabase::removeApplicationFont(int id);
  fn C_ZN13QFontDatabase21removeApplicationFontEi(arg0: c_int) -> c_char;
  // proto:  QString QFontDatabase::styleString(const QFontInfo & fontInfo);
  fn C_ZN13QFontDatabase11styleStringERK9QFontInfo(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFontDatabase::isBitmapScalable(const QString & family, const QString & style);
  fn C_ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  bool QFontDatabase::isSmoothlyScalable(const QString & family, const QString & style);
  fn C_ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  bool QFontDatabase::bold(const QString & family, const QString & style);
  fn C_ZNK13QFontDatabase4boldERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static int QFontDatabase::addApplicationFont(const QString & fileName);
  fn C_ZN13QFontDatabase18addApplicationFontERK7QString(arg0: *mut c_void) -> c_int;
  // proto: static QList<int> QFontDatabase::standardSizes();
  fn C_ZN13QFontDatabase13standardSizesEv() -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QFontDatabase)=8
#[derive(Default)]
pub struct QFontDatabase {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFontDatabase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFontDatabase {
    return QFontDatabase{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn pointSizes<RetType, T: QFontDatabase_pointSizes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pointSizes(self);
    // return 1;
  }
}

pub trait QFontDatabase_pointSizes<RetType> {
  fn pointSizes(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  QList<int> QFontDatabase::pointSizes(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_pointSizes<u64> for (&'a QString, Option<&'a QString>) {
  fn pointSizes(self , rsthis: & QFontDatabase) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase10pointSizesERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {QString::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN13QFontDatabase10pointSizesERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QString QFontDatabase::styleString(const QFont & font);
impl /*struct*/ QFontDatabase {
  pub fn styleString<RetType, T: QFontDatabase_styleString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.styleString(self);
    // return 1;
  }
}

pub trait QFontDatabase_styleString<RetType> {
  fn styleString(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  QString QFontDatabase::styleString(const QFont & font);
impl<'a> /*trait*/ QFontDatabase_styleString<QString> for (&'a QFont) {
  fn styleString(self , rsthis: & QFontDatabase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11styleStringERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QFontDatabase11styleStringERK5QFont(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn smoothSizes<RetType, T: QFontDatabase_smoothSizes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.smoothSizes(self);
    // return 1;
  }
}

pub trait QFontDatabase_smoothSizes<RetType> {
  fn smoothSizes(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  QList<int> QFontDatabase::smoothSizes(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_smoothSizes<u64> for (&'a QString, &'a QString) {
  fn smoothSizes(self , rsthis: & QFontDatabase) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11smoothSizesERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QFontDatabase11smoothSizesERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QStringList QFontDatabase::styles(const QString & family);
impl /*struct*/ QFontDatabase {
  pub fn styles<RetType, T: QFontDatabase_styles<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.styles(self);
    // return 1;
  }
}

pub trait QFontDatabase_styles<RetType> {
  fn styles(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  QStringList QFontDatabase::styles(const QString & family);
impl<'a> /*trait*/ QFontDatabase_styles<QStringList> for (&'a QString) {
  fn styles(self , rsthis: & QFontDatabase) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6stylesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase6stylesERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFontDatabase::italic(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn italic<RetType, T: QFontDatabase_italic<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.italic(self);
    // return 1;
  }
}

pub trait QFontDatabase_italic<RetType> {
  fn italic(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  bool QFontDatabase::italic(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_italic<i8> for (&'a QString, &'a QString) {
  fn italic(self , rsthis: & QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6italicERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase6italicERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFontDatabase::QFontDatabase();
impl /*struct*/ QFontDatabase {
  pub fn new<T: QFontDatabase_new>(value: T) -> QFontDatabase {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFontDatabase_new {
  fn new(self) -> QFontDatabase;
}

  // proto:  void QFontDatabase::QFontDatabase();
impl<'a> /*trait*/ QFontDatabase_new for () {
  fn new(self) -> QFontDatabase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabaseC2Ev()};
    let ctysz: c_int = unsafe{QFontDatabase_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN13QFontDatabaseC2Ev()};
    let rsthis = QFontDatabase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QStringList QFontDatabase::applicationFontFamilies(int id);
impl /*struct*/ QFontDatabase {
  pub fn applicationFontFamilies_s<RetType, T: QFontDatabase_applicationFontFamilies_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationFontFamilies_s();
    // return 1;
  }
}

pub trait QFontDatabase_applicationFontFamilies_s<RetType> {
  fn applicationFontFamilies_s(self ) -> RetType;
}

  // proto: static QStringList QFontDatabase::applicationFontFamilies(int id);
impl<'a> /*trait*/ QFontDatabase_applicationFontFamilies_s<QStringList> for (i32) {
  fn applicationFontFamilies_s(self ) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase23applicationFontFamiliesEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN13QFontDatabase23applicationFontFamiliesEi(arg0)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFontDatabase::hasFamily(const QString & family);
impl /*struct*/ QFontDatabase {
  pub fn hasFamily<RetType, T: QFontDatabase_hasFamily<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFamily(self);
    // return 1;
  }
}

pub trait QFontDatabase_hasFamily<RetType> {
  fn hasFamily(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  bool QFontDatabase::hasFamily(const QString & family);
impl<'a> /*trait*/ QFontDatabase_hasFamily<i8> for (&'a QString) {
  fn hasFamily(self , rsthis: & QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase9hasFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase9hasFamilyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QFontDatabase::isFixedPitch(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn isFixedPitch<RetType, T: QFontDatabase_isFixedPitch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFixedPitch(self);
    // return 1;
  }
}

pub trait QFontDatabase_isFixedPitch<RetType> {
  fn isFixedPitch(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  bool QFontDatabase::isFixedPitch(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isFixedPitch<i8> for (&'a QString, Option<&'a QString>) {
  fn isFixedPitch(self , rsthis: & QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase12isFixedPitchERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {QString::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase12isFixedPitchERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QFont QFontDatabase::font(const QString & family, const QString & style, int pointSize);
impl /*struct*/ QFontDatabase {
  pub fn font<RetType, T: QFontDatabase_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QFontDatabase_font<RetType> {
  fn font(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  QFont QFontDatabase::font(const QString & family, const QString & style, int pointSize);
impl<'a> /*trait*/ QFontDatabase_font<QFont> for (&'a QString, &'a QString, i32) {
  fn font(self , rsthis: & QFontDatabase) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase4fontERK7QStringS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZNK13QFontDatabase4fontERK7QStringS2_i(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QFontDatabase::weight(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn weight<RetType, T: QFontDatabase_weight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.weight(self);
    // return 1;
  }
}

pub trait QFontDatabase_weight<RetType> {
  fn weight(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  int QFontDatabase::weight(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_weight<i32> for (&'a QString, &'a QString) {
  fn weight(self , rsthis: & QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase6weightERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase6weightERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static bool QFontDatabase::removeAllApplicationFonts();
impl /*struct*/ QFontDatabase {
  pub fn removeAllApplicationFonts_s<RetType, T: QFontDatabase_removeAllApplicationFonts_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeAllApplicationFonts_s();
    // return 1;
  }
}

pub trait QFontDatabase_removeAllApplicationFonts_s<RetType> {
  fn removeAllApplicationFonts_s(self ) -> RetType;
}

  // proto: static bool QFontDatabase::removeAllApplicationFonts();
impl<'a> /*trait*/ QFontDatabase_removeAllApplicationFonts_s<i8> for () {
  fn removeAllApplicationFonts_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase25removeAllApplicationFontsEv()};
    let mut ret = unsafe {C_ZN13QFontDatabase25removeAllApplicationFontsEv()};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static int QFontDatabase::addApplicationFontFromData(const QByteArray & fontData);
impl /*struct*/ QFontDatabase {
  pub fn addApplicationFontFromData_s<RetType, T: QFontDatabase_addApplicationFontFromData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.addApplicationFontFromData_s();
    // return 1;
  }
}

pub trait QFontDatabase_addApplicationFontFromData_s<RetType> {
  fn addApplicationFontFromData_s(self ) -> RetType;
}

  // proto: static int QFontDatabase::addApplicationFontFromData(const QByteArray & fontData);
impl<'a> /*trait*/ QFontDatabase_addApplicationFontFromData_s<i32> for (&'a QByteArray) {
  fn addApplicationFontFromData_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static bool QFontDatabase::supportsThreadedFontRendering();
impl /*struct*/ QFontDatabase {
  pub fn supportsThreadedFontRendering_s<RetType, T: QFontDatabase_supportsThreadedFontRendering_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportsThreadedFontRendering_s();
    // return 1;
  }
}

pub trait QFontDatabase_supportsThreadedFontRendering_s<RetType> {
  fn supportsThreadedFontRendering_s(self ) -> RetType;
}

  // proto: static bool QFontDatabase::supportsThreadedFontRendering();
impl<'a> /*trait*/ QFontDatabase_supportsThreadedFontRendering_s<i8> for () {
  fn supportsThreadedFontRendering_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase29supportsThreadedFontRenderingEv()};
    let mut ret = unsafe {C_ZN13QFontDatabase29supportsThreadedFontRenderingEv()};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QFontDatabase::isPrivateFamily(const QString & family);
impl /*struct*/ QFontDatabase {
  pub fn isPrivateFamily<RetType, T: QFontDatabase_isPrivateFamily<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPrivateFamily(self);
    // return 1;
  }
}

pub trait QFontDatabase_isPrivateFamily<RetType> {
  fn isPrivateFamily(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  bool QFontDatabase::isPrivateFamily(const QString & family);
impl<'a> /*trait*/ QFontDatabase_isPrivateFamily<i8> for (&'a QString) {
  fn isPrivateFamily(self , rsthis: & QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase15isPrivateFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase15isPrivateFamilyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QFontDatabase::isScalable(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn isScalable<RetType, T: QFontDatabase_isScalable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isScalable<RetType> {
  fn isScalable(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  bool QFontDatabase::isScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isScalable<i8> for (&'a QString, Option<&'a QString>) {
  fn isScalable(self , rsthis: & QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase10isScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {QString::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase10isScalableERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static bool QFontDatabase::removeApplicationFont(int id);
impl /*struct*/ QFontDatabase {
  pub fn removeApplicationFont_s<RetType, T: QFontDatabase_removeApplicationFont_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeApplicationFont_s();
    // return 1;
  }
}

pub trait QFontDatabase_removeApplicationFont_s<RetType> {
  fn removeApplicationFont_s(self ) -> RetType;
}

  // proto: static bool QFontDatabase::removeApplicationFont(int id);
impl<'a> /*trait*/ QFontDatabase_removeApplicationFont_s<i8> for (i32) {
  fn removeApplicationFont_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase21removeApplicationFontEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN13QFontDatabase21removeApplicationFontEi(arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QFontDatabase::styleString(const QFontInfo & fontInfo);
impl<'a> /*trait*/ QFontDatabase_styleString<QString> for (&'a QFontInfo) {
  fn styleString(self , rsthis: & QFontDatabase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase11styleStringERK9QFontInfo()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QFontDatabase11styleStringERK9QFontInfo(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFontDatabase::isBitmapScalable(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn isBitmapScalable<RetType, T: QFontDatabase_isBitmapScalable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBitmapScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isBitmapScalable<RetType> {
  fn isBitmapScalable(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  bool QFontDatabase::isBitmapScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isBitmapScalable<i8> for (&'a QString, Option<&'a QString>) {
  fn isBitmapScalable(self , rsthis: & QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {QString::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QFontDatabase::isSmoothlyScalable(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn isSmoothlyScalable<RetType, T: QFontDatabase_isSmoothlyScalable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSmoothlyScalable(self);
    // return 1;
  }
}

pub trait QFontDatabase_isSmoothlyScalable<RetType> {
  fn isSmoothlyScalable(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  bool QFontDatabase::isSmoothlyScalable(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_isSmoothlyScalable<i8> for (&'a QString, Option<&'a QString>) {
  fn isSmoothlyScalable(self , rsthis: & QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {QString::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QFontDatabase::bold(const QString & family, const QString & style);
impl /*struct*/ QFontDatabase {
  pub fn bold<RetType, T: QFontDatabase_bold<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bold(self);
    // return 1;
  }
}

pub trait QFontDatabase_bold<RetType> {
  fn bold(self , rsthis: & QFontDatabase) -> RetType;
}

  // proto:  bool QFontDatabase::bold(const QString & family, const QString & style);
impl<'a> /*trait*/ QFontDatabase_bold<i8> for (&'a QString, &'a QString) {
  fn bold(self , rsthis: & QFontDatabase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontDatabase4boldERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QFontDatabase4boldERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static int QFontDatabase::addApplicationFont(const QString & fileName);
impl /*struct*/ QFontDatabase {
  pub fn addApplicationFont_s<RetType, T: QFontDatabase_addApplicationFont_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.addApplicationFont_s();
    // return 1;
  }
}

pub trait QFontDatabase_addApplicationFont_s<RetType> {
  fn addApplicationFont_s(self ) -> RetType;
}

  // proto: static int QFontDatabase::addApplicationFont(const QString & fileName);
impl<'a> /*trait*/ QFontDatabase_addApplicationFont_s<i32> for (&'a QString) {
  fn addApplicationFont_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase18addApplicationFontERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QFontDatabase18addApplicationFontERK7QString(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QList<int> QFontDatabase::standardSizes();
impl /*struct*/ QFontDatabase {
  pub fn standardSizes_s<RetType, T: QFontDatabase_standardSizes_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.standardSizes_s();
    // return 1;
  }
}

pub trait QFontDatabase_standardSizes_s<RetType> {
  fn standardSizes_s(self ) -> RetType;
}

  // proto: static QList<int> QFontDatabase::standardSizes();
impl<'a> /*trait*/ QFontDatabase_standardSizes_s<u64> for () {
  fn standardSizes_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontDatabase13standardSizesEv()};
    let mut ret = unsafe {C_ZN13QFontDatabase13standardSizesEv()};
    return ret as u64; // 5
    // return 1;
  }
}

// <= body block end

