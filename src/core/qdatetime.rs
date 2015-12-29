// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qdatetime.h
// dst-file: /src/core/qdatetime.rs
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
use super::qstring::QString; // 773
// use super::qdatetime::QDate; // 773
// use super::qdatetime::QTime; // 773
use super::qtimezone::QTimeZone; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTime_Class_Size() -> c_int;
  // proto:  QTime QTime::addMSecs(int ms);
  fn _ZNK5QTime8addMSecsEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto: static QTime QTime::fromMSecsSinceStartOfDay(int msecs);
  fn demth_ZN5QTime24fromMSecsSinceStartOfDayEi(arg0: c_int) -> *mut c_void;
  // proto: static QTime QTime::currentTime();
  fn _ZN5QTime11currentTimeEv() -> *mut c_void;
  // proto:  int QTime::second();
  fn _ZNK5QTime6secondEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTime::restart();
  fn _ZN5QTime7restartEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTime::start();
  fn _ZN5QTime5startEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTime::isNull();
  fn _ZNK5QTime6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTime::msecsSinceStartOfDay();
  fn _ZNK5QTime20msecsSinceStartOfDayEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTime::hour();
  fn _ZNK5QTime4hourEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTime::elapsed();
  fn _ZNK5QTime7elapsedEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTime QTime::addSecs(int secs);
  fn _ZNK5QTime7addSecsEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QTime::isValid();
  fn _ZNK5QTime7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTime::QTime(int ms);
  fn dector_ZN5QTimeC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN5QTimeC1Ei(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QTime::msec();
  fn _ZNK5QTime4msecEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTime::QTime(int h, int m, int s, int ms);
  fn dector_ZN5QTimeC1Eiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  fn _ZN5QTimeC1Eiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  int QTime::secsTo(const QTime & );
  fn _ZNK5QTime6secsToERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QTime::QTime();
  fn dector_ZN5QTimeC1Ev() -> *mut c_void;
  fn _ZN5QTimeC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTime::setHMS(int h, int m, int s, int ms);
  fn _ZN5QTime6setHMSEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> c_char;
  // proto:  QString QTime::toString(const QString & format);
  fn _ZNK5QTime8toStringERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTime::msecsTo(const QTime & );
  fn _ZNK5QTime7msecsToERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  int QTime::minute();
  fn _ZNK5QTime6minuteEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static bool QTime::isValid(int h, int m, int s, int ms);
  fn _ZN5QTime7isValidEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> c_char;
  // proto: static QTime QTime::fromString(const QString & s, const QString & format);
  fn _ZN5QTime10fromStringERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn QDateTime_Class_Size() -> c_int;
  // proto:  QDateTime QDateTime::toLocalTime();
  fn demth_ZNK9QDateTime11toLocalTimeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDateTime::setOffsetFromUtc(int offsetSeconds);
  fn _ZN9QDateTime16setOffsetFromUtcEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QTimeZone QDateTime::timeZone();
  fn _ZNK9QDateTime8timeZoneEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDateTime::setTime(const QTime & time);
  fn _ZN9QDateTime7setTimeERK5QTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qint64 QDateTime::toMSecsSinceEpoch();
  fn _ZNK9QDateTime17toMSecsSinceEpochEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QDateTime::setTime_t(uint secsSince1Jan1970UTC);
  fn _ZN9QDateTime9setTime_tEj(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  void QDateTime::QDateTime(const QDateTime & other);
  fn dector_ZN9QDateTimeC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QDateTimeC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDateTime::QDateTime();
  fn dector_ZN9QDateTimeC1Ev() -> *mut c_void;
  fn _ZN9QDateTimeC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QDateTime::isDaylightTime();
  fn _ZNK9QDateTime14isDaylightTimeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QDateTime::isValid();
  fn _ZNK9QDateTime7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QDateTime::toString(const QString & format);
  fn _ZNK9QDateTime8toStringERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QDateTime QDateTime::addYears(int years);
  fn _ZNK9QDateTime8addYearsEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QDateTime::setMSecsSinceEpoch(qint64 msecs);
  fn _ZN9QDateTime18setMSecsSinceEpochEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong);
  // proto:  QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds);
  fn _ZNK9QDateTime15toOffsetFromUtcEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QDateTime::setUtcOffset(int seconds);
  fn _ZN9QDateTime12setUtcOffsetEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QDateTime QDateTime::addSecs(qint64 secs);
  fn _ZNK9QDateTime7addSecsEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs);
  fn _ZN9QDateTime19fromMSecsSinceEpochEx(arg0: c_longlong) -> *mut c_void;
  // proto:  void QDateTime::QDateTime(const QDate & date, const QTime & time, const QTimeZone & timeZone);
  fn dector_ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto: static QDateTime QDateTime::fromString(const QString & s, const QString & format);
  fn _ZN9QDateTime10fromStringERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QDateTime::swap(QDateTime & other);
  fn demth_ZN9QDateTime4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  uint QDateTime::toTime_t();
  fn _ZNK9QDateTime8toTime_tEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  QString QDateTime::timeZoneAbbreviation();
  fn _ZNK9QDateTime20timeZoneAbbreviationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QDateTime QDateTime::toUTC();
  fn demth_ZNK9QDateTime5toUTCEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QDate QDateTime::date();
  fn _ZNK9QDateTime4dateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QDateTime::isNull();
  fn _ZNK9QDateTime6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static qint64 QDateTime::currentMSecsSinceEpoch();
  fn _ZN9QDateTime22currentMSecsSinceEpochEv() -> c_longlong;
  // proto:  int QDateTime::offsetFromUtc();
  fn _ZNK9QDateTime13offsetFromUtcEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDateTime::QDateTime(const QDate & );
  fn dector_ZN9QDateTimeC1ERK5QDate(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QDateTimeC1ERK5QDate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QDateTime QDateTime::addMSecs(qint64 msecs);
  fn _ZNK9QDateTime8addMSecsEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto:  qint64 QDateTime::secsTo(const QDateTime & );
  fn _ZNK9QDateTime6secsToERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_longlong;
  // proto:  void QDateTime::~QDateTime();
  fn _ZN9QDateTimeD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QDateTime QDateTime::addMonths(int months);
  fn _ZNK9QDateTime9addMonthsEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto: static QDateTime QDateTime::currentDateTime();
  fn _ZN9QDateTime15currentDateTimeEv() -> *mut c_void;
  // proto:  QDateTime QDateTime::toTimeZone(const QTimeZone & toZone);
  fn _ZNK9QDateTime10toTimeZoneERK9QTimeZone(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  qint64 QDateTime::msecsTo(const QDateTime & );
  fn _ZNK9QDateTime7msecsToERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_longlong;
  // proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC);
  fn _ZN9QDateTime10fromTime_tEj(arg0: c_uint) -> *mut c_void;
  // proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC, const QTimeZone & timeZone);
  fn _ZN9QDateTime10fromTime_tEjRK9QTimeZone(arg0: c_uint, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QDateTime::setDate(const QDate & date);
  fn _ZN9QDateTime7setDateERK5QDate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QDateTime::utcOffset();
  fn _ZNK9QDateTime9utcOffsetEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, const QTimeZone & timeZone);
  fn _ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone(arg0: c_longlong, arg1: *mut c_void) -> *mut c_void;
  // proto:  QTime QDateTime::time();
  fn _ZNK9QDateTime4timeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qint64 QDateTime::daysTo(const QDateTime & );
  fn _ZNK9QDateTime6daysToERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_longlong;
  // proto:  QDateTime QDateTime::addDays(qint64 days);
  fn _ZNK9QDateTime7addDaysEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto:  void QDateTime::setTimeZone(const QTimeZone & toZone);
  fn _ZN9QDateTime11setTimeZoneERK9QTimeZone(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QDateTime QDateTime::currentDateTimeUtc();
  fn _ZN9QDateTime18currentDateTimeUtcEv() -> *mut c_void;
  fn QDate_Class_Size() -> c_int;
  // proto:  qint64 QDate::daysTo(const QDate & );
  fn _ZNK5QDate6daysToERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_longlong;
  // proto:  QDate QDate::addYears(int years);
  fn _ZNK5QDate8addYearsEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  int QDate::month();
  fn _ZNK5QDate5monthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qint64 QDate::toJulianDay();
  fn _ZNK5QDate11toJulianDayEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QDate::QDate(qint64 julianDay);
  fn dector_ZN5QDateC1Ex(arg0: c_longlong) -> *mut c_void;
  fn _ZN5QDateC1Ex(qthis: u64 /* *mut c_void*/, arg0: c_longlong);
  // proto:  void QDate::QDate();
  fn dector_ZN5QDateC1Ev() -> *mut c_void;
  fn _ZN5QDateC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDate::getDate(int * year, int * month, int * day);
  fn _ZN5QDate7getDateEPiS0_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int);
  // proto: static QDate QDate::currentDate();
  fn _ZN5QDate11currentDateEv() -> *mut c_void;
  // proto:  void QDate::QDate(int y, int m, int d);
  fn dector_ZN5QDateC1Eiii(arg0: c_int, arg1: c_int, arg2: c_int) -> *mut c_void;
  fn _ZN5QDateC1Eiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  int QDate::weekNumber(int * yearNum);
  fn _ZNK5QDate10weekNumberEPi(qthis: u64 /* *mut c_void*/, arg0: *mut c_int) -> c_int;
  // proto:  QString QDate::toString(const QString & format);
  fn _ZNK5QDate8toStringERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QDate::dayOfYear();
  fn _ZNK5QDate9dayOfYearEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QDate::day();
  fn _ZNK5QDate3dayEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QDate::setDate(int year, int month, int day);
  fn _ZN5QDate7setDateEiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int) -> c_char;
  // proto:  bool QDate::isNull();
  fn _ZNK5QDate6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QDate QDate::fromJulianDay(qint64 jd);
  fn demth_ZN5QDate13fromJulianDayEx(arg0: c_longlong) -> *mut c_void;
  // proto:  bool QDate::isValid();
  fn _ZNK5QDate7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QDate QDate::addDays(qint64 days);
  fn _ZNK5QDate7addDaysEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto: static bool QDate::isValid(int y, int m, int d);
  fn _ZN5QDate7isValidEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> c_char;
  // proto:  int QDate::daysInMonth();
  fn _ZNK5QDate11daysInMonthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QDate QDate::fromString(const QString & s, const QString & format);
  fn _ZN5QDate10fromStringERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static bool QDate::isLeapYear(int year);
  fn _ZN5QDate10isLeapYearEi(arg0: c_int) -> c_char;
  // proto:  int QDate::daysInYear();
  fn _ZNK5QDate10daysInYearEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QDate::dayOfWeek();
  fn _ZNK5QDate9dayOfWeekEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QDate QDate::addMonths(int months);
  fn _ZNK5QDate9addMonthsEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  int QDate::year();
  fn _ZNK5QDate4yearEv(qthis: u64 /* *mut c_void*/) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QTime)=4
