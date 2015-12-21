// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qlocale.h
// dst-file: /src/core/qlocale.rs
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
use super::qstring::QString; // 773
use super::qchar::QChar; // 773
use super::qdatetime::QTime; // 773
use super::qstringlist::QStringList; // 773
use super::qdatetime::QDateTime; // 773
use super::qdatetime::QDate; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QString QLocale::pmText();
  fn _ZNK7QLocale6pmTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::nativeLanguageName();
  fn _ZNK7QLocale18nativeLanguageNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toLower(const QString & str);
  fn _ZNK7QLocale7toLowerERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::zeroDigit();
  fn _ZNK7QLocale9zeroDigitEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::name();
  fn _ZNK7QLocale4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(qlonglong , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringExRK7QString(qthis: *mut c_void, arg0: c_longlong, arg1: *mut c_void) -> *mut c_void;
  // proto:  float QLocale::toFloat(const QString & s, bool * ok);
  fn _ZNK7QLocale7toFloatERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_float;
  // proto: static QLocale QLocale::c();
  fn _ZN7QLocale1cEv() -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(uint , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEjRK7QString(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::createSeparatedList(const QStringList & strl);
  fn _ZNK7QLocale19createSeparatedListERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  uint QLocale::toUInt(const QString & s, bool * ok);
  fn _ZNK7QLocale6toUIntERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_uint;
  // proto:  QChar QLocale::decimalPoint();
  fn _ZNK7QLocale12decimalPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::positiveSign();
  fn _ZNK7QLocale12positiveSignEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qlonglong QLocale::toLongLong(const QString & s, bool * ok);
  fn _ZNK7QLocale10toLongLongERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_longlong;
  // proto:  short QLocale::toShort(const QString & s, bool * ok);
  fn _ZNK7QLocale7toShortERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_short;
  // proto:  QString QLocale::toString(float i, char f, int prec);
  fn _ZNK7QLocale8toStringEfci(qthis: *mut c_void, arg0: c_float, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  QString QLocale::toString(const QDateTime & dateTime, const QString & format);
  fn _ZNK7QLocale8toStringERK9QDateTimeRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QDateTime QLocale::toDateTime(const QString & string, const QString & format);
  fn _ZNK7QLocale10toDateTimeERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(short , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEsRK7QString(qthis: *mut c_void, arg0: c_short, arg1: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::groupSeparator();
  fn _ZNK7QLocale14groupSeparatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(double , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEdRK7QString(qthis: *mut c_void, arg0: c_double, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(qulonglong , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEyRK7QString(qthis: *mut c_void, arg0: c_ulonglong, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QLocale::QLocale(const QString & name);
  fn _ZN7QLocaleC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QLocale::toString(const QTime & time, const QString & formatStr);
  fn _ZNK7QLocale8toStringERK5QTimeRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QDate QLocale::toDate(const QString & string, const QString & format);
  fn _ZNK7QLocale6toDateERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::nativeCountryName();
  fn _ZNK7QLocale17nativeCountryNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::negativeSign();
  fn _ZNK7QLocale12negativeSignEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLocale::~QLocale();
  fn _ZN7QLocaleD0Ev(qthis: *mut c_void);
  // proto:  void QLocale::QLocale(const QLocale & other);
  fn _ZN7QLocaleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QLocale::toString(const QDate & date, const QString & formatStr);
  fn _ZNK7QLocale8toStringERK5QDateRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toUpper(const QString & str);
  fn _ZNK7QLocale7toUpperERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::percent();
  fn _ZNK7QLocale7percentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qulonglong QLocale::toULongLong(const QString & s, bool * ok);
  fn _ZNK7QLocale11toULongLongERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_ulonglong;
  // proto:  QString QLocale::toString(double i, char f, int prec);
  fn _ZNK7QLocale8toStringEdci(qthis: *mut c_void, arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  QStringList QLocale::uiLanguages();
  fn _ZNK7QLocale11uiLanguagesEv(qthis: *mut c_void);
  // proto:  QString QLocale::bcp47Name();
  fn _ZNK7QLocale9bcp47NameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTime QLocale::toTime(const QString & string, const QString & format);
  fn _ZNK7QLocale6toTimeERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  ushort QLocale::toUShort(const QString & s, bool * ok);
  fn _ZNK7QLocale8toUShortERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_ushort;
  // proto:  QString QLocale::toCurrencyString(ushort , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEtRK7QString(qthis: *mut c_void, arg0: c_ushort, arg1: *mut c_void) -> *mut c_void;
  // proto:  double QLocale::toDouble(const QString & s, bool * ok);
  fn _ZNK7QLocale8toDoubleERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_double;
  // proto: static QLocale QLocale::system();
  fn _ZN7QLocale6systemEv() -> *mut c_void;
  // proto: static void QLocale::setDefault(const QLocale & locale);
  fn _ZN7QLocale10setDefaultERKS_(arg0: *mut c_void);
  // proto:  QChar QLocale::exponential();
  fn _ZNK7QLocale11exponentialEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(float , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEfRK7QString(qthis: *mut c_void, arg0: c_float, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toString(int i);
  fn _ZNK7QLocale8toStringEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString QLocale::toString(uint i);
  fn _ZNK7QLocale8toStringEj(qthis: *mut c_void, arg0: c_uint) -> *mut c_void;
  // proto:  QString QLocale::toString(qlonglong i);
  fn _ZNK7QLocale8toStringEx(qthis: *mut c_void, arg0: c_longlong) -> *mut c_void;
  // proto:  QString QLocale::toString(qulonglong i);
  fn _ZNK7QLocale8toStringEy(qthis: *mut c_void, arg0: c_ulonglong) -> *mut c_void;
  // proto:  QString QLocale::toString(ushort i);
  fn _ZNK7QLocale8toStringEt(qthis: *mut c_void, arg0: c_ushort) -> *mut c_void;
  // proto:  QString QLocale::amText();
  fn _ZNK7QLocale6amTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(int , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toString(short i);
  fn _ZNK7QLocale8toStringEs(qthis: *mut c_void, arg0: c_short) -> *mut c_void;
  // proto:  void QLocale::QLocale();
  fn _ZN7QLocaleC1Ev(qthis: *mut c_void);
  // proto:  int QLocale::toInt(const QString & s, bool * ok);
  fn _ZNK7QLocale5toIntERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QLocale)=1
pub struct QLocale {
  pub qclsinst: *mut c_void,
}

  // proto:  QString QLocale::pmText();
impl /*struct*/ QLocale {
  pub fn pmText<RetType, T: QLocale_pmText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pmText(self);
    // return 1;
  }
}

pub trait QLocale_pmText<RetType> {
  fn pmText(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::pmText();
impl<'a> /*trait*/ QLocale_pmText<QString> for () {
  fn pmText(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6pmTextEv()};
    let mut ret = unsafe {_ZNK7QLocale6pmTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::nativeLanguageName();
impl /*struct*/ QLocale {
  pub fn nativeLanguageName<RetType, T: QLocale_nativeLanguageName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nativeLanguageName(self);
    // return 1;
  }
}

pub trait QLocale_nativeLanguageName<RetType> {
  fn nativeLanguageName(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::nativeLanguageName();
impl<'a> /*trait*/ QLocale_nativeLanguageName<QString> for () {
  fn nativeLanguageName(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale18nativeLanguageNameEv()};
    let mut ret = unsafe {_ZNK7QLocale18nativeLanguageNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toLower(const QString & str);
impl /*struct*/ QLocale {
  pub fn toLower<RetType, T: QLocale_toLower<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLower(self);
    // return 1;
  }
}

pub trait QLocale_toLower<RetType> {
  fn toLower(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::toLower(const QString & str);
impl<'a> /*trait*/ QLocale_toLower<QString> for (QString) {
  fn toLower(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toLowerERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale7toLowerERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::zeroDigit();
impl /*struct*/ QLocale {
  pub fn zeroDigit<RetType, T: QLocale_zeroDigit<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.zeroDigit(self);
    // return 1;
  }
}

pub trait QLocale_zeroDigit<RetType> {
  fn zeroDigit(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QChar QLocale::zeroDigit();
impl<'a> /*trait*/ QLocale_zeroDigit<QChar> for () {
  fn zeroDigit(self , rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale9zeroDigitEv()};
    let mut ret = unsafe {_ZNK7QLocale9zeroDigitEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::name();
impl /*struct*/ QLocale {
  pub fn name<RetType, T: QLocale_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QLocale_name<RetType> {
  fn name(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::name();
impl<'a> /*trait*/ QLocale_name<QString> for () {
  fn name(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale4nameEv()};
    let mut ret = unsafe {_ZNK7QLocale4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(qlonglong , const QString & symbol);
impl /*struct*/ QLocale {
  pub fn toCurrencyString<RetType, T: QLocale_toCurrencyString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString(self);
    // return 1;
  }
}

pub trait QLocale_toCurrencyString<RetType> {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::toCurrencyString(qlonglong , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (i64, QString) {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringExRK7QString()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringExRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  float QLocale::toFloat(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toFloat<RetType, T: QLocale_toFloat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toFloat(self);
    // return 1;
  }
}

pub trait QLocale_toFloat<RetType> {
  fn toFloat(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  float QLocale::toFloat(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toFloat<f32> for (QString, &'a mut Vec<i8>) {
  fn toFloat(self , rsthis: &mut QLocale) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toFloatERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QLocale7toFloatERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

  // proto: static QLocale QLocale::c();
impl /*struct*/ QLocale {
  pub fn c_s<RetType, T: QLocale_c_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.c_s();
    // return 1;
  }
}

pub trait QLocale_c_s<RetType> {
  fn c_s(self ) -> RetType;
}

  // proto: static QLocale QLocale::c();
impl<'a> /*trait*/ QLocale_c_s<QLocale> for () {
  fn c_s(self ) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale1cEv()};
    let mut ret = unsafe {_ZN7QLocale1cEv()};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(uint , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (u32, QString) {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEjRK7QString()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringEjRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::createSeparatedList(const QStringList & strl);
impl /*struct*/ QLocale {
  pub fn createSeparatedList<RetType, T: QLocale_createSeparatedList<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.createSeparatedList(self);
    // return 1;
  }
}

pub trait QLocale_createSeparatedList<RetType> {
  fn createSeparatedList(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::createSeparatedList(const QStringList & strl);
impl<'a> /*trait*/ QLocale_createSeparatedList<QString> for (QStringList) {
  fn createSeparatedList(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale19createSeparatedListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale19createSeparatedListERK11QStringList(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  uint QLocale::toUInt(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toUInt<RetType, T: QLocale_toUInt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUInt(self);
    // return 1;
  }
}

pub trait QLocale_toUInt<RetType> {
  fn toUInt(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  uint QLocale::toUInt(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toUInt<u32> for (QString, &'a mut Vec<i8>) {
  fn toUInt(self , rsthis: &mut QLocale) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toUIntERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QLocale6toUIntERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QChar QLocale::decimalPoint();
impl /*struct*/ QLocale {
  pub fn decimalPoint<RetType, T: QLocale_decimalPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.decimalPoint(self);
    // return 1;
  }
}

pub trait QLocale_decimalPoint<RetType> {
  fn decimalPoint(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QChar QLocale::decimalPoint();
impl<'a> /*trait*/ QLocale_decimalPoint<QChar> for () {
  fn decimalPoint(self , rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12decimalPointEv()};
    let mut ret = unsafe {_ZNK7QLocale12decimalPointEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::positiveSign();
impl /*struct*/ QLocale {
  pub fn positiveSign<RetType, T: QLocale_positiveSign<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.positiveSign(self);
    // return 1;
  }
}

pub trait QLocale_positiveSign<RetType> {
  fn positiveSign(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QChar QLocale::positiveSign();
impl<'a> /*trait*/ QLocale_positiveSign<QChar> for () {
  fn positiveSign(self , rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12positiveSignEv()};
    let mut ret = unsafe {_ZNK7QLocale12positiveSignEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qlonglong QLocale::toLongLong(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toLongLong<RetType, T: QLocale_toLongLong<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLongLong(self);
    // return 1;
  }
}

pub trait QLocale_toLongLong<RetType> {
  fn toLongLong(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  qlonglong QLocale::toLongLong(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toLongLong<i64> for (QString, &'a mut Vec<i8>) {
  fn toLongLong(self , rsthis: &mut QLocale) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale10toLongLongERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QLocale10toLongLongERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  short QLocale::toShort(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toShort<RetType, T: QLocale_toShort<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toShort(self);
    // return 1;
  }
}

pub trait QLocale_toShort<RetType> {
  fn toShort(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  short QLocale::toShort(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toShort<i16> for (QString, &'a mut Vec<i8>) {
  fn toShort(self , rsthis: &mut QLocale) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toShortERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QLocale7toShortERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i16;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(float i, char f, int prec);
impl /*struct*/ QLocale {
  pub fn toString<RetType, T: QLocale_toString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QLocale_toString<RetType> {
  fn toString(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::toString(float i, char f, int prec);
impl<'a> /*trait*/ QLocale_toString<QString> for (f32, i8, i32) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEfci()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK7QLocale8toStringEfci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(const QDateTime & dateTime, const QString & format);
impl<'a> /*trait*/ QLocale_toString<QString> for (QDateTime, QString) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK9QDateTimeRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale8toStringERK9QDateTimeRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QDateTime QLocale::toDateTime(const QString & string, const QString & format);
impl /*struct*/ QLocale {
  pub fn toDateTime<RetType, T: QLocale_toDateTime<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toDateTime(self);
    // return 1;
  }
}

pub trait QLocale_toDateTime<RetType> {
  fn toDateTime(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QDateTime QLocale::toDateTime(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toDateTime<QDateTime> for (QString, QString) {
  fn toDateTime(self , rsthis: &mut QLocale) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale10toDateTimeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale10toDateTimeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(short , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (i16, QString) {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEsRK7QString()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringEsRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::groupSeparator();
impl /*struct*/ QLocale {
  pub fn groupSeparator<RetType, T: QLocale_groupSeparator<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.groupSeparator(self);
    // return 1;
  }
}

pub trait QLocale_groupSeparator<RetType> {
  fn groupSeparator(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QChar QLocale::groupSeparator();
impl<'a> /*trait*/ QLocale_groupSeparator<QChar> for () {
  fn groupSeparator(self , rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale14groupSeparatorEv()};
    let mut ret = unsafe {_ZNK7QLocale14groupSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(double , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (f64, QString) {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEdRK7QString()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringEdRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(qulonglong , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (u64, QString) {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEyRK7QString()};
    let arg0 = self.0  as c_ulonglong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringEyRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocale::QLocale(const QString & name);
impl /*struct*/ QLocale {
  pub fn NewQLocale<T: QLocale_NewQLocale>(value: T) -> QLocale {
    let rsthis = value.NewQLocale();
    return rsthis;
    // return 1;
  }
}

pub trait QLocale_NewQLocale {
  fn NewQLocale(self) -> QLocale;
}

  // proto:  void QLocale::QLocale(const QString & name);
impl<'a> /*trait*/ QLocale_NewQLocale for (QString) {
  fn NewQLocale(self) -> QLocale {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLocaleC1ERK7QString(qthis, arg0)};
    let rsthis = QLocale{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(const QTime & time, const QString & formatStr);
impl<'a> /*trait*/ QLocale_toString<QString> for (QTime, QString) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK5QTimeRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale8toStringERK5QTimeRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QDate QLocale::toDate(const QString & string, const QString & format);
impl /*struct*/ QLocale {
  pub fn toDate<RetType, T: QLocale_toDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toDate(self);
    // return 1;
  }
}

pub trait QLocale_toDate<RetType> {
  fn toDate(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QDate QLocale::toDate(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toDate<QDate> for (QString, QString) {
  fn toDate(self , rsthis: &mut QLocale) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toDateERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale6toDateERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::nativeCountryName();
impl /*struct*/ QLocale {
  pub fn nativeCountryName<RetType, T: QLocale_nativeCountryName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nativeCountryName(self);
    // return 1;
  }
}

pub trait QLocale_nativeCountryName<RetType> {
  fn nativeCountryName(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::nativeCountryName();
impl<'a> /*trait*/ QLocale_nativeCountryName<QString> for () {
  fn nativeCountryName(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale17nativeCountryNameEv()};
    let mut ret = unsafe {_ZNK7QLocale17nativeCountryNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::negativeSign();
impl /*struct*/ QLocale {
  pub fn negativeSign<RetType, T: QLocale_negativeSign<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.negativeSign(self);
    // return 1;
  }
}

pub trait QLocale_negativeSign<RetType> {
  fn negativeSign(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QChar QLocale::negativeSign();
impl<'a> /*trait*/ QLocale_negativeSign<QChar> for () {
  fn negativeSign(self , rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12negativeSignEv()};
    let mut ret = unsafe {_ZNK7QLocale12negativeSignEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocale::~QLocale();
impl /*struct*/ QLocale {
  pub fn FreeQLocale<RetType, T: QLocale_FreeQLocale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQLocale(self);
    // return 1;
  }
}

pub trait QLocale_FreeQLocale<RetType> {
  fn FreeQLocale(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  void QLocale::~QLocale();
impl<'a> /*trait*/ QLocale_FreeQLocale<()> for () {
  fn FreeQLocale(self , rsthis: &mut QLocale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleD0Ev()};
     unsafe {_ZN7QLocaleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLocale::QLocale(const QLocale & other);
impl<'a> /*trait*/ QLocale_NewQLocale for (QLocale) {
  fn NewQLocale(self) -> QLocale {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLocaleC1ERKS_(qthis, arg0)};
    let rsthis = QLocale{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(const QDate & date, const QString & formatStr);
impl<'a> /*trait*/ QLocale_toString<QString> for (QDate, QString) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK5QDateRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale8toStringERK5QDateRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toUpper(const QString & str);
impl /*struct*/ QLocale {
  pub fn toUpper<RetType, T: QLocale_toUpper<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUpper(self);
    // return 1;
  }
}

pub trait QLocale_toUpper<RetType> {
  fn toUpper(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::toUpper(const QString & str);
impl<'a> /*trait*/ QLocale_toUpper<QString> for (QString) {
  fn toUpper(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toUpperERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale7toUpperERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::percent();
impl /*struct*/ QLocale {
  pub fn percent<RetType, T: QLocale_percent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.percent(self);
    // return 1;
  }
}

pub trait QLocale_percent<RetType> {
  fn percent(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QChar QLocale::percent();
impl<'a> /*trait*/ QLocale_percent<QChar> for () {
  fn percent(self , rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7percentEv()};
    let mut ret = unsafe {_ZNK7QLocale7percentEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qulonglong QLocale::toULongLong(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toULongLong<RetType, T: QLocale_toULongLong<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toULongLong(self);
    // return 1;
  }
}

pub trait QLocale_toULongLong<RetType> {
  fn toULongLong(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  qulonglong QLocale::toULongLong(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toULongLong<u64> for (QString, &'a mut Vec<i8>) {
  fn toULongLong(self , rsthis: &mut QLocale) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11toULongLongERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QLocale11toULongLongERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(double i, char f, int prec);
impl<'a> /*trait*/ QLocale_toString<QString> for (f64, i8, i32) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK7QLocale8toStringEdci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QLocale::uiLanguages();
impl /*struct*/ QLocale {
  pub fn uiLanguages<RetType, T: QLocale_uiLanguages<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.uiLanguages(self);
    // return 1;
  }
}

pub trait QLocale_uiLanguages<RetType> {
  fn uiLanguages(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QStringList QLocale::uiLanguages();
impl<'a> /*trait*/ QLocale_uiLanguages<()> for () {
  fn uiLanguages(self , rsthis: &mut QLocale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11uiLanguagesEv()};
     unsafe {_ZNK7QLocale11uiLanguagesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QLocale::bcp47Name();
impl /*struct*/ QLocale {
  pub fn bcp47Name<RetType, T: QLocale_bcp47Name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bcp47Name(self);
    // return 1;
  }
}

pub trait QLocale_bcp47Name<RetType> {
  fn bcp47Name(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::bcp47Name();
impl<'a> /*trait*/ QLocale_bcp47Name<QString> for () {
  fn bcp47Name(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale9bcp47NameEv()};
    let mut ret = unsafe {_ZNK7QLocale9bcp47NameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTime QLocale::toTime(const QString & string, const QString & format);
impl /*struct*/ QLocale {
  pub fn toTime<RetType, T: QLocale_toTime<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toTime(self);
    // return 1;
  }
}

pub trait QLocale_toTime<RetType> {
  fn toTime(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QTime QLocale::toTime(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toTime<QTime> for (QString, QString) {
  fn toTime(self , rsthis: &mut QLocale) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toTimeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale6toTimeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  ushort QLocale::toUShort(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toUShort<RetType, T: QLocale_toUShort<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUShort(self);
    // return 1;
  }
}

pub trait QLocale_toUShort<RetType> {
  fn toUShort(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  ushort QLocale::toUShort(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toUShort<u16> for (QString, &'a mut Vec<i8>) {
  fn toUShort(self , rsthis: &mut QLocale) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toUShortERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QLocale8toUShortERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(ushort , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (u16, QString) {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEtRK7QString()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringEtRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  double QLocale::toDouble(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toDouble<RetType, T: QLocale_toDouble<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QLocale_toDouble<RetType> {
  fn toDouble(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  double QLocale::toDouble(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toDouble<f64> for (QString, &'a mut Vec<i8>) {
  fn toDouble(self , rsthis: &mut QLocale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toDoubleERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QLocale8toDoubleERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as f64;
    // return 1;
  }
}

  // proto: static QLocale QLocale::system();
impl /*struct*/ QLocale {
  pub fn system_s<RetType, T: QLocale_system_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.system_s();
    // return 1;
  }
}

pub trait QLocale_system_s<RetType> {
  fn system_s(self ) -> RetType;
}

  // proto: static QLocale QLocale::system();
impl<'a> /*trait*/ QLocale_system_s<QLocale> for () {
  fn system_s(self ) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale6systemEv()};
    let mut ret = unsafe {_ZN7QLocale6systemEv()};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static void QLocale::setDefault(const QLocale & locale);
impl /*struct*/ QLocale {
  pub fn setDefault_s<RetType, T: QLocale_setDefault_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefault_s();
    // return 1;
  }
}

pub trait QLocale_setDefault_s<RetType> {
  fn setDefault_s(self ) -> RetType;
}

  // proto: static void QLocale::setDefault(const QLocale & locale);
impl<'a> /*trait*/ QLocale_setDefault_s<()> for (QLocale) {
  fn setDefault_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale10setDefaultERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLocale10setDefaultERKS_(arg0)};
    // return 1;
  }
}

  // proto:  QChar QLocale::exponential();
impl /*struct*/ QLocale {
  pub fn exponential<RetType, T: QLocale_exponential<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.exponential(self);
    // return 1;
  }
}

pub trait QLocale_exponential<RetType> {
  fn exponential(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QChar QLocale::exponential();
impl<'a> /*trait*/ QLocale_exponential<QChar> for () {
  fn exponential(self , rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11exponentialEv()};
    let mut ret = unsafe {_ZNK7QLocale11exponentialEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(float , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (f32, QString) {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEfRK7QString()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringEfRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(int i);
impl<'a> /*trait*/ QLocale_toString<QString> for (i32) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QLocale8toStringEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(uint i);
impl<'a> /*trait*/ QLocale_toString<QString> for (u32) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK7QLocale8toStringEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(qlonglong i);
impl<'a> /*trait*/ QLocale_toString<QString> for (i64) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK7QLocale8toStringEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(qulonglong i);
impl<'a> /*trait*/ QLocale_toString<QString> for (u64) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEy()};
    let arg0 = self  as c_ulonglong;
    let mut ret = unsafe {_ZNK7QLocale8toStringEy(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(ushort i);
impl<'a> /*trait*/ QLocale_toString<QString> for (u16) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEt()};
    let arg0 = self  as c_ushort;
    let mut ret = unsafe {_ZNK7QLocale8toStringEt(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::amText();
impl /*struct*/ QLocale {
  pub fn amText<RetType, T: QLocale_amText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.amText(self);
    // return 1;
  }
}

pub trait QLocale_amText<RetType> {
  fn amText(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  QString QLocale::amText();
impl<'a> /*trait*/ QLocale_amText<QString> for () {
  fn amText(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6amTextEv()};
    let mut ret = unsafe {_ZNK7QLocale6amTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(int , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (i32, QString) {
  fn toCurrencyString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(short i);
impl<'a> /*trait*/ QLocale_toString<QString> for (i16) {
  fn toString(self , rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEs()};
    let arg0 = self  as c_short;
    let mut ret = unsafe {_ZNK7QLocale8toStringEs(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocale::QLocale();
impl<'a> /*trait*/ QLocale_NewQLocale for () {
  fn NewQLocale(self) -> QLocale {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleC1Ev()};
    unsafe {_ZN7QLocaleC1Ev(qthis)};
    let rsthis = QLocale{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QLocale::toInt(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toInt<RetType, T: QLocale_toInt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QLocale_toInt<RetType> {
  fn toInt(self , rsthis: &mut QLocale) -> RetType;
}

  // proto:  int QLocale::toInt(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toInt<i32> for (QString, &'a mut Vec<i8>) {
  fn toInt(self , rsthis: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale5toIntERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QLocale5toIntERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

