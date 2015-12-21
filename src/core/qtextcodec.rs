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
  fn _ZN10QTextCodec13availableMibsEv();
  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
  fn _ZN10QTextCodec12codecForHtmlERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextCodec::QTextCodec(const QTextCodec & );
  fn _ZN10QTextCodecC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
  fn _ZN10QTextCodec17setCodecForLocaleEPS_(arg0: *mut c_void);
  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
  fn _ZN10QTextCodec15codecForUtfTextERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QTextCodec::toUnicode(const char * chars);
  fn _ZNK10QTextCodec9toUnicodeEPKc(qthis: *mut c_void, arg0: *mut c_char) -> *mut c_void;
  // proto:  int QTextCodec::mibEnum();
  fn _ZNK10QTextCodec7mibEnumEv(qthis: *mut c_void) -> c_int;
  // proto: static QTextCodec * QTextCodec::codecForName(const char * name);
  fn _ZN10QTextCodec12codecForNameEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto:  bool QTextCodec::canEncode(const QString & );
  fn _ZNK10QTextCodec9canEncodeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QList<QByteArray> QTextCodec::aliases();
  fn _ZNK10QTextCodec7aliasesEv(qthis: *mut c_void);
  // proto: static QTextCodec * QTextCodec::codecForName(const QByteArray & name);
  fn _ZN10QTextCodec12codecForNameERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QList<QByteArray> QTextCodec::availableCodecs();
  fn _ZN10QTextCodec15availableCodecsEv();
  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba, QTextCodec * defaultCodec);
  fn _ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QTextCodec::~QTextCodec();
  fn _ZN10QTextCodecD0Ev(qthis: *mut c_void);
  // proto: static QTextCodec * QTextCodec::codecForMib(int mib);
  fn _ZN10QTextCodec11codecForMibEi(arg0: c_int) -> *mut c_void;
  // proto:  void QTextCodec::QTextCodec();
  fn _ZN10QTextCodecC1Ev(qthis: *mut c_void);
  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba, QTextCodec * defaultCodec);
  fn _ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QTextCodec::canEncode(QChar );
  fn _ZNK10QTextCodec9canEncodeE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QTextCodec)=8
pub struct QTextCodec {
  pub qclsinst: *mut c_void,
}

  // proto:  QByteArray QTextCodec::name();
impl /*struct*/ QTextCodec {
  pub fn name<RetType, T: QTextCodec_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QTextCodec_name<RetType> {
  fn name(self , rsthis: &mut QTextCodec) -> RetType;
}

  // proto:  QByteArray QTextCodec::name();
impl<'a> /*trait*/ QTextCodec_name<QByteArray> for () {
  fn name(self , rsthis: &mut QTextCodec) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec4nameEv()};
    let mut ret = unsafe {_ZNK10QTextCodec4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextCodec::toUnicode(const QByteArray & );
impl /*struct*/ QTextCodec {
  pub fn toUnicode<RetType, T: QTextCodec_toUnicode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUnicode(self);
    // return 1;
  }
}

pub trait QTextCodec_toUnicode<RetType> {
  fn toUnicode(self , rsthis: &mut QTextCodec) -> RetType;
}

  // proto:  QString QTextCodec::toUnicode(const QByteArray & );
impl<'a> /*trait*/ QTextCodec_toUnicode<QString> for (QByteArray) {
  fn toUnicode(self , rsthis: &mut QTextCodec) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9toUnicodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec9toUnicodeERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QTextCodec::fromUnicode(const QString & uc);
impl /*struct*/ QTextCodec {
  pub fn fromUnicode<RetType, T: QTextCodec_fromUnicode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode(self);
    // return 1;
  }
}

pub trait QTextCodec_fromUnicode<RetType> {
  fn fromUnicode(self , rsthis: &mut QTextCodec) -> RetType;
}

  // proto:  QByteArray QTextCodec::fromUnicode(const QString & uc);
