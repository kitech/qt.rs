// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qstring::QString;
use super::qchar::QChar;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QByteArray QTextCodec::name();
  fn _ZNK10QTextCodec4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextCodec::toUnicode(const QByteArray & );
  fn _ZNK10QTextCodec9toUnicodeERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QTextCodec::fromUnicode(const QString & uc);
  fn _ZNK10QTextCodec11fromUnicodeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QTextCodec * QTextCodec::codecForLocale();
  fn _ZN10QTextCodec14codecForLocaleEv() -> *mut c_void;
  // proto: static QList<int> QTextCodec::availableMibs();
  fn _ZN10QTextCodec13availableMibsEv() ;
  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
  fn _ZN10QTextCodec12codecForHtmlERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextCodec::NewQTextCodec(const QTextCodec & );
  fn _ZN10QTextCodecC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
  fn _ZN10QTextCodec17setCodecForLocaleEPS_(arg0: *mut c_void) ;
  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
  fn _ZN10QTextCodec15codecForUtfTextERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QTextCodec::toUnicode(const char * chars);
  fn _ZNK10QTextCodec9toUnicodeEPKc(qthis: *mut c_void, arg0: *const c_char) -> *mut c_void;
  // proto:  int QTextCodec::mibEnum();
  fn _ZNK10QTextCodec7mibEnumEv(qthis: *mut c_void) -> c_int;
  // proto: static QTextCodec * QTextCodec::codecForName(const char * name);
  fn _ZN10QTextCodec12codecForNameEPKc(arg0: *const c_char) -> *mut c_void;
  // proto:  bool QTextCodec::canEncode(const QString & );
  fn _ZNK10QTextCodec9canEncodeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QList<QByteArray> QTextCodec::aliases();
  fn _ZNK10QTextCodec7aliasesEv(qthis: *mut c_void) ;
  // proto: static QTextCodec * QTextCodec::codecForName(const QByteArray & name);
  fn _ZN10QTextCodec12codecForNameERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QList<QByteArray> QTextCodec::availableCodecs();
  fn _ZN10QTextCodec15availableCodecsEv() ;
  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba, QTextCodec * defaultCodec);
  fn _ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QTextCodec::FreeQTextCodec();
  fn _ZN10QTextCodecD0Ev(qthis: *mut c_void) ;
  // proto: static QTextCodec * QTextCodec::codecForMib(int mib);
  fn _ZN10QTextCodec11codecForMibEi(arg0: c_int) -> *mut c_void;
  // proto:  void QTextCodec::NewQTextCodec();
  fn _ZN10QTextCodecC1Ev(qthis: *mut c_void) ;
  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba, QTextCodec * defaultCodec);
  fn _ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QTextCodec::canEncode(QChar );
  fn _ZNK10QTextCodec9canEncodeE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QTextCodec)=8
pub struct QTextCodec {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextCodec {
  pub fn name<T: QTextCodec_name>(&mut self, value: T) -> QByteArray {
    return value.name(self);
    // return 1;
  }
}

pub trait QTextCodec_name {
  fn name(self, rsthis: &mut QTextCodec) -> QByteArray;
}

// proto:  QByteArray QTextCodec::name();
impl<'a> /*trait*/ QTextCodec_name for () {
  fn name(self, rsthis: &mut QTextCodec) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec4nameEv()};
    let mut ret = unsafe {_ZNK10QTextCodec4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn toUnicode<T: QTextCodec_toUnicode>(&mut self, value: T) -> QString {
    return value.toUnicode(self);
    // return 1;
  }
}

pub trait QTextCodec_toUnicode {
  fn toUnicode(self, rsthis: &mut QTextCodec) -> QString;
}

// proto:  QString QTextCodec::toUnicode(const QByteArray & );
impl<'a> /*trait*/ QTextCodec_toUnicode for (&'a  QByteArray) {
  fn toUnicode(self, rsthis: &mut QTextCodec) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9toUnicodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec9toUnicodeERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn fromUnicode<T: QTextCodec_fromUnicode>(&mut self, value: T) -> QByteArray {
    return value.fromUnicode(self);
    // return 1;
  }
}

pub trait QTextCodec_fromUnicode {
  fn fromUnicode(self, rsthis: &mut QTextCodec) -> QByteArray;
}

