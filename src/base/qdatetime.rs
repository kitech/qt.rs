// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtime::QTime;
use super::qstring::QString;
use super::qdate::QDate;
use super::qtimezone::QTimeZone;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK9QDateTime11toLocalTimeEv() -> i32;
  fn _ZN9QDateTime16setOffsetFromUtcEi(arg0: c_int) -> i32;
  fn _ZNK9QDateTime8timeZoneEv() -> i32;
  fn _ZN9QDateTime7setTimeERK5QTime(arg0: *const c_void) -> i32;
  fn _ZNK9QDateTime17toMSecsSinceEpochEv() -> i32;
  fn _ZN9QDateTime9setTime_tEj(arg0: c_uint) -> i32;
  fn _ZN9QDateTimeC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QDateTimeC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK9QDateTime14isDaylightTimeEv() -> i32;
  fn _ZNK9QDateTime7isValidEv() -> i32;
  fn _ZNK9QDateTime8toStringERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK9QDateTime8addYearsEi(arg0: c_int) -> i32;
  fn _ZN9QDateTime18setMSecsSinceEpochEx(arg0: c_longlong) -> i32;
  fn _ZNK9QDateTime15toOffsetFromUtcEi(arg0: c_int) -> i32;
  fn _ZN9QDateTime12setUtcOffsetEi(arg0: c_int) -> i32;
  fn _ZNK9QDateTime7addSecsEx(arg0: c_longlong) -> i32;
  fn _ZN9QDateTime19fromMSecsSinceEpochEx(arg0: c_longlong) -> i32;
  fn _ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  fn _ZN9QDateTime10fromStringERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN9QDateTime4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK9QDateTime8toTime_tEv() -> i32;
  fn _ZNK9QDateTime20timeZoneAbbreviationEv() -> i32;
  fn _ZNK9QDateTime5toUTCEv() -> i32;
  fn _ZNK9QDateTime4dateEv() -> i32;
  fn _ZNK9QDateTime6isNullEv() -> i32;
  fn _ZN9QDateTime22currentMSecsSinceEpochEv() -> i32;
  fn _ZNK9QDateTime13offsetFromUtcEv() -> i32;
  fn _ZN9QDateTimeC1ERK5QDate(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK9QDateTime8addMSecsEx(arg0: c_longlong) -> i32;
  fn _ZNK9QDateTime6secsToERKS_(arg0: *const c_void) -> i32;
  fn _ZN9QDateTimeD0Ev() -> i32;
  fn _ZNK9QDateTime9addMonthsEi(arg0: c_int) -> i32;
  fn _ZN9QDateTime15currentDateTimeEv() -> i32;
  fn _ZNK9QDateTime10toTimeZoneERK9QTimeZone(arg0: *const c_void) -> i32;
  fn _ZNK9QDateTime7msecsToERKS_(arg0: *const c_void) -> i32;
  fn _ZN9QDateTime10fromTime_tEj(arg0: c_uint) -> i32;
  fn _ZN9QDateTime10fromTime_tEjRK9QTimeZone(arg0: c_uint, arg1: *const c_void) -> i32;
  fn _ZN9QDateTime7setDateERK5QDate(arg0: *const c_void) -> i32;
  fn _ZNK9QDateTime9utcOffsetEv() -> i32;
  fn _ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone(arg0: c_longlong, arg1: *const c_void) -> i32;
  fn _ZNK9QDateTime4timeEv() -> i32;
  fn _ZNK9QDateTime6daysToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK9QDateTime7addDaysEx(arg0: c_longlong) -> i32;
  fn _ZN9QDateTime11setTimeZoneERK9QTimeZone(arg0: *const c_void) -> i32;
  fn _ZN9QDateTime18currentDateTimeUtcEv() -> i32;
}

