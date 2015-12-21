// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  qint64 QDate::daysTo(const QDate & );
  fn _ZNK5QDate6daysToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto:  QDate QDate::addYears(int years);
  fn _ZNK5QDate8addYearsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QDate::month();
  fn _ZNK5QDate5monthEv(qthis: *mut c_void) -> c_int;
  // proto:  qint64 QDate::toJulianDay();
  fn _ZNK5QDate11toJulianDayEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QDate::QDate(qint64 julianDay);
  fn _ZN5QDateC1Ex(qthis: *mut c_void, arg0: c_longlong);
  // proto:  void QDate::QDate();
  fn _ZN5QDateC1Ev(qthis: *mut c_void);
  // proto:  void QDate::getDate(int * year, int * month, int * day);
  fn _ZN5QDate7getDateEPiS0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int);
  // proto: static QDate QDate::currentDate();
  fn _ZN5QDate11currentDateEv() -> *mut c_void;
  // proto:  void QDate::QDate(int y, int m, int d);
  fn _ZN5QDateC1Eiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  int QDate::weekNumber(int * yearNum);
  fn _ZNK5QDate10weekNumberEPi(qthis: *mut c_void, arg0: *mut c_int) -> c_int;
  // proto:  QString QDate::toString(const QString & format);
  fn _ZNK5QDate8toStringERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QDate::dayOfYear();
  fn _ZNK5QDate9dayOfYearEv(qthis: *mut c_void) -> c_int;
  // proto:  int QDate::day();
  fn _ZNK5QDate3dayEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QDate::setDate(int year, int month, int day);
  fn _ZN5QDate7setDateEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) -> c_char;
  // proto:  bool QDate::isNull();
  fn _ZNK5QDate6isNullEv(qthis: *mut c_void) -> c_char;
  // proto: static QDate QDate::fromJulianDay(qint64 jd);
  fn _ZN5QDate13fromJulianDayEx(arg0: c_longlong) -> *mut c_void;
  // proto:  bool QDate::isValid();
  fn _ZNK5QDate7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QDate QDate::addDays(qint64 days);
  fn _ZNK5QDate7addDaysEx(qthis: *mut c_void, arg0: c_longlong) -> *mut c_void;
  // proto: static bool QDate::isValid(int y, int m, int d);
  fn _ZN5QDate7isValidEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> c_char;
  // proto:  int QDate::daysInMonth();
  fn _ZNK5QDate11daysInMonthEv(qthis: *mut c_void) -> c_int;
  // proto: static QDate QDate::fromString(const QString & s, const QString & format);
  fn _ZN5QDate10fromStringERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static bool QDate::isLeapYear(int year);
  fn _ZN5QDate10isLeapYearEi(arg0: c_int) -> c_char;
  // proto:  int QDate::daysInYear();
  fn _ZNK5QDate10daysInYearEv(qthis: *mut c_void) -> c_int;
  // proto:  int QDate::dayOfWeek();
  fn _ZNK5QDate9dayOfWeekEv(qthis: *mut c_void) -> c_int;
  // proto:  QDate QDate::addMonths(int months);
  fn _ZNK5QDate9addMonthsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QDate::year();
  fn _ZNK5QDate4yearEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QDate)=8
pub struct QDate {
  pub qclsinst: *mut c_void,
}

  // proto:  qint64 QDate::daysTo(const QDate & );
impl /*struct*/ QDate {
  pub fn daysTo<RetType, T: QDate_daysTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.daysTo(self);
    // return 1;
  }
}

