

// mod ::widgets::QLCDNumber
// package qtwidgets
// /usr/include/qt/QtWidgets/qlcdnumber.h
// #include <qlcdnumber.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 53
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QLCDNumber) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QLCDNumber) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QLCDNumber)=48
pub struct QLCDNumber {
  qbase: QFrame,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLCDNumber_ITF interface {
//    QFrame_ITF
//    QLCDNumber_PTR() *QLCDNumber
//}
//func (ptr *QLCDNumber) QLCDNumber_PTR() *QLCDNumber { return ptr }

impl /*struct*/ QLCDNumber {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLCDNumber {
    return QLCDNumber{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLCDNumber {
//  type Target = QLCDNumberBASE;
//
//  fn deref(&self) -> &QLCDNumberBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLCDNumberBASE> for QLCDNumber {
//  fn as_ref(& self) -> & QLCDNumberBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlcdnumber.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn metaObject_0<RetType, T: QLCDNumber_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QLCDNumber) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLCDNumber(QWidget *)

/*
Constructs an LCD number, sets the number of digits to 5, the base to decimal, the decimal point mode to 'small' and the frame style to a raised box. The segmentStyle() is set to Outline.

The parent argument is passed to the QFrame constructor.

See also setDigitCount() and setSmallDecimalPoint().
*/
// QLCDNumber(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QLCDNumber {
  pub fn QLCDNumber_0<T: QLCDNumber_QLCDNumber_0>(value: T) -> QLCDNumber {
    let rsthis = value.QLCDNumber_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLCDNumber_QLCDNumber_0 {
  fn QLCDNumber_0(self) -> QLCDNumber;
}
// QLCDNumber(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLCDNumber_QLCDNumber_0 for (usize) {
  fn QLCDNumber_0(self) -> QLCDNumber {
    // unsafe{_ZN10QLCDNumberC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QLCDNumberC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLCDNumber{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:63
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QLCDNumber(uint, QWidget *)

/*
Constructs an LCD number, sets the number of digits to 5, the base to decimal, the decimal point mode to 'small' and the frame style to a raised box. The segmentStyle() is set to Outline.

The parent argument is passed to the QFrame constructor.

See also setDigitCount() and setSmallDecimalPoint().
*/
// QLCDNumber(uint, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QLCDNumber {
  pub fn QLCDNumber_1<T: QLCDNumber_QLCDNumber_1>(value: T) -> QLCDNumber {
    let rsthis = value.QLCDNumber_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLCDNumber_QLCDNumber_1 {
  fn QLCDNumber_1(self) -> QLCDNumber;
}
// QLCDNumber(uint, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLCDNumber_QLCDNumber_1 for (u32,usize) {
  fn QLCDNumber_1(self) -> QLCDNumber {
    // unsafe{_ZN10QLCDNumberC2EjP7QWidget()};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QLCDNumberC2EjP7QWidget", 2,qtrt::FFITY_UINT32,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLCDNumber{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QLCDNumber()

/*

*/
pub fn DeleteQLCDNumber(this :*mut QLCDNumber) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QLCDNumberD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlcdnumber.h:75
// index:0
// Public Visibility=Default Availability=Available
// [1] bool smallDecimalPoint() const

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn smallDecimalPoint_0<RetType, T: QLCDNumber_smallDecimalPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.smallDecimalPoint_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_smallDecimalPoint_0<RetType> {
  fn smallDecimalPoint_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_smallDecimalPoint_0<bool> for () {
  fn smallDecimalPoint_0(self , rsthis: & QLCDNumber) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber17smallDecimalPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int digitCount() const

/*
Returns the current number of digits.

Note: Getter function for property digitCount. 

See also setDigitCount().
*/
impl /*struct*/ QLCDNumber {
  pub fn digitCount_0<RetType, T: QLCDNumber_digitCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.digitCount_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_digitCount_0<RetType> {
  fn digitCount_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_digitCount_0<i32> for () {
  fn digitCount_0(self , rsthis: & QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber10digitCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDigitCount(int)

/*
Sets the current number of digits to numDigits. Must be in the range 0..99.

Note: Setter function for property digitCount. 

See also digitCount().
*/
impl /*struct*/ QLCDNumber {
  pub fn setDigitCount_0<RetType, T: QLCDNumber_setDigitCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDigitCount_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_setDigitCount_0<RetType> {
  fn setDigitCount_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_setDigitCount_0<(/*void*/)> for (i32) {
  fn setDigitCount_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber13setDigitCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:79
// index:0
// Public Visibility=Default Availability=Available
// [1] bool checkOverflow(double) const

/*
Returns true if num is too big to be displayed in its entirety; otherwise returns false.

See also display(), digitCount(), and smallDecimalPoint().
*/
impl /*struct*/ QLCDNumber {
  pub fn checkOverflow_0<RetType, T: QLCDNumber_checkOverflow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkOverflow_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_checkOverflow_0<RetType> {
  fn checkOverflow_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_checkOverflow_0<bool> for (f64) {
  fn checkOverflow_0(self , rsthis: & QLCDNumber) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber13checkOverflowEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:80
// index:1
// Public Visibility=Default Availability=Available
// [1] bool checkOverflow(int) const

/*
Returns true if num is too big to be displayed in its entirety; otherwise returns false.

See also display(), digitCount(), and smallDecimalPoint().
*/
impl /*struct*/ QLCDNumber {
  pub fn checkOverflow_1<RetType, T: QLCDNumber_checkOverflow_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkOverflow_1(self);
    // return 1;
  }
}
pub trait QLCDNumber_checkOverflow_1<RetType> {
  fn checkOverflow_1(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_checkOverflow_1<bool> for (i32) {
  fn checkOverflow_1(self , rsthis: & QLCDNumber) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber13checkOverflowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] QLCDNumber::Mode mode() const

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn mode_0<RetType, T: QLCDNumber_mode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mode_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_mode_0<RetType> {
  fn mode_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_mode_0<i32> for () {
  fn mode_0(self , rsthis: & QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber4modeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMode(QLCDNumber::Mode)

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn setMode_0<RetType, T: QLCDNumber_setMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMode_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_setMode_0<RetType> {
  fn setMode_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_setMode_0<(/*void*/)> for (i32) {
  fn setMode_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber7setModeENS_4ModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] QLCDNumber::SegmentStyle segmentStyle() const

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn segmentStyle_0<RetType, T: QLCDNumber_segmentStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.segmentStyle_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_segmentStyle_0<RetType> {
  fn segmentStyle_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_segmentStyle_0<i32> for () {
  fn segmentStyle_0(self , rsthis: & QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber12segmentStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSegmentStyle(QLCDNumber::SegmentStyle)

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn setSegmentStyle_0<RetType, T: QLCDNumber_setSegmentStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSegmentStyle_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_setSegmentStyle_0<RetType> {
  fn setSegmentStyle_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_setSegmentStyle_0<(/*void*/)> for (i32) {
  fn setSegmentStyle_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber15setSegmentStyleENS_12SegmentStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] double value() const

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn value_0<RetType, T: QLCDNumber_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_value_0<RetType> {
  fn value_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_value_0<f64> for () {
  fn value_0(self , rsthis: & QLCDNumber) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] int intValue() const

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn intValue_0<RetType, T: QLCDNumber_intValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intValue_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_intValue_0<RetType> {
  fn intValue_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_intValue_0<i32> for () {
  fn intValue_0(self , rsthis: & QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber8intValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QLCDNumber {
  pub fn sizeHint_0<RetType, T: QLCDNumber_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QLCDNumber) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QLCDNumber8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void display(const QString &)

/*
Displays the number represented by the string s.

This version of the function disregards mode() and smallDecimalPoint().

These digits and other symbols can be shown: 0/O, 1, 2, 3, 4, 5/S, 6, 7, 8, 9/g, minus, decimal point, A, B, C, D, E, F, h, H, L, o, P, r, u, U, Y, colon, degree sign (which is specified as single quote in the string) and space. QLCDNumber substitutes spaces for illegal characters.

Note: Setter function for property intValue. Setter function for property value.
*/
impl /*struct*/ QLCDNumber {
  pub fn display_0<RetType, T: QLCDNumber_display_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.display_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_display_0<RetType> {
  fn display_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_display_0<(/*void*/)> for (usize) {
  fn display_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber7displayERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:95
// index:1
// Public Visibility=Default Availability=Available
// [-2] void display(int)

/*
Displays the number represented by the string s.

This version of the function disregards mode() and smallDecimalPoint().

These digits and other symbols can be shown: 0/O, 1, 2, 3, 4, 5/S, 6, 7, 8, 9/g, minus, decimal point, A, B, C, D, E, F, h, H, L, o, P, r, u, U, Y, colon, degree sign (which is specified as single quote in the string) and space. QLCDNumber substitutes spaces for illegal characters.

Note: Setter function for property intValue. Setter function for property value.
*/
impl /*struct*/ QLCDNumber {
  pub fn display_1<RetType, T: QLCDNumber_display_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.display_1(self);
    // return 1;
  }
}
pub trait QLCDNumber_display_1<RetType> {
  fn display_1(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_display_1<(/*void*/)> for (i32) {
  fn display_1(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber7displayEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:96
// index:2
// Public Visibility=Default Availability=Available
// [-2] void display(double)

/*
Displays the number represented by the string s.

This version of the function disregards mode() and smallDecimalPoint().

These digits and other symbols can be shown: 0/O, 1, 2, 3, 4, 5/S, 6, 7, 8, 9/g, minus, decimal point, A, B, C, D, E, F, h, H, L, o, P, r, u, U, Y, colon, degree sign (which is specified as single quote in the string) and space. QLCDNumber substitutes spaces for illegal characters.

Note: Setter function for property intValue. Setter function for property value.
*/
impl /*struct*/ QLCDNumber {
  pub fn display_2<RetType, T: QLCDNumber_display_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.display_2(self);
    // return 1;
  }
}
pub trait QLCDNumber_display_2<RetType> {
  fn display_2(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_display_2<(/*void*/)> for (f64) {
  fn display_2(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber7displayEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHexMode()

/*
Calls setMode(Hex). Provided for convenience (e.g. for connecting buttons to it).

See also setMode(), setDecMode(), setOctMode(), setBinMode(), and mode().
*/
impl /*struct*/ QLCDNumber {
  pub fn setHexMode_0<RetType, T: QLCDNumber_setHexMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHexMode_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_setHexMode_0<RetType> {
  fn setHexMode_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_setHexMode_0<(/*void*/)> for () {
  fn setHexMode_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber10setHexModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDecMode()

/*
Calls setMode(Dec). Provided for convenience (e.g. for connecting buttons to it).

See also setMode(), setHexMode(), setOctMode(), setBinMode(), and mode().
*/
impl /*struct*/ QLCDNumber {
  pub fn setDecMode_0<RetType, T: QLCDNumber_setDecMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDecMode_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_setDecMode_0<RetType> {
  fn setDecMode_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_setDecMode_0<(/*void*/)> for () {
  fn setDecMode_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber10setDecModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOctMode()

/*
Calls setMode(Oct). Provided for convenience (e.g. for connecting buttons to it).

See also setMode(), setHexMode(), setDecMode(), setBinMode(), and mode().
*/
impl /*struct*/ QLCDNumber {
  pub fn setOctMode_0<RetType, T: QLCDNumber_setOctMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOctMode_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_setOctMode_0<RetType> {
  fn setOctMode_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_setOctMode_0<(/*void*/)> for () {
  fn setOctMode_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber10setOctModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBinMode()

/*
Calls setMode(Bin). Provided for convenience (e.g. for connecting buttons to it).

See also setMode(), setHexMode(), setDecMode(), setOctMode(), and mode().
*/
impl /*struct*/ QLCDNumber {
  pub fn setBinMode_0<RetType, T: QLCDNumber_setBinMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBinMode_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_setBinMode_0<RetType> {
  fn setBinMode_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_setBinMode_0<(/*void*/)> for () {
  fn setBinMode_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber10setBinModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSmallDecimalPoint(bool)

/*

*/
impl /*struct*/ QLCDNumber {
  pub fn setSmallDecimalPoint_0<RetType, T: QLCDNumber_setSmallDecimalPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSmallDecimalPoint_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_setSmallDecimalPoint_0<RetType> {
  fn setSmallDecimalPoint_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_setSmallDecimalPoint_0<(/*void*/)> for (bool) {
  fn setSmallDecimalPoint_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber20setSmallDecimalPointEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void overflow()

/*
This signal is emitted whenever the QLCDNumber is asked to display a too-large number or a too-long string.

It is never emitted by setDigitCount().
*/
impl /*struct*/ QLCDNumber {
  pub fn overflow_0<RetType, T: QLCDNumber_overflow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overflow_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_overflow_0<RetType> {
  fn overflow_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_overflow_0<(/*void*/)> for () {
  fn overflow_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber8overflowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:107
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QLCDNumber {
  pub fn event_0<RetType, T: QLCDNumber_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_event_0<RetType> {
  fn event_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QLCDNumber) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QLCDNumber5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlcdnumber.h:108
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QLCDNumber {
  pub fn paintEvent_0<RetType, T: QLCDNumber_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QLCDNumber_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QLCDNumber) -> RetType;
}
impl<'a> /*trait*/ QLCDNumber_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QLCDNumber) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QLCDNumber10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This type determines how numbers are shown.



If the display is set to hexadecimal, octal or binary, the integer equivalent of the value is displayed.

*/
pub type QLCDNumber__Mode = i32;
// Hexadecimal
pub const QLCDNumber__Hex :QLCDNumber__Mode = 0;
// Decimal
pub const QLCDNumber__Dec :QLCDNumber__Mode = 1;
// Octal
pub const QLCDNumber__Oct :QLCDNumber__Mode = 2;
// Binary
pub const QLCDNumber__Bin :QLCDNumber__Mode = 3;
pub fn QLCDNumber_ModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QLCDNumber", val);
}
pub fn QLCDNumber_ModeItemName_s(val: i32) ->String {
  //var nilthis *QLCDNumber
  //return nilthis.ModeItemName(val);
  return QLCDNumber_ModeItemName(val);
}


/*
This type determines the visual appearance of the QLCDNumber widget.


*/
pub type QLCDNumber__SegmentStyle = i32;
// gives raised segments filled with the background color.
pub const QLCDNumber__Outline :QLCDNumber__SegmentStyle = 0;
// gives raised segments filled with the windowText color.
pub const QLCDNumber__Filled :QLCDNumber__SegmentStyle = 1;
// gives flat segments filled with the windowText color.
pub const QLCDNumber__Flat :QLCDNumber__SegmentStyle = 2;
pub fn QLCDNumber_SegmentStyleItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QLCDNumber", val);
}
pub fn QLCDNumber_SegmentStyleItemName_s(val: i32) ->String {
  //var nilthis *QLCDNumber
  //return nilthis.SegmentStyleItemName(val);
  return QLCDNumber_SegmentStyleItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
