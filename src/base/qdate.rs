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
  fn _ZNK5QDate6daysToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK5QDate8addYearsEi(arg0: c_int) -> i32;
  fn _ZNK5QDate5monthEv() -> i32;
  fn _ZNK5QDate11toJulianDayEv() -> i32;
  fn _ZN5QDateC1Ex(qthis: *mut c_void, arg0: c_longlong) -> i32;
  fn _ZN5QDateC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN5QDate7getDateEPiS0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int) -> i32;
  fn _ZN5QDate11currentDateEv() -> i32;
  fn _ZN5QDateC1Eiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  fn _ZNK5QDate10weekNumberEPi(arg0: *mut c_int) -> i32;
  fn _ZNK5QDate8toStringERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK5QDate9dayOfYearEv() -> i32;
  fn _ZNK5QDate3dayEv() -> i32;
  fn _ZN5QDate7setDateEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  fn _ZNK5QDate6isNullEv() -> i32;
  fn _ZN5QDate13fromJulianDayEx(arg0: c_longlong) -> i32;
  fn _ZNK5QDate7isValidEv() -> i32;
  fn _ZNK5QDate7addDaysEx(arg0: c_longlong) -> i32;
  fn _ZN5QDate7isValidEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  fn _ZNK5QDate11daysInMonthEv() -> i32;
  fn _ZN5QDate10fromStringERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QDate10isLeapYearEi(arg0: c_int) -> i32;
  fn _ZNK5QDate10daysInYearEv() -> i32;
  fn _ZNK5QDate9dayOfWeekEv() -> i32;
  fn _ZNK5QDate9addMonthsEi(arg0: c_int) -> i32;
  fn _ZNK5QDate4yearEv() -> i32;
}

// body block begin
// class sizeof(QDate)=8
pub struct QDate {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDate {
  pub fn daysTo<T: QDate_daysTo>(&mut self, value: T) -> i32 {
    value.daysTo(self);
    return 1;
  }
}

pub trait QDate_daysTo {
  fn daysTo(self, this: &mut QDate) -> i32;
}

// proto: long long QDate::daysTo(const QDate & );
impl<'a> /*trait*/ QDate_daysTo for (&'a  QDate) {
  fn daysTo(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate6daysToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QDate6daysToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn addYears<T: QDate_addYears>(&mut self, value: T) -> i32 {
    value.addYears(self);
    return 1;
  }
}

pub trait QDate_addYears {
  fn addYears(self, this: &mut QDate) -> i32;
}

// proto: QDate QDate::addYears(int years);
impl<'a> /*trait*/ QDate_addYears for (i32) {
  fn addYears(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate8addYearsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK5QDate8addYearsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn month<T: QDate_month>(&mut self, value: T) -> i32 {
    value.month(self);
    return 1;
  }
}

pub trait QDate_month {
  fn month(self, this: &mut QDate) -> i32;
}

// proto: int QDate::month();
impl<'a> /*trait*/ QDate_month for () {
  fn month(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate5monthEv()};
    unsafe {_ZNK5QDate5monthEv()};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn toJulianDay<T: QDate_toJulianDay>(&mut self, value: T) -> i32 {
    value.toJulianDay(self);
    return 1;
  }
}

pub trait QDate_toJulianDay {
  fn toJulianDay(self, this: &mut QDate) -> i32;
}

// proto: long long QDate::toJulianDay();
impl<'a> /*trait*/ QDate_toJulianDay for () {
  fn toJulianDay(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate11toJulianDayEv()};
    unsafe {_ZNK5QDate11toJulianDayEv()};
    return 1;
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
  pub fn getDate<T: QDate_getDate>(&mut self, value: T) -> i32 {
    value.getDate(self);
    return 1;
  }
}

pub trait QDate_getDate {
  fn getDate(self, this: &mut QDate) -> i32;
}

