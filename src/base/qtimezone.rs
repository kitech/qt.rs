// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdatetime::QDateTime;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN9QTimeZone20availableTimeZoneIdsEv() -> i32;
  fn _ZN9QTimeZone4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK9QTimeZone7isValidEv() -> i32;
  fn _ZNK9QTimeZone15hasDaylightTimeEv() -> i32;
  fn _ZN9QTimeZone3utcEv() -> i32;
  fn _ZN9QTimeZone20availableTimeZoneIdsEi(arg0: c_int) -> i32;
  fn _ZN9QTimeZoneC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  fn _ZNK9QTimeZone12abbreviationERK9QDateTime(arg0: *const c_void) -> i32;
  fn _ZN9QTimeZoneC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN9QTimeZone16systemTimeZoneIdEv() -> i32;
  fn _ZNK9QTimeZone14isDaylightTimeERK9QDateTime(arg0: *const c_void) -> i32;
  fn _ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK9QTimeZone7commentEv() -> i32;
  fn _ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK9QTimeZone14hasTransitionsEv() -> i32;
  fn _ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime(arg0: *const c_void) -> i32;
  fn _ZN9QTimeZone14systemTimeZoneEv() -> i32;
  fn _ZN9QTimeZoneC1ERK10QByteArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QTimeZoneC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QTimeZoneD0Ev() -> i32;
  fn _ZNK9QTimeZone18standardTimeOffsetERK9QDateTime(arg0: *const c_void) -> i32;
  fn _ZNK9QTimeZone2idEv() -> i32;
  fn _ZNK9QTimeZone13offsetFromUtcERK9QDateTime(arg0: *const c_void) -> i32;
  fn _ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTimeZone)=1
pub struct QTimeZone {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimeZone {
  pub fn availableTimeZoneIds<T: QTimeZone_availableTimeZoneIds>(&mut self, value: T) -> i32 {
    value.availableTimeZoneIds(self);
    return 1;
  }
}

pub trait QTimeZone_availableTimeZoneIds {
  fn availableTimeZoneIds(self, this: &mut QTimeZone) -> i32;
}

// proto: QList<QByteArray> QTimeZone::availableTimeZoneIds();
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds for () {
  fn availableTimeZoneIds(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEv()};
    unsafe {_ZN9QTimeZone20availableTimeZoneIdsEv()};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn swap<T: QTimeZone_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QTimeZone_swap {
  fn swap(self, this: &mut QTimeZone) -> i32;
}

// proto: void QTimeZone::swap(QTimeZone & other);
impl<'a> /*trait*/ QTimeZone_swap for (&'a mut QTimeZone) {
  fn swap(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeZone4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn isValid<T: QTimeZone_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTimeZone_isValid {
  fn isValid(self, this: &mut QTimeZone) -> i32;
}

// proto: bool QTimeZone::isValid();
impl<'a> /*trait*/ QTimeZone_isValid for () {
  fn isValid(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone7isValidEv()};
    unsafe {_ZNK9QTimeZone7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn hasDaylightTime<T: QTimeZone_hasDaylightTime>(&mut self, value: T) -> i32 {
    value.hasDaylightTime(self);
    return 1;
  }
}

pub trait QTimeZone_hasDaylightTime {
  fn hasDaylightTime(self, this: &mut QTimeZone) -> i32;
}

// proto: bool QTimeZone::hasDaylightTime();
impl<'a> /*trait*/ QTimeZone_hasDaylightTime for () {
  fn hasDaylightTime(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone15hasDaylightTimeEv()};
    unsafe {_ZNK9QTimeZone15hasDaylightTimeEv()};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn utc<T: QTimeZone_utc>(&mut self, value: T) -> i32 {
    value.utc(self);
    return 1;
  }
}

pub trait QTimeZone_utc {
  fn utc(self, this: &mut QTimeZone) -> i32;
}

// proto: QTimeZone QTimeZone::utc();
impl<'a> /*trait*/ QTimeZone_utc for () {
  fn utc(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone3utcEv()};
    unsafe {_ZN9QTimeZone3utcEv()};
    return 1;
  }
}

