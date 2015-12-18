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
  pub fn availableTimeZoneIds<RetType, T: QTimeZone_availableTimeZoneIds<RetType>>(&mut self, value: T) -> RetType {
    return value.availableTimeZoneIds(self);
    // return 1;
  }
}

pub trait QTimeZone_availableTimeZoneIds<RetType> {
  fn availableTimeZoneIds(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds();
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds<()> for () {
  fn availableTimeZoneIds(self, rsthis: &mut QTimeZone) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEv()};
     unsafe {_ZN9QTimeZone20availableTimeZoneIdsEv()};
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn swap<RetType, T: QTimeZone_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QTimeZone_swap<RetType> {
  fn swap(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  void QTimeZone::swap(QTimeZone & other);
impl<'a> /*trait*/ QTimeZone_swap<()> for (&'a mut QTimeZone) {
  fn swap(self, rsthis: &mut QTimeZone) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTimeZone4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn isValid<RetType, T: QTimeZone_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTimeZone_isValid<RetType> {
  fn isValid(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  bool QTimeZone::isValid();
impl<'a> /*trait*/ QTimeZone_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone7isValidEv()};
    let mut ret = unsafe {_ZNK9QTimeZone7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn hasDaylightTime<RetType, T: QTimeZone_hasDaylightTime<RetType>>(&mut self, value: T) -> RetType {
    return value.hasDaylightTime(self);
    // return 1;
  }
}

pub trait QTimeZone_hasDaylightTime<RetType> {
  fn hasDaylightTime(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  bool QTimeZone::hasDaylightTime();
impl<'a> /*trait*/ QTimeZone_hasDaylightTime<i8> for () {
  fn hasDaylightTime(self, rsthis: &mut QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone15hasDaylightTimeEv()};
    let mut ret = unsafe {_ZNK9QTimeZone15hasDaylightTimeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn utc<RetType, T: QTimeZone_utc<RetType>>(&mut self, value: T) -> RetType {
    return value.utc(self);
    // return 1;
  }
}

pub trait QTimeZone_utc<RetType> {
  fn utc(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto: static QTimeZone QTimeZone::utc();
impl<'a> /*trait*/ QTimeZone_utc<QTimeZone> for () {
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
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds<()> for (i32) {
  fn availableTimeZoneIds(self, rsthis: &mut QTimeZone) -> () {
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
  pub fn abbreviation<RetType, T: QTimeZone_abbreviation<RetType>>(&mut self, value: T) -> RetType {
    return value.abbreviation(self);
    // return 1;
  }
}

pub trait QTimeZone_abbreviation<RetType> {
  fn abbreviation(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  QString QTimeZone::abbreviation(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_abbreviation<QString> for (&'a  QDateTime) {
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
  pub fn ianaIdToWindowsId<RetType, T: QTimeZone_ianaIdToWindowsId<RetType>>(&mut self, value: T) -> RetType {
    return value.ianaIdToWindowsId(self);
    // return 1;
  }
}

pub trait QTimeZone_ianaIdToWindowsId<RetType> {
  fn ianaIdToWindowsId(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto: static QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_ianaIdToWindowsId<QByteArray> for (&'a  QByteArray) {
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
  pub fn systemTimeZoneId<RetType, T: QTimeZone_systemTimeZoneId<RetType>>(&mut self, value: T) -> RetType {
    return value.systemTimeZoneId(self);
    // return 1;
  }
}

pub trait QTimeZone_systemTimeZoneId<RetType> {
  fn systemTimeZoneId(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto: static QByteArray QTimeZone::systemTimeZoneId();
impl<'a> /*trait*/ QTimeZone_systemTimeZoneId<QByteArray> for () {
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
  pub fn isDaylightTime<RetType, T: QTimeZone_isDaylightTime<RetType>>(&mut self, value: T) -> RetType {
    return value.isDaylightTime(self);
    // return 1;
  }
}

pub trait QTimeZone_isDaylightTime<RetType> {
  fn isDaylightTime(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  bool QTimeZone::isDaylightTime(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_isDaylightTime<i8> for (&'a  QDateTime) {
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
  pub fn isTimeZoneIdAvailable<RetType, T: QTimeZone_isTimeZoneIdAvailable<RetType>>(&mut self, value: T) -> RetType {
    return value.isTimeZoneIdAvailable(self);
    // return 1;
  }
}

pub trait QTimeZone_isTimeZoneIdAvailable<RetType> {
  fn isTimeZoneIdAvailable(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto: static bool QTimeZone::isTimeZoneIdAvailable(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_isTimeZoneIdAvailable<i8> for (&'a  QByteArray) {
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
  pub fn comment<RetType, T: QTimeZone_comment<RetType>>(&mut self, value: T) -> RetType {
    return value.comment(self);
    // return 1;
  }
}

pub trait QTimeZone_comment<RetType> {
  fn comment(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  QString QTimeZone::comment();
impl<'a> /*trait*/ QTimeZone_comment<QString> for () {
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
  pub fn windowsIdToDefaultIanaId<RetType, T: QTimeZone_windowsIdToDefaultIanaId<RetType>>(&mut self, value: T) -> RetType {
    return value.windowsIdToDefaultIanaId(self);
    // return 1;
  }
}

pub trait QTimeZone_windowsIdToDefaultIanaId<RetType> {
  fn windowsIdToDefaultIanaId(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto: static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray & windowsId);
impl<'a> /*trait*/ QTimeZone_windowsIdToDefaultIanaId<QByteArray> for (&'a  QByteArray) {
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
  pub fn hasTransitions<RetType, T: QTimeZone_hasTransitions<RetType>>(&mut self, value: T) -> RetType {
    return value.hasTransitions(self);
    // return 1;
  }
}

pub trait QTimeZone_hasTransitions<RetType> {
  fn hasTransitions(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  bool QTimeZone::hasTransitions();
impl<'a> /*trait*/ QTimeZone_hasTransitions<i8> for () {
  fn hasTransitions(self, rsthis: &mut QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone14hasTransitionsEv()};
    let mut ret = unsafe {_ZNK9QTimeZone14hasTransitionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn daylightTimeOffset<RetType, T: QTimeZone_daylightTimeOffset<RetType>>(&mut self, value: T) -> RetType {
    return value.daylightTimeOffset(self);
    // return 1;
  }
}

pub trait QTimeZone_daylightTimeOffset<RetType> {
  fn daylightTimeOffset(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  int QTimeZone::daylightTimeOffset(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_daylightTimeOffset<i32> for (&'a  QDateTime) {
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
  pub fn systemTimeZone<RetType, T: QTimeZone_systemTimeZone<RetType>>(&mut self, value: T) -> RetType {
    return value.systemTimeZone(self);
    // return 1;
  }
}

pub trait QTimeZone_systemTimeZone<RetType> {
  fn systemTimeZone(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto: static QTimeZone QTimeZone::systemTimeZone();
impl<'a> /*trait*/ QTimeZone_systemTimeZone<QTimeZone> for () {
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
  pub fn FreeQTimeZone<RetType, T: QTimeZone_FreeQTimeZone<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTimeZone(self);
    // return 1;
  }
}

pub trait QTimeZone_FreeQTimeZone<RetType> {
  fn FreeQTimeZone(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  void QTimeZone::FreeQTimeZone();
impl<'a> /*trait*/ QTimeZone_FreeQTimeZone<()> for () {
  fn FreeQTimeZone(self, rsthis: &mut QTimeZone) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneD0Ev()};
     unsafe {_ZN9QTimeZoneD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn standardTimeOffset<RetType, T: QTimeZone_standardTimeOffset<RetType>>(&mut self, value: T) -> RetType {
    return value.standardTimeOffset(self);
    // return 1;
  }
}

pub trait QTimeZone_standardTimeOffset<RetType> {
  fn standardTimeOffset(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  int QTimeZone::standardTimeOffset(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_standardTimeOffset<i32> for (&'a  QDateTime) {
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
  pub fn id<RetType, T: QTimeZone_id<RetType>>(&mut self, value: T) -> RetType {
    return value.id(self);
    // return 1;
  }
}

pub trait QTimeZone_id<RetType> {
  fn id(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  QByteArray QTimeZone::id();
impl<'a> /*trait*/ QTimeZone_id<QByteArray> for () {
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
  pub fn offsetFromUtc<RetType, T: QTimeZone_offsetFromUtc<RetType>>(&mut self, value: T) -> RetType {
    return value.offsetFromUtc(self);
    // return 1;
  }
}

pub trait QTimeZone_offsetFromUtc<RetType> {
  fn offsetFromUtc(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto:  int QTimeZone::offsetFromUtc(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_offsetFromUtc<i32> for (&'a  QDateTime) {
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
  pub fn windowsIdToIanaIds<RetType, T: QTimeZone_windowsIdToIanaIds<RetType>>(&mut self, value: T) -> RetType {
    return value.windowsIdToIanaIds(self);
    // return 1;
  }
}

pub trait QTimeZone_windowsIdToIanaIds<RetType> {
  fn windowsIdToIanaIds(self, rsthis: &mut QTimeZone) -> RetType;
}

// proto: static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray & windowsId);
impl<'a> /*trait*/ QTimeZone_windowsIdToIanaIds<()> for (&'a  QByteArray) {
  fn windowsIdToIanaIds(self, rsthis: &mut QTimeZone) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0)};
    // return 1;
  }
}