// proto: void QDate::getDate(int * year, int * month, int * day);
impl<'a> /*trait*/ QDate_getDate for (&'a mut i32, &'a mut i32, &'a mut i32) {
  fn getDate(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate7getDateEPiS0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZN5QDate7getDateEPiS0_S0_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn currentDate<T: QDate_currentDate>(&mut self, value: T) -> i32 {
    value.currentDate(self);
    return 1;
  }
}

pub trait QDate_currentDate {
  fn currentDate(self, this: &mut QDate) -> i32;
}

// proto: QDate QDate::currentDate();
impl<'a> /*trait*/ QDate_currentDate for () {
  fn currentDate(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate11currentDateEv()};
    unsafe {_ZN5QDate11currentDateEv()};
    return 1;
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
    value.weekNumber(self);
    return 1;
  }
}

pub trait QDate_weekNumber {
  fn weekNumber(self, this: &mut QDate) -> i32;
}

// proto: int QDate::weekNumber(int * yearNum);
impl<'a> /*trait*/ QDate_weekNumber for (&'a mut i32) {
  fn weekNumber(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate10weekNumberEPi()};
    let arg0 = self  as *mut c_int;
    unsafe {_ZNK5QDate10weekNumberEPi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn toString<T: QDate_toString>(&mut self, value: T) -> i32 {
    value.toString(self);
    return 1;
  }
}

pub trait QDate_toString {
  fn toString(self, this: &mut QDate) -> i32;
}

// proto: QString QDate::toString(const QString & format);
impl<'a> /*trait*/ QDate_toString for (&'a  QString) {
  fn toString(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate8toStringERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QDate8toStringERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn dayOfYear<T: QDate_dayOfYear>(&mut self, value: T) -> i32 {
    value.dayOfYear(self);
    return 1;
  }
}

pub trait QDate_dayOfYear {
  fn dayOfYear(self, this: &mut QDate) -> i32;
}

// proto: int QDate::dayOfYear();
impl<'a> /*trait*/ QDate_dayOfYear for () {
  fn dayOfYear(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9dayOfYearEv()};
    unsafe {_ZNK5QDate9dayOfYearEv()};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn day<T: QDate_day>(&mut self, value: T) -> i32 {
    value.day(self);
    return 1;
  }
}

pub trait QDate_day {
  fn day(self, this: &mut QDate) -> i32;
}

// proto: int QDate::day();
impl<'a> /*trait*/ QDate_day for () {
  fn day(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate3dayEv()};
    unsafe {_ZNK5QDate3dayEv()};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn setDate<T: QDate_setDate>(&mut self, value: T) -> i32 {
    value.setDate(self);
    return 1;
  }
}

pub trait QDate_setDate {
  fn setDate(self, this: &mut QDate) -> i32;
}

// proto: bool QDate::setDate(int year, int month, int day);
impl<'a> /*trait*/ QDate_setDate for (i32, i32, i32) {
  fn setDate(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate7setDateEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN5QDate7setDateEiii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn isNull<T: QDate_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QDate_isNull {
  fn isNull(self, this: &mut QDate) -> i32;
}

// proto: bool QDate::isNull();
impl<'a> /*trait*/ QDate_isNull for () {
  fn isNull(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate6isNullEv()};
    unsafe {_ZNK5QDate6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn fromJulianDay<T: QDate_fromJulianDay>(&mut self, value: T) -> i32 {
    value.fromJulianDay(self);
    return 1;
  }
}

pub trait QDate_fromJulianDay {
  fn fromJulianDay(self, this: &mut QDate) -> i32;
}

// proto: QDate QDate::fromJulianDay(qint64 jd);
impl<'a> /*trait*/ QDate_fromJulianDay for (i64) {
  fn fromJulianDay(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate13fromJulianDayEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN5QDate13fromJulianDayEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn isValid<T: QDate_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QDate_isValid {
  fn isValid(self, this: &mut QDate) -> i32;
}

// proto: bool QDate::isValid();
impl<'a> /*trait*/ QDate_isValid for () {
  fn isValid(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate7isValidEv()};
    unsafe {_ZNK5QDate7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn addDays<T: QDate_addDays>(&mut self, value: T) -> i32 {
    value.addDays(self);
    return 1;
  }
}

pub trait QDate_addDays {
  fn addDays(self, this: &mut QDate) -> i32;
}

// proto: QDate QDate::addDays(qint64 days);
impl<'a> /*trait*/ QDate_addDays for (i64) {
  fn addDays(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate7addDaysEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZNK5QDate7addDaysEx(arg0)};
    return 1;
  }
}

