// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdatetime::QDateTime;
use super::qstring::QString;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds();
  fn _ZN9QTimeZone20availableTimeZoneIdsEv() ;
  // proto:  void QTimeZone::swap(QTimeZone & other);
  fn _ZN9QTimeZone4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTimeZone::isValid();
  fn _ZNK9QTimeZone7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTimeZone::hasDaylightTime();
  fn _ZNK9QTimeZone15hasDaylightTimeEv(qthis: *mut c_void) -> int8_t;
  // proto: static QTimeZone QTimeZone::utc();
  fn _ZN9QTimeZone3utcEv() -> *mut c_void;
  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds(int offsetSeconds);
  fn _ZN9QTimeZone20availableTimeZoneIdsEi(arg0: c_int) ;
  // proto:  void QTimeZone::NewQTimeZone(int offsetSeconds);
  fn _ZN9QTimeZoneC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QTimeZone::abbreviation(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone12abbreviationERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTimeZone::NewQTimeZone();
  fn _ZN9QTimeZoneC1Ev(qthis: *mut c_void) ;
  // proto: static QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray & ianaId);
  fn _ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QTimeZone::systemTimeZoneId();
  fn _ZN9QTimeZone16systemTimeZoneIdEv() -> *mut c_void;
  // proto:  bool QTimeZone::isDaylightTime(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone14isDaylightTimeERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto: static bool QTimeZone::isTimeZoneIdAvailable(const QByteArray & ianaId);
  fn _ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray(arg0: *mut c_void) -> int8_t;
  // proto:  QString QTimeZone::comment();
  fn _ZNK9QTimeZone7commentEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray & windowsId);
  fn _ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTimeZone::hasTransitions();
  fn _ZNK9QTimeZone14hasTransitionsEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTimeZone::daylightTimeOffset(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto: static QTimeZone QTimeZone::systemTimeZone();
  fn _ZN9QTimeZone14systemTimeZoneEv() -> *mut c_void;
  // proto:  void QTimeZone::NewQTimeZone(const QByteArray & ianaId);
  fn _ZN9QTimeZoneC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTimeZone::NewQTimeZone(const QTimeZone & other);
  fn _ZN9QTimeZoneC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTimeZone::FreeQTimeZone();
  fn _ZN9QTimeZoneD0Ev(qthis: *mut c_void) ;
  // proto:  int QTimeZone::standardTimeOffset(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone18standardTimeOffsetERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QByteArray QTimeZone::id();
  fn _ZNK9QTimeZone2idEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTimeZone::offsetFromUtc(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone13offsetFromUtcERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto: static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray & windowsId);
  fn _ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTimeZone)=1