impl<'a> /*trait*/ QTextCodec_fromUnicode<QByteArray> for (QString) {
  fn fromUnicode(self , rsthis: &mut QTextCodec) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec11fromUnicodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec11fromUnicodeERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForLocale();
impl /*struct*/ QTextCodec {
  pub fn codecForLocale_s<RetType, T: QTextCodec_codecForLocale_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForLocale_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForLocale_s<RetType> {
  fn codecForLocale_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForLocale();
impl<'a> /*trait*/ QTextCodec_codecForLocale_s<QTextCodec> for () {
  fn codecForLocale_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec14codecForLocaleEv()};
    let mut ret = unsafe {_ZN10QTextCodec14codecForLocaleEv()};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QList<int> QTextCodec::availableMibs();
impl /*struct*/ QTextCodec {
  pub fn availableMibs_s<RetType, T: QTextCodec_availableMibs_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.availableMibs_s();
    // return 1;
  }
}

pub trait QTextCodec_availableMibs_s<RetType> {
  fn availableMibs_s(self ) -> RetType;
}

  // proto: static QList<int> QTextCodec::availableMibs();
impl<'a> /*trait*/ QTextCodec_availableMibs_s<()> for () {
  fn availableMibs_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec13availableMibsEv()};
     unsafe {_ZN10QTextCodec13availableMibsEv()};
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
impl /*struct*/ QTextCodec {
  pub fn codecForHtml_s<RetType, T: QTextCodec_codecForHtml_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForHtml_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForHtml_s<RetType> {
  fn codecForHtml_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForHtml_s<QTextCodec> for (QByteArray) {
  fn codecForHtml_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTextCodec12codecForHtmlERK10QByteArray(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCodec::QTextCodec(const QTextCodec & );
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

  // proto:  void QTextCodec::QTextCodec(const QTextCodec & );
impl<'a> /*trait*/ QTextCodec_NewQTextCodec for (QTextCodec) {
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

  // proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
impl /*struct*/ QTextCodec {
  pub fn setCodecForLocale_s<RetType, T: QTextCodec_setCodecForLocale_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCodecForLocale_s();
    // return 1;
  }
}

pub trait QTextCodec_setCodecForLocale_s<RetType> {
  fn setCodecForLocale_s(self ) -> RetType;
}

  // proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
impl<'a> /*trait*/ QTextCodec_setCodecForLocale_s<()> for (QTextCodec) {
  fn setCodecForLocale_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec17setCodecForLocaleEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextCodec17setCodecForLocaleEPS_(arg0)};
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
impl /*struct*/ QTextCodec {
  pub fn codecForUtfText_s<RetType, T: QTextCodec_codecForUtfText_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForUtfText_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForUtfText_s<RetType> {
  fn codecForUtfText_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForUtfText_s<QTextCodec> for (QByteArray) {
  fn codecForUtfText_s(self ) -> QTextCodec {
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
  fn toUnicode(self , rsthis: &mut QTextCodec) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9toUnicodeEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QTextCodec9toUnicodeEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextCodec::mibEnum();
impl /*struct*/ QTextCodec {
  pub fn mibEnum<RetType, T: QTextCodec_mibEnum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mibEnum(self);
    // return 1;
  }
}

pub trait QTextCodec_mibEnum<RetType> {
  fn mibEnum(self , rsthis: &mut QTextCodec) -> RetType;
}

  // proto:  int QTextCodec::mibEnum();
impl<'a> /*trait*/ QTextCodec_mibEnum<i32> for () {
  fn mibEnum(self , rsthis: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7mibEnumEv()};
    let mut ret = unsafe {_ZNK10QTextCodec7mibEnumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForName(const char * name);
impl /*struct*/ QTextCodec {
  pub fn codecForName_s<RetType, T: QTextCodec_codecForName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForName_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForName_s<RetType> {
  fn codecForName_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForName(const char * name);
impl<'a> /*trait*/ QTextCodec_codecForName_s<QTextCodec> for (&'a  String) {
  fn codecForName_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForNameEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QTextCodec12codecForNameEPKc(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextCodec::canEncode(const QString & );
impl /*struct*/ QTextCodec {
  pub fn canEncode<RetType, T: QTextCodec_canEncode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.canEncode(self);
    // return 1;
  }
}

pub trait QTextCodec_canEncode<RetType> {
  fn canEncode(self , rsthis: &mut QTextCodec) -> RetType;
}

  // proto:  bool QTextCodec::canEncode(const QString & );
impl<'a> /*trait*/ QTextCodec_canEncode<i8> for (QString) {
  fn canEncode(self , rsthis: &mut QTextCodec) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec9canEncodeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QTextCodec::aliases();
impl /*struct*/ QTextCodec {
  pub fn aliases<RetType, T: QTextCodec_aliases<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.aliases(self);
    // return 1;
  }
}

pub trait QTextCodec_aliases<RetType> {
  fn aliases(self , rsthis: &mut QTextCodec) -> RetType;
}

  // proto:  QList<QByteArray> QTextCodec::aliases();
impl<'a> /*trait*/ QTextCodec_aliases<()> for () {
  fn aliases(self , rsthis: &mut QTextCodec) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7aliasesEv()};
     unsafe {_ZNK10QTextCodec7aliasesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForName(const QByteArray & name);
impl<'a> /*trait*/ QTextCodec_codecForName_s<QTextCodec> for (QByteArray) {
  fn codecForName_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTextCodec12codecForNameERK10QByteArray(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QTextCodec::availableCodecs();
impl /*struct*/ QTextCodec {
  pub fn availableCodecs_s<RetType, T: QTextCodec_availableCodecs_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.availableCodecs_s();
    // return 1;
  }
}

pub trait QTextCodec_availableCodecs_s<RetType> {
  fn availableCodecs_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QTextCodec::availableCodecs();
impl<'a> /*trait*/ QTextCodec_availableCodecs_s<()> for () {
  fn availableCodecs_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15availableCodecsEv()};
     unsafe {_ZN10QTextCodec15availableCodecsEv()};
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba, QTextCodec * defaultCodec);
impl<'a> /*trait*/ QTextCodec_codecForHtml_s<QTextCodec> for (QByteArray, QTextCodec) {
  fn codecForHtml_s(self ) -> QTextCodec {
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

  // proto:  void QTextCodec::~QTextCodec();
impl /*struct*/ QTextCodec {
  pub fn FreeQTextCodec<RetType, T: QTextCodec_FreeQTextCodec<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextCodec(self);
    // return 1;
  }
}

pub trait QTextCodec_FreeQTextCodec<RetType> {
  fn FreeQTextCodec(self , rsthis: &mut QTextCodec) -> RetType;
}

  // proto:  void QTextCodec::~QTextCodec();
impl<'a> /*trait*/ QTextCodec_FreeQTextCodec<()> for () {
  fn FreeQTextCodec(self , rsthis: &mut QTextCodec) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodecD0Ev()};
     unsafe {_ZN10QTextCodecD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForMib(int mib);
impl /*struct*/ QTextCodec {
  pub fn codecForMib_s<RetType, T: QTextCodec_codecForMib_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForMib_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForMib_s<RetType> {
  fn codecForMib_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForMib(int mib);
impl<'a> /*trait*/ QTextCodec_codecForMib_s<QTextCodec> for (i32) {
  fn codecForMib_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec11codecForMibEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QTextCodec11codecForMibEi(arg0)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCodec::QTextCodec();
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
impl<'a> /*trait*/ QTextCodec_codecForUtfText_s<QTextCodec> for (QByteArray, QTextCodec) {
  fn codecForUtfText_s(self ) -> QTextCodec {
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
  fn canEncode(self , rsthis: &mut QTextCodec) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextCodec9canEncodeE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