// proto: QList<QByteArray> QTimeZone::availableTimeZoneIds(int offsetSeconds);
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds for (i32) {
  fn availableTimeZoneIds(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTimeZone20availableTimeZoneIdsEi(arg0)};
    return 1;
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
  pub fn abbreviation<T: QTimeZone_abbreviation>(&mut self, value: T) -> i32 {
    value.abbreviation(self);
    return 1;
  }
}

pub trait QTimeZone_abbreviation {
  fn abbreviation(self, this: &mut QTimeZone) -> i32;
}

// proto: QString QTimeZone::abbreviation(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_abbreviation for (&'a  QDateTime) {
  fn abbreviation(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone12abbreviationERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTimeZone12abbreviationERK9QDateTime(arg0)};
    return 1;
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
  pub fn ianaIdToWindowsId<T: QTimeZone_ianaIdToWindowsId>(&mut self, value: T) -> i32 {
    value.ianaIdToWindowsId(self);
    return 1;
  }
}

pub trait QTimeZone_ianaIdToWindowsId {
  fn ianaIdToWindowsId(self, this: &mut QTimeZone) -> i32;
}

// proto: QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_ianaIdToWindowsId for (&'a  QByteArray) {
  fn ianaIdToWindowsId(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn systemTimeZoneId<T: QTimeZone_systemTimeZoneId>(&mut self, value: T) -> i32 {
    value.systemTimeZoneId(self);
    return 1;
  }
}

pub trait QTimeZone_systemTimeZoneId {
  fn systemTimeZoneId(self, this: &mut QTimeZone) -> i32;
}

// proto: QByteArray QTimeZone::systemTimeZoneId();
impl<'a> /*trait*/ QTimeZone_systemTimeZoneId for () {
  fn systemTimeZoneId(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone16systemTimeZoneIdEv()};
    unsafe {_ZN9QTimeZone16systemTimeZoneIdEv()};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn isDaylightTime<T: QTimeZone_isDaylightTime>(&mut self, value: T) -> i32 {
    value.isDaylightTime(self);
    return 1;
  }
}

pub trait QTimeZone_isDaylightTime {
  fn isDaylightTime(self, this: &mut QTimeZone) -> i32;
}

// proto: bool QTimeZone::isDaylightTime(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_isDaylightTime for (&'a  QDateTime) {
  fn isDaylightTime(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone14isDaylightTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTimeZone14isDaylightTimeERK9QDateTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn isTimeZoneIdAvailable<T: QTimeZone_isTimeZoneIdAvailable>(&mut self, value: T) -> i32 {
    value.isTimeZoneIdAvailable(self);
    return 1;
  }
}

pub trait QTimeZone_isTimeZoneIdAvailable {
  fn isTimeZoneIdAvailable(self, this: &mut QTimeZone) -> i32;
}

// proto: bool QTimeZone::isTimeZoneIdAvailable(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_isTimeZoneIdAvailable for (&'a  QByteArray) {
  fn isTimeZoneIdAvailable(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn comment<T: QTimeZone_comment>(&mut self, value: T) -> i32 {
    value.comment(self);
    return 1;
  }
}

pub trait QTimeZone_comment {
  fn comment(self, this: &mut QTimeZone) -> i32;
}

// proto: QString QTimeZone::comment();
impl<'a> /*trait*/ QTimeZone_comment for () {
  fn comment(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone7commentEv()};
    unsafe {_ZNK9QTimeZone7commentEv()};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn windowsIdToDefaultIanaId<T: QTimeZone_windowsIdToDefaultIanaId>(&mut self, value: T) -> i32 {
    value.windowsIdToDefaultIanaId(self);
    return 1;
  }
}

pub trait QTimeZone_windowsIdToDefaultIanaId {
  fn windowsIdToDefaultIanaId(self, this: &mut QTimeZone) -> i32;
}

// proto: QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray & windowsId);
impl<'a> /*trait*/ QTimeZone_windowsIdToDefaultIanaId for (&'a  QByteArray) {
  fn windowsIdToDefaultIanaId(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn hasTransitions<T: QTimeZone_hasTransitions>(&mut self, value: T) -> i32 {
    value.hasTransitions(self);
    return 1;
  }
}

pub trait QTimeZone_hasTransitions {
  fn hasTransitions(self, this: &mut QTimeZone) -> i32;
}

// proto: bool QTimeZone::hasTransitions();
impl<'a> /*trait*/ QTimeZone_hasTransitions for () {
  fn hasTransitions(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone14hasTransitionsEv()};
    unsafe {_ZNK9QTimeZone14hasTransitionsEv()};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn daylightTimeOffset<T: QTimeZone_daylightTimeOffset>(&mut self, value: T) -> i32 {
    value.daylightTimeOffset(self);
    return 1;
  }
}

