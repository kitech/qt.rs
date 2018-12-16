

// mod ::gui::QTextOption
// package qtgui
// /usr/include/qt/QtGui/qtextoption.h
// #include <qtextoption.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 32
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTextOption)=32
pub struct QTextOption {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextOption_ITF interface {
//    QTextOption_PTR() *QTextOption
//}
//func (ptr *QTextOption) QTextOption_PTR() *QTextOption { return ptr }

impl /*struct*/ QTextOption {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextOption {
    return QTextOption{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextOption {
//  type Target = QTextOptionBASE;
//
//  fn deref(&self) -> &QTextOptionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextOptionBASE> for QTextOption {
//  fn as_ref(& self) -> & QTextOptionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextoption.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextOption()

/*
Constructs a text option with default properties for text. The text alignment property is set to Qt::AlignLeft. The word wrap property is set to QTextOption::WordWrap. The using of design metrics flag is set to false.
*/
// QTextOption() ctx.fn_proto_cpp
impl /*struct*/ QTextOption {
  pub fn QTextOption_0<T: QTextOption_QTextOption_0>(value: T) -> QTextOption {
    let rsthis = value.QTextOption_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextOption_QTextOption_0 {
  fn QTextOption_0(self) -> QTextOption;
}
// QTextOption() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextOption_QTextOption_0 for () {
  fn QTextOption_0(self) -> QTextOption {
    // unsafe{_ZN11QTextOptionC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextOptionC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:86
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextOption(Qt::Alignment)

/*
Constructs a text option with default properties for text. The text alignment property is set to Qt::AlignLeft. The word wrap property is set to QTextOption::WordWrap. The using of design metrics flag is set to false.
*/
// QTextOption(Qt::Alignment) ctx.fn_proto_cpp
impl /*struct*/ QTextOption {
  pub fn QTextOption_1<T: QTextOption_QTextOption_1>(value: T) -> QTextOption {
    let rsthis = value.QTextOption_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextOption_QTextOption_1 {
  fn QTextOption_1(self) -> QTextOption;
}
// QTextOption(Qt::Alignment) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextOption_QTextOption_1 for (i32) {
  fn QTextOption_1(self) -> QTextOption {
    // unsafe{_ZN11QTextOptionC2E6QFlagsIN2Qt13AlignmentFlagEE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextOptionC2E6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextOption()

/*

*/
pub fn DeleteQTextOption(this :*mut QTextOption) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTextOptionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextoption.h:90
// index:0
// Public Visibility=Default Availability=Available
// [32] QTextOption & operator=(const QTextOption &)

/*

*/
impl /*struct*/ QTextOption {
  pub fn operator_equal_0<RetType, T: QTextOption_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTextOption_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTextOption) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextOptionaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*
Sets the option's text alignment to the specified alignment.

See also alignment().
*/
impl /*struct*/ QTextOption {
  pub fn setAlignment_0<RetType, T: QTextOption_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QTextOption_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QTextOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextOption12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*
Returns the text alignment defined by the option.

See also setAlignment().
*/
impl /*struct*/ QTextOption {
  pub fn alignment_0<RetType, T: QTextOption_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QTextOption_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QTextOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextOption9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:95
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextDirection(Qt::LayoutDirection)

/*
Sets the direction of the text layout defined by the option to the given direction.

See also textDirection().
*/
impl /*struct*/ QTextOption {
  pub fn setTextDirection_0<RetType, T: QTextOption_setTextDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextDirection_0(self);
    // return 1;
  }
}
pub trait QTextOption_setTextDirection_0<RetType> {
  fn setTextDirection_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_setTextDirection_0<(/*void*/)> for (i32) {
  fn setTextDirection_0(self , rsthis: & QTextOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextOption16setTextDirectionEN2Qt15LayoutDirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:96
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::LayoutDirection textDirection() const

/*
Returns the direction of the text layout defined by the option.

See also setTextDirection().
*/
impl /*struct*/ QTextOption {
  pub fn textDirection_0<RetType, T: QTextOption_textDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textDirection_0(self);
    // return 1;
  }
}
pub trait QTextOption_textDirection_0<RetType> {
  fn textDirection_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_textDirection_0<i32> for () {
  fn textDirection_0(self , rsthis: & QTextOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextOption13textDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWrapMode(QTextOption::WrapMode)

/*
Sets the option's text wrap mode to the given mode.

See also wrapMode().
*/
impl /*struct*/ QTextOption {
  pub fn setWrapMode_0<RetType, T: QTextOption_setWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWrapMode_0(self);
    // return 1;
  }
}
pub trait QTextOption_setWrapMode_0<RetType> {
  fn setWrapMode_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_setWrapMode_0<(/*void*/)> for (i32) {
  fn setWrapMode_0(self , rsthis: & QTextOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextOption11setWrapModeENS_8WrapModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:106
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextOption::WrapMode wrapMode() const

/*
Returns the text wrap mode defined by the option.

See also setWrapMode().
*/
impl /*struct*/ QTextOption {
  pub fn wrapMode_0<RetType, T: QTextOption_wrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wrapMode_0(self);
    // return 1;
  }
}
pub trait QTextOption_wrapMode_0<RetType> {
  fn wrapMode_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_wrapMode_0<i32> for () {
  fn wrapMode_0(self , rsthis: & QTextOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextOption8wrapModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:117
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFlags(QTextOption::Flags)

/*
Sets the flags associated with the option to the given flags.

See also flags().
*/
impl /*struct*/ QTextOption {
  pub fn setFlags_0<RetType, T: QTextOption_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QTextOption_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QTextOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextOption8setFlagsE6QFlagsINS_4FlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:118
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextOption::Flags flags() const

/*
Returns the flags associated with the option.

See also setFlags().
*/
impl /*struct*/ QTextOption {
  pub fn flags_0<RetType, T: QTextOption_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QTextOption_flags_0<RetType> {
  fn flags_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QTextOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextOption5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:121
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTabStop(qreal)

/*

*/
impl /*struct*/ QTextOption {
  pub fn setTabStop_0<RetType, T: QTextOption_setTabStop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabStop_0(self);
    // return 1;
  }
}
pub trait QTextOption_setTabStop_0<RetType> {
  fn setTabStop_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_setTabStop_0<(/*void*/)> for (f64) {
  fn setTabStop_0(self , rsthis: & QTextOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextOption10setTabStopEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal tabStop() const

/*

*/
impl /*struct*/ QTextOption {
  pub fn tabStop_0<RetType, T: QTextOption_tabStop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabStop_0(self);
    // return 1;
  }
}
pub trait QTextOption_tabStop_0<RetType> {
  fn tabStop_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_tabStop_0<f64> for () {
  fn tabStop_0(self , rsthis: & QTextOption) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextOption7tabStopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTabStopDistance(qreal)

/*
Sets the default distance in device units between tab stops to the value specified by tabStopDistance.

This function was introduced in  Qt 5.10.

See also tabStopDistance(), setTabArray(), setTabs(), and tabs().
*/
impl /*struct*/ QTextOption {
  pub fn setTabStopDistance_0<RetType, T: QTextOption_setTabStopDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabStopDistance_0(self);
    // return 1;
  }
}
pub trait QTextOption_setTabStopDistance_0<RetType> {
  fn setTabStopDistance_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_setTabStopDistance_0<(/*void*/)> for (f64) {
  fn setTabStopDistance_0(self , rsthis: & QTextOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextOption18setTabStopDistanceEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal tabStopDistance() const

/*
Returns the distance in device units between tab stops.

This function was introduced in  Qt 5.10.

See also setTabStopDistance(), tabArray(), setTabs(), and tabs().
*/
impl /*struct*/ QTextOption {
  pub fn tabStopDistance_0<RetType, T: QTextOption_tabStopDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabStopDistance_0(self);
    // return 1;
  }
}
pub trait QTextOption_tabStopDistance_0<RetType> {
  fn tabStopDistance_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_tabStopDistance_0<f64> for () {
  fn tabStopDistance_0(self , rsthis: & QTextOption) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextOption15tabStopDistanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:134
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setUseDesignMetrics(bool)

/*
If enable is true then the layout will use design metrics; otherwise it will use the metrics of the paint device (which is the default behavior).

See also useDesignMetrics().
*/
impl /*struct*/ QTextOption {
  pub fn setUseDesignMetrics_0<RetType, T: QTextOption_setUseDesignMetrics_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUseDesignMetrics_0(self);
    // return 1;
  }
}
pub trait QTextOption_setUseDesignMetrics_0<RetType> {
  fn setUseDesignMetrics_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_setUseDesignMetrics_0<(/*void*/)> for (bool) {
  fn setUseDesignMetrics_0(self , rsthis: & QTextOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextOption19setUseDesignMetricsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextoption.h:135
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool useDesignMetrics() const

/*
Returns true if the layout uses design rather than device metrics; otherwise returns false.

See also setUseDesignMetrics().
*/
impl /*struct*/ QTextOption {
  pub fn useDesignMetrics_0<RetType, T: QTextOption_useDesignMetrics_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.useDesignMetrics_0(self);
    // return 1;
  }
}
pub trait QTextOption_useDesignMetrics_0<RetType> {
  fn useDesignMetrics_0(self , rsthis: & QTextOption) -> RetType;
}
impl<'a> /*trait*/ QTextOption_useDesignMetrics_0<bool> for () {
  fn useDesignMetrics_0(self , rsthis: & QTextOption) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextOption16useDesignMetricsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum holds the different types of tabulator



This enum was introduced or modified in  Qt 4.4.

*/
pub type QTextOption__TabType = i32;
// A left-tab
pub const QTextOption__LeftTab :QTextOption__TabType = 0;
// A right-tab
pub const QTextOption__RightTab :QTextOption__TabType = 1;
// A centered-tab
pub const QTextOption__CenterTab :QTextOption__TabType = 2;
// A tab stopping at a certain delimiter-character
pub const QTextOption__DelimiterTab :QTextOption__TabType = 3;
pub fn QTextOption_TabTypeItemName(val: i32) ->String {
  match val {
     QTextOption__LeftTab => // 0
     {return String::from("LeftTab");}
     QTextOption__RightTab => // 1
     {return String::from("RightTab");}
     QTextOption__CenterTab => // 2
     {return String::from("CenterTab");}
     QTextOption__DelimiterTab => // 3
     {return String::from("DelimiterTab");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextOption_TabTypeItemName_s(val: i32) ->String {
  //var nilthis *QTextOption
  //return nilthis.TabTypeItemName(val);
  return QTextOption_TabTypeItemName(val);
}


/*
This enum describes how text is wrapped in a document.


*/
pub type QTextOption__WrapMode = i32;
// Text is not wrapped at all.
pub const QTextOption__NoWrap :QTextOption__WrapMode = 0;
// Text is wrapped at word boundaries.
pub const QTextOption__WordWrap :QTextOption__WrapMode = 1;
// Same as QTextOption::NoWrap
pub const QTextOption__ManualWrap :QTextOption__WrapMode = 2;
// Text can be wrapped at any point on a line, even if it occurs in the middle of a word.
pub const QTextOption__WrapAnywhere :QTextOption__WrapMode = 3;
// If possible, wrapping occurs at a word boundary; otherwise it will occur at the appropriate point on the line, even in the middle of a word.
pub const QTextOption__WrapAtWordBoundaryOrAnywhere :QTextOption__WrapMode = 4;
pub fn QTextOption_WrapModeItemName(val: i32) ->String {
  match val {
     QTextOption__NoWrap => // 0
     {return String::from("NoWrap");}
     QTextOption__WordWrap => // 1
     {return String::from("WordWrap");}
     QTextOption__ManualWrap => // 2
     {return String::from("ManualWrap");}
     QTextOption__WrapAnywhere => // 3
     {return String::from("WrapAnywhere");}
     QTextOption__WrapAtWordBoundaryOrAnywhere => // 4
     {return String::from("WrapAtWordBoundaryOrAnywhere");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextOption_WrapModeItemName_s(val: i32) ->String {
  //var nilthis *QTextOption
  //return nilthis.WrapModeItemName(val);
  return QTextOption_WrapModeItemName(val);
}


/*


*/
pub type QTextOption__Flag = i32;
// 
pub const QTextOption__ShowTabsAndSpaces :QTextOption__Flag = 1;
// 
pub const QTextOption__ShowLineAndParagraphSeparators :QTextOption__Flag = 2;
// 
pub const QTextOption__AddSpaceForLineAndParagraphSeparators :QTextOption__Flag = 4;
// 
pub const QTextOption__SuppressColors :QTextOption__Flag = 8;
// 
pub const QTextOption__ShowDocumentTerminator :QTextOption__Flag = 16;
// 
pub const QTextOption__IncludeTrailingSpaces :QTextOption__Flag = -2147483648;
pub fn QTextOption_FlagItemName(val: i32) ->String {
  match val {
     QTextOption__ShowTabsAndSpaces => // 1
     {return String::from("ShowTabsAndSpaces");}
     QTextOption__ShowLineAndParagraphSeparators => // 2
     {return String::from("ShowLineAndParagraphSeparators");}
     QTextOption__AddSpaceForLineAndParagraphSeparators => // 4
     {return String::from("AddSpaceForLineAndParagraphSeparators");}
     QTextOption__SuppressColors => // 8
     {return String::from("SuppressColors");}
     QTextOption__ShowDocumentTerminator => // 16
     {return String::from("ShowDocumentTerminator");}
     QTextOption__IncludeTrailingSpaces => // -2147483648
     {return String::from("IncludeTrailingSpaces");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextOption_FlagItemName_s(val: i32) ->String {
  //var nilthis *QTextOption
  //return nilthis.FlagItemName(val);
  return QTextOption_FlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
