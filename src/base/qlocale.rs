// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qchar::QChar;
use super::qstringlist::QStringList;
use super::qdatetime::QDateTime;
use super::qtime::QTime;
use super::qdate::QDate;

// ext block begin
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
  fn _ZNK7QLocale7toFloatERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> c_float;
  // proto: static QLocale QLocale::c();
  fn _ZN7QLocale1cEv() -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(uint , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEjRK7QString(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::createSeparatedList(const QStringList & strl);
  fn _ZNK7QLocale19createSeparatedListERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  unsigned int QLocale::toUInt(const QString & s, bool * ok);
  fn _ZNK7QLocale6toUIntERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> c_uint;
  // proto:  QChar QLocale::decimalPoint();
  fn _ZNK7QLocale12decimalPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::positiveSign();
  fn _ZNK7QLocale12positiveSignEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qint64 QLocale::toLongLong(const QString & s, bool * ok);
  fn _ZNK7QLocale10toLongLongERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> c_longlong;
  // proto:  short QLocale::toShort(const QString & s, bool * ok);
  fn _ZNK7QLocale7toShortERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> c_short;
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
  fn _ZNK7QLocale16toCurrencyStringEyRK7QString(qthis: *mut c_void, arg0: uint64_t, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QLocale::NewQLocale(const QString & name);
  fn _ZN7QLocaleC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QLocale::toString(const QTime & time, const QString & formatStr);
  fn _ZNK7QLocale8toStringERK5QTimeRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QDate QLocale::toDate(const QString & string, const QString & format);
  fn _ZNK7QLocale6toDateERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::nativeCountryName();
  fn _ZNK7QLocale17nativeCountryNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::negativeSign();
  fn _ZNK7QLocale12negativeSignEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLocale::FreeQLocale();
  fn _ZN7QLocaleD0Ev(qthis: *mut c_void) ;
  // proto:  void QLocale::NewQLocale(const QLocale & other);
  fn _ZN7QLocaleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QLocale::toString(const QDate & date, const QString & formatStr);
  fn _ZNK7QLocale8toStringERK5QDateRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toUpper(const QString & str);
  fn _ZNK7QLocale7toUpperERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::percent();
  fn _ZNK7QLocale7percentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  quint64 QLocale::toULongLong(const QString & s, bool * ok);
  fn _ZNK7QLocale11toULongLongERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> uint64_t;
  // proto:  QString QLocale::toString(double i, char f, int prec);
  fn _ZNK7QLocale8toStringEdci(qthis: *mut c_void, arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  QStringList QLocale::uiLanguages();
  fn _ZNK7QLocale11uiLanguagesEv(qthis: *mut c_void) ;
  // proto:  QString QLocale::bcp47Name();
  fn _ZNK7QLocale9bcp47NameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTime QLocale::toTime(const QString & string, const QString & format);
  fn _ZNK7QLocale6toTimeERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  unsigned short QLocale::toUShort(const QString & s, bool * ok);
  fn _ZNK7QLocale8toUShortERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> c_ushort;
  // proto:  QString QLocale::toCurrencyString(ushort , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEtRK7QString(qthis: *mut c_void, arg0: c_ushort, arg1: *mut c_void) -> *mut c_void;
  // proto:  double QLocale::toDouble(const QString & s, bool * ok);
  fn _ZNK7QLocale8toDoubleERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> c_double;
  // proto: static QLocale QLocale::system();
  fn _ZN7QLocale6systemEv() -> *mut c_void;
  // proto: static void QLocale::setDefault(const QLocale & locale);
  fn _ZN7QLocale10setDefaultERKS_(arg0: *mut c_void) ;
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
  fn _ZNK7QLocale8toStringEy(qthis: *mut c_void, arg0: uint64_t) -> *mut c_void;
  // proto:  QString QLocale::toString(ushort i);
  fn _ZNK7QLocale8toStringEt(qthis: *mut c_void, arg0: c_ushort) -> *mut c_void;
  // proto:  QString QLocale::amText();
  fn _ZNK7QLocale6amTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(int , const QString & symbol);
  fn _ZNK7QLocale16toCurrencyStringEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toString(short i);
  fn _ZNK7QLocale8toStringEs(qthis: *mut c_void, arg0: c_short) -> *mut c_void;
  // proto:  void QLocale::NewQLocale();
  fn _ZN7QLocaleC1Ev(qthis: *mut c_void) ;
  // proto:  int QLocale::toInt(const QString & s, bool * ok);
  fn _ZNK7QLocale5toIntERK7QStringPb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> c_int;
}

// body block begin
// class sizeof(QLocale)=1
pub struct QLocale {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLocale {
  pub fn pmText<T: QLocale_pmText>(&mut self, value: T) -> QString {
    return value.pmText(self);
    // return 1;
  }
}

pub trait QLocale_pmText {
  fn pmText(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::pmText();
impl<'a> /*trait*/ QLocale_pmText for () {
  fn pmText(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6pmTextEv()};
    let mut ret = unsafe {_ZNK7QLocale6pmTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn nativeLanguageName<T: QLocale_nativeLanguageName>(&mut self, value: T) -> QString {
    return value.nativeLanguageName(self);
    // return 1;
  }
}

pub trait QLocale_nativeLanguageName {
  fn nativeLanguageName(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::nativeLanguageName();
impl<'a> /*trait*/ QLocale_nativeLanguageName for () {
  fn nativeLanguageName(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale18nativeLanguageNameEv()};
    let mut ret = unsafe {_ZNK7QLocale18nativeLanguageNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toLower<T: QLocale_toLower>(&mut self, value: T) -> QString {
    return value.toLower(self);
    // return 1;
  }
}

pub trait QLocale_toLower {
  fn toLower(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::toLower(const QString & str);
impl<'a> /*trait*/ QLocale_toLower for (&'a  QString) {
  fn toLower(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toLowerERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale7toLowerERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn zeroDigit<T: QLocale_zeroDigit>(&mut self, value: T) -> QChar {
    return value.zeroDigit(self);
    // return 1;
  }
}

pub trait QLocale_zeroDigit {
  fn zeroDigit(self, rsthis: &mut QLocale) -> QChar;
}

// proto:  QChar QLocale::zeroDigit();
impl<'a> /*trait*/ QLocale_zeroDigit for () {
  fn zeroDigit(self, rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale9zeroDigitEv()};
    let mut ret = unsafe {_ZNK7QLocale9zeroDigitEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn name<T: QLocale_name>(&mut self, value: T) -> QString {
    return value.name(self);
    // return 1;
  }
}

pub trait QLocale_name {
  fn name(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::name();
impl<'a> /*trait*/ QLocale_name for () {
  fn name(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale4nameEv()};
    let mut ret = unsafe {_ZNK7QLocale4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toCurrencyString<T: QLocale_toCurrencyString>(&mut self, value: T) -> QString {
    return value.toCurrencyString(self);
    // return 1;
  }
}

pub trait QLocale_toCurrencyString {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::toCurrencyString(qlonglong , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (i64, &'a  QString) {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString {
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

impl /*struct*/ QLocale {
  pub fn toFloat<T: QLocale_toFloat>(&mut self, value: T) -> f32 {
    return value.toFloat(self);
    // return 1;
  }
}

pub trait QLocale_toFloat {
  fn toFloat(self, rsthis: &mut QLocale) -> f32;
}

// proto:  float QLocale::toFloat(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toFloat for (&'a  QString, &'a mut i8) {
  fn toFloat(self, rsthis: &mut QLocale) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toFloatERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QLocale7toFloatERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn c<T: QLocale_c>(&mut self, value: T) -> QLocale {
    return value.c(self);
    // return 1;
  }
}

pub trait QLocale_c {
  fn c(self, rsthis: &mut QLocale) -> QLocale;
}

// proto: static QLocale QLocale::c();
impl<'a> /*trait*/ QLocale_c for () {
  fn c(self, rsthis: &mut QLocale) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale1cEv()};
    let mut ret = unsafe {_ZN7QLocale1cEv()};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QLocale::toCurrencyString(uint , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (u32, &'a  QString) {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString {
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

impl /*struct*/ QLocale {
  pub fn createSeparatedList<T: QLocale_createSeparatedList>(&mut self, value: T) -> QString {
    return value.createSeparatedList(self);
    // return 1;
  }
}

pub trait QLocale_createSeparatedList {
  fn createSeparatedList(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::createSeparatedList(const QStringList & strl);
impl<'a> /*trait*/ QLocale_createSeparatedList for (&'a  QStringList) {
  fn createSeparatedList(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale19createSeparatedListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale19createSeparatedListERK11QStringList(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toUInt<T: QLocale_toUInt>(&mut self, value: T) -> u32 {
    return value.toUInt(self);
    // return 1;
  }
}

pub trait QLocale_toUInt {
  fn toUInt(self, rsthis: &mut QLocale) -> u32;
}

// proto:  unsigned int QLocale::toUInt(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toUInt for (&'a  QString, &'a mut i8) {
  fn toUInt(self, rsthis: &mut QLocale) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toUIntERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QLocale6toUIntERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn decimalPoint<T: QLocale_decimalPoint>(&mut self, value: T) -> QChar {
    return value.decimalPoint(self);
    // return 1;
  }
}

pub trait QLocale_decimalPoint {
  fn decimalPoint(self, rsthis: &mut QLocale) -> QChar;
}

// proto:  QChar QLocale::decimalPoint();
impl<'a> /*trait*/ QLocale_decimalPoint for () {
  fn decimalPoint(self, rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12decimalPointEv()};
    let mut ret = unsafe {_ZNK7QLocale12decimalPointEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn positiveSign<T: QLocale_positiveSign>(&mut self, value: T) -> QChar {
    return value.positiveSign(self);
    // return 1;
  }
}

pub trait QLocale_positiveSign {
  fn positiveSign(self, rsthis: &mut QLocale) -> QChar;
}

// proto:  QChar QLocale::positiveSign();
impl<'a> /*trait*/ QLocale_positiveSign for () {
  fn positiveSign(self, rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12positiveSignEv()};
    let mut ret = unsafe {_ZNK7QLocale12positiveSignEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toLongLong<T: QLocale_toLongLong>(&mut self, value: T) -> i64 {
    return value.toLongLong(self);
    // return 1;
  }
}

pub trait QLocale_toLongLong {
  fn toLongLong(self, rsthis: &mut QLocale) -> i64;
}

// proto:  qint64 QLocale::toLongLong(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toLongLong for (&'a  QString, &'a mut i8) {
  fn toLongLong(self, rsthis: &mut QLocale) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale10toLongLongERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QLocale10toLongLongERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toShort<T: QLocale_toShort>(&mut self, value: T) -> i16 {
    return value.toShort(self);
    // return 1;
  }
}

pub trait QLocale_toShort {
  fn toShort(self, rsthis: &mut QLocale) -> i16;
}

// proto:  short QLocale::toShort(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toShort for (&'a  QString, &'a mut i8) {
  fn toShort(self, rsthis: &mut QLocale) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toShortERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QLocale7toShortERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i16;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toString<T: QLocale_toString>(&mut self, value: T) -> QString {
    return value.toString(self);
    // return 1;
  }
}

pub trait QLocale_toString {
  fn toString(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::toString(float i, char f, int prec);
impl<'a> /*trait*/ QLocale_toString for (f32, i8, i32) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
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
impl<'a> /*trait*/ QLocale_toString for (&'a  QDateTime, &'a  QString) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
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

impl /*struct*/ QLocale {
  pub fn toDateTime<T: QLocale_toDateTime>(&mut self, value: T) -> QDateTime {
    return value.toDateTime(self);
    // return 1;
  }
}

pub trait QLocale_toDateTime {
  fn toDateTime(self, rsthis: &mut QLocale) -> QDateTime;
}

// proto:  QDateTime QLocale::toDateTime(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toDateTime for (&'a  QString, &'a  QString) {
  fn toDateTime(self, rsthis: &mut QLocale) -> QDateTime {
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
impl<'a> /*trait*/ QLocale_toCurrencyString for (i16, &'a  QString) {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString {
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

impl /*struct*/ QLocale {
  pub fn groupSeparator<T: QLocale_groupSeparator>(&mut self, value: T) -> QChar {
    return value.groupSeparator(self);
    // return 1;
  }
}

pub trait QLocale_groupSeparator {
  fn groupSeparator(self, rsthis: &mut QLocale) -> QChar;
}

// proto:  QChar QLocale::groupSeparator();
impl<'a> /*trait*/ QLocale_groupSeparator for () {
  fn groupSeparator(self, rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale14groupSeparatorEv()};
    let mut ret = unsafe {_ZNK7QLocale14groupSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QLocale::toCurrencyString(double , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (f64, &'a  QString) {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString {
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
impl<'a> /*trait*/ QLocale_toCurrencyString for (u64, &'a  QString) {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEyRK7QString()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale16toCurrencyStringEyRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

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

// proto: void QLocale::NewQLocale(const QString & name);
impl<'a> /*trait*/ QLocale_NewQLocale for (&'a  QString) {
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
impl<'a> /*trait*/ QLocale_toString for (&'a  QTime, &'a  QString) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
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

impl /*struct*/ QLocale {
  pub fn toDate<T: QLocale_toDate>(&mut self, value: T) -> QDate {
    return value.toDate(self);
    // return 1;
  }
}

pub trait QLocale_toDate {
  fn toDate(self, rsthis: &mut QLocale) -> QDate;
}

// proto:  QDate QLocale::toDate(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toDate for (&'a  QString, &'a  QString) {
  fn toDate(self, rsthis: &mut QLocale) -> QDate {
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

impl /*struct*/ QLocale {
  pub fn nativeCountryName<T: QLocale_nativeCountryName>(&mut self, value: T) -> QString {
    return value.nativeCountryName(self);
    // return 1;
  }
}

pub trait QLocale_nativeCountryName {
  fn nativeCountryName(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::nativeCountryName();
impl<'a> /*trait*/ QLocale_nativeCountryName for () {
  fn nativeCountryName(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale17nativeCountryNameEv()};
    let mut ret = unsafe {_ZNK7QLocale17nativeCountryNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn negativeSign<T: QLocale_negativeSign>(&mut self, value: T) -> QChar {
    return value.negativeSign(self);
    // return 1;
  }
}

pub trait QLocale_negativeSign {
  fn negativeSign(self, rsthis: &mut QLocale) -> QChar;
}

// proto:  QChar QLocale::negativeSign();
impl<'a> /*trait*/ QLocale_negativeSign for () {
  fn negativeSign(self, rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12negativeSignEv()};
    let mut ret = unsafe {_ZNK7QLocale12negativeSignEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn FreeQLocale<T: QLocale_FreeQLocale>(&mut self, value: T)  {
     value.FreeQLocale(self);
    // return 1;
  }
}

pub trait QLocale_FreeQLocale {
  fn FreeQLocale(self, rsthis: &mut QLocale) ;
}

// proto:  void QLocale::FreeQLocale();
impl<'a> /*trait*/ QLocale_FreeQLocale for () {
  fn FreeQLocale(self, rsthis: &mut QLocale)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleD0Ev()};
     unsafe {_ZN7QLocaleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QLocale::NewQLocale(const QLocale & other);
impl<'a> /*trait*/ QLocale_NewQLocale for (&'a  QLocale) {
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
impl<'a> /*trait*/ QLocale_toString for (&'a  QDate, &'a  QString) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
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

impl /*struct*/ QLocale {
  pub fn toUpper<T: QLocale_toUpper>(&mut self, value: T) -> QString {
    return value.toUpper(self);
    // return 1;
  }
}

pub trait QLocale_toUpper {
  fn toUpper(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::toUpper(const QString & str);
impl<'a> /*trait*/ QLocale_toUpper for (&'a  QString) {
  fn toUpper(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toUpperERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLocale7toUpperERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn percent<T: QLocale_percent>(&mut self, value: T) -> QChar {
    return value.percent(self);
    // return 1;
  }
}

pub trait QLocale_percent {
  fn percent(self, rsthis: &mut QLocale) -> QChar;
}

// proto:  QChar QLocale::percent();
impl<'a> /*trait*/ QLocale_percent for () {
  fn percent(self, rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7percentEv()};
    let mut ret = unsafe {_ZNK7QLocale7percentEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toULongLong<T: QLocale_toULongLong>(&mut self, value: T) -> u64 {
    return value.toULongLong(self);
    // return 1;
  }
}

pub trait QLocale_toULongLong {
  fn toULongLong(self, rsthis: &mut QLocale) -> u64;
}

// proto:  quint64 QLocale::toULongLong(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toULongLong for (&'a  QString, &'a mut i8) {
  fn toULongLong(self, rsthis: &mut QLocale) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11toULongLongERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QLocale11toULongLongERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u64;
    // return 1;
  }
}

// proto:  QString QLocale::toString(double i, char f, int prec);
impl<'a> /*trait*/ QLocale_toString for (f64, i8, i32) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
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

impl /*struct*/ QLocale {
  pub fn uiLanguages<T: QLocale_uiLanguages>(&mut self, value: T)  {
     value.uiLanguages(self);
    // return 1;
  }
}

pub trait QLocale_uiLanguages {
  fn uiLanguages(self, rsthis: &mut QLocale) ;
}

// proto:  QStringList QLocale::uiLanguages();
impl<'a> /*trait*/ QLocale_uiLanguages for () {
  fn uiLanguages(self, rsthis: &mut QLocale)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11uiLanguagesEv()};
     unsafe {_ZNK7QLocale11uiLanguagesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn bcp47Name<T: QLocale_bcp47Name>(&mut self, value: T) -> QString {
    return value.bcp47Name(self);
    // return 1;
  }
}

pub trait QLocale_bcp47Name {
  fn bcp47Name(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::bcp47Name();
impl<'a> /*trait*/ QLocale_bcp47Name for () {
  fn bcp47Name(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale9bcp47NameEv()};
    let mut ret = unsafe {_ZNK7QLocale9bcp47NameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toTime<T: QLocale_toTime>(&mut self, value: T) -> QTime {
    return value.toTime(self);
    // return 1;
  }
}

pub trait QLocale_toTime {
  fn toTime(self, rsthis: &mut QLocale) -> QTime;
}

// proto:  QTime QLocale::toTime(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toTime for (&'a  QString, &'a  QString) {
  fn toTime(self, rsthis: &mut QLocale) -> QTime {
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

impl /*struct*/ QLocale {
  pub fn toUShort<T: QLocale_toUShort>(&mut self, value: T) -> u16 {
    return value.toUShort(self);
    // return 1;
  }
}

pub trait QLocale_toUShort {
  fn toUShort(self, rsthis: &mut QLocale) -> u16;
}

// proto:  unsigned short QLocale::toUShort(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toUShort for (&'a  QString, &'a mut i8) {
  fn toUShort(self, rsthis: &mut QLocale) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toUShortERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QLocale8toUShortERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u16;
    // return 1;
  }
}

// proto:  QString QLocale::toCurrencyString(ushort , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (u16, &'a  QString) {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString {
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

impl /*struct*/ QLocale {
  pub fn toDouble<T: QLocale_toDouble>(&mut self, value: T) -> f64 {
    return value.toDouble(self);
    // return 1;
  }
}

pub trait QLocale_toDouble {
  fn toDouble(self, rsthis: &mut QLocale) -> f64;
}

// proto:  double QLocale::toDouble(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toDouble for (&'a  QString, &'a mut i8) {
  fn toDouble(self, rsthis: &mut QLocale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toDoubleERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QLocale8toDoubleERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn system<T: QLocale_system>(&mut self, value: T) -> QLocale {
    return value.system(self);
    // return 1;
  }
}

pub trait QLocale_system {
  fn system(self, rsthis: &mut QLocale) -> QLocale;
}

// proto: static QLocale QLocale::system();
impl<'a> /*trait*/ QLocale_system for () {
  fn system(self, rsthis: &mut QLocale) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale6systemEv()};
    let mut ret = unsafe {_ZN7QLocale6systemEv()};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn setDefault<T: QLocale_setDefault>(&mut self, value: T)  {
     value.setDefault(self);
    // return 1;
  }
}

pub trait QLocale_setDefault {
  fn setDefault(self, rsthis: &mut QLocale) ;
}

// proto: static void QLocale::setDefault(const QLocale & locale);
impl<'a> /*trait*/ QLocale_setDefault for (&'a  QLocale) {
  fn setDefault(self, rsthis: &mut QLocale)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale10setDefaultERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLocale10setDefaultERKS_(arg0)};
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn exponential<T: QLocale_exponential>(&mut self, value: T) -> QChar {
    return value.exponential(self);
    // return 1;
  }
}

pub trait QLocale_exponential {
  fn exponential(self, rsthis: &mut QLocale) -> QChar;
}

// proto:  QChar QLocale::exponential();
impl<'a> /*trait*/ QLocale_exponential for () {
  fn exponential(self, rsthis: &mut QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11exponentialEv()};
    let mut ret = unsafe {_ZNK7QLocale11exponentialEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QLocale::toCurrencyString(float , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (f32, &'a  QString) {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString {
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
impl<'a> /*trait*/ QLocale_toString for (i32) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
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
impl<'a> /*trait*/ QLocale_toString for (u32) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
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
impl<'a> /*trait*/ QLocale_toString for (i64) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
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
impl<'a> /*trait*/ QLocale_toString for (u64) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEy()};
    let arg0 = self  as uint64_t;
    let mut ret = unsafe {_ZNK7QLocale8toStringEy(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QLocale::toString(ushort i);
impl<'a> /*trait*/ QLocale_toString for (u16) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEt()};
    let arg0 = self  as c_ushort;
    let mut ret = unsafe {_ZNK7QLocale8toStringEt(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn amText<T: QLocale_amText>(&mut self, value: T) -> QString {
    return value.amText(self);
    // return 1;
  }
}

pub trait QLocale_amText {
  fn amText(self, rsthis: &mut QLocale) -> QString;
}

// proto:  QString QLocale::amText();
impl<'a> /*trait*/ QLocale_amText for () {
  fn amText(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6amTextEv()};
    let mut ret = unsafe {_ZNK7QLocale6amTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QLocale::toCurrencyString(int , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (i32, &'a  QString) {
  fn toCurrencyString(self, rsthis: &mut QLocale) -> QString {
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
impl<'a> /*trait*/ QLocale_toString for (i16) {
  fn toString(self, rsthis: &mut QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEs()};
    let arg0 = self  as c_short;
    let mut ret = unsafe {_ZNK7QLocale8toStringEs(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QLocale::NewQLocale();
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

impl /*struct*/ QLocale {
  pub fn toInt<T: QLocale_toInt>(&mut self, value: T) -> i32 {
    return value.toInt(self);
    // return 1;
  }
}

pub trait QLocale_toInt {
  fn toInt(self, rsthis: &mut QLocale) -> i32;
}

// proto:  int QLocale::toInt(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toInt for (&'a  QString, &'a mut i8) {
  fn toInt(self, rsthis: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale5toIntERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QLocale5toIntERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

