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
  // proto:  long long QDate::daysTo(const QDate & );
  fn _ZNK5QDate6daysToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto:  QDate QDate::addYears(int years);
  fn _ZNK5QDate8addYearsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QDate::month();
  fn _ZNK5QDate5monthEv(qthis: *mut c_void) -> c_int;
  // proto:  long long QDate::toJulianDay();
  fn _ZNK5QDate11toJulianDayEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QDate::NewQDate(qint64 julianDay);
  fn _ZN5QDateC1Ex(qthis: *mut c_void, arg0: c_longlong) ;
  // proto:  void QDate::NewQDate();
  fn _ZN5QDateC1Ev(qthis: *mut c_void) ;
  // proto:  void QDate::getDate(int * year, int * month, int * day);
  fn _ZN5QDate7getDateEPiS0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int) ;
  // proto: static QDate QDate::currentDate();
  fn _ZN5QDate11currentDateEv() -> *mut c_void;
  // proto:  void QDate::NewQDate(int y, int m, int d);
  fn _ZN5QDateC1Eiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
  // proto:  int QDate::weekNumber(int * yearNum);
  fn _ZNK5QDate10weekNumberEPi(qthis: *mut c_void, arg0: *mut c_int) -> c_int;
  // proto:  QString QDate::toString(const QString & format);
  fn _ZNK5QDate8toStringERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QDate::dayOfYear();
  fn _ZNK5QDate9dayOfYearEv(qthis: *mut c_void) -> c_int;
  // proto:  int QDate::day();
  fn _ZNK5QDate3dayEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QDate::setDate(int year, int month, int day);
  fn _ZN5QDate7setDateEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) -> int8_t;
  // proto:  bool QDate::isNull();
  fn _ZNK5QDate6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto: static QDate QDate::fromJulianDay(qint64 jd);
  fn _ZN5QDate13fromJulianDayEx(arg0: c_longlong) -> *mut c_void;
  // proto:  bool QDate::isValid();
  fn _ZNK5QDate7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QDate QDate::addDays(qint64 days);
  fn _ZNK5QDate7addDaysEx(qthis: *mut c_void, arg0: c_longlong) -> *mut c_void;
  // proto: static bool QDate::isValid(int y, int m, int d);
  fn _ZN5QDate7isValidEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> int8_t;
  // proto:  int QDate::daysInMonth();
  fn _ZNK5QDate11daysInMonthEv(qthis: *mut c_void) -> c_int;
  // proto: static QDate QDate::fromString(const QString & s, const QString & format);
  fn _ZN5QDate10fromStringERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static bool QDate::isLeapYear(int year);
  fn _ZN5QDate10isLeapYearEi(arg0: c_int) -> int8_t;
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

impl /*struct*/ QDate {
  pub fn daysTo<T: QDate_daysTo>(&mut self, value: T) -> i64 {
    return value.daysTo(self);
    // return 1;
  }
}

pub trait QDate_daysTo {
  fn daysTo(self, rsthis: &mut QDate) -> i64;
}

// proto:  long long QDate::daysTo(const QDate & );
impl<'a> /*trait*/ QDate_daysTo for (&'a  QDate) {
  fn daysTo(self, rsthis: &mut QDate) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate6daysToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QDate6daysToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn addYears<T: QDate_addYears>(&mut self, value: T) -> QDate {
    return value.addYears(self);
    // return 1;
  }
}

pub trait QDate_addYears {
  fn addYears(self, rsthis: &mut QDate) -> QDate;
}

// proto:  QDate QDate::addYears(int years);
impl<'a> /*trait*/ QDate_addYears for (i32) {
  fn addYears(self, rsthis: &mut QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate8addYearsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QDate8addYearsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn month<T: QDate_month>(&mut self, value: T) -> i32 {
    return value.month(self);
    // return 1;
  }
}

pub trait QDate_month {
  fn month(self, rsthis: &mut QDate) -> i32;
}

// proto:  int QDate::month();
impl<'a> /*trait*/ QDate_month for () {
  fn month(self, rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate5monthEv()};
    let mut ret = unsafe {_ZNK5QDate5monthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn toJulianDay<T: QDate_toJulianDay>(&mut self, value: T) -> i64 {
    return value.toJulianDay(self);
    // return 1;
  }
}

pub trait QDate_toJulianDay {
  fn toJulianDay(self, rsthis: &mut QDate) -> i64;
}

