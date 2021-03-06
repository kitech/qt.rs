// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qtimezone.h
// dst-file: /src/core/qtimezone.rs
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
// use super::qlist::*; // 775
use super::qdatetime::*; // 773
use super::qbytearray::*; // 773
use super::qstring::*; // 773
use super::qlocale::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTimeZone_Class_Size() -> c_int;
  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds();
  fn C_ZN9QTimeZone20availableTimeZoneIdsEv() -> *mut c_void;
  // proto:  void QTimeZone::swap(QTimeZone & other);
  fn C_ZN9QTimeZone4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTimeZone::isValid();
  fn C_ZNK9QTimeZone7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTimeZone::hasDaylightTime();
  fn C_ZNK9QTimeZone15hasDaylightTimeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QTimeZone QTimeZone::utc();
  fn C_ZN9QTimeZone3utcEv() -> *mut c_void;
  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds(int offsetSeconds);
  fn C_ZN9QTimeZone20availableTimeZoneIdsEi(arg0: c_int) -> *mut c_void;
  // proto:  void QTimeZone::QTimeZone(int offsetSeconds);
  fn C_ZN9QTimeZoneC2Ei(arg0: c_int) -> u64;
  // proto:  QString QTimeZone::abbreviation(const QDateTime & atDateTime);
  fn C_ZNK9QTimeZone12abbreviationERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTimeZone::QTimeZone();
  fn C_ZN9QTimeZoneC2Ev() -> u64;
  // proto: static QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray & ianaId);
  fn C_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QTimeZone::systemTimeZoneId();
  fn C_ZN9QTimeZone16systemTimeZoneIdEv() -> *mut c_void;
  // proto:  bool QTimeZone::isDaylightTime(const QDateTime & atDateTime);
  fn C_ZNK9QTimeZone14isDaylightTimeERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto: static bool QTimeZone::isTimeZoneIdAvailable(const QByteArray & ianaId);
  fn C_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray(arg0: *mut c_void) -> c_char;
  // proto:  QString QTimeZone::comment();
  fn C_ZNK9QTimeZone7commentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray & windowsId);
  fn C_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTimeZone::hasTransitions();
  fn C_ZNK9QTimeZone14hasTransitionsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTimeZone::daylightTimeOffset(const QDateTime & atDateTime);
  fn C_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto: static QTimeZone QTimeZone::systemTimeZone();
  fn C_ZN9QTimeZone14systemTimeZoneEv() -> *mut c_void;
  // proto:  void QTimeZone::QTimeZone(const QByteArray & ianaId);
  fn C_ZN9QTimeZoneC2ERK10QByteArray(arg0: *mut c_void) -> u64;
  // proto:  void QTimeZone::QTimeZone(const QTimeZone & other);
  fn C_ZN9QTimeZoneC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QTimeZone::~QTimeZone();
  fn C_ZN9QTimeZoneD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTimeZone::standardTimeOffset(const QDateTime & atDateTime);
  fn C_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QByteArray QTimeZone::id();
  fn C_ZNK9QTimeZone2idEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTimeZone::offsetFromUtc(const QDateTime & atDateTime);
  fn C_ZNK9QTimeZone13offsetFromUtcERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto: static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray & windowsId);
  fn C_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QTimeZone)=1