pub trait QTimeZone_daylightTimeOffset {
  fn daylightTimeOffset(self, this: &mut QTimeZone) -> i32;
}

// proto: int QTimeZone::daylightTimeOffset(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_daylightTimeOffset for (&'a  QDateTime) {
  fn daylightTimeOffset(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn systemTimeZone<T: QTimeZone_systemTimeZone>(&mut self, value: T) -> i32 {
    value.systemTimeZone(self);
    return 1;
  }
}

pub trait QTimeZone_systemTimeZone {
  fn systemTimeZone(self, this: &mut QTimeZone) -> i32;
}

// proto: QTimeZone QTimeZone::systemTimeZone();
impl<'a> /*trait*/ QTimeZone_systemTimeZone for () {
  fn systemTimeZone(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone14systemTimeZoneEv()};
    unsafe {_ZN9QTimeZone14systemTimeZoneEv()};
    return 1;
  }
}

// proto: void QTimeZone::NewQTimeZone(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_NewQTimeZone for (&'a  QByteArray) {
  fn NewQTimeZone(self) -> QTimeZone {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTimeZoneC1ERKS_(qthis, arg0)};
    let rsthis = QTimeZone{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn FreeQTimeZone<T: QTimeZone_FreeQTimeZone>(&mut self, value: T) -> i32 {
    value.FreeQTimeZone(self);
    return 1;
  }
}

pub trait QTimeZone_FreeQTimeZone {
  fn FreeQTimeZone(self, this: &mut QTimeZone) -> i32;
}

// proto: void QTimeZone::FreeQTimeZone();
impl<'a> /*trait*/ QTimeZone_FreeQTimeZone for () {
  fn FreeQTimeZone(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneD0Ev()};
    unsafe {_ZN9QTimeZoneD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn standardTimeOffset<T: QTimeZone_standardTimeOffset>(&mut self, value: T) -> i32 {
    value.standardTimeOffset(self);
    return 1;
  }
}

pub trait QTimeZone_standardTimeOffset {
  fn standardTimeOffset(self, this: &mut QTimeZone) -> i32;
}

// proto: int QTimeZone::standardTimeOffset(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_standardTimeOffset for (&'a  QDateTime) {
  fn standardTimeOffset(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn id<T: QTimeZone_id>(&mut self, value: T) -> i32 {
    value.id(self);
    return 1;
  }
}

pub trait QTimeZone_id {
  fn id(self, this: &mut QTimeZone) -> i32;
}

// proto: QByteArray QTimeZone::id();
impl<'a> /*trait*/ QTimeZone_id for () {
  fn id(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone2idEv()};
    unsafe {_ZNK9QTimeZone2idEv()};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn offsetFromUtc<T: QTimeZone_offsetFromUtc>(&mut self, value: T) -> i32 {
    value.offsetFromUtc(self);
    return 1;
  }
}

pub trait QTimeZone_offsetFromUtc {
  fn offsetFromUtc(self, this: &mut QTimeZone) -> i32;
}

// proto: int QTimeZone::offsetFromUtc(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_offsetFromUtc for (&'a  QDateTime) {
  fn offsetFromUtc(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone13offsetFromUtcERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTimeZone13offsetFromUtcERK9QDateTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeZone {
  pub fn windowsIdToIanaIds<T: QTimeZone_windowsIdToIanaIds>(&mut self, value: T) -> i32 {
    value.windowsIdToIanaIds(self);
    return 1;
  }
}

pub trait QTimeZone_windowsIdToIanaIds {
  fn windowsIdToIanaIds(self, this: &mut QTimeZone) -> i32;
}

// proto: QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray & windowsId);
impl<'a> /*trait*/ QTimeZone_windowsIdToIanaIds for (&'a  QByteArray) {
  fn windowsIdToIanaIds(self, this: &mut QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0)};
    return 1;
  }
}

