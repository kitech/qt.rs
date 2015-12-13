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
use super::qdatetime::QDateTime;
use super::qtime::QTime;
use super::qdate::QDate;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK7QLocale6pmTextEv() -> i32;
  fn _ZNK7QLocale18nativeLanguageNameEv() -> i32;
  fn _ZNK7QLocale7toLowerERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK7QLocale9zeroDigitEv() -> i32;
  fn _ZNK7QLocale4nameEv() -> i32;
  fn _ZNK7QLocale16toCurrencyStringExRK7QString(arg0: c_longlong, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale7toFloatERK7QStringPb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
  fn _ZN7QLocale1cEv() -> i32;
  fn _ZNK7QLocale16toCurrencyStringEjRK7QString(arg0: c_uint, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale19createSeparatedListERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZNK7QLocale6toUIntERK7QStringPb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
  fn _ZNK7QLocale12decimalPointEv() -> i32;
  fn _ZNK7QLocale12positiveSignEv() -> i32;
  fn _ZNK7QLocale10toLongLongERK7QStringPb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
  fn _ZNK7QLocale7toShortERK7QStringPb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
  fn _ZNK7QLocale8toStringEfci(arg0: c_float, arg1: c_char, arg2: c_int) -> i32;
  fn _ZNK7QLocale8toStringERK9QDateTimeRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale10toDateTimeERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale16toCurrencyStringEsRK7QString(arg0: c_short, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale14groupSeparatorEv() -> i32;
  fn _ZNK7QLocale16toCurrencyStringEdRK7QString(arg0: c_double, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale16toCurrencyStringEyRK7QString(arg0: uint64_t, arg1: *const c_void) -> i32;
  fn _ZN7QLocaleC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK7QLocale8toStringERK5QTimeRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale6toDateERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale17nativeCountryNameEv() -> i32;
  fn _ZNK7QLocale12negativeSignEv() -> i32;
  fn _ZN7QLocaleD0Ev() -> i32;
  fn _ZN7QLocaleC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK7QLocale8toStringERK5QDateRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale7toUpperERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK7QLocale7percentEv() -> i32;
  fn _ZNK7QLocale11toULongLongERK7QStringPb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
  fn _ZNK7QLocale8toStringEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> i32;
  fn _ZNK7QLocale11uiLanguagesEv() -> i32;
  fn _ZNK7QLocale9bcp47NameEv() -> i32;
  fn _ZNK7QLocale6toTimeERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale8toUShortERK7QStringPb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
  fn _ZNK7QLocale16toCurrencyStringEtRK7QString(arg0: c_ushort, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale8toDoubleERK7QStringPb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
  fn _ZN7QLocale6systemEv() -> i32;
  fn _ZN7QLocale10setDefaultERKS_(arg0: *const c_void) -> i32;
  fn _ZN7QLocale16setNumberOptionsE6QFlagsINS_12NumberOptionEE(arg0: c_int) -> i32;
  fn _ZNK7QLocale11exponentialEv() -> i32;
  fn _ZNK7QLocale16toCurrencyStringEfRK7QString(arg0: c_float, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale8toStringEi(arg0: c_int) -> i32;
  fn _ZNK7QLocale8toStringEj(arg0: c_uint) -> i32;
  fn _ZNK7QLocale8toStringEx(arg0: c_longlong) -> i32;
  fn _ZNK7QLocale8toStringEy(arg0: uint64_t) -> i32;
  fn _ZNK7QLocale8toStringEt(arg0: c_ushort) -> i32;
  fn _ZNK7QLocale6amTextEv() -> i32;
  fn _ZNK7QLocale16toCurrencyStringEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  fn _ZNK7QLocale8toStringEs(arg0: c_short) -> i32;
  fn _ZN7QLocaleC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK7QLocale5toIntERK7QStringPb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
}

// body block begin
// class sizeof(QLocale)=1
pub struct QLocale {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLocale {
  pub fn pmText<T: QLocale_pmText>(&mut self, value: T) -> i32 {
    value.pmText(self);
    return 1;
  }
}

pub trait QLocale_pmText {
  fn pmText(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::pmText();
impl<'a> /*trait*/ QLocale_pmText for () {
  fn pmText(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6pmTextEv()};
    unsafe {_ZNK7QLocale6pmTextEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn nativeLanguageName<T: QLocale_nativeLanguageName>(&mut self, value: T) -> i32 {
    value.nativeLanguageName(self);
    return 1;
  }
}

pub trait QLocale_nativeLanguageName {
  fn nativeLanguageName(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::nativeLanguageName();
impl<'a> /*trait*/ QLocale_nativeLanguageName for () {
  fn nativeLanguageName(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale18nativeLanguageNameEv()};
    unsafe {_ZNK7QLocale18nativeLanguageNameEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toLower<T: QLocale_toLower>(&mut self, value: T) -> i32 {
    value.toLower(self);
    return 1;
  }
}

pub trait QLocale_toLower {
  fn toLower(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::toLower(const QString & str);
impl<'a> /*trait*/ QLocale_toLower for (&'a  QString) {
  fn toLower(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toLowerERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale7toLowerERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn zeroDigit<T: QLocale_zeroDigit>(&mut self, value: T) -> i32 {
    value.zeroDigit(self);
    return 1;
  }
}

pub trait QLocale_zeroDigit {
  fn zeroDigit(self, this: &mut QLocale) -> i32;
}

// proto: QChar QLocale::zeroDigit();
impl<'a> /*trait*/ QLocale_zeroDigit for () {
  fn zeroDigit(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale9zeroDigitEv()};
    unsafe {_ZNK7QLocale9zeroDigitEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn name<T: QLocale_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QLocale_name {
  fn name(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::name();
impl<'a> /*trait*/ QLocale_name for () {
  fn name(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale4nameEv()};
    unsafe {_ZNK7QLocale4nameEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toCurrencyString<T: QLocale_toCurrencyString>(&mut self, value: T) -> i32 {
    value.toCurrencyString(self);
    return 1;
  }
}

pub trait QLocale_toCurrencyString {
  fn toCurrencyString(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::toCurrencyString(qlonglong , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (i64, &'a  QString) {
  fn toCurrencyString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringExRK7QString()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale16toCurrencyStringExRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toFloat<T: QLocale_toFloat>(&mut self, value: T) -> i32 {
    value.toFloat(self);
    return 1;
  }
}

pub trait QLocale_toFloat {
  fn toFloat(self, this: &mut QLocale) -> i32;
}

// proto: float QLocale::toFloat(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toFloat for (&'a  QString, &'a mut i8) {
  fn toFloat(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toFloatERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK7QLocale7toFloatERK7QStringPb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn c<T: QLocale_c>(&mut self, value: T) -> i32 {
    value.c(self);
    return 1;
  }
}

pub trait QLocale_c {
  fn c(self, this: &mut QLocale) -> i32;
}

// proto: QLocale QLocale::c();
impl<'a> /*trait*/ QLocale_c for () {
  fn c(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale1cEv()};
    unsafe {_ZN7QLocale1cEv()};
    return 1;
  }
}

// proto: QString QLocale::toCurrencyString(uint , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (u32, &'a  QString) {
  fn toCurrencyString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEjRK7QString()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale16toCurrencyStringEjRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn createSeparatedList<T: QLocale_createSeparatedList>(&mut self, value: T) -> i32 {
    value.createSeparatedList(self);
    return 1;
  }
}

pub trait QLocale_createSeparatedList {
  fn createSeparatedList(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::createSeparatedList(const QStringList & strl);
impl<'a> /*trait*/ QLocale_createSeparatedList for (&'a  QStringList) {
  fn createSeparatedList(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale19createSeparatedListERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale19createSeparatedListERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toUInt<T: QLocale_toUInt>(&mut self, value: T) -> i32 {
    value.toUInt(self);
    return 1;
  }
}

pub trait QLocale_toUInt {
  fn toUInt(self, this: &mut QLocale) -> i32;
}

// proto: unsigned int QLocale::toUInt(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toUInt for (&'a  QString, &'a mut i8) {
  fn toUInt(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toUIntERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK7QLocale6toUIntERK7QStringPb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn decimalPoint<T: QLocale_decimalPoint>(&mut self, value: T) -> i32 {
    value.decimalPoint(self);
    return 1;
  }
}

pub trait QLocale_decimalPoint {
  fn decimalPoint(self, this: &mut QLocale) -> i32;
}

// proto: QChar QLocale::decimalPoint();
impl<'a> /*trait*/ QLocale_decimalPoint for () {
  fn decimalPoint(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12decimalPointEv()};
    unsafe {_ZNK7QLocale12decimalPointEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn positiveSign<T: QLocale_positiveSign>(&mut self, value: T) -> i32 {
    value.positiveSign(self);
    return 1;
  }
}

pub trait QLocale_positiveSign {
  fn positiveSign(self, this: &mut QLocale) -> i32;
}

// proto: QChar QLocale::positiveSign();
impl<'a> /*trait*/ QLocale_positiveSign for () {
  fn positiveSign(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12positiveSignEv()};
    unsafe {_ZNK7QLocale12positiveSignEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toLongLong<T: QLocale_toLongLong>(&mut self, value: T) -> i32 {
    value.toLongLong(self);
    return 1;
  }
}

pub trait QLocale_toLongLong {
  fn toLongLong(self, this: &mut QLocale) -> i32;
}

// proto: qint64 QLocale::toLongLong(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toLongLong for (&'a  QString, &'a mut i8) {
  fn toLongLong(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale10toLongLongERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK7QLocale10toLongLongERK7QStringPb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toShort<T: QLocale_toShort>(&mut self, value: T) -> i32 {
    value.toShort(self);
    return 1;
  }
}

pub trait QLocale_toShort {
  fn toShort(self, this: &mut QLocale) -> i32;
}

// proto: short QLocale::toShort(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toShort for (&'a  QString, &'a mut i8) {
  fn toShort(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toShortERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK7QLocale7toShortERK7QStringPb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toString<T: QLocale_toString>(&mut self, value: T) -> i32 {
    value.toString(self);
    return 1;
  }
}

pub trait QLocale_toString {
  fn toString(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::toString(float i, char f, int prec);
impl<'a> /*trait*/ QLocale_toString for (f32, i8, i32) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEfci()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK7QLocale8toStringEfci(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: QString QLocale::toString(const QDateTime & dateTime, const QString & format);
impl<'a> /*trait*/ QLocale_toString for (&'a  QDateTime, &'a  QString) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK9QDateTimeRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale8toStringERK9QDateTimeRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toDateTime<T: QLocale_toDateTime>(&mut self, value: T) -> i32 {
    value.toDateTime(self);
    return 1;
  }
}

pub trait QLocale_toDateTime {
  fn toDateTime(self, this: &mut QLocale) -> i32;
}

// proto: QDateTime QLocale::toDateTime(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toDateTime for (&'a  QString, &'a  QString) {
  fn toDateTime(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale10toDateTimeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale10toDateTimeERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: QString QLocale::toCurrencyString(short , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (i16, &'a  QString) {
  fn toCurrencyString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEsRK7QString()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale16toCurrencyStringEsRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn groupSeparator<T: QLocale_groupSeparator>(&mut self, value: T) -> i32 {
    value.groupSeparator(self);
    return 1;
  }
}

pub trait QLocale_groupSeparator {
  fn groupSeparator(self, this: &mut QLocale) -> i32;
}

// proto: QChar QLocale::groupSeparator();
impl<'a> /*trait*/ QLocale_groupSeparator for () {
  fn groupSeparator(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale14groupSeparatorEv()};
    unsafe {_ZNK7QLocale14groupSeparatorEv()};
    return 1;
  }
}

// proto: QString QLocale::toCurrencyString(double , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (f64, &'a  QString) {
  fn toCurrencyString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEdRK7QString()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale16toCurrencyStringEdRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: QString QLocale::toCurrencyString(qulonglong , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (u64, &'a  QString) {
  fn toCurrencyString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEyRK7QString()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale16toCurrencyStringEyRK7QString(arg0, arg1)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QLocaleC1ERK7QString(qthis, arg0)};
    let rsthis = QLocale{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QString QLocale::toString(const QTime & time, const QString & formatStr);
impl<'a> /*trait*/ QLocale_toString for (&'a  QTime, &'a  QString) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK5QTimeRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale8toStringERK5QTimeRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toDate<T: QLocale_toDate>(&mut self, value: T) -> i32 {
    value.toDate(self);
    return 1;
  }
}

pub trait QLocale_toDate {
  fn toDate(self, this: &mut QLocale) -> i32;
}

// proto: QDate QLocale::toDate(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toDate for (&'a  QString, &'a  QString) {
  fn toDate(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toDateERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale6toDateERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn nativeCountryName<T: QLocale_nativeCountryName>(&mut self, value: T) -> i32 {
    value.nativeCountryName(self);
    return 1;
  }
}

pub trait QLocale_nativeCountryName {
  fn nativeCountryName(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::nativeCountryName();
impl<'a> /*trait*/ QLocale_nativeCountryName for () {
  fn nativeCountryName(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale17nativeCountryNameEv()};
    unsafe {_ZNK7QLocale17nativeCountryNameEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn negativeSign<T: QLocale_negativeSign>(&mut self, value: T) -> i32 {
    value.negativeSign(self);
    return 1;
  }
}

pub trait QLocale_negativeSign {
  fn negativeSign(self, this: &mut QLocale) -> i32;
}

// proto: QChar QLocale::negativeSign();
impl<'a> /*trait*/ QLocale_negativeSign for () {
  fn negativeSign(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12negativeSignEv()};
    unsafe {_ZNK7QLocale12negativeSignEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn FreeQLocale<T: QLocale_FreeQLocale>(&mut self, value: T) -> i32 {
    value.FreeQLocale(self);
    return 1;
  }
}

pub trait QLocale_FreeQLocale {
  fn FreeQLocale(self, this: &mut QLocale) -> i32;
}

// proto: void QLocale::FreeQLocale();
impl<'a> /*trait*/ QLocale_FreeQLocale for () {
  fn FreeQLocale(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleD0Ev()};
    unsafe {_ZN7QLocaleD0Ev()};
    return 1;
  }
}

// proto: void QLocale::NewQLocale(const QLocale & other);
impl<'a> /*trait*/ QLocale_NewQLocale for (&'a  QLocale) {
  fn NewQLocale(self) -> QLocale {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QLocaleC1ERKS_(qthis, arg0)};
    let rsthis = QLocale{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QString QLocale::toString(const QDate & date, const QString & formatStr);
impl<'a> /*trait*/ QLocale_toString for (&'a  QDate, &'a  QString) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK5QDateRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale8toStringERK5QDateRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toUpper<T: QLocale_toUpper>(&mut self, value: T) -> i32 {
    value.toUpper(self);
    return 1;
  }
}

pub trait QLocale_toUpper {
  fn toUpper(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::toUpper(const QString & str);
impl<'a> /*trait*/ QLocale_toUpper for (&'a  QString) {
  fn toUpper(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toUpperERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale7toUpperERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn percent<T: QLocale_percent>(&mut self, value: T) -> i32 {
    value.percent(self);
    return 1;
  }
}

pub trait QLocale_percent {
  fn percent(self, this: &mut QLocale) -> i32;
}

// proto: QChar QLocale::percent();
impl<'a> /*trait*/ QLocale_percent for () {
  fn percent(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7percentEv()};
    unsafe {_ZNK7QLocale7percentEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toULongLong<T: QLocale_toULongLong>(&mut self, value: T) -> i32 {
    value.toULongLong(self);
    return 1;
  }
}

pub trait QLocale_toULongLong {
  fn toULongLong(self, this: &mut QLocale) -> i32;
}

// proto: quint64 QLocale::toULongLong(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toULongLong for (&'a  QString, &'a mut i8) {
  fn toULongLong(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11toULongLongERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK7QLocale11toULongLongERK7QStringPb(arg0, arg1)};
    return 1;
  }
}

// proto: QString QLocale::toString(double i, char f, int prec);
impl<'a> /*trait*/ QLocale_toString for (f64, i8, i32) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK7QLocale8toStringEdci(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn uiLanguages<T: QLocale_uiLanguages>(&mut self, value: T) -> i32 {
    value.uiLanguages(self);
    return 1;
  }
}

pub trait QLocale_uiLanguages {
  fn uiLanguages(self, this: &mut QLocale) -> i32;
}

// proto: QStringList QLocale::uiLanguages();
impl<'a> /*trait*/ QLocale_uiLanguages for () {
  fn uiLanguages(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11uiLanguagesEv()};
    unsafe {_ZNK7QLocale11uiLanguagesEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn bcp47Name<T: QLocale_bcp47Name>(&mut self, value: T) -> i32 {
    value.bcp47Name(self);
    return 1;
  }
}

pub trait QLocale_bcp47Name {
  fn bcp47Name(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::bcp47Name();
impl<'a> /*trait*/ QLocale_bcp47Name for () {
  fn bcp47Name(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale9bcp47NameEv()};
    unsafe {_ZNK7QLocale9bcp47NameEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toTime<T: QLocale_toTime>(&mut self, value: T) -> i32 {
    value.toTime(self);
    return 1;
  }
}

pub trait QLocale_toTime {
  fn toTime(self, this: &mut QLocale) -> i32;
}

// proto: QTime QLocale::toTime(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toTime for (&'a  QString, &'a  QString) {
  fn toTime(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toTimeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale6toTimeERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toUShort<T: QLocale_toUShort>(&mut self, value: T) -> i32 {
    value.toUShort(self);
    return 1;
  }
}

pub trait QLocale_toUShort {
  fn toUShort(self, this: &mut QLocale) -> i32;
}

// proto: unsigned short QLocale::toUShort(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toUShort for (&'a  QString, &'a mut i8) {
  fn toUShort(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toUShortERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK7QLocale8toUShortERK7QStringPb(arg0, arg1)};
    return 1;
  }
}

// proto: QString QLocale::toCurrencyString(ushort , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (u16, &'a  QString) {
  fn toCurrencyString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEtRK7QString()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale16toCurrencyStringEtRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn toDouble<T: QLocale_toDouble>(&mut self, value: T) -> i32 {
    value.toDouble(self);
    return 1;
  }
}

pub trait QLocale_toDouble {
  fn toDouble(self, this: &mut QLocale) -> i32;
}

// proto: double QLocale::toDouble(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toDouble for (&'a  QString, &'a mut i8) {
  fn toDouble(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toDoubleERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK7QLocale8toDoubleERK7QStringPb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn system<T: QLocale_system>(&mut self, value: T) -> i32 {
    value.system(self);
    return 1;
  }
}

pub trait QLocale_system {
  fn system(self, this: &mut QLocale) -> i32;
}

// proto: QLocale QLocale::system();
impl<'a> /*trait*/ QLocale_system for () {
  fn system(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale6systemEv()};
    unsafe {_ZN7QLocale6systemEv()};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn setDefault<T: QLocale_setDefault>(&mut self, value: T) -> i32 {
    value.setDefault(self);
    return 1;
  }
}

pub trait QLocale_setDefault {
  fn setDefault(self, this: &mut QLocale) -> i32;
}

// proto: void QLocale::setDefault(const QLocale & locale);
impl<'a> /*trait*/ QLocale_setDefault for (&'a  QLocale) {
  fn setDefault(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale10setDefaultERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QLocale10setDefaultERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn setNumberOptions<T: QLocale_setNumberOptions>(&mut self, value: T) -> i32 {
    value.setNumberOptions(self);
    return 1;
  }
}

pub trait QLocale_setNumberOptions {
  fn setNumberOptions(self, this: &mut QLocale) -> i32;
}

// proto: void QLocale::setNumberOptions(NumberOptions options);
impl<'a> /*trait*/ QLocale_setNumberOptions for (i32) {
  fn setNumberOptions(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale16setNumberOptionsE6QFlagsINS_12NumberOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QLocale16setNumberOptionsE6QFlagsINS_12NumberOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn exponential<T: QLocale_exponential>(&mut self, value: T) -> i32 {
    value.exponential(self);
    return 1;
  }
}

pub trait QLocale_exponential {
  fn exponential(self, this: &mut QLocale) -> i32;
}

// proto: QChar QLocale::exponential();
impl<'a> /*trait*/ QLocale_exponential for () {
  fn exponential(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11exponentialEv()};
    unsafe {_ZNK7QLocale11exponentialEv()};
    return 1;
  }
}

// proto: QString QLocale::toCurrencyString(float , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (f32, &'a  QString) {
  fn toCurrencyString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEfRK7QString()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale16toCurrencyStringEfRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: QString QLocale::toString(int i);
impl<'a> /*trait*/ QLocale_toString for (i32) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QLocale8toStringEi(arg0)};
    return 1;
  }
}

// proto: QString QLocale::toString(uint i);
impl<'a> /*trait*/ QLocale_toString for (u32) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZNK7QLocale8toStringEj(arg0)};
    return 1;
  }
}

// proto: QString QLocale::toString(qlonglong i);
impl<'a> /*trait*/ QLocale_toString for (i64) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZNK7QLocale8toStringEx(arg0)};
    return 1;
  }
}

// proto: QString QLocale::toString(qulonglong i);
impl<'a> /*trait*/ QLocale_toString for (u64) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEy()};
    let arg0 = self  as uint64_t;
    unsafe {_ZNK7QLocale8toStringEy(arg0)};
    return 1;
  }
}

// proto: QString QLocale::toString(ushort i);
impl<'a> /*trait*/ QLocale_toString for (u16) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEt()};
    let arg0 = self  as c_ushort;
    unsafe {_ZNK7QLocale8toStringEt(arg0)};
    return 1;
  }
}