#[derive(Default)]
pub struct QTimeZone {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTimeZone {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTimeZone {
    return QTimeZone{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds();
impl /*struct*/ QTimeZone {
  pub fn availableTimeZoneIds_s<RetType, T: QTimeZone_availableTimeZoneIds_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.availableTimeZoneIds_s();
    // return 1;
  }
}

pub trait QTimeZone_availableTimeZoneIds_s<RetType> {
  fn availableTimeZoneIds_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds();
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds_s<u64> for () {
  fn availableTimeZoneIds_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEv()};
    let mut ret = unsafe {C_ZN9QTimeZone20availableTimeZoneIdsEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QTimeZone::swap(QTimeZone & other);
impl /*struct*/ QTimeZone {
  pub fn swap<RetType, T: QTimeZone_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QTimeZone_swap<RetType> {
  fn swap(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  void QTimeZone::swap(QTimeZone & other);
impl<'a> /*trait*/ QTimeZone_swap<()> for (&'a QTimeZone) {
  fn swap(self , rsthis: & QTimeZone) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QTimeZone4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTimeZone::isValid();
impl /*struct*/ QTimeZone {
  pub fn isValid<RetType, T: QTimeZone_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTimeZone_isValid<RetType> {
  fn isValid(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  bool QTimeZone::isValid();
impl<'a> /*trait*/ QTimeZone_isValid<i8> for () {
  fn isValid(self , rsthis: & QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone7isValidEv()};
    let mut ret = unsafe {C_ZNK9QTimeZone7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QTimeZone::hasDaylightTime();
impl /*struct*/ QTimeZone {
  pub fn hasDaylightTime<RetType, T: QTimeZone_hasDaylightTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasDaylightTime(self);
    // return 1;
  }
}

pub trait QTimeZone_hasDaylightTime<RetType> {
  fn hasDaylightTime(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  bool QTimeZone::hasDaylightTime();
impl<'a> /*trait*/ QTimeZone_hasDaylightTime<i8> for () {
  fn hasDaylightTime(self , rsthis: & QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone15hasDaylightTimeEv()};
    let mut ret = unsafe {C_ZNK9QTimeZone15hasDaylightTimeEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static QTimeZone QTimeZone::utc();
impl /*struct*/ QTimeZone {
  pub fn utc_s<RetType, T: QTimeZone_utc_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.utc_s();
    // return 1;
  }
}

pub trait QTimeZone_utc_s<RetType> {
  fn utc_s(self ) -> RetType;
}

  // proto: static QTimeZone QTimeZone::utc();
impl<'a> /*trait*/ QTimeZone_utc_s<QTimeZone> for () {
  fn utc_s(self ) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone3utcEv()};
    let mut ret = unsafe {C_ZN9QTimeZone3utcEv()};
    let mut ret1 = QTimeZone::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds(int offsetSeconds);
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds_s<u64> for (i32) {
  fn availableTimeZoneIds_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QTimeZone20availableTimeZoneIdsEi(arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QTimeZone::QTimeZone(int offsetSeconds);
impl /*struct*/ QTimeZone {
  pub fn new<T: QTimeZone_new>(value: T) -> QTimeZone {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeZone_new {
  fn new(self) -> QTimeZone;
}

  // proto:  void QTimeZone::QTimeZone(int offsetSeconds);
impl<'a> /*trait*/ QTimeZone_new for (i32) {
  fn new(self) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC2Ei()};
    let ctysz: c_int = unsafe{QTimeZone_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN9QTimeZoneC2Ei(arg0)};
    let rsthis = QTimeZone{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QTimeZone::abbreviation(const QDateTime & atDateTime);
impl /*struct*/ QTimeZone {
  pub fn abbreviation<RetType, T: QTimeZone_abbreviation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.abbreviation(self);
    // return 1;
  }
}

pub trait QTimeZone_abbreviation<RetType> {
  fn abbreviation(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  QString QTimeZone::abbreviation(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_abbreviation<QString> for (&'a QDateTime) {
  fn abbreviation(self , rsthis: & QTimeZone) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone12abbreviationERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QTimeZone12abbreviationERK9QDateTime(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTimeZone::QTimeZone();
impl<'a> /*trait*/ QTimeZone_new for () {
  fn new(self) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC2Ev()};
    let ctysz: c_int = unsafe{QTimeZone_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN9QTimeZoneC2Ev()};
    let rsthis = QTimeZone{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray & ianaId);
impl /*struct*/ QTimeZone {
  pub fn ianaIdToWindowsId_s<RetType, T: QTimeZone_ianaIdToWindowsId_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.ianaIdToWindowsId_s();
    // return 1;
  }
}

pub trait QTimeZone_ianaIdToWindowsId_s<RetType> {
  fn ianaIdToWindowsId_s(self ) -> RetType;
}

  // proto: static QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_ianaIdToWindowsId_s<QByteArray> for (&'a QByteArray) {
  fn ianaIdToWindowsId_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QByteArray QTimeZone::systemTimeZoneId();
impl /*struct*/ QTimeZone {
  pub fn systemTimeZoneId_s<RetType, T: QTimeZone_systemTimeZoneId_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemTimeZoneId_s();
    // return 1;
  }
}

pub trait QTimeZone_systemTimeZoneId_s<RetType> {
  fn systemTimeZoneId_s(self ) -> RetType;
}

  // proto: static QByteArray QTimeZone::systemTimeZoneId();
impl<'a> /*trait*/ QTimeZone_systemTimeZoneId_s<QByteArray> for () {
  fn systemTimeZoneId_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone16systemTimeZoneIdEv()};
    let mut ret = unsafe {C_ZN9QTimeZone16systemTimeZoneIdEv()};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTimeZone::isDaylightTime(const QDateTime & atDateTime);
impl /*struct*/ QTimeZone {
  pub fn isDaylightTime<RetType, T: QTimeZone_isDaylightTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDaylightTime(self);
    // return 1;
  }
}

pub trait QTimeZone_isDaylightTime<RetType> {
  fn isDaylightTime(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  bool QTimeZone::isDaylightTime(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_isDaylightTime<i8> for (&'a QDateTime) {
  fn isDaylightTime(self , rsthis: & QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone14isDaylightTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QTimeZone14isDaylightTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static bool QTimeZone::isTimeZoneIdAvailable(const QByteArray & ianaId);
impl /*struct*/ QTimeZone {
  pub fn isTimeZoneIdAvailable_s<RetType, T: QTimeZone_isTimeZoneIdAvailable_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isTimeZoneIdAvailable_s();
    // return 1;
  }
}

pub trait QTimeZone_isTimeZoneIdAvailable_s<RetType> {
  fn isTimeZoneIdAvailable_s(self ) -> RetType;
}

  // proto: static bool QTimeZone::isTimeZoneIdAvailable(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_isTimeZoneIdAvailable_s<i8> for (&'a QByteArray) {
  fn isTimeZoneIdAvailable_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray(arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QTimeZone::comment();
impl /*struct*/ QTimeZone {
  pub fn comment<RetType, T: QTimeZone_comment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.comment(self);
    // return 1;
  }
}

pub trait QTimeZone_comment<RetType> {
  fn comment(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  QString QTimeZone::comment();
impl<'a> /*trait*/ QTimeZone_comment<QString> for () {
  fn comment(self , rsthis: & QTimeZone) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone7commentEv()};
    let mut ret = unsafe {C_ZNK9QTimeZone7commentEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray & windowsId);
impl /*struct*/ QTimeZone {
  pub fn windowsIdToDefaultIanaId_s<RetType, T: QTimeZone_windowsIdToDefaultIanaId_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowsIdToDefaultIanaId_s();
    // return 1;
  }
}

pub trait QTimeZone_windowsIdToDefaultIanaId_s<RetType> {
  fn windowsIdToDefaultIanaId_s(self ) -> RetType;
}

  // proto: static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray & windowsId);
impl<'a> /*trait*/ QTimeZone_windowsIdToDefaultIanaId_s<QByteArray> for (&'a QByteArray) {
  fn windowsIdToDefaultIanaId_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTimeZone::hasTransitions();
impl /*struct*/ QTimeZone {
  pub fn hasTransitions<RetType, T: QTimeZone_hasTransitions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasTransitions(self);
    // return 1;
  }
}

pub trait QTimeZone_hasTransitions<RetType> {
  fn hasTransitions(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  bool QTimeZone::hasTransitions();
impl<'a> /*trait*/ QTimeZone_hasTransitions<i8> for () {
  fn hasTransitions(self , rsthis: & QTimeZone) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone14hasTransitionsEv()};
    let mut ret = unsafe {C_ZNK9QTimeZone14hasTransitionsEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QTimeZone::daylightTimeOffset(const QDateTime & atDateTime);
impl /*struct*/ QTimeZone {
  pub fn daylightTimeOffset<RetType, T: QTimeZone_daylightTimeOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.daylightTimeOffset(self);
    // return 1;
  }
}

pub trait QTimeZone_daylightTimeOffset<RetType> {
  fn daylightTimeOffset(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  int QTimeZone::daylightTimeOffset(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_daylightTimeOffset<i32> for (&'a QDateTime) {
  fn daylightTimeOffset(self , rsthis: & QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QTimeZone QTimeZone::systemTimeZone();
impl /*struct*/ QTimeZone {
  pub fn systemTimeZone_s<RetType, T: QTimeZone_systemTimeZone_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemTimeZone_s();
    // return 1;
  }
}

pub trait QTimeZone_systemTimeZone_s<RetType> {
  fn systemTimeZone_s(self ) -> RetType;
}

  // proto: static QTimeZone QTimeZone::systemTimeZone();
impl<'a> /*trait*/ QTimeZone_systemTimeZone_s<QTimeZone> for () {
  fn systemTimeZone_s(self ) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone14systemTimeZoneEv()};
    let mut ret = unsafe {C_ZN9QTimeZone14systemTimeZoneEv()};
    let mut ret1 = QTimeZone::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTimeZone::QTimeZone(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_new for (&'a QByteArray) {
  fn new(self) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC2ERK10QByteArray()};
    let ctysz: c_int = unsafe{QTimeZone_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QTimeZoneC2ERK10QByteArray(arg0)};
    let rsthis = QTimeZone{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimeZone::QTimeZone(const QTimeZone & other);
impl<'a> /*trait*/ QTimeZone_new for (&'a QTimeZone) {
  fn new(self) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC2ERKS_()};
    let ctysz: c_int = unsafe{QTimeZone_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QTimeZoneC2ERKS_(arg0)};
    let rsthis = QTimeZone{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimeZone::~QTimeZone();
impl /*struct*/ QTimeZone {
  pub fn free<RetType, T: QTimeZone_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTimeZone_free<RetType> {
  fn free(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  void QTimeZone::~QTimeZone();
impl<'a> /*trait*/ QTimeZone_free<()> for () {
  fn free(self , rsthis: & QTimeZone) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneD2Ev()};
     unsafe {C_ZN9QTimeZoneD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTimeZone::standardTimeOffset(const QDateTime & atDateTime);
impl /*struct*/ QTimeZone {
  pub fn standardTimeOffset<RetType, T: QTimeZone_standardTimeOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.standardTimeOffset(self);
    // return 1;
  }
}

pub trait QTimeZone_standardTimeOffset<RetType> {
  fn standardTimeOffset(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  int QTimeZone::standardTimeOffset(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_standardTimeOffset<i32> for (&'a QDateTime) {
  fn standardTimeOffset(self , rsthis: & QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QByteArray QTimeZone::id();
impl /*struct*/ QTimeZone {
  pub fn id<RetType, T: QTimeZone_id<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.id(self);
    // return 1;
  }
}

pub trait QTimeZone_id<RetType> {
  fn id(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  QByteArray QTimeZone::id();
impl<'a> /*trait*/ QTimeZone_id<QByteArray> for () {
  fn id(self , rsthis: & QTimeZone) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone2idEv()};
    let mut ret = unsafe {C_ZNK9QTimeZone2idEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTimeZone::offsetFromUtc(const QDateTime & atDateTime);
impl /*struct*/ QTimeZone {
  pub fn offsetFromUtc<RetType, T: QTimeZone_offsetFromUtc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.offsetFromUtc(self);
    // return 1;
  }
}

pub trait QTimeZone_offsetFromUtc<RetType> {
  fn offsetFromUtc(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  int QTimeZone::offsetFromUtc(const QDateTime & atDateTime);
impl<'a> /*trait*/ QTimeZone_offsetFromUtc<i32> for (&'a QDateTime) {
  fn offsetFromUtc(self , rsthis: & QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeZone13offsetFromUtcERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QTimeZone13offsetFromUtcERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray & windowsId);
impl /*struct*/ QTimeZone {
  pub fn windowsIdToIanaIds_s<RetType, T: QTimeZone_windowsIdToIanaIds_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowsIdToIanaIds_s();
    // return 1;
  }
}

pub trait QTimeZone_windowsIdToIanaIds_s<RetType> {
  fn windowsIdToIanaIds_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray & windowsId);
impl<'a> /*trait*/ QTimeZone_windowsIdToIanaIds_s<u64> for (&'a QByteArray) {
  fn windowsIdToIanaIds_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

// <= body block end