#[derive(Default)]
pub struct QTime {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDateTime)=1
#[derive(Default)]
pub struct QDateTime {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDate)=8
#[derive(Default)]
pub struct QDate {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTime {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTime {
    return QTime{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QTime QTime::addMSecs(int ms);
impl /*struct*/ QTime {
  pub fn addMSecs<RetType, T: QTime_addMSecs<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addMSecs(self);
    // return 1;
  }
}

pub trait QTime_addMSecs<RetType> {
  fn addMSecs(self , rsthis: & QTime) -> RetType;
}

  // proto:  QTime QTime::addMSecs(int ms);
impl<'a> /*trait*/ QTime_addMSecs<QTime> for (i32) {
  fn addMSecs(self , rsthis: & QTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime8addMSecsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QTime8addMSecsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTime QTime::fromMSecsSinceStartOfDay(int msecs);
impl /*struct*/ QTime {
  pub fn fromMSecsSinceStartOfDay_s<RetType, T: QTime_fromMSecsSinceStartOfDay_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromMSecsSinceStartOfDay_s();
    // return 1;
  }
}

pub trait QTime_fromMSecsSinceStartOfDay_s<RetType> {
  fn fromMSecsSinceStartOfDay_s(self ) -> RetType;
}

  // proto: static QTime QTime::fromMSecsSinceStartOfDay(int msecs);
impl<'a> /*trait*/ QTime_fromMSecsSinceStartOfDay_s<QTime> for (i32) {
  fn fromMSecsSinceStartOfDay_s(self ) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime24fromMSecsSinceStartOfDayEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {demth_ZN5QTime24fromMSecsSinceStartOfDayEi(arg0)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTime QTime::currentTime();
impl /*struct*/ QTime {
  pub fn currentTime_s<RetType, T: QTime_currentTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentTime_s();
    // return 1;
  }
}

pub trait QTime_currentTime_s<RetType> {
  fn currentTime_s(self ) -> RetType;
}

