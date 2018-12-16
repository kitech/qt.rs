

// mod ::core::QItemSelectionRange
// package qtcore
// /usr/include/qt/QtCore/qitemselectionmodel.h
// #include <qitemselectionmodel.h>
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QItemSelectionRange)=16
pub struct QItemSelectionRange {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QItemSelectionRange_ITF interface {
//    QItemSelectionRange_PTR() *QItemSelectionRange
//}
//func (ptr *QItemSelectionRange) QItemSelectionRange_PTR() *QItemSelectionRange { return ptr }

impl /*struct*/ QItemSelectionRange {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QItemSelectionRange {
    return QItemSelectionRange{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QItemSelectionRange {
//  type Target = QItemSelectionRangeBASE;
//
//  fn deref(&self) -> &QItemSelectionRangeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QItemSelectionRangeBASE> for QItemSelectionRange {
//  fn as_ref(& self) -> & QItemSelectionRangeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qitemselectionmodel.h:56
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QItemSelectionRange()

/*

*/
// QItemSelectionRange() ctx.fn_proto_cpp
impl /*struct*/ QItemSelectionRange {
  pub fn QItemSelectionRange_0<T: QItemSelectionRange_QItemSelectionRange_0>(value: T) -> QItemSelectionRange {
    let rsthis = value.QItemSelectionRange_0();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionRange_QItemSelectionRange_0 {
  fn QItemSelectionRange_0(self) -> QItemSelectionRange;
}
// QItemSelectionRange() ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemSelectionRange_QItemSelectionRange_0 for () {
  fn QItemSelectionRange_0(self) -> QItemSelectionRange {
    // unsafe{_ZN19QItemSelectionRangeC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QItemSelectionRangeC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:70
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QItemSelectionRange(const QModelIndex &, const QModelIndex &)

/*

*/
// QItemSelectionRange(const QModelIndex &, const QModelIndex &) ctx.fn_proto_cpp
impl /*struct*/ QItemSelectionRange {
  pub fn QItemSelectionRange_1<T: QItemSelectionRange_QItemSelectionRange_1>(value: T) -> QItemSelectionRange {
    let rsthis = value.QItemSelectionRange_1();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionRange_QItemSelectionRange_1 {
  fn QItemSelectionRange_1(self) -> QItemSelectionRange;
}
// QItemSelectionRange(const QModelIndex &, const QModelIndex &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemSelectionRange_QItemSelectionRange_1 for (usize,usize) {
  fn QItemSelectionRange_1(self) -> QItemSelectionRange {
    // unsafe{_ZN19QItemSelectionRangeC2ERK11QModelIndexS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QItemSelectionRangeC2ERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:71
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QItemSelectionRange(const QModelIndex &)

/*

*/
// QItemSelectionRange(const QModelIndex &) ctx.fn_proto_cpp
impl /*struct*/ QItemSelectionRange {
  pub fn QItemSelectionRange_2<T: QItemSelectionRange_QItemSelectionRange_2>(value: T) -> QItemSelectionRange {
    let rsthis = value.QItemSelectionRange_2();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionRange_QItemSelectionRange_2 {
  fn QItemSelectionRange_2(self) -> QItemSelectionRange;
}
// QItemSelectionRange(const QModelIndex &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemSelectionRange_QItemSelectionRange_2 for (usize) {
  fn QItemSelectionRange_2(self) -> QItemSelectionRange {
    // unsafe{_ZN19QItemSelectionRangeC2ERK11QModelIndex()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QItemSelectionRangeC2ERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QItemSelectionRange & operator=(QItemSelectionRange &&)

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn operator_equal_0<RetType, T: QItemSelectionRange_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QItemSelectionRange) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QItemSelectionRangeaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:67
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QItemSelectionRange & operator=(const QItemSelectionRange &)

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn operator_equal_1<RetType, T: QItemSelectionRange_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QItemSelectionRange) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QItemSelectionRangeaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QItemSelectionRange &)

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn swap_0<RetType, T: QItemSelectionRange_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_swap_0<RetType> {
  fn swap_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QItemSelectionRange) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionRange4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int top() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn top_0<RetType, T: QItemSelectionRange_top_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.top_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_top_0<RetType> {
  fn top_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_top_0<i32> for () {
  fn top_0(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange3topEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int left() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn left_0<RetType, T: QItemSelectionRange_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_left_0<RetType> {
  fn left_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_left_0<i32> for () {
  fn left_0(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange4leftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int bottom() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn bottom_0<RetType, T: QItemSelectionRange_bottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottom_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_bottom_0<RetType> {
  fn bottom_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_bottom_0<i32> for () {
  fn bottom_0(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange6bottomEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int right() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn right_0<RetType, T: QItemSelectionRange_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_right_0<RetType> {
  fn right_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_right_0<i32> for () {
  fn right_0(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange5rightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int width() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn width_0<RetType, T: QItemSelectionRange_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_width_0<RetType> {
  fn width_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_width_0<i32> for () {
  fn width_0(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int height() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn height_0<RetType, T: QItemSelectionRange_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_height_0<RetType> {
  fn height_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_height_0<i32> for () {
  fn height_0(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPersistentModelIndex & topLeft() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn topLeft_0<RetType, T: QItemSelectionRange_topLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLeft_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_topLeft_0<RetType> {
  fn topLeft_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_topLeft_0<usize> for () {
  fn topLeft_0(self , rsthis: & QItemSelectionRange) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange7topLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:87
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPersistentModelIndex & bottomRight() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn bottomRight_0<RetType, T: QItemSelectionRange_bottomRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomRight_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_bottomRight_0<RetType> {
  fn bottomRight_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_bottomRight_0<usize> for () {
  fn bottomRight_0(self , rsthis: & QItemSelectionRange) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange11bottomRightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:88
// index:0
// Public inline Visibility=Default Availability=Available
// [24] QModelIndex parent() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn parent_0<RetType, T: QItemSelectionRange_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_parent_0<RetType> {
  fn parent_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QItemSelectionRange) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QAbstractItemModel * model() const

/*
Returns the item model operated on by the selection model.

See also setModel().
*/
impl /*struct*/ QItemSelectionRange {
  pub fn model_0<RetType, T: QItemSelectionRange_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_model_0<RetType> {
  fn model_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_model_0<usize> for () {
  fn model_0(self , rsthis: & QItemSelectionRange) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool contains(const QModelIndex &) const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn contains_0<RetType, T: QItemSelectionRange_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_contains_0<RetType> {
  fn contains_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QItemSelectionRange) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange8containsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:98
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool contains(int, int, const QModelIndex &) const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn contains_1<RetType, T: QItemSelectionRange_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_contains_1<RetType> {
  fn contains_1(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_contains_1<bool> for (i32,i32,usize) {
  fn contains_1(self , rsthis: & QItemSelectionRange) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange8containsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QItemSelectionRange &) const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn intersects_0<RetType, T: QItemSelectionRange_intersects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_intersects_0<RetType> {
  fn intersects_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_intersects_0<bool> for (usize) {
  fn intersects_0(self , rsthis: & QItemSelectionRange) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange10intersectsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:110
// index:0
// Public Visibility=Default Availability=Available
// [16] QItemSelectionRange intersected(const QItemSelectionRange &) const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn intersected_0<RetType, T: QItemSelectionRange_intersected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersected_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_intersected_0<RetType> {
  fn intersected_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_intersected_0<usize> for (usize) {
  fn intersected_0(self , rsthis: & QItemSelectionRange) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange11intersectedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:113
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QItemSelectionRange &) const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn operator_equal_equal_0<RetType, T: QItemSelectionRange_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QItemSelectionRange) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRangeeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:115
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QItemSelectionRange &) const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn operator_not_equal_0<RetType, T: QItemSelectionRange_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QItemSelectionRange) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRangeneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:117
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(const QItemSelectionRange &) const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn operator_less_than_0<RetType, T: QItemSelectionRange_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QItemSelectionRange) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRangeltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:119
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn isValid_0<RetType, T: QItemSelectionRange_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QItemSelectionRange) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:125
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn isEmpty_0<RetType, T: QItemSelectionRange_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QItemSelectionRange) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] QModelIndexList indexes() const

/*

*/
impl /*struct*/ QItemSelectionRange {
  pub fn indexes_0<RetType, T: QItemSelectionRange_indexes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexes_0(self);
    // return 1;
  }
}
pub trait QItemSelectionRange_indexes_0<RetType> {
  fn indexes_0(self , rsthis: & QItemSelectionRange) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionRange_indexes_0<usize> for () {
  fn indexes_0(self , rsthis: & QItemSelectionRange) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionRange7indexesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQItemSelectionRange(this :*mut QItemSelectionRange) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN19QItemSelectionRangeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
