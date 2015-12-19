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

// proto:  QDateTime QDateTime::toLocalTime();
impl /*struct*/ QDateTime {
  pub fn toLocalTime<RetType, T: QDateTime_toLocalTime<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toLocalTime(self);
    // return 1;
  }
}

pub trait QDateTime_toLocalTime<RetType> {
  fn toLocalTime(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::toLocalTime();
impl<'a> /*trait*/ QDateTime_toLocalTime<QDateTime> for () {
  fn toLocalTime(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime11toLocalTimeEv()};
    let mut ret = unsafe {_ZNK9QDateTime11toLocalTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDateTime::setOffsetFromUtc(int offsetSeconds);
impl /*struct*/ QDateTime {
  pub fn setOffsetFromUtc<RetType, T: QDateTime_setOffsetFromUtc<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setOffsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_setOffsetFromUtc<RetType> {
  fn setOffsetFromUtc(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::setOffsetFromUtc(int offsetSeconds);
impl<'a> /*trait*/ QDateTime_setOffsetFromUtc<()> for (i32) {
  fn setOffsetFromUtc(self , rsthis: &mut QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime16setOffsetFromUtcEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QDateTime16setOffsetFromUtcEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QTimeZone QDateTime::timeZone();
impl /*struct*/ QDateTime {
  pub fn timeZone<RetType, T: QDateTime_timeZone<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.timeZone(self);
    // return 1;
  }
}

pub trait QDateTime_timeZone<RetType> {
  fn timeZone(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QTimeZone QDateTime::timeZone();
impl<'a> /*trait*/ QDateTime_timeZone<QTimeZone> for () {
  fn timeZone(self , rsthis: &mut QDateTime) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8timeZoneEv()};
    let mut ret = unsafe {_ZNK9QDateTime8timeZoneEv(rsthis.qclsinst)};
    let mut ret1 = QTimeZone{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDateTime::setTime(const QTime & time);
impl /*struct*/ QDateTime {
  pub fn setTime<RetType, T: QDateTime_setTime<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTime(self);
    // return 1;
  }
}

pub trait QDateTime_setTime<RetType> {
  fn setTime(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::setTime(const QTime & time);
impl<'a> /*trait*/ QDateTime_setTime<()> for (&'a  QTime) {
  fn setTime(self , rsthis: &mut QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime7setTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime7setTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  long long QDateTime::toMSecsSinceEpoch();
impl /*struct*/ QDateTime {
  pub fn toMSecsSinceEpoch<RetType, T: QDateTime_toMSecsSinceEpoch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toMSecsSinceEpoch(self);
    // return 1;
  }
}

pub trait QDateTime_toMSecsSinceEpoch<RetType> {
  fn toMSecsSinceEpoch(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  long long QDateTime::toMSecsSinceEpoch();
impl<'a> /*trait*/ QDateTime_toMSecsSinceEpoch<i64> for () {
  fn toMSecsSinceEpoch(self , rsthis: &mut QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime17toMSecsSinceEpochEv()};
    let mut ret = unsafe {_ZNK9QDateTime17toMSecsSinceEpochEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// proto:  void QDateTime::setTime_t(uint secsSince1Jan1970UTC);
impl /*struct*/ QDateTime {
  pub fn setTime_t<RetType, T: QDateTime_setTime_t<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTime_t(self);
    // return 1;
  }
}

pub trait QDateTime_setTime_t<RetType> {
  fn setTime_t(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::setTime_t(uint secsSince1Jan1970UTC);
impl<'a> /*trait*/ QDateTime_setTime_t<()> for (u32) {
  fn setTime_t(self , rsthis: &mut QDateTime) -> () {
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

// proto:  bool QDateTime::isDaylightTime();
impl /*struct*/ QDateTime {
  pub fn isDaylightTime<RetType, T: QDateTime_isDaylightTime<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isDaylightTime(self);
    // return 1;
  }
}

pub trait QDateTime_isDaylightTime<RetType> {
  fn isDaylightTime(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  bool QDateTime::isDaylightTime();
impl<'a> /*trait*/ QDateTime_isDaylightTime<i8> for () {
  fn isDaylightTime(self , rsthis: &mut QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime14isDaylightTimeEv()};
    let mut ret = unsafe {_ZNK9QDateTime14isDaylightTimeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QDateTime::isValid();
impl /*struct*/ QDateTime {
  pub fn isValid<RetType, T: QDateTime_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QDateTime_isValid<RetType> {
  fn isValid(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  bool QDateTime::isValid();
impl<'a> /*trait*/ QDateTime_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7isValidEv()};
    let mut ret = unsafe {_ZNK9QDateTime7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QDateTime::toString(const QString & format);
impl /*struct*/ QDateTime {
  pub fn toString<RetType, T: QDateTime_toString<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QDateTime_toString<RetType> {
  fn toString(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QString QDateTime::toString(const QString & format);
impl<'a> /*trait*/ QDateTime_toString<QString> for (&'a  QString) {
  fn toString(self , rsthis: &mut QDateTime) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QDateTime QDateTime::addYears(int years);
impl /*struct*/ QDateTime {
  pub fn addYears<RetType, T: QDateTime_addYears<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addYears(self);
    // return 1;
  }
}

pub trait QDateTime_addYears<RetType> {
  fn addYears(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::addYears(int years);
impl<'a> /*trait*/ QDateTime_addYears<QDateTime> for (i32) {
  fn addYears(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8addYearsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime8addYearsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDateTime::setMSecsSinceEpoch(qint64 msecs);
impl /*struct*/ QDateTime {
  pub fn setMSecsSinceEpoch<RetType, T: QDateTime_setMSecsSinceEpoch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMSecsSinceEpoch(self);
    // return 1;
  }
}

pub trait QDateTime_setMSecsSinceEpoch<RetType> {
  fn setMSecsSinceEpoch(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::setMSecsSinceEpoch(qint64 msecs);
impl<'a> /*trait*/ QDateTime_setMSecsSinceEpoch<()> for (i64) {
  fn setMSecsSinceEpoch(self , rsthis: &mut QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime18setMSecsSinceEpochEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN9QDateTime18setMSecsSinceEpochEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds);
impl /*struct*/ QDateTime {
  pub fn toOffsetFromUtc<RetType, T: QDateTime_toOffsetFromUtc<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toOffsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_toOffsetFromUtc<RetType> {
  fn toOffsetFromUtc(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds);
impl<'a> /*trait*/ QDateTime_toOffsetFromUtc<QDateTime> for (i32) {
  fn toOffsetFromUtc(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime15toOffsetFromUtcEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime15toOffsetFromUtcEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDateTime::setUtcOffset(int seconds);
impl /*struct*/ QDateTime {
  pub fn setUtcOffset<RetType, T: QDateTime_setUtcOffset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setUtcOffset(self);
    // return 1;
  }
}

pub trait QDateTime_setUtcOffset<RetType> {
  fn setUtcOffset(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::setUtcOffset(int seconds);
impl<'a> /*trait*/ QDateTime_setUtcOffset<()> for (i32) {
  fn setUtcOffset(self , rsthis: &mut QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime12setUtcOffsetEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QDateTime12setUtcOffsetEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QDateTime QDateTime::addSecs(qint64 secs);
impl /*struct*/ QDateTime {
  pub fn addSecs<RetType, T: QDateTime_addSecs<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addSecs(self);
    // return 1;
  }
}

pub trait QDateTime_addSecs<RetType> {
  fn addSecs(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::addSecs(qint64 secs);
impl<'a> /*trait*/ QDateTime_addSecs<QDateTime> for (i64) {
  fn addSecs(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7addSecsEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime7addSecsEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
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
impl<'a> /*trait*/ QDateTime_fromString_s<QDateTime> for (&'a  QString, &'a  QString) {
  fn fromString_s(self ) -> QDateTime {
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

// proto:  void QDateTime::swap(QDateTime & other);
impl /*struct*/ QDateTime {
  pub fn swap<RetType, T: QDateTime_swap<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDateTime_swap<RetType> {
  fn swap(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::swap(QDateTime & other);
impl<'a> /*trait*/ QDateTime_swap<()> for (&'a mut QDateTime) {
  fn swap(self , rsthis: &mut QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  unsigned int QDateTime::toTime_t();
impl /*struct*/ QDateTime {
  pub fn toTime_t<RetType, T: QDateTime_toTime_t<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toTime_t(self);
    // return 1;
  }
}

pub trait QDateTime_toTime_t<RetType> {
  fn toTime_t(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  unsigned int QDateTime::toTime_t();
impl<'a> /*trait*/ QDateTime_toTime_t<u32> for () {
  fn toTime_t(self , rsthis: &mut QDateTime) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8toTime_tEv()};
    let mut ret = unsafe {_ZNK9QDateTime8toTime_tEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

// proto:  QString QDateTime::timeZoneAbbreviation();
impl /*struct*/ QDateTime {
  pub fn timeZoneAbbreviation<RetType, T: QDateTime_timeZoneAbbreviation<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.timeZoneAbbreviation(self);
    // return 1;
  }
}

pub trait QDateTime_timeZoneAbbreviation<RetType> {
  fn timeZoneAbbreviation(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QString QDateTime::timeZoneAbbreviation();
impl<'a> /*trait*/ QDateTime_timeZoneAbbreviation<QString> for () {
  fn timeZoneAbbreviation(self , rsthis: &mut QDateTime) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime20timeZoneAbbreviationEv()};
    let mut ret = unsafe {_ZNK9QDateTime20timeZoneAbbreviationEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QDateTime QDateTime::toUTC();
impl /*struct*/ QDateTime {
  pub fn toUTC<RetType, T: QDateTime_toUTC<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toUTC(self);
    // return 1;
  }
}

pub trait QDateTime_toUTC<RetType> {
  fn toUTC(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::toUTC();
impl<'a> /*trait*/ QDateTime_toUTC<QDateTime> for () {
  fn toUTC(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime5toUTCEv()};
    let mut ret = unsafe {_ZNK9QDateTime5toUTCEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QDate QDateTime::date();
impl /*struct*/ QDateTime {
  pub fn date<RetType, T: QDateTime_date<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.date(self);
    // return 1;
  }
}

pub trait QDateTime_date<RetType> {
  fn date(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDate QDateTime::date();
impl<'a> /*trait*/ QDateTime_date<QDate> for () {
  fn date(self , rsthis: &mut QDateTime) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime4dateEv()};
    let mut ret = unsafe {_ZNK9QDateTime4dateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QDateTime::isNull();
impl /*struct*/ QDateTime {
  pub fn isNull<RetType, T: QDateTime_isNull<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QDateTime_isNull<RetType> {
  fn isNull(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  bool QDateTime::isNull();
impl<'a> /*trait*/ QDateTime_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QDateTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6isNullEv()};
    let mut ret = unsafe {_ZNK9QDateTime6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: static long long QDateTime::currentMSecsSinceEpoch();
impl /*struct*/ QDateTime {
  pub fn currentMSecsSinceEpoch_s<RetType, T: QDateTime_currentMSecsSinceEpoch_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentMSecsSinceEpoch_s();
    // return 1;
  }
}

pub trait QDateTime_currentMSecsSinceEpoch_s<RetType> {
  fn currentMSecsSinceEpoch_s(self ) -> RetType;
}

// proto: static long long QDateTime::currentMSecsSinceEpoch();
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
  pub fn offsetFromUtc<RetType, T: QDateTime_offsetFromUtc<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.offsetFromUtc(self);
    // return 1;
  }
}

pub trait QDateTime_offsetFromUtc<RetType> {
  fn offsetFromUtc(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  int QDateTime::offsetFromUtc();
impl<'a> /*trait*/ QDateTime_offsetFromUtc<i32> for () {
  fn offsetFromUtc(self , rsthis: &mut QDateTime) -> i32 {
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

// proto:  QDateTime QDateTime::addMSecs(qint64 msecs);
impl /*struct*/ QDateTime {
  pub fn addMSecs<RetType, T: QDateTime_addMSecs<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addMSecs(self);
    // return 1;
  }
}

pub trait QDateTime_addMSecs<RetType> {
  fn addMSecs(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::addMSecs(qint64 msecs);
impl<'a> /*trait*/ QDateTime_addMSecs<QDateTime> for (i64) {
  fn addMSecs(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8addMSecsEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime8addMSecsEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  long long QDateTime::secsTo(const QDateTime & );
impl /*struct*/ QDateTime {
  pub fn secsTo<RetType, T: QDateTime_secsTo<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.secsTo(self);
    // return 1;
  }
}

pub trait QDateTime_secsTo<RetType> {
  fn secsTo(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  long long QDateTime::secsTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_secsTo<i64> for (&'a  QDateTime) {
  fn secsTo(self , rsthis: &mut QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6secsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime6secsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

// proto:  void QDateTime::FreeQDateTime();
impl /*struct*/ QDateTime {
  pub fn FreeQDateTime<RetType, T: QDateTime_FreeQDateTime<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQDateTime(self);
    // return 1;
  }
}

pub trait QDateTime_FreeQDateTime<RetType> {
  fn FreeQDateTime(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::FreeQDateTime();
impl<'a> /*trait*/ QDateTime_FreeQDateTime<()> for () {
  fn FreeQDateTime(self , rsthis: &mut QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeD0Ev()};
     unsafe {_ZN9QDateTimeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QDateTime QDateTime::addMonths(int months);
impl /*struct*/ QDateTime {
  pub fn addMonths<RetType, T: QDateTime_addMonths<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addMonths(self);
    // return 1;
  }
}

pub trait QDateTime_addMonths<RetType> {
  fn addMonths(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::addMonths(int months);
impl<'a> /*trait*/ QDateTime_addMonths<QDateTime> for (i32) {
  fn addMonths(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime9addMonthsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QDateTime9addMonthsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
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
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QDateTime QDateTime::toTimeZone(const QTimeZone & toZone);
impl /*struct*/ QDateTime {
  pub fn toTimeZone<RetType, T: QDateTime_toTimeZone<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toTimeZone(self);
    // return 1;
  }
}

pub trait QDateTime_toTimeZone<RetType> {
  fn toTimeZone(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::toTimeZone(const QTimeZone & toZone);
impl<'a> /*trait*/ QDateTime_toTimeZone<QDateTime> for (&'a  QTimeZone) {
  fn toTimeZone(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime10toTimeZoneERK9QTimeZone()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDateTime10toTimeZoneERK9QTimeZone(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  long long QDateTime::msecsTo(const QDateTime & );
impl /*struct*/ QDateTime {
  pub fn msecsTo<RetType, T: QDateTime_msecsTo<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.msecsTo(self);
    // return 1;
  }
}

pub trait QDateTime_msecsTo<RetType> {
  fn msecsTo(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  long long QDateTime::msecsTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_msecsTo<i64> for (&'a  QDateTime) {
  fn msecsTo(self , rsthis: &mut QDateTime) -> i64 {
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
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_fromTime_t_s<QDateTime> for (u32, &'a  QTimeZone) {
  fn fromTime_t_s(self ) -> QDateTime {
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

// proto:  void QDateTime::setDate(const QDate & date);
impl /*struct*/ QDateTime {
  pub fn setDate<RetType, T: QDateTime_setDate<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setDate(self);
    // return 1;
  }
}

pub trait QDateTime_setDate<RetType> {
  fn setDate(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::setDate(const QDate & date);
impl<'a> /*trait*/ QDateTime_setDate<()> for (&'a  QDate) {
  fn setDate(self , rsthis: &mut QDateTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime7setDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateTime7setDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QDateTime::utcOffset();
impl /*struct*/ QDateTime {
  pub fn utcOffset<RetType, T: QDateTime_utcOffset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.utcOffset(self);
    // return 1;
  }
}

pub trait QDateTime_utcOffset<RetType> {
  fn utcOffset(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  int QDateTime::utcOffset();
impl<'a> /*trait*/ QDateTime_utcOffset<i32> for () {
  fn utcOffset(self , rsthis: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime9utcOffsetEv()};
    let mut ret = unsafe {_ZNK9QDateTime9utcOffsetEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch_s<QDateTime> for (i64, &'a  QTimeZone) {
  fn fromMSecsSinceEpoch_s(self ) -> QDateTime {
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

// proto:  QTime QDateTime::time();
impl /*struct*/ QDateTime {
  pub fn time<RetType, T: QDateTime_time<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.time(self);
    // return 1;
  }
}

pub trait QDateTime_time<RetType> {
  fn time(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QTime QDateTime::time();
impl<'a> /*trait*/ QDateTime_time<QTime> for () {
  fn time(self , rsthis: &mut QDateTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime4timeEv()};
    let mut ret = unsafe {_ZNK9QDateTime4timeEv(rsthis.qclsinst)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  long long QDateTime::daysTo(const QDateTime & );
impl /*struct*/ QDateTime {
  pub fn daysTo<RetType, T: QDateTime_daysTo<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.daysTo(self);
    // return 1;
  }
}

pub trait QDateTime_daysTo<RetType> {
  fn daysTo(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  long long QDateTime::daysTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_daysTo<i64> for (&'a  QDateTime) {
  fn daysTo(self , rsthis: &mut QDateTime) -> i64 {
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
  pub fn addDays<RetType, T: QDateTime_addDays<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addDays(self);
    // return 1;
  }
}

pub trait QDateTime_addDays<RetType> {
  fn addDays(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  QDateTime QDateTime::addDays(qint64 days);
impl<'a> /*trait*/ QDateTime_addDays<QDateTime> for (i64) {
  fn addDays(self , rsthis: &mut QDateTime) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7addDaysEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK9QDateTime7addDaysEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDateTime::setTimeZone(const QTimeZone & toZone);
impl /*struct*/ QDateTime {
  pub fn setTimeZone<RetType, T: QDateTime_setTimeZone<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTimeZone(self);
    // return 1;
  }
}

pub trait QDateTime_setTimeZone<RetType> {
  fn setTimeZone(self , rsthis: &mut QDateTime) -> RetType;
}

// proto:  void QDateTime::setTimeZone(const QTimeZone & toZone);
impl<'a> /*trait*/ QDateTime_setTimeZone<()> for (&'a  QTimeZone) {
  fn setTimeZone(self , rsthis: &mut QDateTime) -> () {
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
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

