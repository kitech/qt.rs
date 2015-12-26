// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
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
use super::qdatetime::QDateTime; // 773
use super::qbytearray::QByteArray; // 773
use super::qstring::QString; // 773
use super::qlocale::QLocale; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTimeZone_Class_Size() -> c_int;
  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds();
  fn _ZN9QTimeZone20availableTimeZoneIdsEv();
  // proto:  void QTimeZone::swap(QTimeZone & other);
  fn _ZN9QTimeZone4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTimeZone::isValid();
  fn _ZNK9QTimeZone7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTimeZone::hasDaylightTime();
  fn _ZNK9QTimeZone15hasDaylightTimeEv(qthis: *mut c_void) -> c_char;
  // proto: static QTimeZone QTimeZone::utc();
  fn _ZN9QTimeZone3utcEv() -> *mut c_void;
  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds(int offsetSeconds);
  fn _ZN9QTimeZone20availableTimeZoneIdsEi(arg0: c_int);
  // proto:  void QTimeZone::QTimeZone(int offsetSeconds);
  fn dector_ZN9QTimeZoneC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN9QTimeZoneC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  QString QTimeZone::abbreviation(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone12abbreviationERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTimeZone::QTimeZone();
  fn dector_ZN9QTimeZoneC1Ev() -> *mut c_void;
  fn _ZN9QTimeZoneC1Ev(qthis: *mut c_void);
  // proto: static QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray & ianaId);
  fn _ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QTimeZone::systemTimeZoneId();
  fn _ZN9QTimeZone16systemTimeZoneIdEv() -> *mut c_void;
  // proto:  bool QTimeZone::isDaylightTime(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone14isDaylightTimeERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto: static bool QTimeZone::isTimeZoneIdAvailable(const QByteArray & ianaId);
  fn _ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray(arg0: *mut c_void) -> c_char;
  // proto:  QString QTimeZone::comment();
  fn _ZNK9QTimeZone7commentEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray & windowsId);
  fn _ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTimeZone::hasTransitions();
  fn _ZNK9QTimeZone14hasTransitionsEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTimeZone::daylightTimeOffset(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto: static QTimeZone QTimeZone::systemTimeZone();
  fn _ZN9QTimeZone14systemTimeZoneEv() -> *mut c_void;
  // proto:  void QTimeZone::QTimeZone(const QByteArray & ianaId);
  fn dector_ZN9QTimeZoneC1ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QTimeZoneC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTimeZone::QTimeZone(const QTimeZone & other);
  fn dector_ZN9QTimeZoneC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QTimeZoneC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTimeZone::~QTimeZone();
  fn _ZN9QTimeZoneD0Ev(qthis: *mut c_void);
  // proto:  int QTimeZone::standardTimeOffset(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone18standardTimeOffsetERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QByteArray QTimeZone::id();
  fn _ZNK9QTimeZone2idEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTimeZone::offsetFromUtc(const QDateTime & atDateTime);
  fn _ZNK9QTimeZone13offsetFromUtcERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto: static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray & windowsId);
  fn _ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTimeZone)=1
pub struct QTimeZone {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimeZone {
  pub fn inheritFrom(qthis: *mut c_void) -> QTimeZone {
    return QTimeZone{qclsinst: qthis};
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
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds_s<()> for () {
  fn availableTimeZoneIds_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEv()};
     unsafe {_ZN9QTimeZone20availableTimeZoneIdsEv()};
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
     unsafe {_ZN9QTimeZone4swapERS_(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK9QTimeZone7isValidEv(rsthis.qclsinst)};
    return ret as i8;
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
    let mut ret = unsafe {_ZNK9QTimeZone15hasDaylightTimeEv(rsthis.qclsinst)};
    return ret as i8;
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
    let mut ret = unsafe {_ZN9QTimeZone3utcEv()};
    let mut ret1 = QTimeZone::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QTimeZone::availableTimeZoneIds(int offsetSeconds);
impl<'a> /*trait*/ QTimeZone_availableTimeZoneIds_s<()> for (i32) {
  fn availableTimeZoneIds_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone20availableTimeZoneIdsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeZone20availableTimeZoneIdsEi(arg0)};
    // return 1;
  }
}

