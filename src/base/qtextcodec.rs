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
  pub fn name<RetType, T: QTextCodec_name<RetType>>(&mut self, value: T) -> RetType {
    return value.name(self);
    // return 1;
  }
}

pub trait QTextCodec_name<RetType> {
  fn name(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto:  QByteArray QTextCodec::name();
impl<'a> /*trait*/ QTextCodec_name<QByteArray> for () {
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
  pub fn toUnicode<RetType, T: QTextCodec_toUnicode<RetType>>(&mut self, value: T) -> RetType {
    return value.toUnicode(self);
    // return 1;
  }
}

pub trait QTextCodec_toUnicode<RetType> {
  fn toUnicode(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto:  QString QTextCodec::toUnicode(const QByteArray & );
impl<'a> /*trait*/ QTextCodec_toUnicode<QString> for (&'a  QByteArray) {
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
  pub fn fromUnicode<RetType, T: QTextCodec_fromUnicode<RetType>>(&mut self, value: T) -> RetType {
    return value.fromUnicode(self);
    // return 1;
  }
}

pub trait QTextCodec_fromUnicode<RetType> {
  fn fromUnicode(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto:  QByteArray QTextCodec::fromUnicode(const QString & uc);
impl<'a> /*trait*/ QTextCodec_fromUnicode<QByteArray> for (&'a  QString) {
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
  pub fn codecForLocale<RetType, T: QTextCodec_codecForLocale<RetType>>(&mut self, value: T) -> RetType {
    return value.codecForLocale(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForLocale<RetType> {
  fn codecForLocale(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto: static QTextCodec * QTextCodec::codecForLocale();
impl<'a> /*trait*/ QTextCodec_codecForLocale<QTextCodec> for () {
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
  pub fn availableMibs<RetType, T: QTextCodec_availableMibs<RetType>>(&mut self, value: T) -> RetType {
    return value.availableMibs(self);
    // return 1;
  }
}

pub trait QTextCodec_availableMibs<RetType> {
  fn availableMibs(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto: static QList<int> QTextCodec::availableMibs();
impl<'a> /*trait*/ QTextCodec_availableMibs<()> for () {
  fn availableMibs(self, rsthis: &mut QTextCodec) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec13availableMibsEv()};
     unsafe {_ZN10QTextCodec13availableMibsEv()};
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForHtml<RetType, T: QTextCodec_codecForHtml<RetType>>(&mut self, value: T) -> RetType {
    return value.codecForHtml(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForHtml<RetType> {
  fn codecForHtml(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForHtml<QTextCodec> for (&'a  QByteArray) {
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
  pub fn setCodecForLocale<RetType, T: QTextCodec_setCodecForLocale<RetType>>(&mut self, value: T) -> RetType {
    return value.setCodecForLocale(self);
    // return 1;
  }
}

pub trait QTextCodec_setCodecForLocale<RetType> {
  fn setCodecForLocale(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
impl<'a> /*trait*/ QTextCodec_setCodecForLocale<()> for (&'a mut QTextCodec) {
  fn setCodecForLocale(self, rsthis: &mut QTextCodec) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec17setCodecForLocaleEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextCodec17setCodecForLocaleEPS_(arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForUtfText<RetType, T: QTextCodec_codecForUtfText<RetType>>(&mut self, value: T) -> RetType {
    return value.codecForUtfText(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForUtfText<RetType> {
  fn codecForUtfText(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForUtfText<QTextCodec> for (&'a  QByteArray) {
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
impl<'a> /*trait*/ QTextCodec_toUnicode<QString> for (&'a  String) {
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
  pub fn mibEnum<RetType, T: QTextCodec_mibEnum<RetType>>(&mut self, value: T) -> RetType {
    return value.mibEnum(self);
    // return 1;
  }
}

pub trait QTextCodec_mibEnum<RetType> {
  fn mibEnum(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto:  int QTextCodec::mibEnum();
impl<'a> /*trait*/ QTextCodec_mibEnum<i32> for () {
  fn mibEnum(self, rsthis: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7mibEnumEv()};
    let mut ret = unsafe {_ZNK10QTextCodec7mibEnumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForName<RetType, T: QTextCodec_codecForName<RetType>>(&mut self, value: T) -> RetType {
    return value.codecForName(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForName<RetType> {
  fn codecForName(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto: static QTextCodec * QTextCodec::codecForName(const char * name);
impl<'a> /*trait*/ QTextCodec_codecForName<QTextCodec> for (&'a  String) {
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
  pub fn canEncode<RetType, T: QTextCodec_canEncode<RetType>>(&mut self, value: T) -> RetType {
    return value.canEncode(self);
    // return 1;
  }
}

pub trait QTextCodec_canEncode<RetType> {
  fn canEncode(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto:  bool QTextCodec::canEncode(const QString & );
impl<'a> /*trait*/ QTextCodec_canEncode<i8> for (&'a  QString) {
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
  pub fn aliases<RetType, T: QTextCodec_aliases<RetType>>(&mut self, value: T) -> RetType {
    return value.aliases(self);
    // return 1;
  }
}

pub trait QTextCodec_aliases<RetType> {
  fn aliases(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto:  QList<QByteArray> QTextCodec::aliases();
impl<'a> /*trait*/ QTextCodec_aliases<()> for () {
  fn aliases(self, rsthis: &mut QTextCodec) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7aliasesEv()};
     unsafe {_ZNK10QTextCodec7aliasesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static QTextCodec * QTextCodec::codecForName(const QByteArray & name);
impl<'a> /*trait*/ QTextCodec_codecForName<QTextCodec> for (&'a  QByteArray) {
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
  pub fn availableCodecs<RetType, T: QTextCodec_availableCodecs<RetType>>(&mut self, value: T) -> RetType {
    return value.availableCodecs(self);
    // return 1;
  }
}

pub trait QTextCodec_availableCodecs<RetType> {
  fn availableCodecs(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto: static QList<QByteArray> QTextCodec::availableCodecs();
impl<'a> /*trait*/ QTextCodec_availableCodecs<()> for () {
  fn availableCodecs(self, rsthis: &mut QTextCodec) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15availableCodecsEv()};
     unsafe {_ZN10QTextCodec15availableCodecsEv()};
    // return 1;
  }
}

// proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba, QTextCodec * defaultCodec);
impl<'a> /*trait*/ QTextCodec_codecForHtml<QTextCodec> for (&'a  QByteArray, &'a mut QTextCodec) {
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
  pub fn FreeQTextCodec<RetType, T: QTextCodec_FreeQTextCodec<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTextCodec(self);
    // return 1;
  }
}

pub trait QTextCodec_FreeQTextCodec<RetType> {
  fn FreeQTextCodec(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto:  void QTextCodec::FreeQTextCodec();
impl<'a> /*trait*/ QTextCodec_FreeQTextCodec<()> for () {
  fn FreeQTextCodec(self, rsthis: &mut QTextCodec) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodecD0Ev()};
     unsafe {_ZN10QTextCodecD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForMib<RetType, T: QTextCodec_codecForMib<RetType>>(&mut self, value: T) -> RetType {
    return value.codecForMib(self);
    // return 1;
  }
}

pub trait QTextCodec_codecForMib<RetType> {
  fn codecForMib(self, rsthis: &mut QTextCodec) -> RetType;
}

// proto: static QTextCodec * QTextCodec::codecForMib(int mib);
impl<'a> /*trait*/ QTextCodec_codecForMib<QTextCodec> for (i32) {
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
impl<'a> /*trait*/ QTextCodec_codecForUtfText<QTextCodec> for (&'a  QByteArray, &'a mut QTextCodec) {
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
impl<'a> /*trait*/ QTextCodec_canEncode<i8> for (QChar) {
  fn canEncode(self, rsthis: &mut QTextCodec) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec9canEncodeE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

