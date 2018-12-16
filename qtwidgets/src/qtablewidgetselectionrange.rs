

// mod ::widgets::QTableWidgetSelectionRange
// package qtwidgets
// /usr/include/qt/QtWidgets/qtablewidget.h
// #include <qtablewidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 77
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



/*

*/
#[derive(Default)] // class sizeof(QTableWidgetSelectionRange)=16
pub struct QTableWidgetSelectionRange {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTableWidgetSelectionRange_ITF interface {
//    QTableWidgetSelectionRange_PTR() *QTableWidgetSelectionRange
//}
//func (ptr *QTableWidgetSelectionRange) QTableWidgetSelectionRange_PTR() *QTableWidgetSelectionRange { return ptr }

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTableWidgetSelectionRange {
    return QTableWidgetSelectionRange{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTableWidgetSelectionRange {
//  type Target = QTableWidgetSelectionRangeBASE;
//
//  fn deref(&self) -> &QTableWidgetSelectionRangeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTableWidgetSelectionRangeBASE> for QTableWidgetSelectionRange {
//  fn as_ref(& self) -> & QTableWidgetSelectionRangeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtablewidget.h:55
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTableWidgetSelectionRange()

/*

*/
// QTableWidgetSelectionRange() ctx.fn_proto_cpp
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn QTableWidgetSelectionRange_0<T: QTableWidgetSelectionRange_QTableWidgetSelectionRange_0>(value: T) -> QTableWidgetSelectionRange {
    let rsthis = value.QTableWidgetSelectionRange_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_QTableWidgetSelectionRange_0 {
  fn QTableWidgetSelectionRange_0(self) -> QTableWidgetSelectionRange;
}
// QTableWidgetSelectionRange() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTableWidgetSelectionRange_QTableWidgetSelectionRange_0 for () {
  fn QTableWidgetSelectionRange_0(self) -> QTableWidgetSelectionRange {
    // unsafe{_ZN26QTableWidgetSelectionRangeC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QTableWidgetSelectionRangeC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTableWidgetSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:56
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTableWidgetSelectionRange(int, int, int, int)

/*

*/
// QTableWidgetSelectionRange(int, int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn QTableWidgetSelectionRange_1<T: QTableWidgetSelectionRange_QTableWidgetSelectionRange_1>(value: T) -> QTableWidgetSelectionRange {
    let rsthis = value.QTableWidgetSelectionRange_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_QTableWidgetSelectionRange_1 {
  fn QTableWidgetSelectionRange_1(self) -> QTableWidgetSelectionRange;
}
// QTableWidgetSelectionRange(int, int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTableWidgetSelectionRange_QTableWidgetSelectionRange_1 for (i32,i32,i32,i32) {
  fn QTableWidgetSelectionRange_1(self) -> QTableWidgetSelectionRange {
    // unsafe{_ZN26QTableWidgetSelectionRangeC2Eiiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QTableWidgetSelectionRangeC2Eiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTableWidgetSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTableWidgetSelectionRange()

/*

*/
pub fn DeleteQTableWidgetSelectionRange(this :*mut QTableWidgetSelectionRange) {
    // let rv = qtrt::InvokeQtFunc6("_ZN26QTableWidgetSelectionRangeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtablewidget.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int topRow() const

/*

*/
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn topRow_0<RetType, T: QTableWidgetSelectionRange_topRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topRow_0(self);
    // return 1;
  }
}
pub trait QTableWidgetSelectionRange_topRow_0<RetType> {
  fn topRow_0(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetSelectionRange_topRow_0<i32> for () {
  fn topRow_0(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QTableWidgetSelectionRange6topRowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:61
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int bottomRow() const

/*

*/
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn bottomRow_0<RetType, T: QTableWidgetSelectionRange_bottomRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomRow_0(self);
    // return 1;
  }
}
pub trait QTableWidgetSelectionRange_bottomRow_0<RetType> {
  fn bottomRow_0(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetSelectionRange_bottomRow_0<i32> for () {
  fn bottomRow_0(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QTableWidgetSelectionRange9bottomRowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int leftColumn() const

/*

*/
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn leftColumn_0<RetType, T: QTableWidgetSelectionRange_leftColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftColumn_0(self);
    // return 1;
  }
}
pub trait QTableWidgetSelectionRange_leftColumn_0<RetType> {
  fn leftColumn_0(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetSelectionRange_leftColumn_0<i32> for () {
  fn leftColumn_0(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QTableWidgetSelectionRange10leftColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int rightColumn() const

/*

*/
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn rightColumn_0<RetType, T: QTableWidgetSelectionRange_rightColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightColumn_0(self);
    // return 1;
  }
}
pub trait QTableWidgetSelectionRange_rightColumn_0<RetType> {
  fn rightColumn_0(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetSelectionRange_rightColumn_0<i32> for () {
  fn rightColumn_0(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QTableWidgetSelectionRange11rightColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int rowCount() const

/*
Returns the number of rows.

Note: Getter function for property rowCount. 

See also setRowCount().
*/
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn rowCount_0<RetType, T: QTableWidgetSelectionRange_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QTableWidgetSelectionRange_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetSelectionRange_rowCount_0<i32> for () {
  fn rowCount_0(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QTableWidgetSelectionRange8rowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int columnCount() const

/*
Returns the number of columns.

Note: Getter function for property columnCount. 

See also setColumnCount().
*/
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn columnCount_0<RetType, T: QTableWidgetSelectionRange_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QTableWidgetSelectionRange_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetSelectionRange_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QTableWidgetSelectionRange11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