pub trait QDate_daysTo<RetType> {
  fn daysTo(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  qint64 QDate::daysTo(const QDate & );
impl<'a> /*trait*/ QDate_daysTo<i64> for (QDate) {
  fn daysTo(self , rsthis: &mut QDate) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate6daysToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QDate6daysToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QDate QDate::addYears(int years);
impl /*struct*/ QDate {
  pub fn addYears<RetType, T: QDate_addYears<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addYears(self);
    // return 1;
  }
}

pub trait QDate_addYears<RetType> {
  fn addYears(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  QDate QDate::addYears(int years);
impl<'a> /*trait*/ QDate_addYears<QDate> for (i32) {
  fn addYears(self , rsthis: &mut QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate8addYearsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QDate8addYearsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QDate::month();
impl /*struct*/ QDate {
  pub fn month<RetType, T: QDate_month<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.month(self);
    // return 1;
  }
}

pub trait QDate_month<RetType> {
  fn month(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  int QDate::month();
impl<'a> /*trait*/ QDate_month<i32> for () {
  fn month(self , rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate5monthEv()};
    let mut ret = unsafe {_ZNK5QDate5monthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qint64 QDate::toJulianDay();
impl /*struct*/ QDate {
  pub fn toJulianDay<RetType, T: QDate_toJulianDay<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toJulianDay(self);
    // return 1;
  }
}

pub trait QDate_toJulianDay<RetType> {
  fn toJulianDay(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  qint64 QDate::toJulianDay();
impl<'a> /*trait*/ QDate_toJulianDay<i64> for () {
  fn toJulianDay(self , rsthis: &mut QDate) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate11toJulianDayEv()};
    let mut ret = unsafe {_ZNK5QDate11toJulianDayEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QDate::QDate(qint64 julianDay);
impl /*struct*/ QDate {
  pub fn NewQDate<T: QDate_NewQDate>(value: T) -> QDate {
    let rsthis = value.NewQDate();
    return rsthis;
    // return 1;
  }
}

pub trait QDate_NewQDate {
  fn NewQDate(self) -> QDate;
}

  // proto:  void QDate::QDate(qint64 julianDay);
impl<'a> /*trait*/ QDate_NewQDate for (i64) {
  fn NewQDate(self) -> QDate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDateC1Ex()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN5QDateC1Ex(qthis, arg0)};
    let rsthis = QDate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDate::QDate();
impl<'a> /*trait*/ QDate_NewQDate for () {
  fn NewQDate(self) -> QDate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDateC1Ev()};
    unsafe {_ZN5QDateC1Ev(qthis)};
    let rsthis = QDate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDate::getDate(int * year, int * month, int * day);
impl /*struct*/ QDate {
  pub fn getDate<RetType, T: QDate_getDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.getDate(self);
    // return 1;
  }
}

pub trait QDate_getDate<RetType> {
  fn getDate(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  void QDate::getDate(int * year, int * month, int * day);
impl<'a> /*trait*/ QDate_getDate<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn getDate(self , rsthis: &mut QDate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate7getDateEPiS0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN5QDate7getDateEPiS0_S0_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto: static QDate QDate::currentDate();
impl /*struct*/ QDate {
  pub fn currentDate_s<RetType, T: QDate_currentDate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentDate_s();
    // return 1;
  }
}

pub trait QDate_currentDate_s<RetType> {
  fn currentDate_s(self ) -> RetType;
}

  // proto: static QDate QDate::currentDate();
impl<'a> /*trait*/ QDate_currentDate_s<QDate> for () {
  fn currentDate_s(self ) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate11currentDateEv()};
    let mut ret = unsafe {_ZN5QDate11currentDateEv()};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QDate::QDate(int y, int m, int d);
impl<'a> /*trait*/ QDate_NewQDate for (i32, i32, i32) {
  fn NewQDate(self) -> QDate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDateC1Eiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN5QDateC1Eiii(qthis, arg0, arg1, arg2)};
    let rsthis = QDate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QDate::weekNumber(int * yearNum);
impl /*struct*/ QDate {
  pub fn weekNumber<RetType, T: QDate_weekNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.weekNumber(self);
    // return 1;
  }
}

pub trait QDate_weekNumber<RetType> {
  fn weekNumber(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  int QDate::weekNumber(int * yearNum);
impl<'a> /*trait*/ QDate_weekNumber<i32> for (&'a mut Vec<i32>) {
  fn weekNumber(self , rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate10weekNumberEPi()};
    let arg0 = self.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZNK5QDate10weekNumberEPi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QDate::toString(const QString & format);
impl /*struct*/ QDate {
  pub fn toString<RetType, T: QDate_toString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QDate_toString<RetType> {
  fn toString(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  QString QDate::toString(const QString & format);
impl<'a> /*trait*/ QDate_toString<QString> for (QString) {
  fn toString(self , rsthis: &mut QDate) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QDate8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QDate::dayOfYear();
impl /*struct*/ QDate {
  pub fn dayOfYear<RetType, T: QDate_dayOfYear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dayOfYear(self);
    // return 1;
  }
}

pub trait QDate_dayOfYear<RetType> {
  fn dayOfYear(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  int QDate::dayOfYear();
impl<'a> /*trait*/ QDate_dayOfYear<i32> for () {
  fn dayOfYear(self , rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9dayOfYearEv()};
    let mut ret = unsafe {_ZNK5QDate9dayOfYearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QDate::day();
impl /*struct*/ QDate {
  pub fn day<RetType, T: QDate_day<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.day(self);
    // return 1;
  }
}

pub trait QDate_day<RetType> {
  fn day(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  int QDate::day();
impl<'a> /*trait*/ QDate_day<i32> for () {
  fn day(self , rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate3dayEv()};
    let mut ret = unsafe {_ZNK5QDate3dayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QDate::setDate(int year, int month, int day);
impl /*struct*/ QDate {
  pub fn setDate<RetType, T: QDate_setDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDate(self);
    // return 1;
  }
}

pub trait QDate_setDate<RetType> {
  fn setDate(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  bool QDate::setDate(int year, int month, int day);
impl<'a> /*trait*/ QDate_setDate<i8> for (i32, i32, i32) {
  fn setDate(self , rsthis: &mut QDate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate7setDateEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN5QDate7setDateEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDate::isNull();
impl /*struct*/ QDate {
  pub fn isNull<RetType, T: QDate_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QDate_isNull<RetType> {
  fn isNull(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  bool QDate::isNull();
impl<'a> /*trait*/ QDate_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QDate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate6isNullEv()};
    let mut ret = unsafe {_ZNK5QDate6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QDate QDate::fromJulianDay(qint64 jd);
impl /*struct*/ QDate {
  pub fn fromJulianDay_s<RetType, T: QDate_fromJulianDay_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromJulianDay_s();
    // return 1;
  }
}

pub trait QDate_fromJulianDay_s<RetType> {
  fn fromJulianDay_s(self ) -> RetType;
}

  // proto: static QDate QDate::fromJulianDay(qint64 jd);
impl<'a> /*trait*/ QDate_fromJulianDay_s<QDate> for (i64) {
  fn fromJulianDay_s(self ) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate13fromJulianDayEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN5QDate13fromJulianDayEx(arg0)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDate::isValid();
impl /*struct*/ QDate {
  pub fn isValid<RetType, T: QDate_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QDate_isValid<RetType> {
  fn isValid(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  bool QDate::isValid();
impl<'a> /*trait*/ QDate_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QDate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate7isValidEv()};
    let mut ret = unsafe {_ZNK5QDate7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QDate QDate::addDays(qint64 days);
impl /*struct*/ QDate {
  pub fn addDays<RetType, T: QDate_addDays<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addDays(self);
    // return 1;
  }
}

pub trait QDate_addDays<RetType> {
  fn addDays(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  QDate QDate::addDays(qint64 days);
impl<'a> /*trait*/ QDate_addDays<QDate> for (i64) {
  fn addDays(self , rsthis: &mut QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate7addDaysEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK5QDate7addDaysEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static bool QDate::isValid(int y, int m, int d);
impl /*struct*/ QDate {
  pub fn isValid_s<RetType, T: QDate_isValid_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isValid_s();
    // return 1;
  }
}

pub trait QDate_isValid_s<RetType> {
  fn isValid_s(self ) -> RetType;
}

  // proto: static bool QDate::isValid(int y, int m, int d);
impl<'a> /*trait*/ QDate_isValid_s<i8> for (i32, i32, i32) {
  fn isValid_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate7isValidEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN5QDate7isValidEiii(arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QDate::daysInMonth();
impl /*struct*/ QDate {
  pub fn daysInMonth<RetType, T: QDate_daysInMonth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.daysInMonth(self);
    // return 1;
  }
}

pub trait QDate_daysInMonth<RetType> {
  fn daysInMonth(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  int QDate::daysInMonth();
impl<'a> /*trait*/ QDate_daysInMonth<i32> for () {
  fn daysInMonth(self , rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate11daysInMonthEv()};
    let mut ret = unsafe {_ZNK5QDate11daysInMonthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QDate QDate::fromString(const QString & s, const QString & format);
impl /*struct*/ QDate {
  pub fn fromString_s<RetType, T: QDate_fromString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_s();
    // return 1;
  }
}

pub trait QDate_fromString_s<RetType> {
  fn fromString_s(self ) -> RetType;
}

  // proto: static QDate QDate::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QDate_fromString_s<QDate> for (QString, QString) {
  fn fromString_s(self ) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QDate10fromStringERK7QStringS2_(arg0, arg1)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static bool QDate::isLeapYear(int year);
impl /*struct*/ QDate {
  pub fn isLeapYear_s<RetType, T: QDate_isLeapYear_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLeapYear_s();
    // return 1;
  }
}

pub trait QDate_isLeapYear_s<RetType> {
  fn isLeapYear_s(self ) -> RetType;
}

  // proto: static bool QDate::isLeapYear(int year);
impl<'a> /*trait*/ QDate_isLeapYear_s<i8> for (i32) {
  fn isLeapYear_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate10isLeapYearEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN5QDate10isLeapYearEi(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QDate::daysInYear();
impl /*struct*/ QDate {
  pub fn daysInYear<RetType, T: QDate_daysInYear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.daysInYear(self);
    // return 1;
  }
}

pub trait QDate_daysInYear<RetType> {
  fn daysInYear(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  int QDate::daysInYear();
impl<'a> /*trait*/ QDate_daysInYear<i32> for () {
  fn daysInYear(self , rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate10daysInYearEv()};
    let mut ret = unsafe {_ZNK5QDate10daysInYearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QDate::dayOfWeek();
impl /*struct*/ QDate {
  pub fn dayOfWeek<RetType, T: QDate_dayOfWeek<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dayOfWeek(self);
    // return 1;
  }
}

pub trait QDate_dayOfWeek<RetType> {
  fn dayOfWeek(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  int QDate::dayOfWeek();
impl<'a> /*trait*/ QDate_dayOfWeek<i32> for () {
  fn dayOfWeek(self , rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9dayOfWeekEv()};
    let mut ret = unsafe {_ZNK5QDate9dayOfWeekEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QDate QDate::addMonths(int months);
impl /*struct*/ QDate {
  pub fn addMonths<RetType, T: QDate_addMonths<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addMonths(self);
    // return 1;
  }
}

pub trait QDate_addMonths<RetType> {
  fn addMonths(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  QDate QDate::addMonths(int months);
impl<'a> /*trait*/ QDate_addMonths<QDate> for (i32) {
  fn addMonths(self , rsthis: &mut QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9addMonthsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QDate9addMonthsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QDate::year();
impl /*struct*/ QDate {
  pub fn year<RetType, T: QDate_year<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.year(self);
    // return 1;
  }
}

pub trait QDate_year<RetType> {
  fn year(self , rsthis: &mut QDate) -> RetType;
}

  // proto:  int QDate::year();
impl<'a> /*trait*/ QDate_year<i32> for () {
  fn year(self , rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate4yearEv()};
    let mut ret = unsafe {_ZNK5QDate4yearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