// proto:  long long QDate::toJulianDay();
impl<'a> /*trait*/ QDate_toJulianDay for () {
  fn toJulianDay(self, rsthis: &mut QDate) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate11toJulianDayEv()};
    let mut ret = unsafe {_ZNK5QDate11toJulianDayEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

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

// proto: void QDate::NewQDate(qint64 julianDay);
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

// proto: void QDate::NewQDate();
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

impl /*struct*/ QDate {
  pub fn getDate<T: QDate_getDate>(&mut self, value: T)  {
     value.getDate(self);
    // return 1;
  }
}

pub trait QDate_getDate {
  fn getDate(self, rsthis: &mut QDate) ;
}

// proto:  void QDate::getDate(int * year, int * month, int * day);
impl<'a> /*trait*/ QDate_getDate for (&'a mut i32, &'a mut i32, &'a mut i32) {
  fn getDate(self, rsthis: &mut QDate)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate7getDateEPiS0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZN5QDate7getDateEPiS0_S0_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn currentDate<T: QDate_currentDate>(&mut self, value: T) -> QDate {
    return value.currentDate(self);
    // return 1;
  }
}

pub trait QDate_currentDate {
  fn currentDate(self, rsthis: &mut QDate) -> QDate;
}

// proto: static QDate QDate::currentDate();
impl<'a> /*trait*/ QDate_currentDate for () {
  fn currentDate(self, rsthis: &mut QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate11currentDateEv()};
    let mut ret = unsafe {_ZN5QDate11currentDateEv()};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QDate::NewQDate(int y, int m, int d);
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

impl /*struct*/ QDate {
  pub fn weekNumber<T: QDate_weekNumber>(&mut self, value: T) -> i32 {
    return value.weekNumber(self);
    // return 1;
  }
}

pub trait QDate_weekNumber {
  fn weekNumber(self, rsthis: &mut QDate) -> i32;
}

// proto:  int QDate::weekNumber(int * yearNum);
impl<'a> /*trait*/ QDate_weekNumber for (&'a mut i32) {
  fn weekNumber(self, rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate10weekNumberEPi()};
    let arg0 = self  as *mut c_int;
    let mut ret = unsafe {_ZNK5QDate10weekNumberEPi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn toString<T: QDate_toString>(&mut self, value: T) -> QString {
    return value.toString(self);
    // return 1;
  }
}

pub trait QDate_toString {
  fn toString(self, rsthis: &mut QDate) -> QString;
}

// proto:  QString QDate::toString(const QString & format);
impl<'a> /*trait*/ QDate_toString for (&'a  QString) {
  fn toString(self, rsthis: &mut QDate) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QDate8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn dayOfYear<T: QDate_dayOfYear>(&mut self, value: T) -> i32 {
    return value.dayOfYear(self);
    // return 1;
  }
}

pub trait QDate_dayOfYear {
  fn dayOfYear(self, rsthis: &mut QDate) -> i32;
}

// proto:  int QDate::dayOfYear();
impl<'a> /*trait*/ QDate_dayOfYear for () {
  fn dayOfYear(self, rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9dayOfYearEv()};
    let mut ret = unsafe {_ZNK5QDate9dayOfYearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn day<T: QDate_day>(&mut self, value: T) -> i32 {
    return value.day(self);
    // return 1;
  }
}

pub trait QDate_day {
  fn day(self, rsthis: &mut QDate) -> i32;
}

// proto:  int QDate::day();
impl<'a> /*trait*/ QDate_day for () {
  fn day(self, rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate3dayEv()};
    let mut ret = unsafe {_ZNK5QDate3dayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn setDate<T: QDate_setDate>(&mut self, value: T) -> i8 {
    return value.setDate(self);
    // return 1;
  }
}

pub trait QDate_setDate {
  fn setDate(self, rsthis: &mut QDate) -> i8;
}