// proto: bool QDate::isValid(int y, int m, int d);
impl<'a> /*trait*/ QDate_isValid for (i32, i32, i32) {
  fn isValid(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate7isValidEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN5QDate7isValidEiii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn daysInMonth<T: QDate_daysInMonth>(&mut self, value: T) -> i32 {
    value.daysInMonth(self);
    return 1;
  }
}

pub trait QDate_daysInMonth {
  fn daysInMonth(self, this: &mut QDate) -> i32;
}

// proto: int QDate::daysInMonth();
impl<'a> /*trait*/ QDate_daysInMonth for () {
  fn daysInMonth(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate11daysInMonthEv()};
    unsafe {_ZNK5QDate11daysInMonthEv()};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn fromString<T: QDate_fromString>(&mut self, value: T) -> i32 {
    value.fromString(self);
    return 1;
  }
}

pub trait QDate_fromString {
  fn fromString(self, this: &mut QDate) -> i32;
}

// proto: QDate QDate::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QDate_fromString for (&'a  QString, &'a  QString) {
  fn fromString(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QDate10fromStringERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn isLeapYear<T: QDate_isLeapYear>(&mut self, value: T) -> i32 {
    value.isLeapYear(self);
    return 1;
  }
}

pub trait QDate_isLeapYear {
  fn isLeapYear(self, this: &mut QDate) -> i32;
}

// proto: bool QDate::isLeapYear(int year);
impl<'a> /*trait*/ QDate_isLeapYear for (i32) {
  fn isLeapYear(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate10isLeapYearEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QDate10isLeapYearEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn daysInYear<T: QDate_daysInYear>(&mut self, value: T) -> i32 {
    value.daysInYear(self);
    return 1;
  }
}

pub trait QDate_daysInYear {
  fn daysInYear(self, this: &mut QDate) -> i32;
}

// proto: int QDate::daysInYear();
impl<'a> /*trait*/ QDate_daysInYear for () {
  fn daysInYear(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate10daysInYearEv()};
    unsafe {_ZNK5QDate10daysInYearEv()};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn dayOfWeek<T: QDate_dayOfWeek>(&mut self, value: T) -> i32 {
    value.dayOfWeek(self);
    return 1;
  }
}

pub trait QDate_dayOfWeek {
  fn dayOfWeek(self, this: &mut QDate) -> i32;
}

// proto: int QDate::dayOfWeek();
impl<'a> /*trait*/ QDate_dayOfWeek for () {
  fn dayOfWeek(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9dayOfWeekEv()};
    unsafe {_ZNK5QDate9dayOfWeekEv()};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn addMonths<T: QDate_addMonths>(&mut self, value: T) -> i32 {
    value.addMonths(self);
    return 1;
  }
}

pub trait QDate_addMonths {
  fn addMonths(self, this: &mut QDate) -> i32;
}

// proto: QDate QDate::addMonths(int months);
impl<'a> /*trait*/ QDate_addMonths for (i32) {
  fn addMonths(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9addMonthsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK5QDate9addMonthsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDate {
  pub fn year<T: QDate_year>(&mut self, value: T) -> i32 {
    value.year(self);
    return 1;
  }
}

pub trait QDate_year {
  fn year(self, this: &mut QDate) -> i32;
}

// proto: int QDate::year();
impl<'a> /*trait*/ QDate_year for () {
  fn year(self, this: &mut QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate4yearEv()};
    unsafe {_ZNK5QDate4yearEv()};
    return 1;
  }
}