impl /*struct*/ QLocale {
  pub fn amText<T: QLocale_amText>(&mut self, value: T) -> i32 {
    value.amText(self);
    return 1;
  }
}

pub trait QLocale_amText {
  fn amText(self, this: &mut QLocale) -> i32;
}

// proto: QString QLocale::amText();
impl<'a> /*trait*/ QLocale_amText for () {
  fn amText(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6amTextEv()};
    unsafe {_ZNK7QLocale6amTextEv()};
    return 1;
  }
}

// proto: QString QLocale::toCurrencyString(int , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString for (i32, &'a  QString) {
  fn toCurrencyString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QLocale16toCurrencyStringEiRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: QString QLocale::toString(short i);
impl<'a> /*trait*/ QLocale_toString for (i16) {
  fn toString(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEs()};
    let arg0 = self  as c_short;
    unsafe {_ZNK7QLocale8toStringEs(arg0)};
    return 1;
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
    value.toInt(self);
    return 1;
  }
}

pub trait QLocale_toInt {
  fn toInt(self, this: &mut QLocale) -> i32;
}

// proto: int QLocale::toInt(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toInt for (&'a  QString, &'a mut i8) {
  fn toInt(self, this: &mut QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale5toIntERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK7QLocale5toIntERK7QStringPb(arg0, arg1)};
    return 1;
  }
}