// proto:  bool QDate::setDate(int year, int month, int day);
impl<'a> /*trait*/ QDate_setDate for (i32, i32, i32) {
  fn setDate(self, rsthis: &mut QDate) -> i8 {
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

impl /*struct*/ QDate {
  pub fn isNull<T: QDate_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QDate_isNull {
  fn isNull(self, rsthis: &mut QDate) -> i8;
}

// proto:  bool QDate::isNull();
impl<'a> /*trait*/ QDate_isNull for () {
  fn isNull(self, rsthis: &mut QDate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate6isNullEv()};
    let mut ret = unsafe {_ZNK5QDate6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn fromJulianDay<T: QDate_fromJulianDay>(&mut self, value: T) -> QDate {
    return value.fromJulianDay(self);
    // return 1;
  }
}

pub trait QDate_fromJulianDay {
  fn fromJulianDay(self, rsthis: &mut QDate) -> QDate;
}

// proto: static QDate QDate::fromJulianDay(qint64 jd);
impl<'a> /*trait*/ QDate_fromJulianDay for (i64) {
  fn fromJulianDay(self, rsthis: &mut QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate13fromJulianDayEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN5QDate13fromJulianDayEx(arg0)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn isValid<T: QDate_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QDate_isValid {
  fn isValid(self, rsthis: &mut QDate) -> i8;
}

// proto:  bool QDate::isValid();
impl<'a> /*trait*/ QDate_isValid for () {
  fn isValid(self, rsthis: &mut QDate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate7isValidEv()};
    let mut ret = unsafe {_ZNK5QDate7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn addDays<T: QDate_addDays>(&mut self, value: T) -> QDate {
    return value.addDays(self);
    // return 1;
  }
}

pub trait QDate_addDays {
  fn addDays(self, rsthis: &mut QDate) -> QDate;
}

// proto:  QDate QDate::addDays(qint64 days);
impl<'a> /*trait*/ QDate_addDays for (i64) {
  fn addDays(self, rsthis: &mut QDate) -> QDate {
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
impl<'a> /*trait*/ QDate_isValid for (i32, i32, i32) {
  fn isValid(self, rsthis: &mut QDate) -> i8 {
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

impl /*struct*/ QDate {
  pub fn daysInMonth<T: QDate_daysInMonth>(&mut self, value: T) -> i32 {
    return value.daysInMonth(self);
    // return 1;
  }
}

pub trait QDate_daysInMonth {
  fn daysInMonth(self, rsthis: &mut QDate) -> i32;
}

// proto:  int QDate::daysInMonth();
impl<'a> /*trait*/ QDate_daysInMonth for () {
  fn daysInMonth(self, rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate11daysInMonthEv()};
    let mut ret = unsafe {_ZNK5QDate11daysInMonthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn fromString<T: QDate_fromString>(&mut self, value: T) -> QDate {
    return value.fromString(self);
    // return 1;
  }
}

pub trait QDate_fromString {
  fn fromString(self, rsthis: &mut QDate) -> QDate;
}

// proto: static QDate QDate::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QDate_fromString for (&'a  QString, &'a  QString) {
  fn fromString(self, rsthis: &mut QDate) -> QDate {
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

impl /*struct*/ QDate {
  pub fn isLeapYear<T: QDate_isLeapYear>(&mut self, value: T) -> i8 {
    return value.isLeapYear(self);
    // return 1;
  }
}

pub trait QDate_isLeapYear {
  fn isLeapYear(self, rsthis: &mut QDate) -> i8;
}

// proto: static bool QDate::isLeapYear(int year);
impl<'a> /*trait*/ QDate_isLeapYear for (i32) {
  fn isLeapYear(self, rsthis: &mut QDate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate10isLeapYearEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN5QDate10isLeapYearEi(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn daysInYear<T: QDate_daysInYear>(&mut self, value: T) -> i32 {
    return value.daysInYear(self);
    // return 1;
  }
}

pub trait QDate_daysInYear {
  fn daysInYear(self, rsthis: &mut QDate) -> i32;
}

// proto:  int QDate::daysInYear();
impl<'a> /*trait*/ QDate_daysInYear for () {
  fn daysInYear(self, rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate10daysInYearEv()};
    let mut ret = unsafe {_ZNK5QDate10daysInYearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn dayOfWeek<T: QDate_dayOfWeek>(&mut self, value: T) -> i32 {
    return value.dayOfWeek(self);
    // return 1;
  }
}

pub trait QDate_dayOfWeek {
  fn dayOfWeek(self, rsthis: &mut QDate) -> i32;
}

// proto:  int QDate::dayOfWeek();
impl<'a> /*trait*/ QDate_dayOfWeek for () {
  fn dayOfWeek(self, rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9dayOfWeekEv()};
    let mut ret = unsafe {_ZNK5QDate9dayOfWeekEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn addMonths<T: QDate_addMonths>(&mut self, value: T) -> QDate {
    return value.addMonths(self);
    // return 1;
  }
}

pub trait QDate_addMonths {
  fn addMonths(self, rsthis: &mut QDate) -> QDate;
}

// proto:  QDate QDate::addMonths(int months);
impl<'a> /*trait*/ QDate_addMonths for (i32) {
  fn addMonths(self, rsthis: &mut QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9addMonthsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QDate9addMonthsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn year<T: QDate_year>(&mut self, value: T) -> i32 {
    return value.year(self);
    // return 1;
  }
}

pub trait QDate_year {
  fn year(self, rsthis: &mut QDate) -> i32;
}

// proto:  int QDate::year();
impl<'a> /*trait*/ QDate_year for () {
  fn year(self, rsthis: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate4yearEv()};
    let mut ret = unsafe {_ZNK5QDate4yearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