  // proto: static QTime QTime::currentTime();
impl<'a> /*trait*/ QTime_currentTime_s<QTime> for () {
  fn currentTime_s(self ) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime11currentTimeEv()};
    let mut ret = unsafe {_ZN5QTime11currentTimeEv()};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTime::second();
impl /*struct*/ QTime {
  pub fn second<RetType, T: QTime_second<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.second(self);
    // return 1;
  }
}

pub trait QTime_second<RetType> {
  fn second(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::second();
impl<'a> /*trait*/ QTime_second<i32> for () {
  fn second(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6secondEv()};
    let mut ret = unsafe {_ZNK5QTime6secondEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTime::restart();
impl /*struct*/ QTime {
  pub fn restart<RetType, T: QTime_restart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restart(self);
    // return 1;
  }
}

pub trait QTime_restart<RetType> {
  fn restart(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::restart();
impl<'a> /*trait*/ QTime_restart<i32> for () {
  fn restart(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime7restartEv()};
    let mut ret = unsafe {_ZN5QTime7restartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTime::start();
impl /*struct*/ QTime {
  pub fn start<RetType, T: QTime_start<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QTime_start<RetType> {
  fn start(self , rsthis: & QTime) -> RetType;
}

  // proto:  void QTime::start();
impl<'a> /*trait*/ QTime_start<()> for () {
  fn start(self , rsthis: & QTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime5startEv()};
     unsafe {_ZN5QTime5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTime::isNull();
impl /*struct*/ QTime {
  pub fn isNull<RetType, T: QTime_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QTime_isNull<RetType> {
  fn isNull(self , rsthis: & QTime) -> RetType;
}

  // proto:  bool QTime::isNull();
impl<'a> /*trait*/ QTime_isNull<i8> for () {
  fn isNull(self , rsthis: & QTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6isNullEv()};
    let mut ret = unsafe {_ZNK5QTime6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTime::msecsSinceStartOfDay();
impl /*struct*/ QTime {
  pub fn msecsSinceStartOfDay<RetType, T: QTime_msecsSinceStartOfDay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.msecsSinceStartOfDay(self);
    // return 1;
  }
}

pub trait QTime_msecsSinceStartOfDay<RetType> {
  fn msecsSinceStartOfDay(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::msecsSinceStartOfDay();
impl<'a> /*trait*/ QTime_msecsSinceStartOfDay<i32> for () {
  fn msecsSinceStartOfDay(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime20msecsSinceStartOfDayEv()};
    let mut ret = unsafe {_ZNK5QTime20msecsSinceStartOfDayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTime::hour();
impl /*struct*/ QTime {
  pub fn hour<RetType, T: QTime_hour<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hour(self);
    // return 1;
  }
}

pub trait QTime_hour<RetType> {
  fn hour(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::hour();
impl<'a> /*trait*/ QTime_hour<i32> for () {
  fn hour(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime4hourEv()};
    let mut ret = unsafe {_ZNK5QTime4hourEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTime::elapsed();
impl /*struct*/ QTime {
  pub fn elapsed<RetType, T: QTime_elapsed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.elapsed(self);
    // return 1;
  }
}

pub trait QTime_elapsed<RetType> {
  fn elapsed(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::elapsed();
impl<'a> /*trait*/ QTime_elapsed<i32> for () {
  fn elapsed(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7elapsedEv()};
    let mut ret = unsafe {_ZNK5QTime7elapsedEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTime QTime::addSecs(int secs);
impl /*struct*/ QTime {
  pub fn addSecs<RetType, T: QTime_addSecs<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addSecs(self);
    // return 1;
  }
}

pub trait QTime_addSecs<RetType> {
  fn addSecs(self , rsthis: & QTime) -> RetType;
}

  // proto:  QTime QTime::addSecs(int secs);
impl<'a> /*trait*/ QTime_addSecs<QTime> for (i32) {
  fn addSecs(self , rsthis: & QTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7addSecsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QTime7addSecsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTime::isValid();
impl /*struct*/ QTime {
  pub fn isValid<RetType, T: QTime_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTime_isValid<RetType> {
  fn isValid(self , rsthis: & QTime) -> RetType;
}

  // proto:  bool QTime::isValid();
impl<'a> /*trait*/ QTime_isValid<i8> for () {
  fn isValid(self , rsthis: & QTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7isValidEv()};
    let mut ret = unsafe {_ZNK5QTime7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTime::QTime(int ms);
impl /*struct*/ QTime {
  pub fn New<T: QTime_New>(value: T) -> QTime {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTime_New {
  fn New(self) -> QTime;
}

  // proto:  void QTime::QTime(int ms);
impl<'a> /*trait*/ QTime_New for (i32) {
  fn New(self) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTimeC1Ei()};
    let ctysz: c_int = unsafe{QTime_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    // unsafe {_ZN5QTimeC1Ei(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QTimeC1Ei(arg0)} as u64;
    let rsthis = QTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTime::msec();
impl /*struct*/ QTime {
  pub fn msec<RetType, T: QTime_msec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.msec(self);
    // return 1;
  }
}

pub trait QTime_msec<RetType> {
  fn msec(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::msec();
impl<'a> /*trait*/ QTime_msec<i32> for () {
  fn msec(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime4msecEv()};
    let mut ret = unsafe {_ZNK5QTime4msecEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTime::QTime(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_New for (i32, i32, i32, i32) {
  fn New(self) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTimeC1Eiiii()};
    let ctysz: c_int = unsafe{QTime_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    // unsafe {_ZN5QTimeC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let qthis: u64 = unsafe {dector_ZN5QTimeC1Eiiii(arg0, arg1, arg2, arg3)} as u64;
    let rsthis = QTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTime::secsTo(const QTime & );
impl /*struct*/ QTime {
  pub fn secsTo<RetType, T: QTime_secsTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.secsTo(self);
    // return 1;
  }
}

pub trait QTime_secsTo<RetType> {
  fn secsTo(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::secsTo(const QTime & );
impl<'a> /*trait*/ QTime_secsTo<i32> for (&'a QTime) {
  fn secsTo(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6secsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QTime6secsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTime::QTime();
impl<'a> /*trait*/ QTime_New for () {
  fn New(self) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTimeC1Ev()};
    let ctysz: c_int = unsafe{QTime_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN5QTimeC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN5QTimeC1Ev()} as u64;
    let rsthis = QTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTime::setHMS(int h, int m, int s, int ms);
impl /*struct*/ QTime {
  pub fn setHMS<RetType, T: QTime_setHMS<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHMS(self);
    // return 1;
  }
}

pub trait QTime_setHMS<RetType> {
  fn setHMS(self , rsthis: & QTime) -> RetType;
}

  // proto:  bool QTime::setHMS(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_setHMS<i8> for (i32, i32, i32, i32) {
  fn setHMS(self , rsthis: & QTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime6setHMSEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN5QTime6setHMSEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QTime::toString(const QString & format);
impl /*struct*/ QTime {
  pub fn toString<RetType, T: QTime_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QTime_toString<RetType> {
  fn toString(self , rsthis: & QTime) -> RetType;
}

  // proto:  QString QTime::toString(const QString & format);
impl<'a> /*trait*/ QTime_toString<QString> for (&'a QString) {
  fn toString(self , rsthis: & QTime) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QTime8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTime::msecsTo(const QTime & );
impl /*struct*/ QTime {
  pub fn msecsTo<RetType, T: QTime_msecsTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.msecsTo(self);
    // return 1;
  }
}

pub trait QTime_msecsTo<RetType> {
  fn msecsTo(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::msecsTo(const QTime & );
impl<'a> /*trait*/ QTime_msecsTo<i32> for (&'a QTime) {
  fn msecsTo(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7msecsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QTime7msecsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTime::minute();
impl /*struct*/ QTime {
  pub fn minute<RetType, T: QTime_minute<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minute(self);
    // return 1;
  }
}

pub trait QTime_minute<RetType> {
  fn minute(self , rsthis: & QTime) -> RetType;
}

  // proto:  int QTime::minute();
impl<'a> /*trait*/ QTime_minute<i32> for () {
  fn minute(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6minuteEv()};
    let mut ret = unsafe {_ZNK5QTime6minuteEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static bool QTime::isValid(int h, int m, int s, int ms);
impl /*struct*/ QTime {
  pub fn isValid_s<RetType, T: QTime_isValid_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isValid_s();
    // return 1;
  }
}

pub trait QTime_isValid_s<RetType> {
  fn isValid_s(self ) -> RetType;
}

  // proto: static bool QTime::isValid(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_isValid_s<i8> for (i32, i32, i32, i32) {
  fn isValid_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime7isValidEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN5QTime7isValidEiiii(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QTime QTime::fromString(const QString & s, const QString & format);
impl /*struct*/ QTime {
  pub fn fromString_s<RetType, T: QTime_fromString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_s();
    // return 1;
  }
}

pub trait QTime_fromString_s<RetType> {
  fn fromString_s(self ) -> RetType;
}

  // proto: static QTime QTime::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QTime_fromString_s<QTime> for (&'a QString, &'a QString) {
  fn fromString_s(self ) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QTime10fromStringERK7QStringS2_(arg0, arg1)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDateTime {
    return QDateTime{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QDateTime QDateTime::toLocalTime();
impl /*struct*/ QDateTime {
  pub fn toLocalTime<RetType, T: QDateTime_toLocalTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLocalTime(self);
    // return 1;
  }
}

pub trait QDateTime_toLocalTime<RetType> {
  fn toLocalTime(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::toLocalTime();
impl<'a> /*trait*/ QDateTime_toLocalTime<QDateTime> for () {
  fn toLocalTime(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime11toLocalTimeEv()};
    let mut ret = unsafe {demth_ZNK9QDateTime11toLocalTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTime::setOffsetFromUtc(int offsetSeconds);
impl /*struct*/ QDateTime {
  pub fn setOffsetFromUtc<RetType, T: QDateTime_setOffsetFromUtc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOffsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_setOffsetFromUtc<RetType> {
  fn setOffsetFromUtc(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::setOffsetFromUtc(int offsetSeconds);
impl<'a> /*trait*/ QDateTime_setOffsetFromUtc<()> for (i32) {
  fn setOffsetFromUtc(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime16setOffsetFromUtcEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QDateTime16setOffsetFromUtcEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTimeZone QDateTime::timeZone();
impl /*struct*/ QDateTime {
  pub fn timeZone<RetType, T: QDateTime_timeZone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timeZone(self);
    // return 1;
  }
}

pub trait QDateTime_timeZone<RetType> {
  fn timeZone(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QTimeZone QDateTime::timeZone();
impl<'a> /*trait*/ QDateTime_timeZone<QTimeZone> for () {
  fn timeZone(self , rsthis: & QDateTime) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8timeZoneEv()};
    let mut ret = unsafe {_ZNK9QDateTime8timeZoneEv(rsthis.qclsinst)};
    let mut ret1 = QTimeZone::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTime::setTime(const QTime & time);
impl /*struct*/ QDateTime {
  pub fn setTime<RetType, T: QDateTime_setTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTime(self);
    // return 1;
  }
}

pub trait QDateTime_setTime<RetType> {
  fn setTime(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::setTime(const QTime & time);
impl<'a> /*trait*/ QDateTime_setTime<()> for (&'a QTime) {
  fn setTime(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime7setTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime7setTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qint64 QDateTime::toMSecsSinceEpoch();
impl /*struct*/ QDateTime {
  pub fn toMSecsSinceEpoch<RetType, T: QDateTime_toMSecsSinceEpoch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toMSecsSinceEpoch(self);
    // return 1;
  }
}

pub trait QDateTime_toMSecsSinceEpoch<RetType> {
  fn toMSecsSinceEpoch(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  qint64 QDateTime::toMSecsSinceEpoch();
impl<'a> /*trait*/ QDateTime_toMSecsSinceEpoch<i64> for () {
  fn toMSecsSinceEpoch(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime17toMSecsSinceEpochEv()};
    let mut ret = unsafe {_ZNK9QDateTime17toMSecsSinceEpochEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QDateTime::setTime_t(uint secsSince1Jan1970UTC);
impl /*struct*/ QDateTime {
  pub fn setTime_t<RetType, T: QDateTime_setTime_t<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTime_t(self);
    // return 1;
  }
}

pub trait QDateTime_setTime_t<RetType> {
  fn setTime_t(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::setTime_t(uint secsSince1Jan1970UTC);
impl<'a> /*trait*/ QDateTime_setTime_t<()> for (u32) {
  fn setTime_t(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime9setTime_tEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN9QDateTime9setTime_tEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTime::QDateTime(const QDateTime & other);
impl /*struct*/ QDateTime {
  pub fn New<T: QDateTime_New>(value: T) -> QDateTime {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTime_New {
  fn New(self) -> QDateTime;
}

  // proto:  void QDateTime::QDateTime(const QDateTime & other);
impl<'a> /*trait*/ QDateTime_New for (&'a QDateTime) {
  fn New(self) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1ERKS_()};
    let ctysz: c_int = unsafe{QDateTime_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QDateTimeC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QDateTimeC1ERKS_(arg0)} as u64;
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDateTime::QDateTime();
impl<'a> /*trait*/ QDateTime_New for () {
  fn New(self) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1Ev()};
    let ctysz: c_int = unsafe{QDateTime_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN9QDateTimeC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN9QDateTimeC1Ev()} as u64;
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QDateTime::isDaylightTime();
impl /*struct*/ QDateTime {
  pub fn isDaylightTime<RetType, T: QDateTime_isDaylightTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDaylightTime(self);
    // return 1;
  }
}

pub trait QDateTime_isDaylightTime<RetType> {
  fn isDaylightTime(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  bool QDateTime::isDaylightTime();
impl<'a> /*trait*/ QDateTime_isDaylightTime<i8> for () {
  fn isDaylightTime(self , rsthis: & QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime14isDaylightTimeEv()};
    let mut ret = unsafe {_ZNK9QDateTime14isDaylightTimeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDateTime::isValid();
impl /*struct*/ QDateTime {
  pub fn isValid<RetType, T: QDateTime_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QDateTime_isValid<RetType> {
  fn isValid(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  bool QDateTime::isValid();
impl<'a> /*trait*/ QDateTime_isValid<i8> for () {
  fn isValid(self , rsthis: & QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7isValidEv()};
    let mut ret = unsafe {_ZNK9QDateTime7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QDateTime::toString(const QString & format);
impl /*struct*/ QDateTime {
  pub fn toString<RetType, T: QDateTime_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QDateTime_toString<RetType> {
  fn toString(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QString QDateTime::toString(const QString & format);
impl<'a> /*trait*/ QDateTime_toString<QString> for (&'a QString) {
  fn toString(self , rsthis: & QDateTime) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDateTime QDateTime::addYears(int years);
impl /*struct*/ QDateTime {
  pub fn addYears<RetType, T: QDateTime_addYears<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addYears(self);
    // return 1;
  }
}

pub trait QDateTime_addYears<RetType> {
  fn addYears(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::addYears(int years);
impl<'a> /*trait*/ QDateTime_addYears<QDateTime> for (i32) {
  fn addYears(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8addYearsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime8addYearsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTime::setMSecsSinceEpoch(qint64 msecs);
impl /*struct*/ QDateTime {
  pub fn setMSecsSinceEpoch<RetType, T: QDateTime_setMSecsSinceEpoch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMSecsSinceEpoch(self);
    // return 1;
  }
}

pub trait QDateTime_setMSecsSinceEpoch<RetType> {
  fn setMSecsSinceEpoch(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::setMSecsSinceEpoch(qint64 msecs);
impl<'a> /*trait*/ QDateTime_setMSecsSinceEpoch<()> for (i64) {
  fn setMSecsSinceEpoch(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime18setMSecsSinceEpochEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN9QDateTime18setMSecsSinceEpochEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds);
impl /*struct*/ QDateTime {
  pub fn toOffsetFromUtc<RetType, T: QDateTime_toOffsetFromUtc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toOffsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_toOffsetFromUtc<RetType> {
  fn toOffsetFromUtc(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds);
impl<'a> /*trait*/ QDateTime_toOffsetFromUtc<QDateTime> for (i32) {
  fn toOffsetFromUtc(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime15toOffsetFromUtcEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime15toOffsetFromUtcEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTime::setUtcOffset(int seconds);
impl /*struct*/ QDateTime {
  pub fn setUtcOffset<RetType, T: QDateTime_setUtcOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUtcOffset(self);
    // return 1;
  }
}

pub trait QDateTime_setUtcOffset<RetType> {
  fn setUtcOffset(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::setUtcOffset(int seconds);
impl<'a> /*trait*/ QDateTime_setUtcOffset<()> for (i32) {
  fn setUtcOffset(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime12setUtcOffsetEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QDateTime12setUtcOffsetEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDateTime QDateTime::addSecs(qint64 secs);
impl /*struct*/ QDateTime {
  pub fn addSecs<RetType, T: QDateTime_addSecs<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addSecs(self);
    // return 1;
  }
}

pub trait QDateTime_addSecs<RetType> {
  fn addSecs(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::addSecs(qint64 secs);
impl<'a> /*trait*/ QDateTime_addSecs<QDateTime> for (i64) {
  fn addSecs(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7addSecsEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime7addSecsEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs);
impl /*struct*/ QDateTime {
  pub fn fromMSecsSinceEpoch_s<RetType, T: QDateTime_fromMSecsSinceEpoch_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromMSecsSinceEpoch_s();
    // return 1;
  }
}

pub trait QDateTime_fromMSecsSinceEpoch_s<RetType> {
  fn fromMSecsSinceEpoch_s(self ) -> RetType;
}

  // proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs);
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch_s<QDateTime> for (i64) {
  fn fromMSecsSinceEpoch_s(self ) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime19fromMSecsSinceEpochEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN9QDateTime19fromMSecsSinceEpochEx(arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTime::QDateTime(const QDate & date, const QTime & time, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_New for (&'a QDate, &'a QTime, &'a QTimeZone) {
  fn New(self) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone()};
    let ctysz: c_int = unsafe{QDateTime_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone(qthis, arg0, arg1, arg2)};
    let qthis: u64 = unsafe {dector_ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone(arg0, arg1, arg2)} as u64;
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QDateTime QDateTime::fromString(const QString & s, const QString & format);
impl /*struct*/ QDateTime {
  pub fn fromString_s<RetType, T: QDateTime_fromString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_s();
    // return 1;
  }
}

pub trait QDateTime_fromString_s<RetType> {
  fn fromString_s(self ) -> RetType;
}

  // proto: static QDateTime QDateTime::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QDateTime_fromString_s<QDateTime> for (&'a QString, &'a QString) {
  fn fromString_s(self ) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDateTime10fromStringERK7QStringS2_(arg0, arg1)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTime::swap(QDateTime & other);
impl /*struct*/ QDateTime {
  pub fn swap<RetType, T: QDateTime_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDateTime_swap<RetType> {
  fn swap(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::swap(QDateTime & other);
impl<'a> /*trait*/ QDateTime_swap<()> for (&'a QDateTime) {
  fn swap(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN9QDateTime4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  uint QDateTime::toTime_t();
impl /*struct*/ QDateTime {
  pub fn toTime_t<RetType, T: QDateTime_toTime_t<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toTime_t(self);
    // return 1;
  }
}

pub trait QDateTime_toTime_t<RetType> {
  fn toTime_t(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  uint QDateTime::toTime_t();
impl<'a> /*trait*/ QDateTime_toTime_t<u32> for () {
  fn toTime_t(self , rsthis: & QDateTime) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8toTime_tEv()};
    let mut ret = unsafe {_ZNK9QDateTime8toTime_tEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QString QDateTime::timeZoneAbbreviation();
impl /*struct*/ QDateTime {
  pub fn timeZoneAbbreviation<RetType, T: QDateTime_timeZoneAbbreviation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timeZoneAbbreviation(self);
    // return 1;
  }
}

pub trait QDateTime_timeZoneAbbreviation<RetType> {
  fn timeZoneAbbreviation(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QString QDateTime::timeZoneAbbreviation();
impl<'a> /*trait*/ QDateTime_timeZoneAbbreviation<QString> for () {
  fn timeZoneAbbreviation(self , rsthis: & QDateTime) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime20timeZoneAbbreviationEv()};
    let mut ret = unsafe {_ZNK9QDateTime20timeZoneAbbreviationEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDateTime QDateTime::toUTC();
impl /*struct*/ QDateTime {
  pub fn toUTC<RetType, T: QDateTime_toUTC<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUTC(self);
    // return 1;
  }
}

pub trait QDateTime_toUTC<RetType> {
  fn toUTC(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::toUTC();
impl<'a> /*trait*/ QDateTime_toUTC<QDateTime> for () {
  fn toUTC(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime5toUTCEv()};
    let mut ret = unsafe {demth_ZNK9QDateTime5toUTCEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDate QDateTime::date();
impl /*struct*/ QDateTime {
  pub fn date<RetType, T: QDateTime_date<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.date(self);
    // return 1;
  }
}

pub trait QDateTime_date<RetType> {
  fn date(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDate QDateTime::date();
impl<'a> /*trait*/ QDateTime_date<QDate> for () {
  fn date(self , rsthis: & QDateTime) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime4dateEv()};
    let mut ret = unsafe {_ZNK9QDateTime4dateEv(rsthis.qclsinst)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDateTime::isNull();
impl /*struct*/ QDateTime {
  pub fn isNull<RetType, T: QDateTime_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QDateTime_isNull<RetType> {
  fn isNull(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  bool QDateTime::isNull();
impl<'a> /*trait*/ QDateTime_isNull<i8> for () {
  fn isNull(self , rsthis: & QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6isNullEv()};
    let mut ret = unsafe {_ZNK9QDateTime6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static qint64 QDateTime::currentMSecsSinceEpoch();
impl /*struct*/ QDateTime {
  pub fn currentMSecsSinceEpoch_s<RetType, T: QDateTime_currentMSecsSinceEpoch_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentMSecsSinceEpoch_s();
    // return 1;
  }
}

pub trait QDateTime_currentMSecsSinceEpoch_s<RetType> {
  fn currentMSecsSinceEpoch_s(self ) -> RetType;
}

  // proto: static qint64 QDateTime::currentMSecsSinceEpoch();
impl<'a> /*trait*/ QDateTime_currentMSecsSinceEpoch_s<i64> for () {
  fn currentMSecsSinceEpoch_s(self ) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime22currentMSecsSinceEpochEv()};
    let mut ret = unsafe {_ZN9QDateTime22currentMSecsSinceEpochEv()};
    return ret as i64;
    // return 1;
  }
}

  // proto:  int QDateTime::offsetFromUtc();
impl /*struct*/ QDateTime {
  pub fn offsetFromUtc<RetType, T: QDateTime_offsetFromUtc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.offsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_offsetFromUtc<RetType> {
  fn offsetFromUtc(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  int QDateTime::offsetFromUtc();
impl<'a> /*trait*/ QDateTime_offsetFromUtc<i32> for () {
  fn offsetFromUtc(self , rsthis: & QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime13offsetFromUtcEv()};
    let mut ret = unsafe {_ZNK9QDateTime13offsetFromUtcEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDateTime::QDateTime(const QDate & );
impl<'a> /*trait*/ QDateTime_New for (&'a QDate) {
  fn New(self) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1ERK5QDate()};
    let ctysz: c_int = unsafe{QDateTime_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QDateTimeC1ERK5QDate(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QDateTimeC1ERK5QDate(arg0)} as u64;
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDateTime QDateTime::addMSecs(qint64 msecs);
impl /*struct*/ QDateTime {
  pub fn addMSecs<RetType, T: QDateTime_addMSecs<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addMSecs(self);
    // return 1;
  }
}

pub trait QDateTime_addMSecs<RetType> {
  fn addMSecs(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::addMSecs(qint64 msecs);
impl<'a> /*trait*/ QDateTime_addMSecs<QDateTime> for (i64) {
  fn addMSecs(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8addMSecsEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime8addMSecsEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QDateTime::secsTo(const QDateTime & );
impl /*struct*/ QDateTime {
  pub fn secsTo<RetType, T: QDateTime_secsTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.secsTo(self);
    // return 1;
  }
}

pub trait QDateTime_secsTo<RetType> {
  fn secsTo(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  qint64 QDateTime::secsTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_secsTo<i64> for (&'a QDateTime) {
  fn secsTo(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6secsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime6secsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QDateTime::~QDateTime();
impl /*struct*/ QDateTime {
  pub fn Free<RetType, T: QDateTime_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDateTime_Free<RetType> {
  fn Free(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::~QDateTime();
impl<'a> /*trait*/ QDateTime_Free<()> for () {
  fn Free(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeD0Ev()};
     unsafe {_ZN9QDateTimeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QDateTime QDateTime::addMonths(int months);
impl /*struct*/ QDateTime {
  pub fn addMonths<RetType, T: QDateTime_addMonths<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addMonths(self);
    // return 1;
  }
}

pub trait QDateTime_addMonths<RetType> {
  fn addMonths(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::addMonths(int months);
impl<'a> /*trait*/ QDateTime_addMonths<QDateTime> for (i32) {
  fn addMonths(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime9addMonthsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime9addMonthsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QDateTime QDateTime::currentDateTime();
impl /*struct*/ QDateTime {
  pub fn currentDateTime_s<RetType, T: QDateTime_currentDateTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentDateTime_s();
    // return 1;
  }
}

pub trait QDateTime_currentDateTime_s<RetType> {
  fn currentDateTime_s(self ) -> RetType;
}

  // proto: static QDateTime QDateTime::currentDateTime();
impl<'a> /*trait*/ QDateTime_currentDateTime_s<QDateTime> for () {
  fn currentDateTime_s(self ) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime15currentDateTimeEv()};
    let mut ret = unsafe {_ZN9QDateTime15currentDateTimeEv()};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDateTime QDateTime::toTimeZone(const QTimeZone & toZone);
impl /*struct*/ QDateTime {
  pub fn toTimeZone<RetType, T: QDateTime_toTimeZone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toTimeZone(self);
    // return 1;
  }
}

pub trait QDateTime_toTimeZone<RetType> {
  fn toTimeZone(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::toTimeZone(const QTimeZone & toZone);
impl<'a> /*trait*/ QDateTime_toTimeZone<QDateTime> for (&'a QTimeZone) {
  fn toTimeZone(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime10toTimeZoneERK9QTimeZone()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime10toTimeZoneERK9QTimeZone(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QDateTime::msecsTo(const QDateTime & );
impl /*struct*/ QDateTime {
  pub fn msecsTo<RetType, T: QDateTime_msecsTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.msecsTo(self);
    // return 1;
  }
}

pub trait QDateTime_msecsTo<RetType> {
  fn msecsTo(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  qint64 QDateTime::msecsTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_msecsTo<i64> for (&'a QDateTime) {
  fn msecsTo(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7msecsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime7msecsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC);
impl /*struct*/ QDateTime {
  pub fn fromTime_t_s<RetType, T: QDateTime_fromTime_t_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTime_t_s();
    // return 1;
  }
}

pub trait QDateTime_fromTime_t_s<RetType> {
  fn fromTime_t_s(self ) -> RetType;
}

  // proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC);
impl<'a> /*trait*/ QDateTime_fromTime_t_s<QDateTime> for (u32) {
  fn fromTime_t_s(self ) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromTime_tEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN9QDateTime10fromTime_tEj(arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_fromTime_t_s<QDateTime> for (u32, &'a QTimeZone) {
  fn fromTime_t_s(self ) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromTime_tEjRK9QTimeZone()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDateTime10fromTime_tEjRK9QTimeZone(arg0, arg1)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTime::setDate(const QDate & date);
impl /*struct*/ QDateTime {
  pub fn setDate<RetType, T: QDateTime_setDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDate(self);
    // return 1;
  }
}

pub trait QDateTime_setDate<RetType> {
  fn setDate(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::setDate(const QDate & date);
impl<'a> /*trait*/ QDateTime_setDate<()> for (&'a QDate) {
  fn setDate(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime7setDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime7setDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QDateTime::utcOffset();
impl /*struct*/ QDateTime {
  pub fn utcOffset<RetType, T: QDateTime_utcOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.utcOffset(self);
    // return 1;
  }
}

pub trait QDateTime_utcOffset<RetType> {
  fn utcOffset(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  int QDateTime::utcOffset();
impl<'a> /*trait*/ QDateTime_utcOffset<i32> for () {
  fn utcOffset(self , rsthis: & QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime9utcOffsetEv()};
    let mut ret = unsafe {_ZNK9QDateTime9utcOffsetEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch_s<QDateTime> for (i64, &'a QTimeZone) {
  fn fromMSecsSinceEpoch_s(self ) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone(arg0, arg1)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTime QDateTime::time();
impl /*struct*/ QDateTime {
  pub fn time<RetType, T: QDateTime_time<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.time(self);
    // return 1;
  }
}

pub trait QDateTime_time<RetType> {
  fn time(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QTime QDateTime::time();
impl<'a> /*trait*/ QDateTime_time<QTime> for () {
  fn time(self , rsthis: & QDateTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime4timeEv()};
    let mut ret = unsafe {_ZNK9QDateTime4timeEv(rsthis.qclsinst)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QDateTime::daysTo(const QDateTime & );
impl /*struct*/ QDateTime {
  pub fn daysTo<RetType, T: QDateTime_daysTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.daysTo(self);
    // return 1;
  }
}

pub trait QDateTime_daysTo<RetType> {
  fn daysTo(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  qint64 QDateTime::daysTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_daysTo<i64> for (&'a QDateTime) {
  fn daysTo(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6daysToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime6daysToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QDateTime QDateTime::addDays(qint64 days);
impl /*struct*/ QDateTime {
  pub fn addDays<RetType, T: QDateTime_addDays<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addDays(self);
    // return 1;
  }
}

pub trait QDateTime_addDays<RetType> {
  fn addDays(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  QDateTime QDateTime::addDays(qint64 days);
impl<'a> /*trait*/ QDateTime_addDays<QDateTime> for (i64) {
  fn addDays(self , rsthis: & QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7addDaysEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime7addDaysEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTime::setTimeZone(const QTimeZone & toZone);
impl /*struct*/ QDateTime {
  pub fn setTimeZone<RetType, T: QDateTime_setTimeZone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTimeZone(self);
    // return 1;
  }
}

pub trait QDateTime_setTimeZone<RetType> {
  fn setTimeZone(self , rsthis: & QDateTime) -> RetType;
}

  // proto:  void QDateTime::setTimeZone(const QTimeZone & toZone);
impl<'a> /*trait*/ QDateTime_setTimeZone<()> for (&'a QTimeZone) {
  fn setTimeZone(self , rsthis: & QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime11setTimeZoneERK9QTimeZone()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime11setTimeZoneERK9QTimeZone(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QDateTime QDateTime::currentDateTimeUtc();
impl /*struct*/ QDateTime {
  pub fn currentDateTimeUtc_s<RetType, T: QDateTime_currentDateTimeUtc_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentDateTimeUtc_s();
    // return 1;
  }
}

pub trait QDateTime_currentDateTimeUtc_s<RetType> {
  fn currentDateTimeUtc_s(self ) -> RetType;
}

  // proto: static QDateTime QDateTime::currentDateTimeUtc();
impl<'a> /*trait*/ QDateTime_currentDateTimeUtc_s<QDateTime> for () {
  fn currentDateTimeUtc_s(self ) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime18currentDateTimeUtcEv()};
    let mut ret = unsafe {_ZN9QDateTime18currentDateTimeUtcEv()};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDate {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDate {
    return QDate{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  qint64 QDate::daysTo(const QDate & );
impl /*struct*/ QDate {
  pub fn daysTo<RetType, T: QDate_daysTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.daysTo(self);
    // return 1;
  }
}

pub trait QDate_daysTo<RetType> {
  fn daysTo(self , rsthis: & QDate) -> RetType;
}

  // proto:  qint64 QDate::daysTo(const QDate & );
impl<'a> /*trait*/ QDate_daysTo<i64> for (&'a QDate) {
  fn daysTo(self , rsthis: & QDate) -> i64 {
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
  pub fn addYears<RetType, T: QDate_addYears<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addYears(self);
    // return 1;
  }
}

pub trait QDate_addYears<RetType> {
  fn addYears(self , rsthis: & QDate) -> RetType;
}

  // proto:  QDate QDate::addYears(int years);
impl<'a> /*trait*/ QDate_addYears<QDate> for (i32) {
  fn addYears(self , rsthis: & QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate8addYearsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QDate8addYearsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QDate::month();
impl /*struct*/ QDate {
  pub fn month<RetType, T: QDate_month<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.month(self);
    // return 1;
  }
}

pub trait QDate_month<RetType> {
  fn month(self , rsthis: & QDate) -> RetType;
}

  // proto:  int QDate::month();
impl<'a> /*trait*/ QDate_month<i32> for () {
  fn month(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate5monthEv()};
    let mut ret = unsafe {_ZNK5QDate5monthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qint64 QDate::toJulianDay();
impl /*struct*/ QDate {
  pub fn toJulianDay<RetType, T: QDate_toJulianDay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toJulianDay(self);
    // return 1;
  }
}

pub trait QDate_toJulianDay<RetType> {
  fn toJulianDay(self , rsthis: & QDate) -> RetType;
}

  // proto:  qint64 QDate::toJulianDay();
impl<'a> /*trait*/ QDate_toJulianDay<i64> for () {
  fn toJulianDay(self , rsthis: & QDate) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate11toJulianDayEv()};
    let mut ret = unsafe {_ZNK5QDate11toJulianDayEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QDate::QDate(qint64 julianDay);
impl /*struct*/ QDate {
  pub fn New<T: QDate_New>(value: T) -> QDate {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDate_New {
  fn New(self) -> QDate;
}

  // proto:  void QDate::QDate(qint64 julianDay);
impl<'a> /*trait*/ QDate_New for (i64) {
  fn New(self) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDateC1Ex()};
    let ctysz: c_int = unsafe{QDate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_longlong;
    // unsafe {_ZN5QDateC1Ex(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QDateC1Ex(arg0)} as u64;
    let rsthis = QDate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDate::QDate();
impl<'a> /*trait*/ QDate_New for () {
  fn New(self) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDateC1Ev()};
    let ctysz: c_int = unsafe{QDate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN5QDateC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN5QDateC1Ev()} as u64;
    let rsthis = QDate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDate::getDate(int * year, int * month, int * day);
impl /*struct*/ QDate {
  pub fn getDate<RetType, T: QDate_getDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getDate(self);
    // return 1;
  }
}

pub trait QDate_getDate<RetType> {
  fn getDate(self , rsthis: & QDate) -> RetType;
}

  // proto:  void QDate::getDate(int * year, int * month, int * day);
impl<'a> /*trait*/ QDate_getDate<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn getDate(self , rsthis: & QDate) -> () {
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
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDate::QDate(int y, int m, int d);
impl<'a> /*trait*/ QDate_New for (i32, i32, i32) {
  fn New(self) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDateC1Eiii()};
    let ctysz: c_int = unsafe{QDate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    // unsafe {_ZN5QDateC1Eiii(qthis, arg0, arg1, arg2)};
    let qthis: u64 = unsafe {dector_ZN5QDateC1Eiii(arg0, arg1, arg2)} as u64;
    let rsthis = QDate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QDate::weekNumber(int * yearNum);
impl /*struct*/ QDate {
  pub fn weekNumber<RetType, T: QDate_weekNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.weekNumber(self);
    // return 1;
  }
}

pub trait QDate_weekNumber<RetType> {
  fn weekNumber(self , rsthis: & QDate) -> RetType;
}

  // proto:  int QDate::weekNumber(int * yearNum);
impl<'a> /*trait*/ QDate_weekNumber<i32> for (&'a mut Vec<i32>) {
  fn weekNumber(self , rsthis: & QDate) -> i32 {
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
  pub fn toString<RetType, T: QDate_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QDate_toString<RetType> {
  fn toString(self , rsthis: & QDate) -> RetType;
}

  // proto:  QString QDate::toString(const QString & format);
impl<'a> /*trait*/ QDate_toString<QString> for (&'a QString) {
  fn toString(self , rsthis: & QDate) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QDate8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QDate::dayOfYear();
impl /*struct*/ QDate {
  pub fn dayOfYear<RetType, T: QDate_dayOfYear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dayOfYear(self);
    // return 1;
  }
}

pub trait QDate_dayOfYear<RetType> {
  fn dayOfYear(self , rsthis: & QDate) -> RetType;
}

  // proto:  int QDate::dayOfYear();
impl<'a> /*trait*/ QDate_dayOfYear<i32> for () {
  fn dayOfYear(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9dayOfYearEv()};
    let mut ret = unsafe {_ZNK5QDate9dayOfYearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QDate::day();
impl /*struct*/ QDate {
  pub fn day<RetType, T: QDate_day<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.day(self);
    // return 1;
  }
}

pub trait QDate_day<RetType> {
  fn day(self , rsthis: & QDate) -> RetType;
}

  // proto:  int QDate::day();
impl<'a> /*trait*/ QDate_day<i32> for () {
  fn day(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate3dayEv()};
    let mut ret = unsafe {_ZNK5QDate3dayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QDate::setDate(int year, int month, int day);
impl /*struct*/ QDate {
  pub fn setDate<RetType, T: QDate_setDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDate(self);
    // return 1;
  }
}

pub trait QDate_setDate<RetType> {
  fn setDate(self , rsthis: & QDate) -> RetType;
}

  // proto:  bool QDate::setDate(int year, int month, int day);
impl<'a> /*trait*/ QDate_setDate<i8> for (i32, i32, i32) {
  fn setDate(self , rsthis: & QDate) -> i8 {
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
  pub fn isNull<RetType, T: QDate_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QDate_isNull<RetType> {
  fn isNull(self , rsthis: & QDate) -> RetType;
}

  // proto:  bool QDate::isNull();
impl<'a> /*trait*/ QDate_isNull<i8> for () {
  fn isNull(self , rsthis: & QDate) -> i8 {
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
    let mut ret = unsafe {demth_ZN5QDate13fromJulianDayEx(arg0)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDate::isValid();
impl /*struct*/ QDate {
  pub fn isValid<RetType, T: QDate_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QDate_isValid<RetType> {
  fn isValid(self , rsthis: & QDate) -> RetType;
}

  // proto:  bool QDate::isValid();
impl<'a> /*trait*/ QDate_isValid<i8> for () {
  fn isValid(self , rsthis: & QDate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate7isValidEv()};
    let mut ret = unsafe {_ZNK5QDate7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QDate QDate::addDays(qint64 days);
impl /*struct*/ QDate {
  pub fn addDays<RetType, T: QDate_addDays<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addDays(self);
    // return 1;
  }
}

pub trait QDate_addDays<RetType> {
  fn addDays(self , rsthis: & QDate) -> RetType;
}

  // proto:  QDate QDate::addDays(qint64 days);
impl<'a> /*trait*/ QDate_addDays<QDate> for (i64) {
  fn addDays(self , rsthis: & QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate7addDaysEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK5QDate7addDaysEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
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
  pub fn daysInMonth<RetType, T: QDate_daysInMonth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.daysInMonth(self);
    // return 1;
  }
}

pub trait QDate_daysInMonth<RetType> {
  fn daysInMonth(self , rsthis: & QDate) -> RetType;
}

  // proto:  int QDate::daysInMonth();
impl<'a> /*trait*/ QDate_daysInMonth<i32> for () {
  fn daysInMonth(self , rsthis: & QDate) -> i32 {
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
impl<'a> /*trait*/ QDate_fromString_s<QDate> for (&'a QString, &'a QString) {
  fn fromString_s(self ) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDate10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QDate10fromStringERK7QStringS2_(arg0, arg1)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
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
  pub fn daysInYear<RetType, T: QDate_daysInYear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.daysInYear(self);
    // return 1;
  }
}

pub trait QDate_daysInYear<RetType> {
  fn daysInYear(self , rsthis: & QDate) -> RetType;
}

  // proto:  int QDate::daysInYear();
impl<'a> /*trait*/ QDate_daysInYear<i32> for () {
  fn daysInYear(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate10daysInYearEv()};
    let mut ret = unsafe {_ZNK5QDate10daysInYearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QDate::dayOfWeek();
impl /*struct*/ QDate {
  pub fn dayOfWeek<RetType, T: QDate_dayOfWeek<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dayOfWeek(self);
    // return 1;
  }
}

pub trait QDate_dayOfWeek<RetType> {
  fn dayOfWeek(self , rsthis: & QDate) -> RetType;
}

  // proto:  int QDate::dayOfWeek();
impl<'a> /*trait*/ QDate_dayOfWeek<i32> for () {
  fn dayOfWeek(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9dayOfWeekEv()};
    let mut ret = unsafe {_ZNK5QDate9dayOfWeekEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QDate QDate::addMonths(int months);
impl /*struct*/ QDate {
  pub fn addMonths<RetType, T: QDate_addMonths<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addMonths(self);
    // return 1;
  }
}

pub trait QDate_addMonths<RetType> {
  fn addMonths(self , rsthis: & QDate) -> RetType;
}

  // proto:  QDate QDate::addMonths(int months);
impl<'a> /*trait*/ QDate_addMonths<QDate> for (i32) {
  fn addMonths(self , rsthis: & QDate) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate9addMonthsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QDate9addMonthsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QDate::year();
impl /*struct*/ QDate {
  pub fn year<RetType, T: QDate_year<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.year(self);
    // return 1;
  }
}

pub trait QDate_year<RetType> {
  fn year(self , rsthis: & QDate) -> RetType;
}

  // proto:  int QDate::year();
impl<'a> /*trait*/ QDate_year<i32> for () {
  fn year(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDate4yearEv()};
    let mut ret = unsafe {_ZNK5QDate4yearEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