// proto:  QByteArray QTextCodec::fromUnicode(const QString & uc);
impl<'a> /*trait*/ QTextCodec_fromUnicode for (&'a  QString) {
  fn fromUnicode(self, rsthis: &mut QTextCodec) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec11fromUnicodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec11fromUnicodeERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForLocale<T: QTextCodec_codecForLocale>(&mut self, value: T) -> QTextCodec {
    return value.codecForLocale(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForLocale {
  fn codecForLocale(self, rsthis: &mut QTextCodec) -> QTextCodec;
}

// proto: static QTextCodec * QTextCodec::codecForLocale();
impl<'a> /*trait*/ QTextCodec_codecForLocale for () {
  fn codecForLocale(self, rsthis: &mut QTextCodec) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec14codecForLocaleEv()};
    let mut ret = unsafe {_ZN10QTextCodec14codecForLocaleEv()};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn availableMibs<T: QTextCodec_availableMibs>(&mut self, value: T)  {
     value.availableMibs(self);
    // return 1;
  }
}

pub trait QTextCodec_availableMibs {
  fn availableMibs(self, rsthis: &mut QTextCodec) ;
}

// proto: static QList<int> QTextCodec::availableMibs();
impl<'a> /*trait*/ QTextCodec_availableMibs for () {
  fn availableMibs(self, rsthis: &mut QTextCodec)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec13availableMibsEv()};
     unsafe {_ZN10QTextCodec13availableMibsEv()};
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForHtml<T: QTextCodec_codecForHtml>(&mut self, value: T) -> QTextCodec {
    return value.codecForHtml(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForHtml {
  fn codecForHtml(self, rsthis: &mut QTextCodec) -> QTextCodec;
}

// proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForHtml for (&'a  QByteArray) {
  fn codecForHtml(self, rsthis: &mut QTextCodec) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTextCodec12codecForHtmlERK10QByteArray(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn NewQTextCodec<T: QTextCodec_NewQTextCodec>(value: T) -> QTextCodec {
    let rsthis = value.NewQTextCodec();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCodec_NewQTextCodec {
  fn NewQTextCodec(self) -> QTextCodec;
}

// proto: void QTextCodec::NewQTextCodec(const QTextCodec & );
impl<'a> /*trait*/ QTextCodec_NewQTextCodec for (&'a  QTextCodec) {
  fn NewQTextCodec(self) -> QTextCodec {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodecC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextCodecC1ERKS_(qthis, arg0)};
    let rsthis = QTextCodec{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn setCodecForLocale<T: QTextCodec_setCodecForLocale>(&mut self, value: T)  {
     value.setCodecForLocale(self);
    // return 1;
  }
}

pub trait QTextCodec_setCodecForLocale {
  fn setCodecForLocale(self, rsthis: &mut QTextCodec) ;
}

// proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
impl<'a> /*trait*/ QTextCodec_setCodecForLocale for (&'a mut QTextCodec) {
  fn setCodecForLocale(self, rsthis: &mut QTextCodec)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec17setCodecForLocaleEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextCodec17setCodecForLocaleEPS_(arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForUtfText<T: QTextCodec_codecForUtfText>(&mut self, value: T) -> QTextCodec {
    return value.codecForUtfText(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForUtfText {
  fn codecForUtfText(self, rsthis: &mut QTextCodec) -> QTextCodec;
}

// proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForUtfText for (&'a  QByteArray) {
  fn codecForUtfText(self, rsthis: &mut QTextCodec) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15codecForUtfTextERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTextCodec15codecForUtfTextERK10QByteArray(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QTextCodec::toUnicode(const char * chars);
impl<'a> /*trait*/ QTextCodec_toUnicode for (&'a  String) {
  fn toUnicode(self, rsthis: &mut QTextCodec) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9toUnicodeEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZNK10QTextCodec9toUnicodeEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn mibEnum<T: QTextCodec_mibEnum>(&mut self, value: T) -> i32 {
    return value.mibEnum(self);
    // return 1;
  }
}

pub trait QTextCodec_mibEnum {
  fn mibEnum(self, rsthis: &mut QTextCodec) -> i32;
}

// proto:  int QTextCodec::mibEnum();
impl<'a> /*trait*/ QTextCodec_mibEnum for () {
  fn mibEnum(self, rsthis: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7mibEnumEv()};
    let mut ret = unsafe {_ZNK10QTextCodec7mibEnumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForName<T: QTextCodec_codecForName>(&mut self, value: T) -> QTextCodec {
    return value.codecForName(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForName {
  fn codecForName(self, rsthis: &mut QTextCodec) -> QTextCodec;
}

// proto: static QTextCodec * QTextCodec::codecForName(const char * name);
impl<'a> /*trait*/ QTextCodec_codecForName for (&'a  String) {
  fn codecForName(self, rsthis: &mut QTextCodec) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForNameEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN10QTextCodec12codecForNameEPKc(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn canEncode<T: QTextCodec_canEncode>(&mut self, value: T) -> i8 {
    return value.canEncode(self);
    // return 1;
  }
}

pub trait QTextCodec_canEncode {
  fn canEncode(self, rsthis: &mut QTextCodec) -> i8;
}

// proto:  bool QTextCodec::canEncode(const QString & );
impl<'a> /*trait*/ QTextCodec_canEncode for (&'a  QString) {
  fn canEncode(self, rsthis: &mut QTextCodec) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec9canEncodeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn aliases<T: QTextCodec_aliases>(&mut self, value: T)  {
     value.aliases(self);
    // return 1;
  }
}

pub trait QTextCodec_aliases {
  fn aliases(self, rsthis: &mut QTextCodec) ;
}

// proto:  QList<QByteArray> QTextCodec::aliases();
impl<'a> /*trait*/ QTextCodec_aliases for () {
  fn aliases(self, rsthis: &mut QTextCodec)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7aliasesEv()};
     unsafe {_ZNK10QTextCodec7aliasesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static QTextCodec * QTextCodec::codecForName(const QByteArray & name);
impl<'a> /*trait*/ QTextCodec_codecForName for (&'a  QByteArray) {
  fn codecForName(self, rsthis: &mut QTextCodec) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTextCodec12codecForNameERK10QByteArray(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn availableCodecs<T: QTextCodec_availableCodecs>(&mut self, value: T)  {
     value.availableCodecs(self);
    // return 1;
  }
}

pub trait QTextCodec_availableCodecs {
  fn availableCodecs(self, rsthis: &mut QTextCodec) ;
}

// proto: static QList<QByteArray> QTextCodec::availableCodecs();
impl<'a> /*trait*/ QTextCodec_availableCodecs for () {
  fn availableCodecs(self, rsthis: &mut QTextCodec)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15availableCodecsEv()};
     unsafe {_ZN10QTextCodec15availableCodecsEv()};
    // return 1;
  }
}

// proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba, QTextCodec * defaultCodec);
impl<'a> /*trait*/ QTextCodec_codecForHtml for (&'a  QByteArray, &'a mut QTextCodec) {
  fn codecForHtml(self, rsthis: &mut QTextCodec) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_(arg0, arg1)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn FreeQTextCodec<T: QTextCodec_FreeQTextCodec>(&mut self, value: T)  {
     value.FreeQTextCodec(self);
    // return 1;
  }
}

pub trait QTextCodec_FreeQTextCodec {
  fn FreeQTextCodec(self, rsthis: &mut QTextCodec) ;
}

// proto:  void QTextCodec::FreeQTextCodec();
impl<'a> /*trait*/ QTextCodec_FreeQTextCodec for () {
  fn FreeQTextCodec(self, rsthis: &mut QTextCodec)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodecD0Ev()};
     unsafe {_ZN10QTextCodecD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForMib<T: QTextCodec_codecForMib>(&mut self, value: T) -> QTextCodec {
    return value.codecForMib(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForMib {
  fn codecForMib(self, rsthis: &mut QTextCodec) -> QTextCodec;
}

// proto: static QTextCodec * QTextCodec::codecForMib(int mib);
impl<'a> /*trait*/ QTextCodec_codecForMib for (i32) {
  fn codecForMib(self, rsthis: &mut QTextCodec) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec11codecForMibEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QTextCodec11codecForMibEi(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextCodec::NewQTextCodec();
impl<'a> /*trait*/ QTextCodec_NewQTextCodec for () {
  fn NewQTextCodec(self) -> QTextCodec {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodecC1Ev()};
    unsafe {_ZN10QTextCodecC1Ev(qthis)};
    let rsthis = QTextCodec{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba, QTextCodec * defaultCodec);
impl<'a> /*trait*/ QTextCodec_codecForUtfText for (&'a  QByteArray, &'a mut QTextCodec) {
  fn codecForUtfText(self, rsthis: &mut QTextCodec) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_(arg0, arg1)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QTextCodec::canEncode(QChar );
impl<'a> /*trait*/ QTextCodec_canEncode for (QChar) {
  fn canEncode(self, rsthis: &mut QTextCodec) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec9canEncodeE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

