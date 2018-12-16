

// mod ::core::QDateTime
// package qtcore
// /usr/include/qt/QtCore/qdatetime.h
// #include <qdatetime.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 31
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
#[derive(Default)] // class sizeof(QDateTime)=8
pub struct QDateTime {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDateTime_ITF interface {
//    QDateTime_PTR() *QDateTime
//}
//func (ptr *QDateTime) QDateTime_PTR() *QDateTime { return ptr }

impl /*struct*/ QDateTime {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDateTime {
    return QDateTime{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDateTime {
//  type Target = QDateTimeBASE;
//
//  fn deref(&self) -> &QDateTimeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDateTimeBASE> for QDateTime {
//  fn as_ref(& self) -> & QDateTimeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qdatetime.h:261
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDateTime()

/*
Constructs a null datetime (i.e. null date and null time). A null datetime is invalid, since the date is invalid.

See also isValid().
*/
// QDateTime() ctx.fn_proto_cpp
impl /*struct*/ QDateTime {
  pub fn QDateTime_0<T: QDateTime_QDateTime_0>(value: T) -> QDateTime {
    let rsthis = value.QDateTime_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTime_QDateTime_0 {
  fn QDateTime_0(self) -> QDateTime;
}
// QDateTime() ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTime_QDateTime_0 for () {
  fn QDateTime_0(self) -> QDateTime {
    // unsafe{_ZN9QDateTimeC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDateTimeC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:262
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDateTime(const QDate &)

/*
Constructs a null datetime (i.e. null date and null time). A null datetime is invalid, since the date is invalid.

See also isValid().
*/
// QDateTime(const QDate &) ctx.fn_proto_cpp
impl /*struct*/ QDateTime {
  pub fn QDateTime_1<T: QDateTime_QDateTime_1>(value: T) -> QDateTime {
    let rsthis = value.QDateTime_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTime_QDateTime_1 {
  fn QDateTime_1(self) -> QDateTime;
}
// QDateTime(const QDate &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTime_QDateTime_1 for (usize) {
  fn QDateTime_1(self) -> QDateTime {
    // unsafe{_ZN9QDateTimeC2ERK5QDate()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDateTimeC2ERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:263
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QDateTime(const QDate &, const QTime &, Qt::TimeSpec)

/*
Constructs a null datetime (i.e. null date and null time). A null datetime is invalid, since the date is invalid.

See also isValid().
*/
// QDateTime(const QDate &, const QTime &, Qt::TimeSpec) ctx.fn_proto_cpp
impl /*struct*/ QDateTime {
  pub fn QDateTime_2<T: QDateTime_QDateTime_2>(value: T) -> QDateTime {
    let rsthis = value.QDateTime_2();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTime_QDateTime_2 {
  fn QDateTime_2(self) -> QDateTime;
}
// QDateTime(const QDate &, const QTime &, Qt::TimeSpec) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTime_QDateTime_2 for (usize,usize,i32) {
  fn QDateTime_2(self) -> QDateTime {
    // unsafe{_ZN9QDateTimeC2ERK5QDateRK5QTimeN2Qt8TimeSpecE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDateTimeC2ERK5QDateRK5QTimeN2Qt8TimeSpecE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:265
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QDateTime(const QDate &, const QTime &, Qt::TimeSpec, int)

/*
Constructs a null datetime (i.e. null date and null time). A null datetime is invalid, since the date is invalid.

See also isValid().
*/
// QDateTime(const QDate &, const QTime &, Qt::TimeSpec, int) ctx.fn_proto_cpp
impl /*struct*/ QDateTime {
  pub fn QDateTime_3<T: QDateTime_QDateTime_3>(value: T) -> QDateTime {
    let rsthis = value.QDateTime_3();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTime_QDateTime_3 {
  fn QDateTime_3(self) -> QDateTime;
}
// QDateTime(const QDate &, const QTime &, Qt::TimeSpec, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTime_QDateTime_3 for (usize,usize,i32,i32) {
  fn QDateTime_3(self) -> QDateTime {
    // unsafe{_ZN9QDateTimeC2ERK5QDateRK5QTimeN2Qt8TimeSpecEi()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDateTimeC2ERK5QDateRK5QTimeN2Qt8TimeSpecEi", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:267
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QDateTime(const QDate &, const QTime &, const QTimeZone &)

/*
Constructs a null datetime (i.e. null date and null time). A null datetime is invalid, since the date is invalid.

See also isValid().
*/
// QDateTime(const QDate &, const QTime &, const QTimeZone &) ctx.fn_proto_cpp
impl /*struct*/ QDateTime {
  pub fn QDateTime_4<T: QDateTime_QDateTime_4>(value: T) -> QDateTime {
    let rsthis = value.QDateTime_4();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTime_QDateTime_4 {
  fn QDateTime_4(self) -> QDateTime;
}
// QDateTime(const QDate &, const QTime &, const QTimeZone &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTime_QDateTime_4 for (usize,usize,usize) {
  fn QDateTime_4(self) -> QDateTime {
    // unsafe{_ZN9QDateTimeC2ERK5QDateRK5QTimeRK9QTimeZone()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDateTimeC2ERK5QDateRK5QTimeRK9QTimeZone", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:271
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QDateTime()

/*

*/
pub fn DeleteQDateTime(this :*mut QDateTime) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QDateTimeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qdatetime.h:274
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QDateTime & operator=(QDateTime &&)

/*

*/
impl /*struct*/ QDateTime {
  pub fn operator_equal_0<RetType, T: QDateTime_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QDateTime_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTimeaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:276
// index:1
// Public Visibility=Default Availability=Available
// [8] QDateTime & operator=(const QDateTime &)

/*

*/
impl /*struct*/ QDateTime {
  pub fn operator_equal_1<RetType, T: QDateTime_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QDateTime_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTimeaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:278
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QDateTime &)

/*
Swaps this datetime with other. This operation is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QDateTime {
  pub fn swap_0<RetType, T: QDateTime_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QDateTime_swap_0<RetType> {
  fn swap_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:280
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if both the date and the time are null; otherwise returns false. A null datetime is invalid.

See also QDate::isNull(), QTime::isNull(), and isValid().
*/
impl /*struct*/ QDateTime {
  pub fn isNull_0<RetType, T: QDateTime_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QDateTime_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:281
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if both the date and the time are valid and they are valid in the current Qt::TimeSpec, otherwise returns false.

If the timeSpec() is Qt::LocalTime or Qt::TimeZone then the date and time are checked to see if they fall in the Standard Time to Daylight-Saving Time transition hour, i.e. if the transition is at 2am and the clock goes forward to 3am then the time from 02:00:00 to 02:59:59.999 is considered to be invalid.

See also QDate::isValid() and QTime::isValid().
*/
impl /*struct*/ QDateTime {
  pub fn isValid_0<RetType, T: QDateTime_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QDateTime_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:283
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate date() const

/*
Returns the date part of the datetime.

See also setDate(), time(), and timeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn date_0<RetType, T: QDateTime_date_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.date_0(self);
    // return 1;
  }
}
pub trait QDateTime_date_0<RetType> {
  fn date_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_date_0<usize> for () {
  fn date_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime4dateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:284
// index:0
// Public Visibility=Default Availability=Available
// [4] QTime time() const

/*
Returns the time part of the datetime.

See also setTime(), date(), and timeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn time_0<RetType, T: QDateTime_time_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.time_0(self);
    // return 1;
  }
}
pub trait QDateTime_time_0<RetType> {
  fn time_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_time_0<usize> for () {
  fn time_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime4timeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:285
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TimeSpec timeSpec() const

/*
Returns the time specification of the datetime.

See also setTimeSpec(), date(), time(), and Qt::TimeSpec.
*/
impl /*struct*/ QDateTime {
  pub fn timeSpec_0<RetType, T: QDateTime_timeSpec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timeSpec_0(self);
    // return 1;
  }
}
pub trait QDateTime_timeSpec_0<RetType> {
  fn timeSpec_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_timeSpec_0<i32> for () {
  fn timeSpec_0(self , rsthis: & QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime8timeSpecEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:286
// index:0
// Public Visibility=Default Availability=Available
// [4] int offsetFromUtc() const

/*
Returns the current Offset From UTC in seconds.

If the timeSpec() is Qt::OffsetFromUTC this will be the value originally set.

If the timeSpec() is Qt::TimeZone this will be the offset effective in the Time Zone including any Daylight-Saving Offset.

If the timeSpec() is Qt::LocalTime this will be the difference between the Local Time and UTC including any Daylight-Saving Offset.

If the timeSpec() is Qt::UTC this will be 0.

This function was introduced in  Qt 5.2.

See also setOffsetFromUtc().
*/
impl /*struct*/ QDateTime {
  pub fn offsetFromUtc_0<RetType, T: QDateTime_offsetFromUtc_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offsetFromUtc_0(self);
    // return 1;
  }
}
pub trait QDateTime_offsetFromUtc_0<RetType> {
  fn offsetFromUtc_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_offsetFromUtc_0<i32> for () {
  fn offsetFromUtc_0(self , rsthis: & QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime13offsetFromUtcEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:288
// index:0
// Public Visibility=Default Availability=Available
// [8] QTimeZone timeZone() const

/*
Returns the time zone of the datetime.

If the timeSpec() is Qt::LocalTime then an instance of the current system time zone will be returned. Note however that if you copy this time zone the instance will not remain in sync if the system time zone changes.

This function was introduced in  Qt 5.2.

See also setTimeZone() and Qt::TimeSpec.
*/
impl /*struct*/ QDateTime {
  pub fn timeZone_0<RetType, T: QDateTime_timeZone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timeZone_0(self);
    // return 1;
  }
}
pub trait QDateTime_timeZone_0<RetType> {
  fn timeZone_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_timeZone_0<usize> for () {
  fn timeZone_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime8timeZoneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:290
// index:0
// Public Visibility=Default Availability=Available
// [8] QString timeZoneAbbreviation() const

/*
Returns the Time Zone Abbreviation for the datetime.

If the timeSpec() is Qt::UTC this will be "UTC".

If the timeSpec() is Qt::OffsetFromUTC this will be in the format "UTC[+-]00:00".

If the timeSpec() is Qt::LocalTime then the host system is queried for the correct abbreviation.

Note that abbreviations may or may not be localized.

Note too that the abbreviation is not guaranteed to be a unique value, i.e. different time zones may have the same abbreviation.

This function was introduced in  Qt 5.2.

See also timeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn timeZoneAbbreviation_0<RetType, T: QDateTime_timeZoneAbbreviation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timeZoneAbbreviation_0(self);
    // return 1;
  }
}
pub trait QDateTime_timeZoneAbbreviation_0<RetType> {
  fn timeZoneAbbreviation_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_timeZoneAbbreviation_0<usize> for () {
  fn timeZoneAbbreviation_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime20timeZoneAbbreviationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:291
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDaylightTime() const

/*
Returns if this datetime falls in Daylight-Saving Time.

If the Qt::TimeSpec is not Qt::LocalTime or Qt::TimeZone then will always return false.

This function was introduced in  Qt 5.2.

See also timeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn isDaylightTime_0<RetType, T: QDateTime_isDaylightTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDaylightTime_0(self);
    // return 1;
  }
}
pub trait QDateTime_isDaylightTime_0<RetType> {
  fn isDaylightTime_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_isDaylightTime_0<bool> for () {
  fn isDaylightTime_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime14isDaylightTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:293
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 toMSecsSinceEpoch() const

/*
Returns the datetime as the number of milliseconds that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC).

On systems that do not support time zones, this function will behave as if local time were Qt::UTC.

The behavior for this function is undefined if the datetime stored in this object is not valid. However, for all valid dates, this function returns a unique value.

This function was introduced in  Qt 4.7.

See also toSecsSinceEpoch() and setMSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn toMSecsSinceEpoch_0<RetType, T: QDateTime_toMSecsSinceEpoch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toMSecsSinceEpoch_0(self);
    // return 1;
  }
}
pub trait QDateTime_toMSecsSinceEpoch_0<RetType> {
  fn toMSecsSinceEpoch_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toMSecsSinceEpoch_0<i64> for () {
  fn toMSecsSinceEpoch_0(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime17toMSecsSinceEpochEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:294
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 toSecsSinceEpoch() const

/*
Returns the datetime as the number of seconds that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC).

On systems that do not support time zones, this function will behave as if local time were Qt::UTC.

The behavior for this function is undefined if the datetime stored in this object is not valid. However, for all valid dates, this function returns a unique value.

This function was introduced in  Qt 5.8.

See also toMSecsSinceEpoch() and setSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn toSecsSinceEpoch_0<RetType, T: QDateTime_toSecsSinceEpoch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toSecsSinceEpoch_0(self);
    // return 1;
  }
}
pub trait QDateTime_toSecsSinceEpoch_0<RetType> {
  fn toSecsSinceEpoch_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toSecsSinceEpoch_0<i64> for () {
  fn toSecsSinceEpoch_0(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime16toSecsSinceEpochEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:296
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDate(const QDate &)

/*
Sets the date part of this datetime to date. If no time is set yet, it is set to midnight. If date is invalid, this QDateTime becomes invalid.

See also date(), setTime(), and setTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn setDate_0<RetType, T: QDateTime_setDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDate_0(self);
    // return 1;
  }
}
pub trait QDateTime_setDate_0<RetType> {
  fn setDate_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setDate_0<(/*void*/)> for (usize) {
  fn setDate_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime7setDateERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:297
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTime(const QTime &)

/*
Sets the time part of this datetime to time. If time is not valid, this function sets it to midnight. Therefore, it's possible to clear any set time in a QDateTime by setting it to a default QTime:


  QDateTime dt = QDateTime::currentDateTime();
  dt.setTime(QTime());



See also time(), setDate(), and setTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn setTime_0<RetType, T: QDateTime_setTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTime_0(self);
    // return 1;
  }
}
pub trait QDateTime_setTime_0<RetType> {
  fn setTime_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setTime_0<(/*void*/)> for (usize) {
  fn setTime_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime7setTimeERK5QTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:298
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTimeSpec(Qt::TimeSpec)

/*
Sets the time specification used in this datetime to spec. The datetime will refer to a different point in time.

If spec is Qt::OffsetFromUTC then the timeSpec() will be set to Qt::UTC, i.e. an effective offset of 0.

If spec is Qt::TimeZone then the spec will be set to Qt::LocalTime, i.e. the current system time zone.

Example:


  QDateTime local(QDateTime::currentDateTime());
  qDebug() << "Local time is:" << local;

  QDateTime UTC(local);
  UTC.setTimeSpec(Qt::UTC);
  qDebug() << "UTC time is:" << UTC;

  qDebug() << "There are" << local.secsTo(UTC) << "seconds difference between the datetimes.";



See also timeSpec(), setDate(), setTime(), setTimeZone(), and Qt::TimeSpec.
*/
impl /*struct*/ QDateTime {
  pub fn setTimeSpec_0<RetType, T: QDateTime_setTimeSpec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTimeSpec_0(self);
    // return 1;
  }
}
pub trait QDateTime_setTimeSpec_0<RetType> {
  fn setTimeSpec_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setTimeSpec_0<(/*void*/)> for (i32) {
  fn setTimeSpec_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime11setTimeSpecEN2Qt8TimeSpecE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:299
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOffsetFromUtc(int)

/*
Sets the timeSpec() to Qt::OffsetFromUTC and the offset to offsetSeconds. The datetime will refer to a different point in time.

The maximum and minimum offset is 14 positive or negative hours. If offsetSeconds is larger or smaller than that, then the result is undefined.

If offsetSeconds is 0 then the timeSpec() will be set to Qt::UTC.

This function was introduced in  Qt 5.2.

See also isValid() and offsetFromUtc().
*/
impl /*struct*/ QDateTime {
  pub fn setOffsetFromUtc_0<RetType, T: QDateTime_setOffsetFromUtc_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffsetFromUtc_0(self);
    // return 1;
  }
}
pub trait QDateTime_setOffsetFromUtc_0<RetType> {
  fn setOffsetFromUtc_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setOffsetFromUtc_0<(/*void*/)> for (i32) {
  fn setOffsetFromUtc_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime16setOffsetFromUtcEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:301
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTimeZone(const QTimeZone &)

/*
Sets the time zone used in this datetime to toZone. The datetime will refer to a different point in time.

If toZone is invalid then the datetime will be invalid.

This function was introduced in  Qt 5.2.

See also timeZone() and Qt::TimeSpec.
*/
impl /*struct*/ QDateTime {
  pub fn setTimeZone_0<RetType, T: QDateTime_setTimeZone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTimeZone_0(self);
    // return 1;
  }
}
pub trait QDateTime_setTimeZone_0<RetType> {
  fn setTimeZone_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setTimeZone_0<(/*void*/)> for (usize) {
  fn setTimeZone_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime11setTimeZoneERK9QTimeZone", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:303
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMSecsSinceEpoch(qint64)

/*
Sets the date and time given the number of milliseconds msecs that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC). On systems that do not support time zones this function will behave as if local time were Qt::UTC.

Note that passing the minimum of qint64 (std::numeric_limits<qint64>::min()) to msecs will result in undefined behavior.

This function was introduced in  Qt 4.7.

See also toMSecsSinceEpoch() and setSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn setMSecsSinceEpoch_0<RetType, T: QDateTime_setMSecsSinceEpoch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMSecsSinceEpoch_0(self);
    // return 1;
  }
}
pub trait QDateTime_setMSecsSinceEpoch_0<RetType> {
  fn setMSecsSinceEpoch_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setMSecsSinceEpoch_0<(/*void*/)> for (i64) {
  fn setMSecsSinceEpoch_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime18setMSecsSinceEpochEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:304
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSecsSinceEpoch(qint64)

/*
Sets the date and time given the number of seconds secs that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC). On systems that do not support time zones this function will behave as if local time were Qt::UTC.

This function was introduced in  Qt 5.8.

See also toSecsSinceEpoch() and setMSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn setSecsSinceEpoch_0<RetType, T: QDateTime_setSecsSinceEpoch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSecsSinceEpoch_0(self);
    // return 1;
  }
}
pub trait QDateTime_setSecsSinceEpoch_0<RetType> {
  fn setSecsSinceEpoch_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setSecsSinceEpoch_0<(/*void*/)> for (i64) {
  fn setSecsSinceEpoch_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime17setSecsSinceEpochEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:307
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString(Qt::DateFormat) const

/*
Returns the datetime as a string. The format parameter determines the format of the result string.

These expressions may be used for the date:


 ExpressionOutput
dthe day as number without a leading zero (1 to 31)
ddthe day as number with a leading zero (01 to 31)
dddthe abbreviated localized day name (e.g. 'Mon' to 'Sun'). Uses the system locale to localize the name, i.e. QLocale::system().
ddddthe long localized day name (e.g. 'Monday' to 'Sunday'). Uses the system locale to localize the name, i.e. QLocale::system().
Mthe month as number without a leading zero (1-12)
MMthe month as number with a leading zero (01-12)
MMMthe abbreviated localized month name (e.g. 'Jan' to 'Dec'). Uses the system locale to localize the name, i.e. QLocale::system().
MMMMthe long localized month name (e.g. 'January' to 'December'). Uses the system locale to localize the name, i.e. QLocale::system().
yythe year as two digit number (00-99)
yyyythe year as four digit number


These expressions may be used for the time:


 ExpressionOutput
hthe hour without a leading zero (0 to 23 or 1 to 12 if AM/PM display)
hhthe hour with a leading zero (00 to 23 or 01 to 12 if AM/PM display)
Hthe hour without a leading zero (0 to 23, even with AM/PM display)
HHthe hour with a leading zero (00 to 23, even with AM/PM display)
mthe minute without a leading zero (0 to 59)
mmthe minute with a leading zero (00 to 59)
sthe whole second without a leading zero (0 to 59)
ssthe whole second with a leading zero where applicable (00 to 59)
zthe fractional part of the second, to go after a decimal point, without trailing zeroes (0 to 999). Thus "s.z" reports the seconds to full available (millisecond) precision without trailing zeroes.
zzzthe fractional part of the second, to millisecond precision, including trailing zeroes where applicable (000 to 999).
AP or Ause AM/PM display. A/AP will be replaced by either "AM" or "PM".
ap or ause am/pm display. a/ap will be replaced by either "am" or "pm".
tthe timezone (for example "CEST")


All other input characters will be ignored. Any sequence of characters that are enclosed in single quotes will be treated as text and not be used as an expression. Two consecutive single quotes ("''") are replaced by a singlequote in the output. Formats without separators (e.g. "HHmm") are currently not supported.

Example format strings (assumed that the QDateTime is 21 May 2001 14:13:09.120):


 FormatResult
dd.MM.yyyy21.05.2001
ddd MMMM d yyTue May 21 01
hh:mm:ss.zzz14:13:09.120
hh:mm:ss.z14:13:09.12
h:m:s ap2:13:9 pm


If the datetime is invalid, an empty string will be returned.

See also fromString(), QDate::toString(), QTime::toString(), and QLocale::toString().
*/
impl /*struct*/ QDateTime {
  pub fn toString_0<RetType, T: QDateTime_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QDateTime_toString_0<RetType> {
  fn toString_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toString_0<usize> for (i32) {
  fn toString_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime8toStringEN2Qt10DateFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:309
// index:1
// Public Visibility=Default Availability=Available
// [8] QString toString(const QString &) const

/*
Returns the datetime as a string. The format parameter determines the format of the result string.

These expressions may be used for the date:


 ExpressionOutput
dthe day as number without a leading zero (1 to 31)
ddthe day as number with a leading zero (01 to 31)
dddthe abbreviated localized day name (e.g. 'Mon' to 'Sun'). Uses the system locale to localize the name, i.e. QLocale::system().
ddddthe long localized day name (e.g. 'Monday' to 'Sunday'). Uses the system locale to localize the name, i.e. QLocale::system().
Mthe month as number without a leading zero (1-12)
MMthe month as number with a leading zero (01-12)
MMMthe abbreviated localized month name (e.g. 'Jan' to 'Dec'). Uses the system locale to localize the name, i.e. QLocale::system().
MMMMthe long localized month name (e.g. 'January' to 'December'). Uses the system locale to localize the name, i.e. QLocale::system().
yythe year as two digit number (00-99)
yyyythe year as four digit number


These expressions may be used for the time:


 ExpressionOutput
hthe hour without a leading zero (0 to 23 or 1 to 12 if AM/PM display)
hhthe hour with a leading zero (00 to 23 or 01 to 12 if AM/PM display)
Hthe hour without a leading zero (0 to 23, even with AM/PM display)
HHthe hour with a leading zero (00 to 23, even with AM/PM display)
mthe minute without a leading zero (0 to 59)
mmthe minute with a leading zero (00 to 59)
sthe whole second without a leading zero (0 to 59)
ssthe whole second with a leading zero where applicable (00 to 59)
zthe fractional part of the second, to go after a decimal point, without trailing zeroes (0 to 999). Thus "s.z" reports the seconds to full available (millisecond) precision without trailing zeroes.
zzzthe fractional part of the second, to millisecond precision, including trailing zeroes where applicable (000 to 999).
AP or Ause AM/PM display. A/AP will be replaced by either "AM" or "PM".
ap or ause am/pm display. a/ap will be replaced by either "am" or "pm".
tthe timezone (for example "CEST")


All other input characters will be ignored. Any sequence of characters that are enclosed in single quotes will be treated as text and not be used as an expression. Two consecutive single quotes ("''") are replaced by a singlequote in the output. Formats without separators (e.g. "HHmm") are currently not supported.

Example format strings (assumed that the QDateTime is 21 May 2001 14:13:09.120):


 FormatResult
dd.MM.yyyy21.05.2001
ddd MMMM d yyTue May 21 01
hh:mm:ss.zzz14:13:09.120
hh:mm:ss.z14:13:09.12
h:m:s ap2:13:9 pm


If the datetime is invalid, an empty string will be returned.

See also fromString(), QDate::toString(), QTime::toString(), and QLocale::toString().
*/
impl /*struct*/ QDateTime {
  pub fn toString_1<RetType, T: QDateTime_toString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_1(self);
    // return 1;
  }
}
pub trait QDateTime_toString_1<RetType> {
  fn toString_1(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toString_1<usize> for (usize) {
  fn toString_1(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime8toStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:311
// index:2
// Public Visibility=Default Availability=Available
// [8] QString toString(QStringView) const

/*
Returns the datetime as a string. The format parameter determines the format of the result string.

These expressions may be used for the date:


 ExpressionOutput
dthe day as number without a leading zero (1 to 31)
ddthe day as number with a leading zero (01 to 31)
dddthe abbreviated localized day name (e.g. 'Mon' to 'Sun'). Uses the system locale to localize the name, i.e. QLocale::system().
ddddthe long localized day name (e.g. 'Monday' to 'Sunday'). Uses the system locale to localize the name, i.e. QLocale::system().
Mthe month as number without a leading zero (1-12)
MMthe month as number with a leading zero (01-12)
MMMthe abbreviated localized month name (e.g. 'Jan' to 'Dec'). Uses the system locale to localize the name, i.e. QLocale::system().
MMMMthe long localized month name (e.g. 'January' to 'December'). Uses the system locale to localize the name, i.e. QLocale::system().
yythe year as two digit number (00-99)
yyyythe year as four digit number


These expressions may be used for the time:


 ExpressionOutput
hthe hour without a leading zero (0 to 23 or 1 to 12 if AM/PM display)
hhthe hour with a leading zero (00 to 23 or 01 to 12 if AM/PM display)
Hthe hour without a leading zero (0 to 23, even with AM/PM display)
HHthe hour with a leading zero (00 to 23, even with AM/PM display)
mthe minute without a leading zero (0 to 59)
mmthe minute with a leading zero (00 to 59)
sthe whole second without a leading zero (0 to 59)
ssthe whole second with a leading zero where applicable (00 to 59)
zthe fractional part of the second, to go after a decimal point, without trailing zeroes (0 to 999). Thus "s.z" reports the seconds to full available (millisecond) precision without trailing zeroes.
zzzthe fractional part of the second, to millisecond precision, including trailing zeroes where applicable (000 to 999).
AP or Ause AM/PM display. A/AP will be replaced by either "AM" or "PM".
ap or ause am/pm display. a/ap will be replaced by either "am" or "pm".
tthe timezone (for example "CEST")


All other input characters will be ignored. Any sequence of characters that are enclosed in single quotes will be treated as text and not be used as an expression. Two consecutive single quotes ("''") are replaced by a singlequote in the output. Formats without separators (e.g. "HHmm") are currently not supported.

Example format strings (assumed that the QDateTime is 21 May 2001 14:13:09.120):


 FormatResult
dd.MM.yyyy21.05.2001
ddd MMMM d yyTue May 21 01
hh:mm:ss.zzz14:13:09.120
hh:mm:ss.z14:13:09.12
h:m:s ap2:13:9 pm


If the datetime is invalid, an empty string will be returned.

See also fromString(), QDate::toString(), QTime::toString(), and QLocale::toString().
*/
impl /*struct*/ QDateTime {
  pub fn toString_2<RetType, T: QDateTime_toString_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_2(self);
    // return 1;
  }
}
pub trait QDateTime_toString_2<RetType> {
  fn toString_2(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toString_2<usize> for (usize) {
  fn toString_2(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime8toStringE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:313
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime addDays(qint64) const

/*
Returns a QDateTime object containing a datetime ndays days later than the datetime of this object (or earlier if ndays is negative).

If the timeSpec() is Qt::LocalTime and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be adjusted accordingly, i.e. if the transition is at 2am and the clock goes forward to 3am and the result falls between 2am and 3am then the result will be adjusted to fall after 3am.

See also daysTo(), addMonths(), addYears(), and addSecs().
*/
impl /*struct*/ QDateTime {
  pub fn addDays_0<RetType, T: QDateTime_addDays_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addDays_0(self);
    // return 1;
  }
}
pub trait QDateTime_addDays_0<RetType> {
  fn addDays_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_addDays_0<usize> for (i64) {
  fn addDays_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime7addDaysEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:314
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime addMonths(int) const

/*
Returns a QDateTime object containing a datetime nmonths months later than the datetime of this object (or earlier if nmonths is negative).

If the timeSpec() is Qt::LocalTime and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be adjusted accordingly, i.e. if the transition is at 2am and the clock goes forward to 3am and the result falls between 2am and 3am then the result will be adjusted to fall after 3am.

See also daysTo(), addDays(), addYears(), and addSecs().
*/
impl /*struct*/ QDateTime {
  pub fn addMonths_0<RetType, T: QDateTime_addMonths_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMonths_0(self);
    // return 1;
  }
}
pub trait QDateTime_addMonths_0<RetType> {
  fn addMonths_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_addMonths_0<usize> for (i32) {
  fn addMonths_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime9addMonthsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:315
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime addYears(int) const

/*
Returns a QDateTime object containing a datetime nyears years later than the datetime of this object (or earlier if nyears is negative).

If the timeSpec() is Qt::LocalTime and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be adjusted accordingly, i.e. if the transition is at 2am and the clock goes forward to 3am and the result falls between 2am and 3am then the result will be adjusted to fall after 3am.

See also daysTo(), addDays(), addMonths(), and addSecs().
*/
impl /*struct*/ QDateTime {
  pub fn addYears_0<RetType, T: QDateTime_addYears_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addYears_0(self);
    // return 1;
  }
}
pub trait QDateTime_addYears_0<RetType> {
  fn addYears_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_addYears_0<usize> for (i32) {
  fn addYears_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime8addYearsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:316
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime addSecs(qint64) const

/*
Returns a QDateTime object containing a datetime s seconds later than the datetime of this object (or earlier if s is negative).

If this datetime is invalid, an invalid datetime will be returned.

See also addMSecs(), secsTo(), addDays(), addMonths(), and addYears().
*/
impl /*struct*/ QDateTime {
  pub fn addSecs_0<RetType, T: QDateTime_addSecs_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSecs_0(self);
    // return 1;
  }
}
pub trait QDateTime_addSecs_0<RetType> {
  fn addSecs_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_addSecs_0<usize> for (i64) {
  fn addSecs_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime7addSecsEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:317
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime addMSecs(qint64) const

/*
Returns a QDateTime object containing a datetime msecs miliseconds later than the datetime of this object (or earlier if msecs is negative).

If this datetime is invalid, an invalid datetime will be returned.

See also addSecs(), msecsTo(), addDays(), addMonths(), and addYears().
*/
impl /*struct*/ QDateTime {
  pub fn addMSecs_0<RetType, T: QDateTime_addMSecs_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMSecs_0(self);
    // return 1;
  }
}
pub trait QDateTime_addMSecs_0<RetType> {
  fn addMSecs_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_addMSecs_0<usize> for (i64) {
  fn addMSecs_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime8addMSecsEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:319
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime toTimeSpec(Qt::TimeSpec) const

/*
Returns a copy of this datetime converted to the given time spec.

If spec is Qt::OffsetFromUTC then it is set to Qt::UTC. To set to a spec of Qt::OffsetFromUTC use toOffsetFromUtc().

If spec is Qt::TimeZone then it is set to Qt::LocalTime, i.e. the local Time Zone.

Example:


  QDateTime local(QDateTime::currentDateTime());
  QDateTime UTC(local.toTimeSpec(Qt::UTC));
  qDebug() << "Local time is:" << local;
  qDebug() << "UTC time is:" << UTC;
  qDebug() << "No difference between times:" << local.secsTo(UTC);



See also timeSpec(), toTimeZone(), toUTC(), and toLocalTime().
*/
impl /*struct*/ QDateTime {
  pub fn toTimeSpec_0<RetType, T: QDateTime_toTimeSpec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTimeSpec_0(self);
    // return 1;
  }
}
pub trait QDateTime_toTimeSpec_0<RetType> {
  fn toTimeSpec_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toTimeSpec_0<usize> for (i32) {
  fn toTimeSpec_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime10toTimeSpecEN2Qt8TimeSpecE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:320
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QDateTime toLocalTime() const

/*
Returns a datetime containing the date and time information in this datetime, but specified using the Qt::LocalTime definition.

Example:


  QDateTime UTC(QDateTime::currentDateTimeUtc());
  QDateTime local(UTC.toLocalTime());
  qDebug() << "UTC time is:" << UTC;
  qDebug() << "Local time is:" << local;
  qDebug() << "No difference between times:" << UTC.secsTo(local);



See also toTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn toLocalTime_0<RetType, T: QDateTime_toLocalTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLocalTime_0(self);
    // return 1;
  }
}
pub trait QDateTime_toLocalTime_0<RetType> {
  fn toLocalTime_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toLocalTime_0<usize> for () {
  fn toLocalTime_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime11toLocalTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:321
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QDateTime toUTC() const

/*
Returns a datetime containing the date and time information in this datetime, but specified using the Qt::UTC definition.

Example:


  QDateTime local(QDateTime::currentDateTime());
  QDateTime UTC(local.toUTC());
  qDebug() << "Local time is:" << local;
  qDebug() << "UTC time is:" << UTC;
  qDebug() << "No difference between times:" << local.secsTo(UTC);



See also toTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn toUTC_0<RetType, T: QDateTime_toUTC_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUTC_0(self);
    // return 1;
  }
}
pub trait QDateTime_toUTC_0<RetType> {
  fn toUTC_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toUTC_0<usize> for () {
  fn toUTC_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime5toUTCEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:322
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime toOffsetFromUtc(int) const

/*
Returns a copy of this datetime converted to a spec of Qt::OffsetFromUTC with the given offsetSeconds.

If the offsetSeconds equals 0 then a UTC datetime will be returned

This function was introduced in  Qt 5.2.

See also setOffsetFromUtc(), offsetFromUtc(), and toTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn toOffsetFromUtc_0<RetType, T: QDateTime_toOffsetFromUtc_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toOffsetFromUtc_0(self);
    // return 1;
  }
}
pub trait QDateTime_toOffsetFromUtc_0<RetType> {
  fn toOffsetFromUtc_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toOffsetFromUtc_0<usize> for (i32) {
  fn toOffsetFromUtc_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime15toOffsetFromUtcEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:324
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime toTimeZone(const QTimeZone &) const

/*
Returns a copy of this datetime converted to the given timeZone

This function was introduced in  Qt 5.2.

See also timeZone() and toTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn toTimeZone_0<RetType, T: QDateTime_toTimeZone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTimeZone_0(self);
    // return 1;
  }
}
pub trait QDateTime_toTimeZone_0<RetType> {
  fn toTimeZone_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toTimeZone_0<usize> for (usize) {
  fn toTimeZone_0(self , rsthis: & QDateTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime10toTimeZoneERK9QTimeZone", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:327
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 daysTo(const QDateTime &) const

/*
Returns the number of days from this datetime to the other datetime. The number of days is counted as the number of times midnight is reached between this datetime to the other datetime. This means that a 10 minute difference from 23:55 to 0:05 the next day counts as one day.

If the other datetime is earlier than this datetime, the value returned is negative.

Example:


  QDateTime startDate(QDate(2012, 7, 6), QTime(8, 30, 0));
  QDateTime endDate(QDate(2012, 7, 7), QTime(16, 30, 0));
  qDebug() << "Days from startDate to endDate: " << startDate.daysTo(endDate);

  startDate = QDateTime(QDate(2012, 7, 6), QTime(23, 55, 0));
  endDate = QDateTime(QDate(2012, 7, 7), QTime(0, 5, 0));
  qDebug() << "Days from startDate to endDate: " << startDate.daysTo(endDate);

  qSwap(startDate, endDate); // Make endDate before startDate.
  qDebug() << "Days from startDate to endDate: " << startDate.daysTo(endDate);



See also addDays(), secsTo(), and msecsTo().
*/
impl /*struct*/ QDateTime {
  pub fn daysTo_0<RetType, T: QDateTime_daysTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.daysTo_0(self);
    // return 1;
  }
}
pub trait QDateTime_daysTo_0<RetType> {
  fn daysTo_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_daysTo_0<i64> for (usize) {
  fn daysTo_0(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime6daysToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:328
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 secsTo(const QDateTime &) const

/*
Returns the number of seconds from this datetime to the other datetime. If the other datetime is earlier than this datetime, the value returned is negative.

Before performing the comparison, the two datetimes are converted to Qt::UTC to ensure that the result is correct if daylight-saving (DST) applies to one of the two datetimes but not the other.

Returns 0 if either datetime is invalid.

Example:


  QDateTime now = QDateTime::currentDateTime();
  QDateTime xmas(QDate(now.date().year(), 12, 25), QTime(0, 0));
  qDebug("There are %d seconds to Christmas", now.secsTo(xmas));



See also addSecs(), daysTo(), and QTime::secsTo().
*/
impl /*struct*/ QDateTime {
  pub fn secsTo_0<RetType, T: QDateTime_secsTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.secsTo_0(self);
    // return 1;
  }
}
pub trait QDateTime_secsTo_0<RetType> {
  fn secsTo_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_secsTo_0<i64> for (usize) {
  fn secsTo_0(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime6secsToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:329
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 msecsTo(const QDateTime &) const

/*
Returns the number of milliseconds from this datetime to the other datetime. If the other datetime is earlier than this datetime, the value returned is negative.

Before performing the comparison, the two datetimes are converted to Qt::UTC to ensure that the result is correct if daylight-saving (DST) applies to one of the two datetimes and but not the other.

Returns 0 if either datetime is invalid.

See also addMSecs(), daysTo(), and QTime::msecsTo().
*/
impl /*struct*/ QDateTime {
  pub fn msecsTo_0<RetType, T: QDateTime_msecsTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.msecsTo_0(self);
    // return 1;
  }
}
pub trait QDateTime_msecsTo_0<RetType> {
  fn msecsTo_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_msecsTo_0<i64> for (usize) {
  fn msecsTo_0(self , rsthis: & QDateTime) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime7msecsToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:331
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QDateTime &) const

/*

*/
impl /*struct*/ QDateTime {
  pub fn operator_equal_equal_0<RetType, T: QDateTime_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QDateTime_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTimeeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:332
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QDateTime &) const

/*

*/
impl /*struct*/ QDateTime {
  pub fn operator_not_equal_0<RetType, T: QDateTime_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QDateTime_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTimeneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:333
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(const QDateTime &) const

/*

*/
impl /*struct*/ QDateTime {
  pub fn operator_less_than_0<RetType, T: QDateTime_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QDateTime_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTimeltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:334
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QDateTime &) const

/*

*/
impl /*struct*/ QDateTime {
  pub fn operator_less_than_equal_0<RetType, T: QDateTime_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QDateTime_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTimeleERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:335
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QDateTime &) const

/*

*/
impl /*struct*/ QDateTime {
  pub fn operator_greater_than_0<RetType, T: QDateTime_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QDateTime_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTimegtERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:336
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QDateTime &) const

/*

*/
impl /*struct*/ QDateTime {
  pub fn operator_greater_than_equal_0<RetType, T: QDateTime_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QDateTime_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QDateTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTimegeERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:339
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUtcOffset(int)

/*

*/
impl /*struct*/ QDateTime {
  pub fn setUtcOffset_0<RetType, T: QDateTime_setUtcOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUtcOffset_0(self);
    // return 1;
  }
}
pub trait QDateTime_setUtcOffset_0<RetType> {
  fn setUtcOffset_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setUtcOffset_0<(/*void*/)> for (i32) {
  fn setUtcOffset_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime12setUtcOffsetEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:340
// index:0
// Public Visibility=Default Availability=Available
// [4] int utcOffset() const

/*

*/
impl /*struct*/ QDateTime {
  pub fn utcOffset_0<RetType, T: QDateTime_utcOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.utcOffset_0(self);
    // return 1;
  }
}
pub trait QDateTime_utcOffset_0<RetType> {
  fn utcOffset_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_utcOffset_0<i32> for () {
  fn utcOffset_0(self , rsthis: & QDateTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime9utcOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:343
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDateTime currentDateTime()

/*
Returns the current datetime, as reported by the system clock, in the local time zone.

See also currentDateTimeUtc(), QDate::currentDate(), QTime::currentTime(), and toTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn currentDateTime_0<RetType, T: QDateTime_currentDateTime_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentDateTime_0();
    // return 1;
  }
}
pub trait QDateTime_currentDateTime_0<RetType> {
  fn currentDateTime_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_currentDateTime_0<usize> for () {
  fn currentDateTime_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime15currentDateTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:344
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDateTime currentDateTimeUtc()

/*
Returns the current datetime, as reported by the system clock, in UTC.

This function was introduced in  Qt 4.7.

See also currentDateTime(), QDate::currentDate(), QTime::currentTime(), and toTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn currentDateTimeUtc_0<RetType, T: QDateTime_currentDateTimeUtc_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentDateTimeUtc_0();
    // return 1;
  }
}
pub trait QDateTime_currentDateTimeUtc_0<RetType> {
  fn currentDateTimeUtc_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_currentDateTimeUtc_0<usize> for () {
  fn currentDateTimeUtc_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime18currentDateTimeUtcEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:346
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromString(const QString &, Qt::DateFormat)

/*
Returns the QDateTime represented by the string, using the format given, or an invalid datetime if this is not possible.

Note for Qt::TextDate: It is recommended that you use the English short month names (e.g. "Jan"). Although localized month names can also be used, they depend on the user's locale settings.

See also toString() and QLocale::toDateTime().
*/
impl /*struct*/ QDateTime {
  pub fn fromString_0<RetType, T: QDateTime_fromString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_0();
    // return 1;
  }
}
pub trait QDateTime_fromString_0<RetType> {
  fn fromString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromString_0<usize> for (usize,i32) {
  fn fromString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime10fromStringERK7QStringN2Qt10DateFormatE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:347
// index:1
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromString(const QString &, const QString &)

/*
Returns the QDateTime represented by the string, using the format given, or an invalid datetime if this is not possible.

Note for Qt::TextDate: It is recommended that you use the English short month names (e.g. "Jan"). Although localized month names can also be used, they depend on the user's locale settings.

See also toString() and QLocale::toDateTime().
*/
impl /*struct*/ QDateTime {
  pub fn fromString_1<RetType, T: QDateTime_fromString_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_1();
    // return 1;
  }
}
pub trait QDateTime_fromString_1<RetType> {
  fn fromString_1(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromString_1<usize> for (usize,usize) {
  fn fromString_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime10fromStringERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:351
// index:0
// Public Visibility=Default Availability=Available
// [4] uint toTime_t() const

/*

*/
impl /*struct*/ QDateTime {
  pub fn toTime_t_0<RetType, T: QDateTime_toTime_t_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTime_t_0(self);
    // return 1;
  }
}
pub trait QDateTime_toTime_t_0<RetType> {
  fn toTime_t_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_toTime_t_0<u32> for () {
  fn toTime_t_0(self , rsthis: & QDateTime) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateTime8toTime_tEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:352
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTime_t(uint)

/*

*/
impl /*struct*/ QDateTime {
  pub fn setTime_t_0<RetType, T: QDateTime_setTime_t_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTime_t_0(self);
    // return 1;
  }
}
pub trait QDateTime_setTime_t_0<RetType> {
  fn setTime_t_0(self , rsthis: & QDateTime) -> RetType;
}
impl<'a> /*trait*/ QDateTime_setTime_t_0<(/*void*/)> for (u32) {
  fn setTime_t_0(self , rsthis: & QDateTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateTime9setTime_tEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:353
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromTime_t(uint)

/*

*/
impl /*struct*/ QDateTime {
  pub fn fromTime_t_0<RetType, T: QDateTime_fromTime_t_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTime_t_0();
    // return 1;
  }
}
pub trait QDateTime_fromTime_t_0<RetType> {
  fn fromTime_t_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromTime_t_0<usize> for (u32) {
  fn fromTime_t_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime10fromTime_tEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:354
// index:1
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromTime_t(uint, Qt::TimeSpec, int)

/*

*/
impl /*struct*/ QDateTime {
  pub fn fromTime_t_1<RetType, T: QDateTime_fromTime_t_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTime_t_1();
    // return 1;
  }
}
pub trait QDateTime_fromTime_t_1<RetType> {
  fn fromTime_t_1(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromTime_t_1<usize> for (u32,i32,i32) {
  fn fromTime_t_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime10fromTime_tEjN2Qt8TimeSpecEi", 3,qtrt::FFITY_UINT32,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:356
// index:2
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromTime_t(uint, const QTimeZone &)

/*

*/
impl /*struct*/ QDateTime {
  pub fn fromTime_t_2<RetType, T: QDateTime_fromTime_t_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTime_t_2();
    // return 1;
  }
}
pub trait QDateTime_fromTime_t_2<RetType> {
  fn fromTime_t_2(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromTime_t_2<usize> for (u32,usize) {
  fn fromTime_t_2(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime10fromTime_tEjRK9QTimeZone", 2,qtrt::FFITY_UINT32,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:359
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromMSecsSinceEpoch(qint64)

/*
Returns a datetime whose date and time are the number of milliseconds, msecs, that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC), and converted to Qt::LocalTime. On systems that do not support time zones, the time will be set as if local time were Qt::UTC.

Note that there are possible values for msecs that lie outside the valid range of QDateTime, both negative and positive. The behavior of this function is undefined for those values.

This function was introduced in  Qt 4.7.

See also toMSecsSinceEpoch() and setMSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn fromMSecsSinceEpoch_0<RetType, T: QDateTime_fromMSecsSinceEpoch_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromMSecsSinceEpoch_0();
    // return 1;
  }
}
pub trait QDateTime_fromMSecsSinceEpoch_0<RetType> {
  fn fromMSecsSinceEpoch_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch_0<usize> for (i64) {
  fn fromMSecsSinceEpoch_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime19fromMSecsSinceEpochEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:361
// index:1
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromMSecsSinceEpoch(qint64, Qt::TimeSpec, int)

/*
Returns a datetime whose date and time are the number of milliseconds, msecs, that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC), and converted to Qt::LocalTime. On systems that do not support time zones, the time will be set as if local time were Qt::UTC.

Note that there are possible values for msecs that lie outside the valid range of QDateTime, both negative and positive. The behavior of this function is undefined for those values.

This function was introduced in  Qt 4.7.

See also toMSecsSinceEpoch() and setMSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn fromMSecsSinceEpoch_1<RetType, T: QDateTime_fromMSecsSinceEpoch_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromMSecsSinceEpoch_1();
    // return 1;
  }
}
pub trait QDateTime_fromMSecsSinceEpoch_1<RetType> {
  fn fromMSecsSinceEpoch_1(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch_1<usize> for (i64,i32,i32) {
  fn fromMSecsSinceEpoch_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime19fromMSecsSinceEpochExN2Qt8TimeSpecEi", 3,qtrt::FFITY_SINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:365
// index:2
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromMSecsSinceEpoch(qint64, const QTimeZone &)

/*
Returns a datetime whose date and time are the number of milliseconds, msecs, that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC), and converted to Qt::LocalTime. On systems that do not support time zones, the time will be set as if local time were Qt::UTC.

Note that there are possible values for msecs that lie outside the valid range of QDateTime, both negative and positive. The behavior of this function is undefined for those values.

This function was introduced in  Qt 4.7.

See also toMSecsSinceEpoch() and setMSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn fromMSecsSinceEpoch_2<RetType, T: QDateTime_fromMSecsSinceEpoch_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromMSecsSinceEpoch_2();
    // return 1;
  }
}
pub trait QDateTime_fromMSecsSinceEpoch_2<RetType> {
  fn fromMSecsSinceEpoch_2(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromMSecsSinceEpoch_2<usize> for (i64,usize) {
  fn fromMSecsSinceEpoch_2(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime19fromMSecsSinceEpochExRK9QTimeZone", 2,qtrt::FFITY_SINT64,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:362
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromSecsSinceEpoch(qint64, Qt::TimeSpec, int)

/*
Returns a datetime whose date and time are the number of seconds secs that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC) and converted to the given spec.

Note that there are possible values for secs that lie outside the valid range of QDateTime, both negative and positive. The behavior of this function is undefined for those values.

If the spec is not Qt::OffsetFromUTC then the offsetSeconds will be ignored. If the spec is Qt::OffsetFromUTC and the offsetSeconds is 0 then the spec will be set to Qt::UTC, i.e. an offset of 0 seconds.

If spec is Qt::TimeZone then the spec will be set to Qt::LocalTime, i.e. the current system time zone.

This function was introduced in  Qt 5.8.

See also toSecsSinceEpoch() and setSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn fromSecsSinceEpoch_0<RetType, T: QDateTime_fromSecsSinceEpoch_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromSecsSinceEpoch_0();
    // return 1;
  }
}
pub trait QDateTime_fromSecsSinceEpoch_0<RetType> {
  fn fromSecsSinceEpoch_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromSecsSinceEpoch_0<usize> for (i64,i32,i32) {
  fn fromSecsSinceEpoch_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime18fromSecsSinceEpochExN2Qt8TimeSpecEi", 3,qtrt::FFITY_SINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:366
// index:1
// Public static Visibility=Default Availability=Available
// [8] QDateTime fromSecsSinceEpoch(qint64, const QTimeZone &)

/*
Returns a datetime whose date and time are the number of seconds secs that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC) and converted to the given spec.

Note that there are possible values for secs that lie outside the valid range of QDateTime, both negative and positive. The behavior of this function is undefined for those values.

If the spec is not Qt::OffsetFromUTC then the offsetSeconds will be ignored. If the spec is Qt::OffsetFromUTC and the offsetSeconds is 0 then the spec will be set to Qt::UTC, i.e. an offset of 0 seconds.

If spec is Qt::TimeZone then the spec will be set to Qt::LocalTime, i.e. the current system time zone.

This function was introduced in  Qt 5.8.

See also toSecsSinceEpoch() and setSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn fromSecsSinceEpoch_1<RetType, T: QDateTime_fromSecsSinceEpoch_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromSecsSinceEpoch_1();
    // return 1;
  }
}
pub trait QDateTime_fromSecsSinceEpoch_1<RetType> {
  fn fromSecsSinceEpoch_1(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_fromSecsSinceEpoch_1<usize> for (i64,usize) {
  fn fromSecsSinceEpoch_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime18fromSecsSinceEpochExRK9QTimeZone", 2,qtrt::FFITY_SINT64,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:369
// index:0
// Public static Visibility=Default Availability=Available
// [8] qint64 currentMSecsSinceEpoch()

/*
Returns the number of milliseconds since 1970-01-01T00:00:00 Universal Coordinated Time. This number is like the POSIX time_t variable, but expressed in milliseconds instead.

This function was introduced in  Qt 4.7.

See also currentDateTime(), currentDateTimeUtc(), toTime_t(), and toTimeSpec().
*/
impl /*struct*/ QDateTime {
  pub fn currentMSecsSinceEpoch_0<RetType, T: QDateTime_currentMSecsSinceEpoch_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentMSecsSinceEpoch_0();
    // return 1;
  }
}
pub trait QDateTime_currentMSecsSinceEpoch_0<RetType> {
  fn currentMSecsSinceEpoch_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_currentMSecsSinceEpoch_0<i64> for () {
  fn currentMSecsSinceEpoch_0(self ) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime22currentMSecsSinceEpochEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:370
// index:0
// Public static Visibility=Default Availability=Available
// [8] qint64 currentSecsSinceEpoch()

/*
Returns the number of seconds since 1970-01-01T00:00:00 Universal Coordinated Time.

This function was introduced in  Qt 5.8.

See also currentMSecsSinceEpoch().
*/
impl /*struct*/ QDateTime {
  pub fn currentSecsSinceEpoch_0<RetType, T: QDateTime_currentSecsSinceEpoch_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentSecsSinceEpoch_0();
    // return 1;
  }
}
pub trait QDateTime_currentSecsSinceEpoch_0<RetType> {
  fn currentSecsSinceEpoch_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDateTime_currentSecsSinceEpoch_0<i64> for () {
  fn currentSecsSinceEpoch_0(self ) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDateTime21currentSecsSinceEpochEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
