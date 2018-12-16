

// mod ::core::QDate
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
// extern C begin: 49
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
#[derive(Default)] // class sizeof(QDate)=8
pub struct QDate {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDate_ITF interface {
//    QDate_PTR() *QDate
//}
//func (ptr *QDate) QDate_PTR() *QDate { return ptr }

impl /*struct*/ QDate {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDate {
    return QDate{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDate {
//  type Target = QDateBASE;
//
//  fn deref(&self) -> &QDateBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDateBASE> for QDate {
//  fn as_ref(& self) -> & QDateBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qdatetime.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QDate()

/*

*/
// QDate() ctx.fn_proto_cpp
impl /*struct*/ QDate {
  pub fn QDate_0<T: QDate_QDate_0>(value: T) -> QDate {
    let rsthis = value.QDate_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDate_QDate_0 {
  fn QDate_0(self) -> QDate;
}
// QDate() ctx.fn_proto_cpp
impl<'a> /*trait*/ QDate_QDate_0 for () {
  fn QDate_0(self) -> QDate {
    // unsafe{_ZN5QDateC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QDateC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:70
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDate(int, int, int)

/*

*/
// QDate(int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QDate {
  pub fn QDate_1<T: QDate_QDate_1>(value: T) -> QDate {
    let rsthis = value.QDate_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDate_QDate_1 {
  fn QDate_1(self) -> QDate;
}
// QDate(int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDate_QDate_1 for (i32,i32,i32) {
  fn QDate_1(self) -> QDate {
    // unsafe{_ZN5QDateC2Eiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QDateC2Eiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:72
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if both the date and the time are null; otherwise returns false. A null datetime is invalid.

See also QDate::isNull(), QTime::isNull(), and isValid().
*/
impl /*struct*/ QDate {
  pub fn isNull_0<RetType, T: QDate_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QDate_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if both the date and the time are valid and they are valid in the current Qt::TimeSpec, otherwise returns false.

If the timeSpec() is Qt::LocalTime or Qt::TimeZone then the date and time are checked to see if they fall in the Standard Time to Daylight-Saving Time transition hour, i.e. if the transition is at 2am and the clock goes forward to 3am then the time from 02:00:00 to 02:59:59.999 is considered to be invalid.

See also QDate::isValid() and QTime::isValid().
*/
impl /*struct*/ QDate {
  pub fn isValid_0<RetType, T: QDate_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QDate_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:130
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool isValid(int, int, int)

/*
Returns true if both the date and the time are valid and they are valid in the current Qt::TimeSpec, otherwise returns false.

If the timeSpec() is Qt::LocalTime or Qt::TimeZone then the date and time are checked to see if they fall in the Standard Time to Daylight-Saving Time transition hour, i.e. if the transition is at 2am and the clock goes forward to 3am then the time from 02:00:00 to 02:59:59.999 is considered to be invalid.

See also QDate::isValid() and QTime::isValid().
*/
impl /*struct*/ QDate {
  pub fn isValid_1<RetType, T: QDate_isValid_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isValid_1();
    // return 1;
  }
}
pub trait QDate_isValid_1<RetType> {
  fn isValid_1(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_isValid_1<bool> for (i32,i32,i32) {
  fn isValid_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate7isValidEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:75
// index:0
// Public Visibility=Default Availability=Available
// [4] int year() const

/*

*/
impl /*struct*/ QDate {
  pub fn year_0<RetType, T: QDate_year_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.year_0(self);
    // return 1;
  }
}
pub trait QDate_year_0<RetType> {
  fn year_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_year_0<i32> for () {
  fn year_0(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate4yearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int month() const

/*

*/
impl /*struct*/ QDate {
  pub fn month_0<RetType, T: QDate_month_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.month_0(self);
    // return 1;
  }
}
pub trait QDate_month_0<RetType> {
  fn month_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_month_0<i32> for () {
  fn month_0(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate5monthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] int day() const

/*

*/
impl /*struct*/ QDate {
  pub fn day_0<RetType, T: QDate_day_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.day_0(self);
    // return 1;
  }
}
pub trait QDate_day_0<RetType> {
  fn day_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_day_0<i32> for () {
  fn day_0(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate3dayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] int dayOfWeek() const

/*

*/
impl /*struct*/ QDate {
  pub fn dayOfWeek_0<RetType, T: QDate_dayOfWeek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dayOfWeek_0(self);
    // return 1;
  }
}
pub trait QDate_dayOfWeek_0<RetType> {
  fn dayOfWeek_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_dayOfWeek_0<i32> for () {
  fn dayOfWeek_0(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate9dayOfWeekEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] int dayOfYear() const

/*

*/
impl /*struct*/ QDate {
  pub fn dayOfYear_0<RetType, T: QDate_dayOfYear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dayOfYear_0(self);
    // return 1;
  }
}
pub trait QDate_dayOfYear_0<RetType> {
  fn dayOfYear_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_dayOfYear_0<i32> for () {
  fn dayOfYear_0(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate9dayOfYearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:80
// index:0
// Public Visibility=Default Availability=Available
// [4] int daysInMonth() const

/*

*/
impl /*struct*/ QDate {
  pub fn daysInMonth_0<RetType, T: QDate_daysInMonth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.daysInMonth_0(self);
    // return 1;
  }
}
pub trait QDate_daysInMonth_0<RetType> {
  fn daysInMonth_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_daysInMonth_0<i32> for () {
  fn daysInMonth_0(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate11daysInMonthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] int daysInYear() const

/*

*/
impl /*struct*/ QDate {
  pub fn daysInYear_0<RetType, T: QDate_daysInYear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.daysInYear_0(self);
    // return 1;
  }
}
pub trait QDate_daysInYear_0<RetType> {
  fn daysInYear_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_daysInYear_0<i32> for () {
  fn daysInYear_0(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate10daysInYearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] int weekNumber(int *) const

/*

*/
impl /*struct*/ QDate {
  pub fn weekNumber_0<RetType, T: QDate_weekNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.weekNumber_0(self);
    // return 1;
  }
}
pub trait QDate_weekNumber_0<RetType> {
  fn weekNumber_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_weekNumber_0<i32> for (usize) {
  fn weekNumber_0(self , rsthis: & QDate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate10weekNumberEPi", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:86
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString shortMonthName(int, QDate::MonthNameType)

/*

*/
impl /*struct*/ QDate {
  pub fn shortMonthName_0<RetType, T: QDate_shortMonthName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.shortMonthName_0();
    // return 1;
  }
}
pub trait QDate_shortMonthName_0<RetType> {
  fn shortMonthName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_shortMonthName_0<usize> for (i32,i32) {
  fn shortMonthName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate14shortMonthNameEiNS_13MonthNameTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:88
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString shortDayName(int, QDate::MonthNameType)

/*

*/
impl /*struct*/ QDate {
  pub fn shortDayName_0<RetType, T: QDate_shortDayName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.shortDayName_0();
    // return 1;
  }
}
pub trait QDate_shortDayName_0<RetType> {
  fn shortDayName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_shortDayName_0<usize> for (i32,i32) {
  fn shortDayName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate12shortDayNameEiNS_13MonthNameTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:90
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString longMonthName(int, QDate::MonthNameType)

/*

*/
impl /*struct*/ QDate {
  pub fn longMonthName_0<RetType, T: QDate_longMonthName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.longMonthName_0();
    // return 1;
  }
}
pub trait QDate_longMonthName_0<RetType> {
  fn longMonthName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_longMonthName_0<usize> for (i32,i32) {
  fn longMonthName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate13longMonthNameEiNS_13MonthNameTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:92
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString longDayName(int, QDate::MonthNameType)

/*

*/
impl /*struct*/ QDate {
  pub fn longDayName_0<RetType, T: QDate_longDayName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.longDayName_0();
    // return 1;
  }
}
pub trait QDate_longDayName_0<RetType> {
  fn longDayName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_longDayName_0<usize> for (i32,i32) {
  fn longDayName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate11longDayNameEiNS_13MonthNameTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:95
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
impl /*struct*/ QDate {
  pub fn toString_0<RetType, T: QDate_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QDate_toString_0<RetType> {
  fn toString_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_toString_0<usize> for (i32) {
  fn toString_0(self , rsthis: & QDate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate8toStringEN2Qt10DateFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:97
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
impl /*struct*/ QDate {
  pub fn toString_1<RetType, T: QDate_toString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_1(self);
    // return 1;
  }
}
pub trait QDate_toString_1<RetType> {
  fn toString_1(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_toString_1<usize> for (usize) {
  fn toString_1(self , rsthis: & QDate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate8toStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:99
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
impl /*struct*/ QDate {
  pub fn toString_2<RetType, T: QDate_toString_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_2(self);
    // return 1;
  }
}
pub trait QDate_toString_2<RetType> {
  fn toString_2(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_toString_2<usize> for (usize) {
  fn toString_2(self , rsthis: & QDate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate8toStringE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:106
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setDate(int, int, int)

/*
Sets the date part of this datetime to date. If no time is set yet, it is set to midnight. If date is invalid, this QDateTime becomes invalid.

See also date(), setTime(), and setTimeSpec().
*/
impl /*struct*/ QDate {
  pub fn setDate_0<RetType, T: QDate_setDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDate_0(self);
    // return 1;
  }
}
pub trait QDate_setDate_0<RetType> {
  fn setDate_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_setDate_0<bool> for (i32,i32,i32) {
  fn setDate_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate7setDateEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getDate(int *, int *, int *)

/*

*/
impl /*struct*/ QDate {
  pub fn getDate_0<RetType, T: QDate_getDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getDate_0(self);
    // return 1;
  }
}
pub trait QDate_getDate_0<RetType> {
  fn getDate_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_getDate_0<(/*void*/)> for (usize,usize,usize) {
  fn getDate_0(self , rsthis: & QDate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDate7getDateEPiS0_S0_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:111
// index:1
// Public Visibility=Default Availability=Available
// [-2] void getDate(int *, int *, int *) const

/*

*/
impl /*struct*/ QDate {
  pub fn getDate_1<RetType, T: QDate_getDate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getDate_1(self);
    // return 1;
  }
}
pub trait QDate_getDate_1<RetType> {
  fn getDate_1(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_getDate_1<(/*void*/)> for (usize,usize,usize) {
  fn getDate_1(self , rsthis: & QDate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK5QDate7getDateEPiS0_S0_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:113
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate addDays(qint64) const

/*
Returns a QDateTime object containing a datetime ndays days later than the datetime of this object (or earlier if ndays is negative).

If the timeSpec() is Qt::LocalTime and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be adjusted accordingly, i.e. if the transition is at 2am and the clock goes forward to 3am and the result falls between 2am and 3am then the result will be adjusted to fall after 3am.

See also daysTo(), addMonths(), addYears(), and addSecs().
*/
impl /*struct*/ QDate {
  pub fn addDays_0<RetType, T: QDate_addDays_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addDays_0(self);
    // return 1;
  }
}
pub trait QDate_addDays_0<RetType> {
  fn addDays_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_addDays_0<usize> for (i64) {
  fn addDays_0(self , rsthis: & QDate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate7addDaysEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:114
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate addMonths(int) const

/*
Returns a QDateTime object containing a datetime nmonths months later than the datetime of this object (or earlier if nmonths is negative).

If the timeSpec() is Qt::LocalTime and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be adjusted accordingly, i.e. if the transition is at 2am and the clock goes forward to 3am and the result falls between 2am and 3am then the result will be adjusted to fall after 3am.

See also daysTo(), addDays(), addYears(), and addSecs().
*/
impl /*struct*/ QDate {
  pub fn addMonths_0<RetType, T: QDate_addMonths_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMonths_0(self);
    // return 1;
  }
}
pub trait QDate_addMonths_0<RetType> {
  fn addMonths_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_addMonths_0<usize> for (i32) {
  fn addMonths_0(self , rsthis: & QDate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate9addMonthsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate addYears(int) const

/*
Returns a QDateTime object containing a datetime nyears years later than the datetime of this object (or earlier if nyears is negative).

If the timeSpec() is Qt::LocalTime and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be adjusted accordingly, i.e. if the transition is at 2am and the clock goes forward to 3am and the result falls between 2am and 3am then the result will be adjusted to fall after 3am.

See also daysTo(), addDays(), addMonths(), and addSecs().
*/
impl /*struct*/ QDate {
  pub fn addYears_0<RetType, T: QDate_addYears_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addYears_0(self);
    // return 1;
  }
}
pub trait QDate_addYears_0<RetType> {
  fn addYears_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_addYears_0<usize> for (i32) {
  fn addYears_0(self , rsthis: & QDate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate8addYearsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:116
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 daysTo(const QDate &) const

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
impl /*struct*/ QDate {
  pub fn daysTo_0<RetType, T: QDate_daysTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.daysTo_0(self);
    // return 1;
  }
}
pub trait QDate_daysTo_0<RetType> {
  fn daysTo_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_daysTo_0<i64> for (usize) {
  fn daysTo_0(self , rsthis: & QDate) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate6daysToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:118
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QDate &) const

/*

*/
impl /*struct*/ QDate {
  pub fn operator_equal_equal_0<RetType, T: QDate_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QDate_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDateeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:119
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QDate &) const

/*

*/
impl /*struct*/ QDate {
  pub fn operator_not_equal_0<RetType, T: QDate_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QDate_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDateneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QDate &) const

/*

*/
impl /*struct*/ QDate {
  pub fn operator_less_than_0<RetType, T: QDate_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QDate_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDateltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:121
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QDate &) const

/*

*/
impl /*struct*/ QDate {
  pub fn operator_less_than_equal_0<RetType, T: QDate_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QDate_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDateleERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QDate &) const

/*

*/
impl /*struct*/ QDate {
  pub fn operator_greater_than_0<RetType, T: QDate_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QDate_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDategtERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:123
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QDate &) const

/*

*/
impl /*struct*/ QDate {
  pub fn operator_greater_than_equal_0<RetType, T: QDate_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QDate_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QDate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDategeERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:125
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDate currentDate()

/*

*/
impl /*struct*/ QDate {
  pub fn currentDate_0<RetType, T: QDate_currentDate_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentDate_0();
    // return 1;
  }
}
pub trait QDate_currentDate_0<RetType> {
  fn currentDate_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_currentDate_0<usize> for () {
  fn currentDate_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate11currentDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:127
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDate fromString(const QString &, Qt::DateFormat)

/*
Returns the QDateTime represented by the string, using the format given, or an invalid datetime if this is not possible.

Note for Qt::TextDate: It is recommended that you use the English short month names (e.g. "Jan"). Although localized month names can also be used, they depend on the user's locale settings.

See also toString() and QLocale::toDateTime().
*/
impl /*struct*/ QDate {
  pub fn fromString_0<RetType, T: QDate_fromString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_0();
    // return 1;
  }
}
pub trait QDate_fromString_0<RetType> {
  fn fromString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_fromString_0<usize> for (usize,i32) {
  fn fromString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate10fromStringERK7QStringN2Qt10DateFormatE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:128
// index:1
// Public static Visibility=Default Availability=Available
// [8] QDate fromString(const QString &, const QString &)

/*
Returns the QDateTime represented by the string, using the format given, or an invalid datetime if this is not possible.

Note for Qt::TextDate: It is recommended that you use the English short month names (e.g. "Jan"). Although localized month names can also be used, they depend on the user's locale settings.

See also toString() and QLocale::toDateTime().
*/
impl /*struct*/ QDate {
  pub fn fromString_1<RetType, T: QDate_fromString_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_1();
    // return 1;
  }
}
pub trait QDate_fromString_1<RetType> {
  fn fromString_1(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_fromString_1<usize> for (usize,usize) {
  fn fromString_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate10fromStringERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:131
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isLeapYear(int)

/*

*/
impl /*struct*/ QDate {
  pub fn isLeapYear_0<RetType, T: QDate_isLeapYear_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLeapYear_0();
    // return 1;
  }
}
pub trait QDate_isLeapYear_0<RetType> {
  fn isLeapYear_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_isLeapYear_0<bool> for (i32) {
  fn isLeapYear_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate10isLeapYearEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:133
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QDate fromJulianDay(qint64)

/*

*/
impl /*struct*/ QDate {
  pub fn fromJulianDay_0<RetType, T: QDate_fromJulianDay_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromJulianDay_0();
    // return 1;
  }
}
pub trait QDate_fromJulianDay_0<RetType> {
  fn fromJulianDay_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDate_fromJulianDay_0<usize> for (i64) {
  fn fromJulianDay_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDate13fromJulianDayEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatetime.h:135
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qint64 toJulianDay() const

/*

*/
impl /*struct*/ QDate {
  pub fn toJulianDay_0<RetType, T: QDate_toJulianDay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toJulianDay_0(self);
    // return 1;
  }
}
pub trait QDate_toJulianDay_0<RetType> {
  fn toJulianDay_0(self , rsthis: & QDate) -> RetType;
}
impl<'a> /*trait*/ QDate_toJulianDay_0<i64> for () {
  fn toJulianDay_0(self , rsthis: & QDate) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDate11toJulianDayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}


pub fn DeleteQDate(this :*mut QDate) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN5QDateD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QDate__MonthNameType = i32;
// 
pub const QDate__DateFormat :QDate__MonthNameType = 0;
// 
pub const QDate__StandaloneFormat :QDate__MonthNameType = 1;
pub fn QDate_MonthNameTypeItemName(val: i32) ->String {
  match val {
     QDate__DateFormat => // 0
     {return String::from("DateFormat");}
     QDate__StandaloneFormat => // 1
     {return String::from("StandaloneFormat");}
  _ => {return format!("{}", val);}
}
}
pub fn QDate_MonthNameTypeItemName_s(val: i32) ->String {
  //var nilthis *QDate
  //return nilthis.MonthNameTypeItemName(val);
  return QDate_MonthNameTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