  // proto:  void QTimeZone::QTimeZone(int offsetSeconds);
impl /*struct*/ QTimeZone {
  pub fn New<T: QTimeZone_New>(value: T) -> QTimeZone {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeZone_New {
  fn New(self) -> QTimeZone;
}

  // proto:  void QTimeZone::QTimeZone(int offsetSeconds);
impl<'a> /*trait*/ QTimeZone_New for (i32) {
  fn New(self) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1Ei()};
    let ctysz: c_int = unsafe{QTimeZone_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN9QTimeZoneC1Ei(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QTimeZoneC1Ei(arg0)};
    let rsthis = QTimeZone{qclsinst: qthis};
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
    let mut ret = unsafe {_ZNK9QTimeZone12abbreviationERK9QDateTime(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTimeZone::QTimeZone();
impl<'a> /*trait*/ QTimeZone_New for () {
  fn New(self) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1Ev()};
    let ctysz: c_int = unsafe{QTimeZone_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN9QTimeZoneC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN9QTimeZoneC1Ev()};
    let rsthis = QTimeZone{qclsinst: qthis};
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
    let mut ret = unsafe {_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
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
    let mut ret = unsafe {_ZN9QTimeZone16systemTimeZoneIdEv()};
    let mut ret1 = QByteArray::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK9QTimeZone14isDaylightTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i8;
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
    let mut ret = unsafe {_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray(arg0)};
    return ret as i8;
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
    let mut ret = unsafe {_ZNK9QTimeZone7commentEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
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
    let mut ret = unsafe {_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK9QTimeZone14hasTransitionsEv(rsthis.qclsinst)};
    return ret as i8;
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
    let mut ret = unsafe {_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32;
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
    let mut ret = unsafe {_ZN9QTimeZone14systemTimeZoneEv()};
    let mut ret1 = QTimeZone::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTimeZone::QTimeZone(const QByteArray & ianaId);
impl<'a> /*trait*/ QTimeZone_New for (&'a QByteArray) {
  fn New(self) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1ERK10QByteArray()};
    let ctysz: c_int = unsafe{QTimeZone_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QTimeZoneC1ERK10QByteArray(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QTimeZoneC1ERK10QByteArray(arg0)};
    let rsthis = QTimeZone{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimeZone::QTimeZone(const QTimeZone & other);
impl<'a> /*trait*/ QTimeZone_New for (&'a QTimeZone) {
  fn New(self) -> QTimeZone {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneC1ERKS_()};
    let ctysz: c_int = unsafe{QTimeZone_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QTimeZoneC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QTimeZoneC1ERKS_(arg0)};
    let rsthis = QTimeZone{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimeZone::~QTimeZone();
impl /*struct*/ QTimeZone {
  pub fn Free<RetType, T: QTimeZone_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTimeZone_Free<RetType> {
  fn Free(self , rsthis: & QTimeZone) -> RetType;
}

  // proto:  void QTimeZone::~QTimeZone();
impl<'a> /*trait*/ QTimeZone_Free<()> for () {
  fn Free(self , rsthis: & QTimeZone) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZoneD0Ev()};
     unsafe {_ZN9QTimeZoneD0Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32;
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
    let mut ret = unsafe {_ZNK9QTimeZone2idEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK9QTimeZone13offsetFromUtcERK9QDateTime(rsthis.qclsinst, arg0)};
    return ret as i32;
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
impl<'a> /*trait*/ QTimeZone_windowsIdToIanaIds_s<()> for (&'a QByteArray) {
  fn windowsIdToIanaIds_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTimeZone18windowsIdToIanaIdsERK10QByteArray(arg0)};
    // return 1;
  }
}

// <= body block end