// body block begin
// class sizeof(QDateTime)=1
pub struct QDateTime {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDateTime {
  pub fn toLocalTime<T: QDateTime_toLocalTime>(&mut self, value: T) -> i32 {
    value.toLocalTime(self);
    return 1;
  }
}

pub trait QDateTime_toLocalTime {
  fn toLocalTime(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::toLocalTime();
impl<'a> /*trait*/ QDateTime_toLocalTime for () {
  fn toLocalTime(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime11toLocalTimeEv()};
    unsafe {_ZNK9QDateTime11toLocalTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setOffsetFromUtc<T: QDateTime_setOffsetFromUtc>(&mut self, value: T) -> i32 {
    value.setOffsetFromUtc(self);
    return 1;
  }
}

pub trait QDateTime_setOffsetFromUtc {
  fn setOffsetFromUtc(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::setOffsetFromUtc(int offsetSeconds);
impl<'a> /*trait*/ QDateTime_setOffsetFromUtc for (i32) {
  fn setOffsetFromUtc(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime16setOffsetFromUtcEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QDateTime16setOffsetFromUtcEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn timeZone<T: QDateTime_timeZone>(&mut self, value: T) -> i32 {
    value.timeZone(self);
    return 1;
  }
}

pub trait QDateTime_timeZone {
  fn timeZone(self, this: &mut QDateTime) -> i32;
}

// proto: QTimeZone QDateTime::timeZone();
impl<'a> /*trait*/ QDateTime_timeZone for () {
  fn timeZone(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8timeZoneEv()};
    unsafe {_ZNK9QDateTime8timeZoneEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setTime<T: QDateTime_setTime>(&mut self, value: T) -> i32 {
    value.setTime(self);
    return 1;
  }
}

pub trait QDateTime_setTime {
  fn setTime(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::setTime(const QTime & time);
impl<'a> /*trait*/ QDateTime_setTime for (&'a  QTime) {
  fn setTime(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime7setTimeERK5QTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QDateTime7setTimeERK5QTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toMSecsSinceEpoch<T: QDateTime_toMSecsSinceEpoch>(&mut self, value: T) -> i32 {
    value.toMSecsSinceEpoch(self);
    return 1;
  }
}

pub trait QDateTime_toMSecsSinceEpoch {
  fn toMSecsSinceEpoch(self, this: &mut QDateTime) -> i32;
}

// proto: long long QDateTime::toMSecsSinceEpoch();
impl<'a> /*trait*/ QDateTime_toMSecsSinceEpoch for () {
  fn toMSecsSinceEpoch(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime17toMSecsSinceEpochEv()};
    unsafe {_ZNK9QDateTime17toMSecsSinceEpochEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setTime_t<T: QDateTime_setTime_t>(&mut self, value: T) -> i32 {
    value.setTime_t(self);
    return 1;
  }
}

pub trait QDateTime_setTime_t {
  fn setTime_t(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::setTime_t(uint secsSince1Jan1970UTC);
impl<'a> /*trait*/ QDateTime_setTime_t for (u32) {
  fn setTime_t(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime9setTime_tEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN9QDateTime9setTime_tEj(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
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
  pub fn isDaylightTime<T: QDateTime_isDaylightTime>(&mut self, value: T) -> i32 {
    value.isDaylightTime(self);
    return 1;
  }
}

pub trait QDateTime_isDaylightTime {
  fn isDaylightTime(self, this: &mut QDateTime) -> i32;
}

// proto: bool QDateTime::isDaylightTime();
impl<'a> /*trait*/ QDateTime_isDaylightTime for () {
  fn isDaylightTime(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime14isDaylightTimeEv()};
    unsafe {_ZNK9QDateTime14isDaylightTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn isValid<T: QDateTime_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QDateTime_isValid {
  fn isValid(self, this: &mut QDateTime) -> i32;
}

// proto: bool QDateTime::isValid();
impl<'a> /*trait*/ QDateTime_isValid for () {
  fn isValid(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7isValidEv()};
    unsafe {_ZNK9QDateTime7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toString<T: QDateTime_toString>(&mut self, value: T) -> i32 {
    value.toString(self);
    return 1;
  }
}

pub trait QDateTime_toString {
  fn toString(self, this: &mut QDateTime) -> i32;
}

// proto: QString QDateTime::toString(const QString & format);
impl<'a> /*trait*/ QDateTime_toString for (&'a  QString) {
  fn toString(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8toStringERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QDateTime8toStringERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addYears<T: QDateTime_addYears>(&mut self, value: T) -> i32 {
    value.addYears(self);
    return 1;
  }
}

pub trait QDateTime_addYears {
  fn addYears(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::addYears(int years);
impl<'a> /*trait*/ QDateTime_addYears for (i32) {
  fn addYears(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8addYearsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QDateTime8addYearsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setMSecsSinceEpoch<T: QDateTime_setMSecsSinceEpoch>(&mut self, value: T) -> i32 {
    value.setMSecsSinceEpoch(self);
    return 1;
  }
}

pub trait QDateTime_setMSecsSinceEpoch {
  fn setMSecsSinceEpoch(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::setMSecsSinceEpoch(qint64 msecs);
impl<'a> /*trait*/ QDateTime_setMSecsSinceEpoch for (i64) {
  fn setMSecsSinceEpoch(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime18setMSecsSinceEpochEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN9QDateTime18setMSecsSinceEpochEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toOffsetFromUtc<T: QDateTime_toOffsetFromUtc>(&mut self, value: T) -> i32 {
    value.toOffsetFromUtc(self);
    return 1;
  }
}

pub trait QDateTime_toOffsetFromUtc {
  fn toOffsetFromUtc(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds);
impl<'a> /*trait*/ QDateTime_toOffsetFromUtc for (i32) {
  fn toOffsetFromUtc(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime15toOffsetFromUtcEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QDateTime15toOffsetFromUtcEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setUtcOffset<T: QDateTime_setUtcOffset>(&mut self, value: T) -> i32 {
    value.setUtcOffset(self);
    return 1;
  }
}

pub trait QDateTime_setUtcOffset {
  fn setUtcOffset(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::setUtcOffset(int seconds);
impl<'a> /*trait*/ QDateTime_setUtcOffset for (i32) {
  fn setUtcOffset(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime12setUtcOffsetEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QDateTime12setUtcOffsetEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addSecs<T: QDateTime_addSecs>(&mut self, value: T) -> i32 {
    value.addSecs(self);
    return 1;
  }
}

pub trait QDateTime_addSecs {
  fn addSecs(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::addSecs(qint64 secs);
impl<'a> /*trait*/ QDateTime_addSecs for (i64) {
  fn addSecs(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7addSecsEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZNK9QDateTime7addSecsEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn fromMSecsSinceEpoch<T: QDateTime_fromMSecsSinceEpoch>(&mut self, value: T) -> i32 {
    value.fromMSecsSinceEpoch(self);
    return 1;
  }
}

pub trait QDateTime_fromMSecsSinceEpoch {
  fn fromMSecsSinceEpoch(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs);
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch for (i64) {
  fn fromMSecsSinceEpoch(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime19fromMSecsSinceEpochEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN9QDateTime19fromMSecsSinceEpochEx(arg0)};
    return 1;
  }
}

// proto: void QDateTime::NewQDateTime(const QDate & date, const QTime & time, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_NewQDateTime for (&'a  QDate, &'a  QTime, &'a  QTimeZone) {
  fn NewQDateTime(self) -> QDateTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN9QDateTimeC1ERK5QDateRK5QTimeRK9QTimeZone(qthis, arg0, arg1, arg2)};
    let rsthis = QDateTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn fromString<T: QDateTime_fromString>(&mut self, value: T) -> i32 {
    value.fromString(self);
    return 1;
  }
}

pub trait QDateTime_fromString {
  fn fromString(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QDateTime_fromString for (&'a  QString, &'a  QString) {
  fn fromString(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QDateTime10fromStringERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn swap<T: QDateTime_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QDateTime_swap {
  fn swap(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::swap(QDateTime & other);
impl<'a> /*trait*/ QDateTime_swap for (&'a mut QDateTime) {
  fn swap(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QDateTime4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toTime_t<T: QDateTime_toTime_t>(&mut self, value: T) -> i32 {
    value.toTime_t(self);
    return 1;
  }
}

pub trait QDateTime_toTime_t {
  fn toTime_t(self, this: &mut QDateTime) -> i32;
}

// proto: unsigned int QDateTime::toTime_t();
impl<'a> /*trait*/ QDateTime_toTime_t for () {
  fn toTime_t(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8toTime_tEv()};
    unsafe {_ZNK9QDateTime8toTime_tEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn timeZoneAbbreviation<T: QDateTime_timeZoneAbbreviation>(&mut self, value: T) -> i32 {
    value.timeZoneAbbreviation(self);
    return 1;
  }
}

pub trait QDateTime_timeZoneAbbreviation {
  fn timeZoneAbbreviation(self, this: &mut QDateTime) -> i32;
}

// proto: QString QDateTime::timeZoneAbbreviation();
impl<'a> /*trait*/ QDateTime_timeZoneAbbreviation for () {
  fn timeZoneAbbreviation(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime20timeZoneAbbreviationEv()};
    unsafe {_ZNK9QDateTime20timeZoneAbbreviationEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toUTC<T: QDateTime_toUTC>(&mut self, value: T) -> i32 {
    value.toUTC(self);
    return 1;
  }
}

pub trait QDateTime_toUTC {
  fn toUTC(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::toUTC();
impl<'a> /*trait*/ QDateTime_toUTC for () {
  fn toUTC(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime5toUTCEv()};
    unsafe {_ZNK9QDateTime5toUTCEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn date<T: QDateTime_date>(&mut self, value: T) -> i32 {
    value.date(self);
    return 1;
  }
}

pub trait QDateTime_date {
  fn date(self, this: &mut QDateTime) -> i32;
}

// proto: QDate QDateTime::date();
impl<'a> /*trait*/ QDateTime_date for () {
  fn date(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime4dateEv()};
    unsafe {_ZNK9QDateTime4dateEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn isNull<T: QDateTime_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QDateTime_isNull {
  fn isNull(self, this: &mut QDateTime) -> i32;
}

// proto: bool QDateTime::isNull();
impl<'a> /*trait*/ QDateTime_isNull for () {
  fn isNull(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6isNullEv()};
    unsafe {_ZNK9QDateTime6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn currentMSecsSinceEpoch<T: QDateTime_currentMSecsSinceEpoch>(&mut self, value: T) -> i32 {
    value.currentMSecsSinceEpoch(self);
    return 1;
  }
}

pub trait QDateTime_currentMSecsSinceEpoch {
  fn currentMSecsSinceEpoch(self, this: &mut QDateTime) -> i32;
}

// proto: long long QDateTime::currentMSecsSinceEpoch();
impl<'a> /*trait*/ QDateTime_currentMSecsSinceEpoch for () {
  fn currentMSecsSinceEpoch(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime22currentMSecsSinceEpochEv()};
    unsafe {_ZN9QDateTime22currentMSecsSinceEpochEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn offsetFromUtc<T: QDateTime_offsetFromUtc>(&mut self, value: T) -> i32 {
    value.offsetFromUtc(self);
    return 1;
  }
}

pub trait QDateTime_offsetFromUtc {
  fn offsetFromUtc(self, this: &mut QDateTime) -> i32;
}

// proto: int QDateTime::offsetFromUtc();
impl<'a> /*trait*/ QDateTime_offsetFromUtc for () {
  fn offsetFromUtc(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime13offsetFromUtcEv()};
    unsafe {_ZNK9QDateTime13offsetFromUtcEv()};
    return 1;
  }
}

// proto: void QDateTime::NewQDateTime(const QDate & );
impl<'a> /*trait*/ QDateTime_NewQDateTime for (&'a  QDate) {
  fn NewQDateTime(self) -> QDateTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeC1ERK5QDate()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QDateTimeC1ERK5QDate(qthis, arg0)};
    let rsthis = QDateTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addMSecs<T: QDateTime_addMSecs>(&mut self, value: T) -> i32 {
    value.addMSecs(self);
    return 1;
  }
}

pub trait QDateTime_addMSecs {
  fn addMSecs(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::addMSecs(qint64 msecs);
impl<'a> /*trait*/ QDateTime_addMSecs for (i64) {
  fn addMSecs(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime8addMSecsEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZNK9QDateTime8addMSecsEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn secsTo<T: QDateTime_secsTo>(&mut self, value: T) -> i32 {
    value.secsTo(self);
    return 1;
  }
}

pub trait QDateTime_secsTo {
  fn secsTo(self, this: &mut QDateTime) -> i32;
}

// proto: long long QDateTime::secsTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_secsTo for (&'a  QDateTime) {
  fn secsTo(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6secsToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QDateTime6secsToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn FreeQDateTime<T: QDateTime_FreeQDateTime>(&mut self, value: T) -> i32 {
    value.FreeQDateTime(self);
    return 1;
  }
}

pub trait QDateTime_FreeQDateTime {
  fn FreeQDateTime(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::FreeQDateTime();
impl<'a> /*trait*/ QDateTime_FreeQDateTime for () {
  fn FreeQDateTime(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTimeD0Ev()};
    unsafe {_ZN9QDateTimeD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addMonths<T: QDateTime_addMonths>(&mut self, value: T) -> i32 {
    value.addMonths(self);
    return 1;
  }
}

pub trait QDateTime_addMonths {
  fn addMonths(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::addMonths(int months);
impl<'a> /*trait*/ QDateTime_addMonths for (i32) {
  fn addMonths(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime9addMonthsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QDateTime9addMonthsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn currentDateTime<T: QDateTime_currentDateTime>(&mut self, value: T) -> i32 {
    value.currentDateTime(self);
    return 1;
  }
}

pub trait QDateTime_currentDateTime {
  fn currentDateTime(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::currentDateTime();
impl<'a> /*trait*/ QDateTime_currentDateTime for () {
  fn currentDateTime(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime15currentDateTimeEv()};
    unsafe {_ZN9QDateTime15currentDateTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn toTimeZone<T: QDateTime_toTimeZone>(&mut self, value: T) -> i32 {
    value.toTimeZone(self);
    return 1;
  }
}

pub trait QDateTime_toTimeZone {
  fn toTimeZone(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::toTimeZone(const QTimeZone & toZone);
impl<'a> /*trait*/ QDateTime_toTimeZone for (&'a  QTimeZone) {
  fn toTimeZone(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime10toTimeZoneERK9QTimeZone()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QDateTime10toTimeZoneERK9QTimeZone(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn msecsTo<T: QDateTime_msecsTo>(&mut self, value: T) -> i32 {
    value.msecsTo(self);
    return 1;
  }
}

pub trait QDateTime_msecsTo {
  fn msecsTo(self, this: &mut QDateTime) -> i32;
}

// proto: long long QDateTime::msecsTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_msecsTo for (&'a  QDateTime) {
  fn msecsTo(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7msecsToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QDateTime7msecsToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn fromTime_t<T: QDateTime_fromTime_t>(&mut self, value: T) -> i32 {
    value.fromTime_t(self);
    return 1;
  }
}

pub trait QDateTime_fromTime_t {
  fn fromTime_t(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC);
impl<'a> /*trait*/ QDateTime_fromTime_t for (u32) {
  fn fromTime_t(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromTime_tEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN9QDateTime10fromTime_tEj(arg0)};
    return 1;
  }
}

// proto: QDateTime QDateTime::fromTime_t(uint secsSince1Jan1970UTC, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_fromTime_t for (u32, &'a  QTimeZone) {
  fn fromTime_t(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime10fromTime_tEjRK9QTimeZone()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QDateTime10fromTime_tEjRK9QTimeZone(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setDate<T: QDateTime_setDate>(&mut self, value: T) -> i32 {
    value.setDate(self);
    return 1;
  }
}

pub trait QDateTime_setDate {
  fn setDate(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::setDate(const QDate & date);
impl<'a> /*trait*/ QDateTime_setDate for (&'a  QDate) {
  fn setDate(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime7setDateERK5QDate()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QDateTime7setDateERK5QDate(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn utcOffset<T: QDateTime_utcOffset>(&mut self, value: T) -> i32 {
    value.utcOffset(self);
    return 1;
  }
}

pub trait QDateTime_utcOffset {
  fn utcOffset(self, this: &mut QDateTime) -> i32;
}

// proto: int QDateTime::utcOffset();
impl<'a> /*trait*/ QDateTime_utcOffset for () {
  fn utcOffset(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime9utcOffsetEv()};
    unsafe {_ZNK9QDateTime9utcOffsetEv()};
    return 1;
  }
}

// proto: QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, const QTimeZone & timeZone);
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch for (i64, &'a  QTimeZone) {
  fn fromMSecsSinceEpoch(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn time<T: QDateTime_time>(&mut self, value: T) -> i32 {
    value.time(self);
    return 1;
  }
}

pub trait QDateTime_time {
  fn time(self, this: &mut QDateTime) -> i32;
}

// proto: QTime QDateTime::time();
impl<'a> /*trait*/ QDateTime_time for () {
  fn time(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime4timeEv()};
    unsafe {_ZNK9QDateTime4timeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn daysTo<T: QDateTime_daysTo>(&mut self, value: T) -> i32 {
    value.daysTo(self);
    return 1;
  }
}

pub trait QDateTime_daysTo {
  fn daysTo(self, this: &mut QDateTime) -> i32;
}

// proto: long long QDateTime::daysTo(const QDateTime & );
impl<'a> /*trait*/ QDateTime_daysTo for (&'a  QDateTime) {
  fn daysTo(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime6daysToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QDateTime6daysToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn addDays<T: QDateTime_addDays>(&mut self, value: T) -> i32 {
    value.addDays(self);
    return 1;
  }
}

pub trait QDateTime_addDays {
  fn addDays(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::addDays(qint64 days);
impl<'a> /*trait*/ QDateTime_addDays for (i64) {
  fn addDays(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateTime7addDaysEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZNK9QDateTime7addDaysEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn setTimeZone<T: QDateTime_setTimeZone>(&mut self, value: T) -> i32 {
    value.setTimeZone(self);
    return 1;
  }
}

pub trait QDateTime_setTimeZone {
  fn setTimeZone(self, this: &mut QDateTime) -> i32;
}

// proto: void QDateTime::setTimeZone(const QTimeZone & toZone);
impl<'a> /*trait*/ QDateTime_setTimeZone for (&'a  QTimeZone) {
  fn setTimeZone(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime11setTimeZoneERK9QTimeZone()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QDateTime11setTimeZoneERK9QTimeZone(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTime {
  pub fn currentDateTimeUtc<T: QDateTime_currentDateTimeUtc>(&mut self, value: T) -> i32 {
    value.currentDateTimeUtc(self);
    return 1;
  }
}

pub trait QDateTime_currentDateTimeUtc {
  fn currentDateTimeUtc(self, this: &mut QDateTime) -> i32;
}

// proto: QDateTime QDateTime::currentDateTimeUtc();
impl<'a> /*trait*/ QDateTime_currentDateTimeUtc for () {
  fn currentDateTimeUtc(self, this: &mut QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateTime18currentDateTimeUtcEv()};
    unsafe {_ZN9QDateTime18currentDateTimeUtcEv()};
    return 1;
  }
}