pub struct QTimeZone {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimeZone {
  pub fn availableTimeZoneIds<T: QTimeZone_availableTimeZoneIds>(&mut self, value: T)  {
     value.availableTimeZoneIds(self);
    // return 1;
  }
}

pub trait QTimeZone_availableTimeZoneIds {
  fn availableTimeZoneIds(self, rsthis: &mut QTimeZone) ;
}

// proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds();
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds for () {
  fn availableTimeZoneIds(self, rsthis: &mut QTimeZone)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEv()};
     unsafe {_ZN9QTimeZone20availableTimeZoneIdsEv()};
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn swap<T: QTimeZone_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QTimeZone_swap {
  fn swap(self, rsthis: &mut QTimeZone) ;
}

// proto:  void QTimeZone::swap(QTimeZone & other);
impl<'a> /*trait*/ QTimeZone_swap for (&'a mut QTimeZone) {
  fn swap(self, rsthis: &mut QTimeZone)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTimeZone4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn isValid<T: QTimeZone_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTimeZone_isValid {
  fn isValid(self, rsthis: &mut QTimeZone) -> i8;
}

// proto:  bool QTimeZone::isValid();
impl<'a> /*trait*/ QTimeZone_isValid for () {
  fn isValid(self, rsthis: &mut QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone7isValidEv()};
    let mut ret = unsafe {_ZNK9QTimeZone7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn hasDaylightTime<T: QTimeZone_hasDaylightTime>(&mut self, value: T) -> i8 {
    return value.hasDaylightTime(self);
    // return 1;
  }
}

pub trait QTimeZone_hasDaylightTime {
  fn hasDaylightTime(self, rsthis: &mut QTimeZone) -> i8;
}

// proto:  bool QTimeZone::hasDaylightTime();
impl<'a> /*trait*/ QTimeZone_hasDaylightTime for () {
  fn hasDaylightTime(self, rsthis: &mut QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone15hasDaylightTimeEv()};
    let mut ret = unsafe {_ZNK9QTimeZone15hasDaylightTimeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn utc<T: QTimeZone_utc>(&mut self, value: T) -> QTimeZone {
    return value.utc(self);
    // return 1;
  }
}

pub trait QTimeZone_utc {
  fn utc(self, rsthis: &mut QTimeZone) -> QTimeZone;
}

// proto: static QTimeZone QTimeZone::utc();
impl<'a> /*trait*/ QTimeZone_utc for () {
  fn utc(self, rsthis: &mut QTimeZone) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone3utcEv()};
    let mut ret = unsafe {_ZN9QTimeZone3utcEv()};
    let mut ret1 = QTimeZone{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds(int offsetSeconds);
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds for (i32) {
  fn availableTimeZoneIds(self, rsthis: &mut QTimeZone)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeZone20availableTimeZoneIdsEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn NewQTimeZone<T: QTimeZone_NewQTimeZone>(value: T) -> QTimeZone {
    let rsthis = value.NewQTimeZone();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeZone_NewQTimeZone {
  fn NewQTimeZone(self) -> QTimeZone;
}

// proto: void QTimeZone::NewQTimeZone(int offsetSeconds);
impl<'a> /*trait*/ QTimeZone_NewQTimeZone for (i32) {
  fn NewQTimeZone(self) -> QTimeZone {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTimeZoneC1Ei(qthis, arg0)};
    let rsthis = QTimeZone{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn abbreviation<T: QTimeZone_abbreviation>(&mut self, value: T) -> QString {
    return value.abbreviation(self);
    // return 1;
  }
}

pub trait QTimeZone_abbreviation {
  fn abbreviation(self, rsthis: &mut QTimeZone) -> QString;
}

// proto:  QString QTimeZone::abbreviation(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_abbreviation for (&'a  QDateTime) {
  fn abbreviation(self, rsthis: &mut QTimeZone) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone12abbreviationERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTimeZone12abbreviationERK9QDateTime(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTimeZone::NewQTimeZone();
impl<'a> /*trait*/ QTimeZone_NewQTimeZone for () {
  fn NewQTimeZone(self) -> QTimeZone {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1Ev()};
    unsafe {_ZN9QTimeZoneC1Ev(qthis)};
    let rsthis = QTimeZone{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn ianaIdToWindowsId<T: QTimeZone_ianaIdToWindowsId>(&mut self, value: T) -> QByteArray {
    return value.ianaIdToWindowsId(self);
    // return 1;
  }
}

pub trait QTimeZone_ianaIdToWindowsId {
  fn ianaIdToWindowsId(self, rsthis: &mut QTimeZone) -> QByteArray;
}

// proto: static QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_ianaIdToWindowsId for (&'a  QByteArray) {
  fn ianaIdToWindowsId(self, rsthis: &mut QTimeZone) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn systemTimeZoneId<T: QTimeZone_systemTimeZoneId>(&mut self, value: T) -> QByteArray {
    return value.systemTimeZoneId(self);
    // return 1;
  }
}

pub trait QTimeZone_systemTimeZoneId {
  fn systemTimeZoneId(self, rsthis: &mut QTimeZone) -> QByteArray;
}

// proto: static QByteArray QTimeZone::systemTimeZoneId();
impl<'a> /*trait*/ QTimeZone_systemTimeZoneId for () {
  fn systemTimeZoneId(self, rsthis: &mut QTimeZone) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone16systemTimeZoneIdEv()};
    let mut ret = unsafe {_ZN9QTimeZone16systemTimeZoneIdEv()};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn isDaylightTime<T: QTimeZone_isDaylightTime>(&mut self, value: T) -> i8 {
    return value.isDaylightTime(self);
    // return 1;
  }
}

pub trait QTimeZone_isDaylightTime {
  fn isDaylightTime(self, rsthis: &mut QTimeZone) -> i8;
}

// proto:  bool QTimeZone::isDaylightTime(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_isDaylightTime for (&'a  QDateTime) {
  fn isDaylightTime(self, rsthis: &mut QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone14isDaylightTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTimeZone14isDaylightTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn isTimeZoneIdAvailable<T: QTimeZone_isTimeZoneIdAvailable>(&mut self, value: T) -> i8 {
    return value.isTimeZoneIdAvailable(self);
    // return 1;
  }
}

pub trait QTimeZone_isTimeZoneIdAvailable {
  fn isTimeZoneIdAvailable(self, rsthis: &mut QTimeZone) -> i8;
}

// proto: static bool QTimeZone::isTimeZoneIdAvailable(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_isTimeZoneIdAvailable for (&'a  QByteArray) {
  fn isTimeZoneIdAvailable(self, rsthis: &mut QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn comment<T: QTimeZone_comment>(&mut self, value: T) -> QString {
    return value.comment(self);
    // return 1;
  }
}

pub trait QTimeZone_comment {
  fn comment(self, rsthis: &mut QTimeZone) -> QString;
}

// proto:  QString QTimeZone::comment();
impl<'a> /*trait*/ QTimeZone_comment for () {
  fn comment(self, rsthis: &mut QTimeZone) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone7commentEv()};
    let mut ret = unsafe {_ZNK9QTimeZone7commentEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn windowsIdToDefaultIanaId<T: QTimeZone_windowsIdToDefaultIanaId>(&mut self, value: T) -> QByteArray {
    return value.windowsIdToDefaultIanaId(self);
    // return 1;
  }
}

