// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtimezone::QTimeZone;
use super::qtime::QTime;
use super::qstring::QString;
use super::qdate::QDate;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QDateTime QDateTime::toLocalTime();
  fn _ZNK9QDateTime11toLocalTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDateTime::setOffsetFromUtc(int offsetSeconds);
  fn _ZN9QDateTime16setOffsetFromUtcEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QTimeZone QDateTime::timeZone();
  fn _ZNK9QDateTime8timeZoneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDateTime::setTime(const QTime & time);
  fn _ZN9QDateTime7setTimeERK5QTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  long long QDateTime::toMSecsSinceEpoch();
  fn _ZNK9QDateTime17toMSecsSinceEpochEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QDateTime::setTime_t(uint secsSince1Jan1970UTC);
  fn _ZN9QDateTime9setTime_tEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QDateTime::NewQDateTime(const QDateTime & other);
  fn _ZN9QDateTimeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTime::NewQDateTime();
  fn _ZN9QDateTimeC1Ev(qthis: *mut c_void) ;
  // proto:  bool QDateTime::isDaylightTime();
  fn _ZNK9QDateTime14isDaylightTimeEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QDateTime::isValid();
  fn _ZNK9QDateTime7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QDateTime::toString(const QString & format);
  fn _ZNK9QDateTime8toStringERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QDateTime QDateTime::addYears(int years);
  fn _ZNK9QDateTime8addYearsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QDateTime::setMSecsSinceEpoch(qint64 msecs);
  fn _ZN9QDateTime18setMSecsSinceEpochEx(qthis: *mut c_void, arg0: c_longlong) ;
  // proto:  QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds);
  fn _ZNK9QDateTime15toOffsetFromUtcEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QDateTime::setUtcOffset(int seconds);
  fn _ZN9QDateTime12setUtcOffsetEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QDateTime QDateTime::addSecs(qint64 secs);
  fn _ZNK9QDateTime7addSecsEx(qthis: *mut c_void, arg0: c_longlong) -> *mut c_void;
  // proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs);
  fn _ZN9QDateTime19fromMSecsSinceEpochEx(arg0: c_longlong) -> *mut c_void;
  // proto:  void QDateTime::NewQDateTime(const QDate & date, const QTime & time, const QTimeZone & timeZone);
  fn _ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto: static QDateTime QDateTime::fromString(const QString & s, const QString & format);
  fn _ZN9QDateTime10fromStringERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QDateTime::swap(QDateTime & other);
  fn _ZN9QDateTime4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  unsigned int QDateTime::toTime_t();
  fn _ZNK9QDateTime8toTime_tEv(qthis: *mut c_void) -> c_uint;
  // proto:  QString QDateTime::timeZoneAbbreviation();
  fn _ZNK9QDateTime20timeZoneAbbreviationEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDateTime QDateTime::toUTC();
  fn _ZNK9QDateTime5toUTCEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDate QDateTime::date();
  fn _ZNK9QDateTime4dateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QDateTime::isNull();
  fn _ZNK9QDateTime6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto: static long long QDateTime::currentMSecsSinceEpoch();
  fn _ZN9QDateTime22currentMSecsSinceEpochEv() -> c_longlong;
  // proto:  int QDateTime::offsetFromUtc();
  fn _ZNK9QDateTime13offsetFromUtcEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDateTime::NewQDateTime(const QDate & );
  fn _ZN9QDateTimeC1ERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDateTime QDateTime::addMSecs(qint64 msecs);
  fn _ZNK9QDateTime8addMSecsEx(qthis: *mut c_void, arg0: c_longlong) -> *mut c_void;
  // proto:  long long QDateTime::secsTo(const QDateTime & );
  fn _ZNK9QDateTime6secsToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto:  void QDateTime::FreeQDateTime();
  fn _ZN9QDateTimeD0Ev(qthis: *mut c_void) ;
  // proto:  QDateTime QDateTime::addMonths(int months);
  fn _ZNK9QDateTime9addMonthsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto: static QDateTime QDateTime::currentDateTime();
  fn _ZN9QDateTime15currentDateTimeEv() -> *mut c_void;
  // proto:  QDateTime QDateTime::toTimeZone(const QTimeZone & toZone);
  fn _ZNK9QDateTime10toTimeZoneERK9QTimeZone(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  long long QDateTime::msecsTo(const QDateTime & );
  fn _ZNK9QDateTime7msecsToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC);
  fn _ZN9QDateTime10fromTime_tEj(arg0: c_uint) -> *mut c_void;
  // proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC, const QTimeZone & timeZone);
  fn _ZN9QDateTime10fromTime_tEjRK9QTimeZone(arg0: c_uint, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QDateTime::setDate(const QDate & date);
  fn _ZN9QDateTime7setDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QDateTime::utcOffset();
  fn _ZNK9QDateTime9utcOffsetEv(qthis: *mut c_void) -> c_int;
  // proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, const QTimeZone & timeZone);
  fn _ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone(arg0: c_longlong, arg1: *mut c_void) -> *mut c_void;
  // proto:  QTime QDateTime::time();
  fn _ZNK9QDateTime4timeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  long long QDateTime::daysTo(const QDateTime & );
  fn _ZNK9QDateTime6daysToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto:  QDateTime QDateTime::addDays(qint64 days);
  fn _ZNK9QDateTime7addDaysEx(qthis: *mut c_void, arg0: c_longlong) -> *mut c_void;
  // proto:  void QDateTime::setTimeZone(const QTimeZone & toZone);
  fn _ZN9QDateTime11setTimeZoneERK9QTimeZone(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QDateTime QDateTime::currentDateTimeUtc();
  fn _ZN9QDateTime18currentDateTimeUtcEv() -> *mut c_void;
}

// body block begin
// class sizeof(QDateTime)=1
pub struct QDateTime {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDateTime {
  pub fn toLocalTime<T: QDateTime_toLocalTime>(&mut self, value: T) -> QDateTime {
    return value.toLocalTime(self);
    // return 1;
  }
}

pub trait QDateTime_toLocalTime {
  fn toLocalTime(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::toLocalTime();
impl<'a> /*trait*/ QDateTime_toLocalTime for () {
  fn toLocalTime(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime11toLocalTimeEv()};
    let mut ret = unsafe {_ZNK9QDateTime11toLocalTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setOffsetFromUtc<T: QDateTime_setOffsetFromUtc>(&mut self, value: T)  {
     value.setOffsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_setOffsetFromUtc {
  fn setOffsetFromUtc(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::setOffsetFromUtc(int offsetSeconds);
impl<'a> /*trait*/ QDateTime_setOffsetFromUtc for (i32) {
  fn setOffsetFromUtc(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime16setOffsetFromUtcEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QDateTime16setOffsetFromUtcEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn timeZone<T: QDateTime_timeZone>(&mut self, value: T) -> QTimeZone {
    return value.timeZone(self);
    // return 1;
  }
}

pub trait QDateTime_timeZone {
  fn timeZone(self, rsthis: &mut QDateTime) -> QTimeZone;
}

// proto:  QTimeZone QDateTime::timeZone();
impl<'a> /*trait*/ QDateTime_timeZone for () {
  fn timeZone(self, rsthis: &mut QDateTime) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8timeZoneEv()};
    let mut ret = unsafe {_ZNK9QDateTime8timeZoneEv(rsthis.qclsinst)};
    let mut ret1 = QTimeZone{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setTime<T: QDateTime_setTime>(&mut self, value: T)  {
     value.setTime(self);
    // return 1;
  }
}

pub trait QDateTime_setTime {
  fn setTime(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::setTime(const QTime & time);
impl<'a> /*trait*/ QDateTime_setTime for (&'a  QTime) {
  fn setTime(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime7setTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime7setTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toMSecsSinceEpoch<T: QDateTime_toMSecsSinceEpoch>(&mut self, value: T) -> i64 {
    return value.toMSecsSinceEpoch(self);
    // return 1;
  }
}

pub trait QDateTime_toMSecsSinceEpoch {
  fn toMSecsSinceEpoch(self, rsthis: &mut QDateTime) -> i64;
}

// proto:  long long QDateTime::toMSecsSinceEpoch();
impl<'a> /*trait*/ QDateTime_toMSecsSinceEpoch for () {
  fn toMSecsSinceEpoch(self, rsthis: &mut QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime17toMSecsSinceEpochEv()};
    let mut ret = unsafe {_ZNK9QDateTime17toMSecsSinceEpochEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setTime_t<T: QDateTime_setTime_t>(&mut self, value: T)  {
     value.setTime_t(self);
    // return 1;
  }
}

pub trait QDateTime_setTime_t {
  fn setTime_t(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::setTime_t(uint secsSince1Jan1970UTC);
impl<'a> /*trait*/ QDateTime_setTime_t for (u32) {
  fn setTime_t(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime9setTime_tEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN9QDateTime9setTime_tEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn NewQDateTime<T: QDateTime_NewQDateTime>(value: T) -> QDateTime {
    let rsthis = value.NewQDateTime();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTime_NewQDateTime {
  fn NewQDateTime(self) -> QDateTime;
}

// proto: void QDateTime::NewQDateTime(const QDateTime & other);
impl<'a> /*trait*/ QDateTime_NewQDateTime for (&'a  QDateTime) {
  fn NewQDateTime(self) -> QDateTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QDateTimeC1ERKS_(qthis, arg0)};
    let rsthis = QDateTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QDateTime::NewQDateTime();
impl<'a> /*trait*/ QDateTime_NewQDateTime for () {
  fn NewQDateTime(self) -> QDateTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1Ev()};
    unsafe {_ZN9QDateTimeC1Ev(qthis)};
    let rsthis = QDateTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn isDaylightTime<T: QDateTime_isDaylightTime>(&mut self, value: T) -> i8 {
    return value.isDaylightTime(self);
    // return 1;
  }
}

pub trait QDateTime_isDaylightTime {
  fn isDaylightTime(self, rsthis: &mut QDateTime) -> i8;
}

// proto:  bool QDateTime::isDaylightTime();
impl<'a> /*trait*/ QDateTime_isDaylightTime for () {
  fn isDaylightTime(self, rsthis: &mut QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime14isDaylightTimeEv()};
    let mut ret = unsafe {_ZNK9QDateTime14isDaylightTimeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn isValid<T: QDateTime_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QDateTime_isValid {
  fn isValid(self, rsthis: &mut QDateTime) -> i8;
}

// proto:  bool QDateTime::isValid();
impl<'a> /*trait*/ QDateTime_isValid for () {
  fn isValid(self, rsthis: &mut QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7isValidEv()};
    let mut ret = unsafe {_ZNK9QDateTime7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toString<T: QDateTime_toString>(&mut self, value: T) -> QString {
    return value.toString(self);
    // return 1;
  }
}

pub trait QDateTime_toString {
  fn toString(self, rsthis: &mut QDateTime) -> QString;
}

// proto:  QString QDateTime::toString(const QString & format);
impl<'a> /*trait*/ QDateTime_toString for (&'a  QString) {
  fn toString(self, rsthis: &mut QDateTime) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addYears<T: QDateTime_addYears>(&mut self, value: T) -> QDateTime {
    return value.addYears(self);
    // return 1;
  }
}

pub trait QDateTime_addYears {
  fn addYears(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::addYears(int years);
impl<'a> /*trait*/ QDateTime_addYears for (i32) {
  fn addYears(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8addYearsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime8addYearsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setMSecsSinceEpoch<T: QDateTime_setMSecsSinceEpoch>(&mut self, value: T)  {
     value.setMSecsSinceEpoch(self);
    // return 1;
  }
}

pub trait QDateTime_setMSecsSinceEpoch {
  fn setMSecsSinceEpoch(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::setMSecsSinceEpoch(qint64 msecs);
impl<'a> /*trait*/ QDateTime_setMSecsSinceEpoch for (i64) {
  fn setMSecsSinceEpoch(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime18setMSecsSinceEpochEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN9QDateTime18setMSecsSinceEpochEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toOffsetFromUtc<T: QDateTime_toOffsetFromUtc>(&mut self, value: T) -> QDateTime {
    return value.toOffsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_toOffsetFromUtc {
  fn toOffsetFromUtc(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds);
impl<'a> /*trait*/ QDateTime_toOffsetFromUtc for (i32) {
  fn toOffsetFromUtc(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime15toOffsetFromUtcEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime15toOffsetFromUtcEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setUtcOffset<T: QDateTime_setUtcOffset>(&mut self, value: T)  {
     value.setUtcOffset(self);
    // return 1;
  }
}

pub trait QDateTime_setUtcOffset {
  fn setUtcOffset(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::setUtcOffset(int seconds);
impl<'a> /*trait*/ QDateTime_setUtcOffset for (i32) {
  fn setUtcOffset(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime12setUtcOffsetEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QDateTime12setUtcOffsetEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addSecs<T: QDateTime_addSecs>(&mut self, value: T) -> QDateTime {
    return value.addSecs(self);
    // return 1;
  }
}

pub trait QDateTime_addSecs {
  fn addSecs(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::addSecs(qint64 secs);
impl<'a> /*trait*/ QDateTime_addSecs for (i64) {
  fn addSecs(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7addSecsEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime7addSecsEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn fromMSecsSinceEpoch<T: QDateTime_fromMSecsSinceEpoch>(&mut self, value: T) -> QDateTime {
    return value.fromMSecsSinceEpoch(self);
    // return 1;
  }
}

pub trait QDateTime_fromMSecsSinceEpoch {
  fn fromMSecsSinceEpoch(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs);
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch for (i64) {
  fn fromMSecsSinceEpoch(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime19fromMSecsSinceEpochEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN9QDateTime19fromMSecsSinceEpochEx(arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QDateTime::NewQDateTime(const QDate & date, const QTime & time, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_NewQDateTime for (&'a  QDate, &'a  QTime, &'a  QTimeZone) {
  fn NewQDateTime(self) -> QDateTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone(qthis, arg0, arg1, arg2)};
    let rsthis = QDateTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn fromString<T: QDateTime_fromString>(&mut self, value: T) -> QDateTime {
    return value.fromString(self);
    // return 1;
  }
}

pub trait QDateTime_fromString {
  fn fromString(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto: static QDateTime QDateTime::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QDateTime_fromString for (&'a  QString, &'a  QString) {
  fn fromString(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDateTime10fromStringERK7QStringS2_(arg0, arg1)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn swap<T: QDateTime_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QDateTime_swap {
  fn swap(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::swap(QDateTime & other);
impl<'a> /*trait*/ QDateTime_swap for (&'a mut QDateTime) {
  fn swap(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toTime_t<T: QDateTime_toTime_t>(&mut self, value: T) -> u32 {
    return value.toTime_t(self);
    // return 1;
  }
}

pub trait QDateTime_toTime_t {
  fn toTime_t(self, rsthis: &mut QDateTime) -> u32;
}

// proto:  unsigned int QDateTime::toTime_t();
impl<'a> /*trait*/ QDateTime_toTime_t for () {
  fn toTime_t(self, rsthis: &mut QDateTime) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8toTime_tEv()};
    let mut ret = unsafe {_ZNK9QDateTime8toTime_tEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn timeZoneAbbreviation<T: QDateTime_timeZoneAbbreviation>(&mut self, value: T) -> QString {
    return value.timeZoneAbbreviation(self);
    // return 1;
  }
}

pub trait QDateTime_timeZoneAbbreviation {
  fn timeZoneAbbreviation(self, rsthis: &mut QDateTime) -> QString;
}

// proto:  QString QDateTime::timeZoneAbbreviation();
impl<'a> /*trait*/ QDateTime_timeZoneAbbreviation for () {
  fn timeZoneAbbreviation(self, rsthis: &mut QDateTime) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime20timeZoneAbbreviationEv()};
    let mut ret = unsafe {_ZNK9QDateTime20timeZoneAbbreviationEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toUTC<T: QDateTime_toUTC>(&mut self, value: T) -> QDateTime {
    return value.toUTC(self);
    // return 1;
  }
}

pub trait QDateTime_toUTC {
  fn toUTC(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::toUTC();
impl<'a> /*trait*/ QDateTime_toUTC for () {
  fn toUTC(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime5toUTCEv()};
    let mut ret = unsafe {_ZNK9QDateTime5toUTCEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn date<T: QDateTime_date>(&mut self, value: T) -> QDate {
    return value.date(self);
    // return 1;
  }
}

pub trait QDateTime_date {
  fn date(self, rsthis: &mut QDateTime) -> QDate;
}

// proto:  QDate QDateTime::date();
impl<'a> /*trait*/ QDateTime_date for () {
  fn date(self, rsthis: &mut QDateTime) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime4dateEv()};
    let mut ret = unsafe {_ZNK9QDateTime4dateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn isNull<T: QDateTime_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QDateTime_isNull {
  fn isNull(self, rsthis: &mut QDateTime) -> i8;
}

// proto:  bool QDateTime::isNull();
impl<'a> /*trait*/ QDateTime_isNull for () {
  fn isNull(self, rsthis: &mut QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6isNullEv()};
    let mut ret = unsafe {_ZNK9QDateTime6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn currentMSecsSinceEpoch<T: QDateTime_currentMSecsSinceEpoch>(&mut self, value: T) -> i64 {
    return value.currentMSecsSinceEpoch(self);
    // return 1;
  }
}

pub trait QDateTime_currentMSecsSinceEpoch {
  fn currentMSecsSinceEpoch(self, rsthis: &mut QDateTime) -> i64;
}

// proto: static long long QDateTime::currentMSecsSinceEpoch();
impl<'a> /*trait*/ QDateTime_currentMSecsSinceEpoch for () {
  fn currentMSecsSinceEpoch(self, rsthis: &mut QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime22currentMSecsSinceEpochEv()};
    let mut ret = unsafe {_ZN9QDateTime22currentMSecsSinceEpochEv()};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn offsetFromUtc<T: QDateTime_offsetFromUtc>(&mut self, value: T) -> i32 {
    return value.offsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_offsetFromUtc {
  fn offsetFromUtc(self, rsthis: &mut QDateTime) -> i32;
}

// proto:  int QDateTime::offsetFromUtc();
impl<'a> /*trait*/ QDateTime_offsetFromUtc for () {
  fn offsetFromUtc(self, rsthis: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime13offsetFromUtcEv()};
    let mut ret = unsafe {_ZNK9QDateTime13offsetFromUtcEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QDateTime::NewQDateTime(const QDate & );
impl<'a> /*trait*/ QDateTime_NewQDateTime for (&'a  QDate) {
  fn NewQDateTime(self) -> QDateTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1ERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QDateTimeC1ERK5QDate(qthis, arg0)};
    let rsthis = QDateTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addMSecs<T: QDateTime_addMSecs>(&mut self, value: T) -> QDateTime {
    return value.addMSecs(self);
    // return 1;
  }
}

pub trait QDateTime_addMSecs {
  fn addMSecs(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::addMSecs(qint64 msecs);
impl<'a> /*trait*/ QDateTime_addMSecs for (i64) {
  fn addMSecs(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8addMSecsEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime8addMSecsEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn secsTo<T: QDateTime_secsTo>(&mut self, value: T) -> i64 {
    return value.secsTo(self);
    // return 1;
  }
}

pub trait QDateTime_secsTo {
  fn secsTo(self, rsthis: &mut QDateTime) -> i64;
}

// proto:  long long QDateTime::secsTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_secsTo for (&'a  QDateTime) {
  fn secsTo(self, rsthis: &mut QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6secsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime6secsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn FreeQDateTime<T: QDateTime_FreeQDateTime>(&mut self, value: T)  {
     value.FreeQDateTime(self);
    // return 1;
  }
}

pub trait QDateTime_FreeQDateTime {
  fn FreeQDateTime(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::FreeQDateTime();
impl<'a> /*trait*/ QDateTime_FreeQDateTime for () {
  fn FreeQDateTime(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeD0Ev()};
     unsafe {_ZN9QDateTimeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addMonths<T: QDateTime_addMonths>(&mut self, value: T) -> QDateTime {
    return value.addMonths(self);
    // return 1;
  }
}

pub trait QDateTime_addMonths {
  fn addMonths(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::addMonths(int months);
impl<'a> /*trait*/ QDateTime_addMonths for (i32) {
  fn addMonths(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime9addMonthsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime9addMonthsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn currentDateTime<T: QDateTime_currentDateTime>(&mut self, value: T) -> QDateTime {
    return value.currentDateTime(self);
    // return 1;
  }
}

pub trait QDateTime_currentDateTime {
  fn currentDateTime(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto: static QDateTime QDateTime::currentDateTime();
impl<'a> /*trait*/ QDateTime_currentDateTime for () {
  fn currentDateTime(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime15currentDateTimeEv()};
    let mut ret = unsafe {_ZN9QDateTime15currentDateTimeEv()};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toTimeZone<T: QDateTime_toTimeZone>(&mut self, value: T) -> QDateTime {
    return value.toTimeZone(self);
    // return 1;
  }
}

pub trait QDateTime_toTimeZone {
  fn toTimeZone(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::toTimeZone(const QTimeZone & toZone);
impl<'a> /*trait*/ QDateTime_toTimeZone for (&'a  QTimeZone) {
  fn toTimeZone(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime10toTimeZoneERK9QTimeZone()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime10toTimeZoneERK9QTimeZone(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn msecsTo<T: QDateTime_msecsTo>(&mut self, value: T) -> i64 {
    return value.msecsTo(self);
    // return 1;
  }
}

pub trait QDateTime_msecsTo {
  fn msecsTo(self, rsthis: &mut QDateTime) -> i64;
}

// proto:  long long QDateTime::msecsTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_msecsTo for (&'a  QDateTime) {
  fn msecsTo(self, rsthis: &mut QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7msecsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime7msecsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn fromTime_t<T: QDateTime_fromTime_t>(&mut self, value: T) -> QDateTime {
    return value.fromTime_t(self);
    // return 1;
  }
}

pub trait QDateTime_fromTime_t {
  fn fromTime_t(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC);
impl<'a> /*trait*/ QDateTime_fromTime_t for (u32) {
  fn fromTime_t(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromTime_tEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN9QDateTime10fromTime_tEj(arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_fromTime_t for (u32, &'a  QTimeZone) {
  fn fromTime_t(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromTime_tEjRK9QTimeZone()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDateTime10fromTime_tEjRK9QTimeZone(arg0, arg1)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setDate<T: QDateTime_setDate>(&mut self, value: T)  {
     value.setDate(self);
    // return 1;
  }
}

pub trait QDateTime_setDate {
  fn setDate(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::setDate(const QDate & date);
impl<'a> /*trait*/ QDateTime_setDate for (&'a  QDate) {
  fn setDate(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime7setDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime7setDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn utcOffset<T: QDateTime_utcOffset>(&mut self, value: T) -> i32 {
    return value.utcOffset(self);
    // return 1;
  }
}

pub trait QDateTime_utcOffset {
  fn utcOffset(self, rsthis: &mut QDateTime) -> i32;
}

// proto:  int QDateTime::utcOffset();
impl<'a> /*trait*/ QDateTime_utcOffset for () {
  fn utcOffset(self, rsthis: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime9utcOffsetEv()};
    let mut ret = unsafe {_ZNK9QDateTime9utcOffsetEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch for (i64, &'a  QTimeZone) {
  fn fromMSecsSinceEpoch(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone(arg0, arg1)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn time<T: QDateTime_time>(&mut self, value: T) -> QTime {
    return value.time(self);
    // return 1;
  }
}

pub trait QDateTime_time {
  fn time(self, rsthis: &mut QDateTime) -> QTime;
}

// proto:  QTime QDateTime::time();
impl<'a> /*trait*/ QDateTime_time for () {
  fn time(self, rsthis: &mut QDateTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime4timeEv()};
    let mut ret = unsafe {_ZNK9QDateTime4timeEv(rsthis.qclsinst)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn daysTo<T: QDateTime_daysTo>(&mut self, value: T) -> i64 {
    return value.daysTo(self);
    // return 1;
  }
}

pub trait QDateTime_daysTo {
  fn daysTo(self, rsthis: &mut QDateTime) -> i64;
}

// proto:  long long QDateTime::daysTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_daysTo for (&'a  QDateTime) {
  fn daysTo(self, rsthis: &mut QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6daysToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime6daysToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addDays<T: QDateTime_addDays>(&mut self, value: T) -> QDateTime {
    return value.addDays(self);
    // return 1;
  }
}

pub trait QDateTime_addDays {
  fn addDays(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto:  QDateTime QDateTime::addDays(qint64 days);
impl<'a> /*trait*/ QDateTime_addDays for (i64) {
  fn addDays(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7addDaysEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime7addDaysEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setTimeZone<T: QDateTime_setTimeZone>(&mut self, value: T)  {
     value.setTimeZone(self);
    // return 1;
  }
}

pub trait QDateTime_setTimeZone {
  fn setTimeZone(self, rsthis: &mut QDateTime) ;
}

// proto:  void QDateTime::setTimeZone(const QTimeZone & toZone);
impl<'a> /*trait*/ QDateTime_setTimeZone for (&'a  QTimeZone) {
  fn setTimeZone(self, rsthis: &mut QDateTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime11setTimeZoneERK9QTimeZone()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime11setTimeZoneERK9QTimeZone(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn currentDateTimeUtc<T: QDateTime_currentDateTimeUtc>(&mut self, value: T) -> QDateTime {
    return value.currentDateTimeUtc(self);
    // return 1;
  }
}

pub trait QDateTime_currentDateTimeUtc {
  fn currentDateTimeUtc(self, rsthis: &mut QDateTime) -> QDateTime;
}

// proto: static QDateTime QDateTime::currentDateTimeUtc();
impl<'a> /*trait*/ QDateTime_currentDateTimeUtc for () {
  fn currentDateTimeUtc(self, rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime18currentDateTimeUtcEv()};
    let mut ret = unsafe {_ZN9QDateTime18currentDateTimeUtcEv()};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

