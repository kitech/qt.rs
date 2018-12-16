

// mod ::core::QTime
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
// extern C begin: 39
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
#[derive(Default)] // class sizeof(QTime)=4
pub struct QTime {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTime_ITF interface {
//    QTime_PTR() *QTime
//}
//func (ptr *QTime) QTime_PTR() *QTime { return ptr }

impl /*struct*/ QTime {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTime {
    return QTime{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTime {
//  type Target = QTimeBASE;
//
//  fn deref(&self) -> &QTimeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTimeBASE> for QTime {
//  fn as_ref(& self) -> & QTimeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qdatetime.h:159
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTime()

/*

*/
// QTime() ctx.fn_proto_cpp
impl /*struct*/ QTime {
  pub fn QTime_0<T: QTime_QTime_0>(value: T) -> QTime {
    let rsthis = value.QTime_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTime_QTime_0 {
  fn QTime_0(self) -> QTime;
}
// QTime() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTime_QTime_0 for () {
  fn QTime_0(self) -> QTime {
    // unsafe{_ZN5QTimeC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QTimeC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:161
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTime(int, int, int, int)

/*

*/
// QTime(int, int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QTime {
  pub fn QTime_1<T: QTime_QTime_1>(value: T) -> QTime {
    let rsthis = value.QTime_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTime_QTime_1 {
  fn QTime_1(self) -> QTime;
}
// QTime(int, int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTime_QTime_1 for (i32,i32,i32,i32) {
  fn QTime_1(self) -> QTime {
    // unsafe{_ZN5QTimeC2Eiiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QTimeC2Eiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTime{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:163
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if both the date and the time are null; otherwise returns false. A null datetime is invalid.

See also QDate::isNull(), QTime::isNull(), and isValid().
*/
impl /*struct*/ QTime {
  pub fn isNull_0<RetType, T: QTime_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QTime_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if both the date and the time are valid and they are valid in the current Qt::TimeSpec, otherwise returns false.

If the timeSpec() is Qt::LocalTime or Qt::TimeZone then the date and time are checked to see if they fall in the Standard Time to Daylight-Saving Time transition hour, i.e. if the transition is at 2am and the clock goes forward to 3am then the time from 02:00:00 to 02:59:59.999 is considered to be invalid.

See also QDate::isValid() and QTime::isValid().
*/
impl /*struct*/ QTime {
  pub fn isValid_0<RetType, T: QTime_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTime_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:199
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool isValid(int, int, int, int)

/*
Returns true if both the date and the time are valid and they are valid in the current Qt::TimeSpec, otherwise returns false.

If the timeSpec() is Qt::LocalTime or Qt::TimeZone then the date and time are checked to see if they fall in the Standard Time to Daylight-Saving Time transition hour, i.e. if the transition is at 2am and the clock goes forward to 3am then the time from 02:00:00 to 02:59:59.999 is considered to be invalid.

See also QDate::isValid() and QTime::isValid().
*/
impl /*struct*/ QTime {
  pub fn isValid_1<RetType, T: QTime_isValid_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isValid_1();
    // return 1;
  }
}
pub trait QTime_isValid_1<RetType> {
  fn isValid_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTime_isValid_1<bool> for (i32,i32,i32,i32) {
  fn isValid_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QTime7isValidEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:166
// index:0
// Public Visibility=Default Availability=Available
// [4] int hour() const

/*

*/
impl /*struct*/ QTime {
  pub fn hour_0<RetType, T: QTime_hour_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hour_0(self);
    // return 1;
  }
}
pub trait QTime_hour_0<RetType> {
  fn hour_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_hour_0<i32> for () {
  fn hour_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime4hourEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:167
// index:0
// Public Visibility=Default Availability=Available
// [4] int minute() const

/*

*/
impl /*struct*/ QTime {
  pub fn minute_0<RetType, T: QTime_minute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minute_0(self);
    // return 1;
  }
}
pub trait QTime_minute_0<RetType> {
  fn minute_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_minute_0<i32> for () {
  fn minute_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime6minuteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:168
// index:0
// Public Visibility=Default Availability=Available
// [4] int second() const

/*

*/
impl /*struct*/ QTime {
  pub fn second_0<RetType, T: QTime_second_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.second_0(self);
    // return 1;
  }
}
pub trait QTime_second_0<RetType> {
  fn second_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_second_0<i32> for () {
  fn second_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime6secondEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:169
// index:0
// Public Visibility=Default Availability=Available
// [4] int msec() const

/*

*/
impl /*struct*/ QTime {
  pub fn msec_0<RetType, T: QTime_msec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.msec_0(self);
    // return 1;
  }
}
pub trait QTime_msec_0<RetType> {
  fn msec_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_msec_0<i32> for () {
  fn msec_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime4msecEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:171
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
impl /*struct*/ QTime {
  pub fn toString_0<RetType, T: QTime_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QTime_toString_0<RetType> {
  fn toString_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_toString_0<usize> for (i32) {
  fn toString_0(self , rsthis: & QTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime8toStringEN2Qt10DateFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:173
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
impl /*struct*/ QTime {
  pub fn toString_1<RetType, T: QTime_toString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_1(self);
    // return 1;
  }
}
pub trait QTime_toString_1<RetType> {
  fn toString_1(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_toString_1<usize> for (usize) {
  fn toString_1(self , rsthis: & QTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime8toStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:175
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
impl /*struct*/ QTime {
  pub fn toString_2<RetType, T: QTime_toString_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_2(self);
    // return 1;
  }
}
pub trait QTime_toString_2<RetType> {
  fn toString_2(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_toString_2<usize> for (usize) {
  fn toString_2(self , rsthis: & QTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime8toStringE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:177
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setHMS(int, int, int, int)

/*

*/
impl /*struct*/ QTime {
  pub fn setHMS_0<RetType, T: QTime_setHMS_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHMS_0(self);
    // return 1;
  }
}
pub trait QTime_setHMS_0<RetType> {
  fn setHMS_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_setHMS_0<bool> for (i32,i32,i32,i32) {
  fn setHMS_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QTime6setHMSEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:179
// index:0
// Public Visibility=Default Availability=Available
// [4] QTime addSecs(int) const

/*
Returns a QDateTime object containing a datetime s seconds later than the datetime of this object (or earlier if s is negative).

If this datetime is invalid, an invalid datetime will be returned.

See also addMSecs(), secsTo(), addDays(), addMonths(), and addYears().
*/
impl /*struct*/ QTime {
  pub fn addSecs_0<RetType, T: QTime_addSecs_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSecs_0(self);
    // return 1;
  }
}
pub trait QTime_addSecs_0<RetType> {
  fn addSecs_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_addSecs_0<usize> for (i32) {
  fn addSecs_0(self , rsthis: & QTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime7addSecsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:180
// index:0
// Public Visibility=Default Availability=Available
// [4] int secsTo(const QTime &) const

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
impl /*struct*/ QTime {
  pub fn secsTo_0<RetType, T: QTime_secsTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.secsTo_0(self);
    // return 1;
  }
}
pub trait QTime_secsTo_0<RetType> {
  fn secsTo_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_secsTo_0<i32> for (usize) {
  fn secsTo_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime6secsToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:181
// index:0
// Public Visibility=Default Availability=Available
// [4] QTime addMSecs(int) const

/*
Returns a QDateTime object containing a datetime msecs miliseconds later than the datetime of this object (or earlier if msecs is negative).

If this datetime is invalid, an invalid datetime will be returned.

See also addSecs(), msecsTo(), addDays(), addMonths(), and addYears().
*/
impl /*struct*/ QTime {
  pub fn addMSecs_0<RetType, T: QTime_addMSecs_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMSecs_0(self);
    // return 1;
  }
}
pub trait QTime_addMSecs_0<RetType> {
  fn addMSecs_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_addMSecs_0<usize> for (i32) {
  fn addMSecs_0(self , rsthis: & QTime) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime8addMSecsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:182
// index:0
// Public Visibility=Default Availability=Available
// [4] int msecsTo(const QTime &) const

/*
Returns the number of milliseconds from this datetime to the other datetime. If the other datetime is earlier than this datetime, the value returned is negative.

Before performing the comparison, the two datetimes are converted to Qt::UTC to ensure that the result is correct if daylight-saving (DST) applies to one of the two datetimes and but not the other.

Returns 0 if either datetime is invalid.

See also addMSecs(), daysTo(), and QTime::msecsTo().
*/
impl /*struct*/ QTime {
  pub fn msecsTo_0<RetType, T: QTime_msecsTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.msecsTo_0(self);
    // return 1;
  }
}
pub trait QTime_msecsTo_0<RetType> {
  fn msecsTo_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_msecsTo_0<i32> for (usize) {
  fn msecsTo_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime7msecsToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:184
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QTime &) const

/*

*/
impl /*struct*/ QTime {
  pub fn operator_equal_equal_0<RetType, T: QTime_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTime_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTimeeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:185
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QTime &) const

/*

*/
impl /*struct*/ QTime {
  pub fn operator_not_equal_0<RetType, T: QTime_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTime_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTimeneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:186
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QTime &) const

/*

*/
impl /*struct*/ QTime {
  pub fn operator_less_than_0<RetType, T: QTime_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QTime_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTimeltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:187
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QTime &) const

/*

*/
impl /*struct*/ QTime {
  pub fn operator_less_than_equal_0<RetType, T: QTime_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QTime_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTimeleERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:188
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QTime &) const

/*

*/
impl /*struct*/ QTime {
  pub fn operator_greater_than_0<RetType, T: QTime_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QTime_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTimegtERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:189
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QTime &) const

/*

*/
impl /*struct*/ QTime {
  pub fn operator_greater_than_equal_0<RetType, T: QTime_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QTime_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QTime) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTimegeERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:191
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] QTime fromMSecsSinceStartOfDay(int)

/*

*/
impl /*struct*/ QTime {
  pub fn fromMSecsSinceStartOfDay_0<RetType, T: QTime_fromMSecsSinceStartOfDay_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromMSecsSinceStartOfDay_0();
    // return 1;
  }
}
pub trait QTime_fromMSecsSinceStartOfDay_0<RetType> {
  fn fromMSecsSinceStartOfDay_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTime_fromMSecsSinceStartOfDay_0<usize> for (i32) {
  fn fromMSecsSinceStartOfDay_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QTime24fromMSecsSinceStartOfDayEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:192
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int msecsSinceStartOfDay() const

/*

*/
impl /*struct*/ QTime {
  pub fn msecsSinceStartOfDay_0<RetType, T: QTime_msecsSinceStartOfDay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.msecsSinceStartOfDay_0(self);
    // return 1;
  }
}
pub trait QTime_msecsSinceStartOfDay_0<RetType> {
  fn msecsSinceStartOfDay_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_msecsSinceStartOfDay_0<i32> for () {
  fn msecsSinceStartOfDay_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime20msecsSinceStartOfDayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:194
// index:0
// Public static Visibility=Default Availability=Available
// [4] QTime currentTime()

/*

*/
impl /*struct*/ QTime {
  pub fn currentTime_0<RetType, T: QTime_currentTime_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentTime_0();
    // return 1;
  }
}
pub trait QTime_currentTime_0<RetType> {
  fn currentTime_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTime_currentTime_0<usize> for () {
  fn currentTime_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QTime11currentTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:196
// index:0
// Public static Visibility=Default Availability=Available
// [4] QTime fromString(const QString &, Qt::DateFormat)

/*
Returns the QDateTime represented by the string, using the format given, or an invalid datetime if this is not possible.

Note for Qt::TextDate: It is recommended that you use the English short month names (e.g. "Jan"). Although localized month names can also be used, they depend on the user's locale settings.

See also toString() and QLocale::toDateTime().
*/
impl /*struct*/ QTime {
  pub fn fromString_0<RetType, T: QTime_fromString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_0();
    // return 1;
  }
}
pub trait QTime_fromString_0<RetType> {
  fn fromString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTime_fromString_0<usize> for (usize,i32) {
  fn fromString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QTime10fromStringERK7QStringN2Qt10DateFormatE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:197
// index:1
// Public static Visibility=Default Availability=Available
// [4] QTime fromString(const QString &, const QString &)

/*
Returns the QDateTime represented by the string, using the format given, or an invalid datetime if this is not possible.

Note for Qt::TextDate: It is recommended that you use the English short month names (e.g. "Jan"). Although localized month names can also be used, they depend on the user's locale settings.

See also toString() and QLocale::toDateTime().
*/
impl /*struct*/ QTime {
  pub fn fromString_1<RetType, T: QTime_fromString_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_1();
    // return 1;
  }
}
pub trait QTime_fromString_1<RetType> {
  fn fromString_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTime_fromString_1<usize> for (usize,usize) {
  fn fromString_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QTime10fromStringERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:201
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start()

/*

*/
impl /*struct*/ QTime {
  pub fn start_0<RetType, T: QTime_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QTime_start_0<RetType> {
  fn start_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_start_0<(/*void*/)> for () {
  fn start_0(self , rsthis: & QTime) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QTime5startEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:202
// index:0
// Public Visibility=Default Availability=Available
// [4] int restart()

/*

*/
impl /*struct*/ QTime {
  pub fn restart_0<RetType, T: QTime_restart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restart_0(self);
    // return 1;
  }
}
pub trait QTime_restart_0<RetType> {
  fn restart_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_restart_0<i32> for () {
  fn restart_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QTime7restartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:203
// index:0
// Public Visibility=Default Availability=Available
// [4] int elapsed() const

/*

*/
impl /*struct*/ QTime {
  pub fn elapsed_0<RetType, T: QTime_elapsed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elapsed_0(self);
    // return 1;
  }
}
pub trait QTime_elapsed_0<RetType> {
  fn elapsed_0(self , rsthis: & QTime) -> RetType;
}
impl<'a> /*trait*/ QTime_elapsed_0<i32> for () {
  fn elapsed_0(self , rsthis: & QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QTime7elapsedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQTime(this :*mut QTime) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN5QTimeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTime__TimeFlag = i32;
// 
pub const QTime__NullTime :QTime__TimeFlag = -1;
pub fn QTime_TimeFlagItemName(val: i32) ->String {
  match val {
     QTime__NullTime => // -1
     {return String::from("NullTime");}
  _ => {return format!("{}", val);}
}
}
pub fn QTime_TimeFlagItemName_s(val: i32) ->String {
  //var nilthis *QTime
  //return nilthis.TimeFlagItemName(val);
  return QTime_TimeFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