pub trait QTimeZone_windowsIdToDefaultIanaId {
  fn windowsIdToDefaultIanaId(self, rsthis: &mut QTimeZone) -> QByteArray;
}

// proto: static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray & windowsId);
impl<'a> /*trait*/ QTimeZone_windowsIdToDefaultIanaId for (&'a  QByteArray) {
  fn windowsIdToDefaultIanaId(self, rsthis: &mut QTimeZone) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn hasTransitions<T: QTimeZone_hasTransitions>(&mut self, value: T) -> i8 {
    return value.hasTransitions(self);
    // return 1;
  }
}

pub trait QTimeZone_hasTransitions {
  fn hasTransitions(self, rsthis: &mut QTimeZone) -> i8;
}

// proto:  bool QTimeZone::hasTransitions();
impl<'a> /*trait*/ QTimeZone_hasTransitions for () {
  fn hasTransitions(self, rsthis: &mut QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone14hasTransitionsEv()};
    let mut ret = unsafe {_ZNK9QTimeZone14hasTransitionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn daylightTimeOffset<T: QTimeZone_daylightTimeOffset>(&mut self, value: T) -> i32 {
    return value.daylightTimeOffset(self);
    // return 1;
  }
}

pub trait QTimeZone_daylightTimeOffset {
  fn daylightTimeOffset(self, rsthis: &mut QTimeZone) -> i32;
}

// proto:  int QTimeZone::daylightTimeOffset(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_daylightTimeOffset for (&'a  QDateTime) {
  fn daylightTimeOffset(self, rsthis: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn systemTimeZone<T: QTimeZone_systemTimeZone>(&mut self, value: T) -> QTimeZone {
    return value.systemTimeZone(self);
    // return 1;
  }
}

