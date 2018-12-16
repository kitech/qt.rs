

// mod ::core::QTimeZone
// package qtcore
// /usr/include/qt/QtCore/qtimezone.h
// #include <qtimezone.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTimeZone)=8
pub struct QTimeZone {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTimeZone_ITF interface {
//    QTimeZone_PTR() *QTimeZone
//}
//func (ptr *QTimeZone) QTimeZone_PTR() *QTimeZone { return ptr }

impl /*struct*/ QTimeZone {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTimeZone {
    return QTimeZone{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTimeZone {
//  type Target = QTimeZoneBASE;
//
//  fn deref(&self) -> &QTimeZoneBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTimeZoneBASE> for QTimeZone {
//  fn as_ref(& self) -> & QTimeZoneBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtimezone.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTimeZone()

/*
Create a null/invalid time zone instance.
*/
// QTimeZone() ctx.fn_proto_cpp
impl /*struct*/ QTimeZone {
  pub fn QTimeZone_0<T: QTimeZone_QTimeZone_0>(value: T) -> QTimeZone {
    let rsthis = value.QTimeZone_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeZone_QTimeZone_0 {
  fn QTimeZone_0(self) -> QTimeZone;
}
// QTimeZone() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimeZone_QTimeZone_0 for () {
  fn QTimeZone_0(self) -> QTimeZone {
    // unsafe{_ZN9QTimeZoneC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTimeZoneC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimeZone{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:93
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTimeZone(const QByteArray &)

/*
Create a null/invalid time zone instance.
*/
// QTimeZone(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QTimeZone {
  pub fn QTimeZone_1<T: QTimeZone_QTimeZone_1>(value: T) -> QTimeZone {
    let rsthis = value.QTimeZone_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeZone_QTimeZone_1 {
  fn QTimeZone_1(self) -> QTimeZone;
}
// QTimeZone(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimeZone_QTimeZone_1 for (usize) {
  fn QTimeZone_1(self) -> QTimeZone {
    // unsafe{_ZN9QTimeZoneC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTimeZoneC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimeZone{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:94
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTimeZone(int)

/*
Create a null/invalid time zone instance.
*/
// QTimeZone(int) ctx.fn_proto_cpp
impl /*struct*/ QTimeZone {
  pub fn QTimeZone_2<T: QTimeZone_QTimeZone_2>(value: T) -> QTimeZone {
    let rsthis = value.QTimeZone_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeZone_QTimeZone_2 {
  fn QTimeZone_2(self) -> QTimeZone;
}
// QTimeZone(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimeZone_QTimeZone_2 for (i32) {
  fn QTimeZone_2(self) -> QTimeZone {
    // unsafe{_ZN9QTimeZoneC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTimeZoneC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimeZone{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:95
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QTimeZone(const QByteArray &, int, const QString &, const QString &, QLocale::Country, const QString &)

/*
Create a null/invalid time zone instance.
*/
// QTimeZone(const QByteArray &, int, const QString &, const QString &, QLocale::Country, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QTimeZone {
  pub fn QTimeZone_3<T: QTimeZone_QTimeZone_3>(value: T) -> QTimeZone {
    let rsthis = value.QTimeZone_3();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeZone_QTimeZone_3 {
  fn QTimeZone_3(self) -> QTimeZone;
}
// QTimeZone(const QByteArray &, int, const QString &, const QString &, QLocale::Country, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimeZone_QTimeZone_3 for (usize,i32,usize,usize,i32,usize) {
  fn QTimeZone_3(self) -> QTimeZone {
    // unsafe{_ZN9QTimeZoneC2ERK10QByteArrayiRK7QStringS5_N7QLocale7CountryES5_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTimeZoneC2ERK10QByteArrayiRK7QStringS5_N7QLocale7CountryES5_", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimeZone{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTimeZone()

/*

*/
pub fn DeleteQTimeZone(this :*mut QTimeZone) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QTimeZoneD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtimezone.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QTimeZone & operator=(const QTimeZone &)

/*

*/
impl /*struct*/ QTimeZone {
  pub fn operator_equal_0<RetType, T: QTimeZone_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTimeZone_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZoneaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:103
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QTimeZone & operator=(QTimeZone &&)

/*

*/
impl /*struct*/ QTimeZone {
  pub fn operator_equal_1<RetType, T: QTimeZone_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QTimeZone_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZoneaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:106
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QTimeZone &)

/*
Swaps this time zone instance with other. This function is very fast and never fails.
*/
impl /*struct*/ QTimeZone {
  pub fn swap_0<RetType, T: QTimeZone_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QTimeZone_swap_0<RetType> {
  fn swap_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QTimeZone) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeZone4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QTimeZone &) const

/*

*/
impl /*struct*/ QTimeZone {
  pub fn operator_equal_equal_0<RetType, T: QTimeZone_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTimeZone_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTimeZone) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZoneeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:110
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QTimeZone &) const

/*

*/
impl /*struct*/ QTimeZone {
  pub fn operator_not_equal_0<RetType, T: QTimeZone_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTimeZone_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTimeZone) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZoneneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:112
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if this time zone is valid.
*/
impl /*struct*/ QTimeZone {
  pub fn isValid_0<RetType, T: QTimeZone_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTimeZone_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTimeZone) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:114
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray id() const

/*
Returns the IANA ID for the time zone.

IANA IDs are used on all platforms. On Windows these are translated from the Windows ID into the closest IANA ID for the time zone and country.
*/
impl /*struct*/ QTimeZone {
  pub fn id_0<RetType, T: QTimeZone_id_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.id_0(self);
    // return 1;
  }
}
pub trait QTimeZone_id_0<RetType> {
  fn id_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_id_0<usize> for () {
  fn id_0(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone2idEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:115
// index:0
// Public Visibility=Default Availability=Available
// [4] QLocale::Country country() const

/*
Returns the country for the time zone.
*/
impl /*struct*/ QTimeZone {
  pub fn country_0<RetType, T: QTimeZone_country_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.country_0(self);
    // return 1;
  }
}
pub trait QTimeZone_country_0<RetType> {
  fn country_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_country_0<i32> for () {
  fn country_0(self , rsthis: & QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone7countryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:116
// index:0
// Public Visibility=Default Availability=Available
// [8] QString comment() const

/*
Returns any comment for the time zone.

A comment may be provided by the host platform to assist users in choosing the correct time zone. Depending on the platform this may not be localized.
*/
impl /*struct*/ QTimeZone {
  pub fn comment_0<RetType, T: QTimeZone_comment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.comment_0(self);
    // return 1;
  }
}
pub trait QTimeZone_comment_0<RetType> {
  fn comment_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_comment_0<usize> for () {
  fn comment_0(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone7commentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:118
// index:0
// Public Visibility=Default Availability=Available
// [8] QString displayName(const QDateTime &, QTimeZone::NameType, const QLocale &) const

/*
Returns the localized time zone display name at the given atDateTime for the given nameType in the given locale. The nameType and locale requested may not be supported on all platforms, in which case the best available option will be returned.

If the locale is not provided then the application default locale will be used.

The display name may change depending on DST or historical events.

See also abbreviation().
*/
impl /*struct*/ QTimeZone {
  pub fn displayName_0<RetType, T: QTimeZone_displayName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.displayName_0(self);
    // return 1;
  }
}
pub trait QTimeZone_displayName_0<RetType> {
  fn displayName_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_displayName_0<usize> for (usize,i32,usize) {
  fn displayName_0(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone11displayNameERK9QDateTimeNS_8NameTypeERK7QLocale", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:121
// index:1
// Public Visibility=Default Availability=Available
// [8] QString displayName(QTimeZone::TimeType, QTimeZone::NameType, const QLocale &) const

/*
Returns the localized time zone display name at the given atDateTime for the given nameType in the given locale. The nameType and locale requested may not be supported on all platforms, in which case the best available option will be returned.

If the locale is not provided then the application default locale will be used.

The display name may change depending on DST or historical events.

See also abbreviation().
*/
impl /*struct*/ QTimeZone {
  pub fn displayName_1<RetType, T: QTimeZone_displayName_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.displayName_1(self);
    // return 1;
  }
}
pub trait QTimeZone_displayName_1<RetType> {
  fn displayName_1(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_displayName_1<usize> for (i32,i32,usize) {
  fn displayName_1(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone11displayNameENS_8TimeTypeENS_8NameTypeERK7QLocale", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:124
// index:0
// Public Visibility=Default Availability=Available
// [8] QString abbreviation(const QDateTime &) const

/*
Returns the time zone abbreviation at the given atDateTime. The abbreviation may change depending on DST or even historical events.

Note that the abbreviation is not guaranteed to be unique to this time zone and should not be used in place of the ID or display name.

See also displayName().
*/
impl /*struct*/ QTimeZone {
  pub fn abbreviation_0<RetType, T: QTimeZone_abbreviation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.abbreviation_0(self);
    // return 1;
  }
}
pub trait QTimeZone_abbreviation_0<RetType> {
  fn abbreviation_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_abbreviation_0<usize> for (usize) {
  fn abbreviation_0(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone12abbreviationERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:126
// index:0
// Public Visibility=Default Availability=Available
// [4] int offsetFromUtc(const QDateTime &) const

/*
Returns the total effective offset at the given atDateTime, i.e. the number of seconds to add to UTC to obtain the local time. This includes any DST offset that may be in effect, i.e. it is the sum of standardTimeOffset() and daylightTimeOffset() for the given datetime.

For example, for the time zone "Europe/Berlin" the standard time offset is +3600 seconds and the DST offset is +3600 seconds. During standard time offsetFromUtc() will return +3600 (UTC+01:00), and during DST it will return +7200 (UTC+02:00).

See also standardTimeOffset() and daylightTimeOffset().
*/
impl /*struct*/ QTimeZone {
  pub fn offsetFromUtc_0<RetType, T: QTimeZone_offsetFromUtc_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offsetFromUtc_0(self);
    // return 1;
  }
}
pub trait QTimeZone_offsetFromUtc_0<RetType> {
  fn offsetFromUtc_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_offsetFromUtc_0<i32> for (usize) {
  fn offsetFromUtc_0(self , rsthis: & QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone13offsetFromUtcERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:127
// index:0
// Public Visibility=Default Availability=Available
// [4] int standardTimeOffset(const QDateTime &) const

/*
Returns the standard time offset at the given atDateTime, i.e. the number of seconds to add to UTC to obtain the local Standard Time. This excludes any DST offset that may be in effect.

For example, for the time zone "Europe/Berlin" the standard time offset is +3600 seconds. During both standard and DST offsetFromUtc() will return +3600 (UTC+01:00).

See also offsetFromUtc() and daylightTimeOffset().
*/
impl /*struct*/ QTimeZone {
  pub fn standardTimeOffset_0<RetType, T: QTimeZone_standardTimeOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardTimeOffset_0(self);
    // return 1;
  }
}
pub trait QTimeZone_standardTimeOffset_0<RetType> {
  fn standardTimeOffset_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_standardTimeOffset_0<i32> for (usize) {
  fn standardTimeOffset_0(self , rsthis: & QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone18standardTimeOffsetERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:128
// index:0
// Public Visibility=Default Availability=Available
// [4] int daylightTimeOffset(const QDateTime &) const

/*
Returns the daylight-saving time offset at the given atDateTime, i.e. the number of seconds to add to the standard time offset to obtain the local daylight-saving time.

For example, for the time zone "Europe/Berlin" the DST offset is +3600 seconds. During standard time daylightTimeOffset() will return 0, and when daylight-saving is in effect it will return +3600.

See also offsetFromUtc() and standardTimeOffset().
*/
impl /*struct*/ QTimeZone {
  pub fn daylightTimeOffset_0<RetType, T: QTimeZone_daylightTimeOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.daylightTimeOffset_0(self);
    // return 1;
  }
}
pub trait QTimeZone_daylightTimeOffset_0<RetType> {
  fn daylightTimeOffset_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_daylightTimeOffset_0<i32> for (usize) {
  fn daylightTimeOffset_0(self , rsthis: & QTimeZone) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone18daylightTimeOffsetERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasDaylightTime() const

/*
Returns true if the time zone has practiced daylight-saving at any time.

See also isDaylightTime() and daylightTimeOffset().
*/
impl /*struct*/ QTimeZone {
  pub fn hasDaylightTime_0<RetType, T: QTimeZone_hasDaylightTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasDaylightTime_0(self);
    // return 1;
  }
}
pub trait QTimeZone_hasDaylightTime_0<RetType> {
  fn hasDaylightTime_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_hasDaylightTime_0<bool> for () {
  fn hasDaylightTime_0(self , rsthis: & QTimeZone) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone15hasDaylightTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:131
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDaylightTime(const QDateTime &) const

/*
Returns true if daylight-saving was in effect at the given atDateTime.

See also hasDaylightTime() and daylightTimeOffset().
*/
impl /*struct*/ QTimeZone {
  pub fn isDaylightTime_0<RetType, T: QTimeZone_isDaylightTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDaylightTime_0(self);
    // return 1;
  }
}
pub trait QTimeZone_isDaylightTime_0<RetType> {
  fn isDaylightTime_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_isDaylightTime_0<bool> for (usize) {
  fn isDaylightTime_0(self , rsthis: & QTimeZone) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone14isDaylightTimeERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:133
// index:0
// Public Visibility=Default Availability=Available
// [32] QTimeZone::OffsetData offsetData(const QDateTime &) const

/*
Returns the effective offset details at the given forDateTime. This is the equivalent of calling offsetFromUtc(), abbreviation(), etc individually but is more efficient.

See also offsetFromUtc(), standardTimeOffset(), daylightTimeOffset(), and abbreviation().
*/
impl /*struct*/ QTimeZone {
  pub fn offsetData_0<RetType, T: QTimeZone_offsetData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offsetData_0(self);
    // return 1;
  }
}
pub trait QTimeZone_offsetData_0<RetType> {
  fn offsetData_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_offsetData_0<usize> for (usize) {
  fn offsetData_0(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone10offsetDataERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:135
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasTransitions() const

/*
Returns true if the system backend supports obtaining transitions.

Transitions are changes in the time-zone: these happen when DST turns on or off and when authorities alter the offsets for the time-zone.

See also nextTransition(), previousTransition(), and transitions().
*/
impl /*struct*/ QTimeZone {
  pub fn hasTransitions_0<RetType, T: QTimeZone_hasTransitions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasTransitions_0(self);
    // return 1;
  }
}
pub trait QTimeZone_hasTransitions_0<RetType> {
  fn hasTransitions_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_hasTransitions_0<bool> for () {
  fn hasTransitions_0(self , rsthis: & QTimeZone) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone14hasTransitionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:136
// index:0
// Public Visibility=Default Availability=Available
// [32] QTimeZone::OffsetData nextTransition(const QDateTime &) const

/*
Returns the first time zone Transition after the given afterDateTime. This is most useful when you have a Transition time and wish to find the Transition after it.

If there is no transition after the given afterDateTime then an invalid OffsetData will be returned with an invalid QDateTime.

The given afterDateTime is exclusive.

See also hasTransitions(), previousTransition(), and transitions().
*/
impl /*struct*/ QTimeZone {
  pub fn nextTransition_0<RetType, T: QTimeZone_nextTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextTransition_0(self);
    // return 1;
  }
}
pub trait QTimeZone_nextTransition_0<RetType> {
  fn nextTransition_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_nextTransition_0<usize> for (usize) {
  fn nextTransition_0(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone14nextTransitionERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:137
// index:0
// Public Visibility=Default Availability=Available
// [32] QTimeZone::OffsetData previousTransition(const QDateTime &) const

/*
Returns the first time zone Transition before the given beforeDateTime. This is most useful when you have a Transition time and wish to find the Transition before it.

If there is no transition before the given beforeDateTime then an invalid OffsetData will be returned with an invalid QDateTime.

The given beforeDateTime is exclusive.

See also hasTransitions(), nextTransition(), and transitions().
*/
impl /*struct*/ QTimeZone {
  pub fn previousTransition_0<RetType, T: QTimeZone_previousTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.previousTransition_0(self);
    // return 1;
  }
}
pub trait QTimeZone_previousTransition_0<RetType> {
  fn previousTransition_0(self , rsthis: & QTimeZone) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_previousTransition_0<usize> for (usize) {
  fn previousTransition_0(self , rsthis: & QTimeZone) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeZone18previousTransitionERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:140
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray systemTimeZoneId()

/*
Returns the current system time zone IANA ID.

On Windows this ID is translated from the Windows ID using an internal translation table and the user's selected country. As a consequence there is a small chance any Windows install may have IDs not known by Qt, in which case "UTC" will be returned.
*/
impl /*struct*/ QTimeZone {
  pub fn systemTimeZoneId_0<RetType, T: QTimeZone_systemTimeZoneId_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemTimeZoneId_0();
    // return 1;
  }
}
pub trait QTimeZone_systemTimeZoneId_0<RetType> {
  fn systemTimeZoneId_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_systemTimeZoneId_0<usize> for () {
  fn systemTimeZoneId_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZone16systemTimeZoneIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:141
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTimeZone systemTimeZone()

/*
Returns a QTimeZone object that refers to the local system time, as specified by systemTimeZoneId().

This function was introduced in  Qt 5.5.

See also utc().
*/
impl /*struct*/ QTimeZone {
  pub fn systemTimeZone_0<RetType, T: QTimeZone_systemTimeZone_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemTimeZone_0();
    // return 1;
  }
}
pub trait QTimeZone_systemTimeZone_0<RetType> {
  fn systemTimeZone_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_systemTimeZone_0<usize> for () {
  fn systemTimeZone_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZone14systemTimeZoneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:142
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTimeZone utc()

/*
Returns a QTimeZone object that refers to UTC (Universal Time Coordinated).

This function was introduced in  Qt 5.5.

See also systemTimeZone().
*/
impl /*struct*/ QTimeZone {
  pub fn utc_0<RetType, T: QTimeZone_utc_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.utc_0();
    // return 1;
  }
}
pub trait QTimeZone_utc_0<RetType> {
  fn utc_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_utc_0<usize> for () {
  fn utc_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZone3utcEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:144
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isTimeZoneIdAvailable(const QByteArray &)

/*
Returns true if a given time zone ianaId is available on this system.

See also availableTimeZoneIds().
*/
impl /*struct*/ QTimeZone {
  pub fn isTimeZoneIdAvailable_0<RetType, T: QTimeZone_isTimeZoneIdAvailable_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isTimeZoneIdAvailable_0();
    // return 1;
  }
}
pub trait QTimeZone_isTimeZoneIdAvailable_0<RetType> {
  fn isTimeZoneIdAvailable_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_isTimeZoneIdAvailable_0<bool> for (usize) {
  fn isTimeZoneIdAvailable_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZone21isTimeZoneIdAvailableERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:150
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray ianaIdToWindowsId(const QByteArray &)

/*
Returns the Windows ID equivalent to the given ianaId.

See also windowsIdToDefaultIanaId() and windowsIdToIanaIds().
*/
impl /*struct*/ QTimeZone {
  pub fn ianaIdToWindowsId_0<RetType, T: QTimeZone_ianaIdToWindowsId_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.ianaIdToWindowsId_0();
    // return 1;
  }
}
pub trait QTimeZone_ianaIdToWindowsId_0<RetType> {
  fn ianaIdToWindowsId_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_ianaIdToWindowsId_0<usize> for (usize) {
  fn ianaIdToWindowsId_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZone17ianaIdToWindowsIdERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:151
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray windowsIdToDefaultIanaId(const QByteArray &)

/*
Returns the default IANA ID for a given windowsId.

Because a Windows ID can cover several IANA IDs in several different countries, this function returns the most frequently used IANA ID with no regard for the country and should thus be used with care. It is usually best to request the default for a specific country.

See also ianaIdToWindowsId() and windowsIdToIanaIds().
*/
impl /*struct*/ QTimeZone {
  pub fn windowsIdToDefaultIanaId_0<RetType, T: QTimeZone_windowsIdToDefaultIanaId_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowsIdToDefaultIanaId_0();
    // return 1;
  }
}
pub trait QTimeZone_windowsIdToDefaultIanaId_0<RetType> {
  fn windowsIdToDefaultIanaId_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_windowsIdToDefaultIanaId_0<usize> for (usize) {
  fn windowsIdToDefaultIanaId_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimezone.h:152
// index:1
// Public static Visibility=Default Availability=Available
// [8] QByteArray windowsIdToDefaultIanaId(const QByteArray &, QLocale::Country)

/*
Returns the default IANA ID for a given windowsId.

Because a Windows ID can cover several IANA IDs in several different countries, this function returns the most frequently used IANA ID with no regard for the country and should thus be used with care. It is usually best to request the default for a specific country.

See also ianaIdToWindowsId() and windowsIdToIanaIds().
*/
impl /*struct*/ QTimeZone {
  pub fn windowsIdToDefaultIanaId_1<RetType, T: QTimeZone_windowsIdToDefaultIanaId_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowsIdToDefaultIanaId_1();
    // return 1;
  }
}
pub trait QTimeZone_windowsIdToDefaultIanaId_1<RetType> {
  fn windowsIdToDefaultIanaId_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTimeZone_windowsIdToDefaultIanaId_1<usize> for (usize,i32) {
  fn windowsIdToDefaultIanaId_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTimeZone24windowsIdToDefaultIanaIdERK10QByteArrayN7QLocale7CountryE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QTimeZone__ = i32;
// 
pub const QTimeZone__MinUtcOffsetSecs :QTimeZone__ = -50400;
// 
pub const QTimeZone__MaxUtcOffsetSecs :QTimeZone__ = 50400;
pub fn QTimeZone_ItemName(val: i32) ->String {
  match val {
     QTimeZone__MinUtcOffsetSecs => // -50400
     {return String::from("MinUtcOffsetSecs");}
     QTimeZone__MaxUtcOffsetSecs => // 50400
     {return String::from("MaxUtcOffsetSecs");}
  _ => {return format!("{}", val);}
}
}
pub fn QTimeZone_ItemName_s(val: i32) ->String {
  //var nilthis *QTimeZone
  //return nilthis.ItemName(val);
  return QTimeZone_ItemName(val);
}


/*
The type of time zone time, for example when requesting the name. In time zones that do not apply DST, all three values may return the same result.


*/
pub type QTimeZone__TimeType = i32;
// The standard time in a time zone, i.e. when Daylight-Saving is not in effect. For example when formatting a display name this will show something like "Pacific Standard Time".
pub const QTimeZone__StandardTime :QTimeZone__TimeType = 0;
// A time when Daylight-Saving is in effect. For example when formatting a display name this will show something like "Pacific daylight-saving time".
pub const QTimeZone__DaylightTime :QTimeZone__TimeType = 1;
// A time which is not specifically Standard or Daylight-Saving time, either an unknown time or a neutral form. For example when formatting a display name this will show something like "Pacific Time".
pub const QTimeZone__GenericTime :QTimeZone__TimeType = 2;
pub fn QTimeZone_TimeTypeItemName(val: i32) ->String {
  match val {
     QTimeZone__StandardTime => // 0
     {return String::from("StandardTime");}
     QTimeZone__DaylightTime => // 1
     {return String::from("DaylightTime");}
     QTimeZone__GenericTime => // 2
     {return String::from("GenericTime");}
  _ => {return format!("{}", val);}
}
}
pub fn QTimeZone_TimeTypeItemName_s(val: i32) ->String {
  //var nilthis *QTimeZone
  //return nilthis.TimeTypeItemName(val);
  return QTimeZone_TimeTypeItemName(val);
}


/*
The type of time zone name.


*/
pub type QTimeZone__NameType = i32;
// The default form of the time zone name, e.g. LongName, ShortName or OffsetName
pub const QTimeZone__DefaultName :QTimeZone__NameType = 0;
// The long form of the time zone name, e.g. "Central European Time"
pub const QTimeZone__LongName :QTimeZone__NameType = 1;
// The short form of the time zone name, usually an abbreviation, e.g. "CET"
pub const QTimeZone__ShortName :QTimeZone__NameType = 2;
// 
pub const QTimeZone__OffsetName :QTimeZone__NameType = 3;
pub fn QTimeZone_NameTypeItemName(val: i32) ->String {
  match val {
     QTimeZone__DefaultName => // 0
     {return String::from("DefaultName");}
     QTimeZone__LongName => // 1
     {return String::from("LongName");}
     QTimeZone__ShortName => // 2
     {return String::from("ShortName");}
     QTimeZone__OffsetName => // 3
     {return String::from("OffsetName");}
  _ => {return format!("{}", val);}
}
}
pub fn QTimeZone_NameTypeItemName_s(val: i32) ->String {
  //var nilthis *QTimeZone
  //return nilthis.NameTypeItemName(val);
  return QTimeZone_NameTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