pub trait QTimeZone_systemTimeZone {
  fn systemTimeZone(self, rsthis: &mut QTimeZone) -> QTimeZone;
}

// proto: static QTimeZone QTimeZone::systemTimeZone();
impl<'a> /*trait*/ QTimeZone_systemTimeZone for () {
  fn systemTimeZone(self, rsthis: &mut QTimeZone) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone14systemTimeZoneEv()};
    let mut ret = unsafe {_ZN9QTimeZone14systemTimeZoneEv()};
    let mut ret1 = QTimeZone{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTimeZone::NewQTimeZone(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_NewQTimeZone for (&'a  QByteArray) {
  fn NewQTimeZone(self) -> QTimeZone {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeZoneC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QTimeZone{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTimeZone::NewQTimeZone(const QTimeZone & other);
impl<'a> /*trait*/ QTimeZone_NewQTimeZone for (&'a  QTimeZone) {
  fn NewQTimeZone(self) -> QTimeZone {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeZoneC1ERKS_(qthis, arg0)};
    let rsthis = QTimeZone{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn FreeQTimeZone<T: QTimeZone_FreeQTimeZone>(&mut self, value: T)  {
     value.FreeQTimeZone(self);
    // return 1;
  }
}

pub trait QTimeZone_FreeQTimeZone {
  fn FreeQTimeZone(self, rsthis: &mut QTimeZone) ;
}

// proto:  void QTimeZone::FreeQTimeZone();
impl<'a> /*trait*/ QTimeZone_FreeQTimeZone for () {
  fn FreeQTimeZone(self, rsthis: &mut QTimeZone)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneD0Ev()};
     unsafe {_ZN9QTimeZoneD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn standardTimeOffset<T: QTimeZone_standardTimeOffset>(&mut self, value: T) -> i32 {
    return value.standardTimeOffset(self);
    // return 1;
  }
}

pub trait QTimeZone_standardTimeOffset {
  fn standardTimeOffset(self, rsthis: &mut QTimeZone) -> i32;
}

// proto:  int QTimeZone::standardTimeOffset(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_standardTimeOffset for (&'a  QDateTime) {
  fn standardTimeOffset(self, rsthis: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn id<T: QTimeZone_id>(&mut self, value: T) -> QByteArray {
    return value.id(self);
    // return 1;
  }
}

pub trait QTimeZone_id {
  fn id(self, rsthis: &mut QTimeZone) -> QByteArray;
}

// proto:  QByteArray QTimeZone::id();
impl<'a> /*trait*/ QTimeZone_id for () {
  fn id(self, rsthis: &mut QTimeZone) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone2idEv()};
    let mut ret = unsafe {_ZNK9QTimeZone2idEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn offsetFromUtc<T: QTimeZone_offsetFromUtc>(&mut self, value: T) -> i32 {
    return value.offsetFromUtc(self);
    // return 1;
  }
}

pub trait QTimeZone_offsetFromUtc {
  fn offsetFromUtc(self, rsthis: &mut QTimeZone) -> i32;
}

// proto:  int QTimeZone::offsetFromUtc(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_offsetFromUtc for (&'a  QDateTime) {
  fn offsetFromUtc(self, rsthis: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone13offsetFromUtcERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTimeZone13offsetFromUtcERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn windowsIdToIanaIds<T: QTimeZone_windowsIdToIanaIds>(&mut self, value: T)  {
     value.windowsIdToIanaIds(self);
    // return 1;
  }
}

pub trait QTimeZone_windowsIdToIanaIds {
  fn windowsIdToIanaIds(self, rsthis: &mut QTimeZone) ;
}

// proto: static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray & windowsId);
impl<'a> /*trait*/ QTimeZone_windowsIdToIanaIds for (&'a  QByteArray) {
  fn windowsIdToIanaIds(self, rsthis: &mut QTimeZone)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0)};
    // return 1;
  }
}

