

// mod ::gui::QPageLayout
// package qtgui
// /usr/include/qt/QtGui/qpagelayout.h
// #include <qpagelayout.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 33
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
#[derive(Default)] // class sizeof(QPageLayout)=8
pub struct QPageLayout {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPageLayout_ITF interface {
//    QPageLayout_PTR() *QPageLayout
//}
//func (ptr *QPageLayout) QPageLayout_PTR() *QPageLayout { return ptr }

impl /*struct*/ QPageLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPageLayout {
    return QPageLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPageLayout {
//  type Target = QPageLayoutBASE;
//
//  fn deref(&self) -> &QPageLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPageLayoutBASE> for QPageLayout {
//  fn as_ref(& self) -> & QPageLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpagelayout.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPageLayout()

/*
Creates an invalid QPageLayout.
*/
// QPageLayout() ctx.fn_proto_cpp
impl /*struct*/ QPageLayout {
  pub fn QPageLayout_0<T: QPageLayout_QPageLayout_0>(value: T) -> QPageLayout {
    let rsthis = value.QPageLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPageLayout_QPageLayout_0 {
  fn QPageLayout_0(self) -> QPageLayout;
}
// QPageLayout() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPageLayout_QPageLayout_0 for () {
  fn QPageLayout_0(self) -> QPageLayout {
    // unsafe{_ZN11QPageLayoutC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QPageLayoutC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPageLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:81
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPageLayout(const QPageSize &, QPageLayout::Orientation, const QMarginsF &, QPageLayout::Unit, const QMarginsF &)

/*
Creates an invalid QPageLayout.
*/
// QPageLayout(const QPageSize &, QPageLayout::Orientation, const QMarginsF &, QPageLayout::Unit, const QMarginsF &) ctx.fn_proto_cpp
impl /*struct*/ QPageLayout {
  pub fn QPageLayout_1<T: QPageLayout_QPageLayout_1>(value: T) -> QPageLayout {
    let rsthis = value.QPageLayout_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPageLayout_QPageLayout_1 {
  fn QPageLayout_1(self) -> QPageLayout;
}
// QPageLayout(const QPageSize &, QPageLayout::Orientation, const QMarginsF &, QPageLayout::Unit, const QMarginsF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPageLayout_QPageLayout_1 for (usize,i32,usize,i32,usize) {
  fn QPageLayout_1(self) -> QPageLayout {
    // unsafe{_ZN11QPageLayoutC2ERK9QPageSizeNS_11OrientationERK9QMarginsFNS_4UnitES6_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QPageLayoutC2ERK9QPageSizeNS_11OrientationERK9QMarginsFNS_4UnitES6_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPageLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPageLayout & operator=(QPageLayout &&)

/*

*/
impl /*struct*/ QPageLayout {
  pub fn operator_equal_0<RetType, T: QPageLayout_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPageLayout_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QPageLayoutaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:88
// index:1
// Public Visibility=Default Availability=Available
// [8] QPageLayout & operator=(const QPageLayout &)

/*

*/
impl /*struct*/ QPageLayout {
  pub fn operator_equal_1<RetType, T: QPageLayout_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPageLayout_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QPageLayoutaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPageLayout()

/*

*/
pub fn DeleteQPageLayout(this :*mut QPageLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QPageLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpagelayout.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPageLayout &)

/*
Swaps this page layout with other. This function is very fast and never fails.
*/
impl /*struct*/ QPageLayout {
  pub fn swap_0<RetType, T: QPageLayout_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPageLayout_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPageLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPageLayout4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEquivalentTo(const QPageLayout &) const

/*
Returns true if this page layout is equivalent to the other page layout, i.e. if the page has the same size, margins and orientation.
*/
impl /*struct*/ QPageLayout {
  pub fn isEquivalentTo_0<RetType, T: QPageLayout_isEquivalentTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEquivalentTo_0(self);
    // return 1;
  }
}
pub trait QPageLayout_isEquivalentTo_0<RetType> {
  fn isEquivalentTo_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_isEquivalentTo_0<bool> for (usize) {
  fn isEquivalentTo_0(self , rsthis: & QPageLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout14isEquivalentToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if this page layout is valid.
*/
impl /*struct*/ QPageLayout {
  pub fn isValid_0<RetType, T: QPageLayout_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QPageLayout_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QPageLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMode(QPageLayout::Mode)

/*
Sets a page layout mode to mode.

See also mode().
*/
impl /*struct*/ QPageLayout {
  pub fn setMode_0<RetType, T: QPageLayout_setMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMode_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setMode_0<RetType> {
  fn setMode_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setMode_0<(/*void*/)> for (i32) {
  fn setMode_0(self , rsthis: & QPageLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QPageLayout7setModeENS_4ModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] QPageLayout::Mode mode() const

/*
Returns the page layout mode.

See also setMode().
*/
impl /*struct*/ QPageLayout {
  pub fn mode_0<RetType, T: QPageLayout_mode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mode_0(self);
    // return 1;
  }
}
pub trait QPageLayout_mode_0<RetType> {
  fn mode_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_mode_0<i32> for () {
  fn mode_0(self , rsthis: & QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout4modeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPageSize(const QPageSize &, const QMarginsF &)

/*
Sets the page size of the page layout to pageSize.

Optionally define the minimum allowed margins minMargins, e.g. the minimum margins able to be printed by a physical print device, otherwise the minimum margins will default to 0.

If StandardMode is set then the existing margins will be clamped to the new minimum margins and the maximum margins allowed by the page size. If FullPageMode is set then the existing margins will be unchanged.

See also pageSize().
*/
impl /*struct*/ QPageLayout {
  pub fn setPageSize_0<RetType, T: QPageLayout_setPageSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageSize_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setPageSize_0<RetType> {
  fn setPageSize_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setPageSize_0<(/*void*/)> for (usize,usize) {
  fn setPageSize_0(self , rsthis: & QPageLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QPageSize pageSize() const

/*
Returns the page size of the page layout.

Note that the QPageSize is always defined in a Portrait orientation. To obtain a size that takes the set orientation into account you must use fullRect().

See also setPageSize().
*/
impl /*struct*/ QPageLayout {
  pub fn pageSize_0<RetType, T: QPageLayout_pageSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageSize_0(self);
    // return 1;
  }
}
pub trait QPageLayout_pageSize_0<RetType> {
  fn pageSize_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_pageSize_0<usize> for () {
  fn pageSize_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout8pageSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(QPageLayout::Orientation)

/*
Sets the page orientation of the page layout to orientation.

Changing the orientation does not affect the current margins or the minimum margins.

See also orientation().
*/
impl /*struct*/ QPageLayout {
  pub fn setOrientation_0<RetType, T: QPageLayout_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QPageLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QPageLayout14setOrientationENS_11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:106
// index:0
// Public Visibility=Default Availability=Available
// [4] QPageLayout::Orientation orientation() const

/*
Returns the page orientation of the page layout.

See also setOrientation().
*/
impl /*struct*/ QPageLayout {
  pub fn orientation_0<RetType, T: QPageLayout_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QPageLayout_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUnits(QPageLayout::Unit)

/*
Sets the units used to define the page layout.

See also units().
*/
impl /*struct*/ QPageLayout {
  pub fn setUnits_0<RetType, T: QPageLayout_setUnits_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUnits_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setUnits_0<RetType> {
  fn setUnits_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setUnits_0<(/*void*/)> for (i32) {
  fn setUnits_0(self , rsthis: & QPageLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QPageLayout8setUnitsENS_4UnitE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:109
// index:0
// Public Visibility=Default Availability=Available
// [4] QPageLayout::Unit units() const

/*
Returns the units the page layout is currently defined in.

See also setUnits().
*/
impl /*struct*/ QPageLayout {
  pub fn units_0<RetType, T: QPageLayout_units_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.units_0(self);
    // return 1;
  }
}
pub trait QPageLayout_units_0<RetType> {
  fn units_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_units_0<i32> for () {
  fn units_0(self , rsthis: & QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout5unitsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setMargins(const QMarginsF &)

/*
Sets the page margins of the page layout to margins Returns true if the margins were successfully set.

The units used are those currently defined for the layout. To use different units then call setUnits() first.

If in the default StandardMode then all the new margins must fall between the minimum margins set and the maximum margins allowed by the page size, otherwise the margins will not be set.

If in FullPageMode then any margin values will be accepted.

See also margins() and units().
*/
impl /*struct*/ QPageLayout {
  pub fn setMargins_0<RetType, T: QPageLayout_setMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMargins_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setMargins_0<RetType> {
  fn setMargins_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setMargins_0<bool> for (usize) {
  fn setMargins_0(self , rsthis: & QPageLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QPageLayout10setMarginsERK9QMarginsF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:112
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setLeftMargin(qreal)

/*
Sets the left page margin of the page layout to leftMargin. Returns true if the margin was successfully set.

The units used are those currently defined for the layout. To use different units call setUnits() first.

If in the default StandardMode then the new margin must fall between the minimum margin set and the maximum margin allowed by the page size, otherwise the margin will not be set.

If in FullPageMode then any margin values will be accepted.

See also setMargins() and margins().
*/
impl /*struct*/ QPageLayout {
  pub fn setLeftMargin_0<RetType, T: QPageLayout_setLeftMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeftMargin_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setLeftMargin_0<RetType> {
  fn setLeftMargin_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setLeftMargin_0<bool> for (f64) {
  fn setLeftMargin_0(self , rsthis: & QPageLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QPageLayout13setLeftMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setRightMargin(qreal)

/*
Sets the right page margin of the page layout to rightMargin. Returns true if the margin was successfully set.

The units used are those currently defined for the layout. To use different units call setUnits() first.

If in the default StandardMode then the new margin must fall between the minimum margin set and the maximum margin allowed by the page size, otherwise the margin will not be set.

If in FullPageMode then any margin values will be accepted.

See also setMargins() and margins().
*/
impl /*struct*/ QPageLayout {
  pub fn setRightMargin_0<RetType, T: QPageLayout_setRightMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRightMargin_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setRightMargin_0<RetType> {
  fn setRightMargin_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setRightMargin_0<bool> for (f64) {
  fn setRightMargin_0(self , rsthis: & QPageLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QPageLayout14setRightMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:114
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setTopMargin(qreal)

/*
Sets the top page margin of the page layout to topMargin. Returns true if the margin was successfully set.

The units used are those currently defined for the layout. To use different units call setUnits() first.

If in the default StandardMode then the new margin must fall between the minimum margin set and the maximum margin allowed by the page size, otherwise the margin will not be set.

If in FullPageMode then any margin values will be accepted.

See also setMargins() and margins().
*/
impl /*struct*/ QPageLayout {
  pub fn setTopMargin_0<RetType, T: QPageLayout_setTopMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTopMargin_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setTopMargin_0<RetType> {
  fn setTopMargin_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setTopMargin_0<bool> for (f64) {
  fn setTopMargin_0(self , rsthis: & QPageLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QPageLayout12setTopMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setBottomMargin(qreal)

/*
Sets the bottom page margin of the page layout to bottomMargin. Returns true if the margin was successfully set.

The units used are those currently defined for the layout. To use different units call setUnits() first.

If in the default StandardMode then the new margin must fall between the minimum margin set and the maximum margin allowed by the page size, otherwise the margin will not be set.

If in FullPageMode then any margin values will be accepted.

See also setMargins() and margins().
*/
impl /*struct*/ QPageLayout {
  pub fn setBottomMargin_0<RetType, T: QPageLayout_setBottomMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottomMargin_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setBottomMargin_0<RetType> {
  fn setBottomMargin_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setBottomMargin_0<bool> for (f64) {
  fn setBottomMargin_0(self , rsthis: & QPageLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QPageLayout15setBottomMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:117
// index:0
// Public Visibility=Default Availability=Available
// [32] QMarginsF margins() const

/*
Returns the margins of the page layout using the currently set units.

See also setMargins() and units().
*/
impl /*struct*/ QPageLayout {
  pub fn margins_0<RetType, T: QPageLayout_margins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.margins_0(self);
    // return 1;
  }
}
pub trait QPageLayout_margins_0<RetType> {
  fn margins_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_margins_0<usize> for () {
  fn margins_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout7marginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:118
// index:1
// Public Visibility=Default Availability=Available
// [32] QMarginsF margins(QPageLayout::Unit) const

/*
Returns the margins of the page layout using the currently set units.

See also setMargins() and units().
*/
impl /*struct*/ QPageLayout {
  pub fn margins_1<RetType, T: QPageLayout_margins_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.margins_1(self);
    // return 1;
  }
}
pub trait QPageLayout_margins_1<RetType> {
  fn margins_1(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_margins_1<usize> for (i32) {
  fn margins_1(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout7marginsENS_4UnitE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:119
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins marginsPoints() const

/*
Returns the margins of the page layout in Postscript Points (1/72 of an inch).

See also setMargins() and margins().
*/
impl /*struct*/ QPageLayout {
  pub fn marginsPoints_0<RetType, T: QPageLayout_marginsPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.marginsPoints_0(self);
    // return 1;
  }
}
pub trait QPageLayout_marginsPoints_0<RetType> {
  fn marginsPoints_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_marginsPoints_0<usize> for () {
  fn marginsPoints_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout13marginsPointsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:120
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins marginsPixels(int) const

/*
Returns the margins of the page layout in device pixels for the given resolution.

See also setMargins().
*/
impl /*struct*/ QPageLayout {
  pub fn marginsPixels_0<RetType, T: QPageLayout_marginsPixels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.marginsPixels_0(self);
    // return 1;
  }
}
pub trait QPageLayout_marginsPixels_0<RetType> {
  fn marginsPixels_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_marginsPixels_0<usize> for (i32) {
  fn marginsPixels_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout13marginsPixelsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumMargins(const QMarginsF &)

/*
Sets the minimum page margins of the page layout to minMargins.

It is not recommended to override the default values set for a page size as this may be the minimum printable area for a physical print device.

If the StandardMode mode is set then the existing margins will be clamped to the new minMargins and the maximum allowed by the page size. If the FullPageMode is set then the existing margins will be unchanged.

See also minimumMargins() and setMargins().
*/
impl /*struct*/ QPageLayout {
  pub fn setMinimumMargins_0<RetType, T: QPageLayout_setMinimumMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumMargins_0(self);
    // return 1;
  }
}
pub trait QPageLayout_setMinimumMargins_0<RetType> {
  fn setMinimumMargins_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_setMinimumMargins_0<(/*void*/)> for (usize) {
  fn setMinimumMargins_0(self , rsthis: & QPageLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPageLayout17setMinimumMarginsERK9QMarginsF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:123
// index:0
// Public Visibility=Default Availability=Available
// [32] QMarginsF minimumMargins() const

/*
Returns the minimum margins of the page layout.

See also setMinimumMargins() and maximumMargins().
*/
impl /*struct*/ QPageLayout {
  pub fn minimumMargins_0<RetType, T: QPageLayout_minimumMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumMargins_0(self);
    // return 1;
  }
}
pub trait QPageLayout_minimumMargins_0<RetType> {
  fn minimumMargins_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_minimumMargins_0<usize> for () {
  fn minimumMargins_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout14minimumMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:124
// index:0
// Public Visibility=Default Availability=Available
// [32] QMarginsF maximumMargins() const

/*
Returns the maximum margins that would be applied if the page layout was in StandardMode.

The maximum margins allowed are calculated as the full size of the page minus the minimum margins set. For example, if the page width is 100 points and the minimum right margin is 10 points, then the maximum left margin will be 90 points.

See also setMinimumMargins() and minimumMargins().
*/
impl /*struct*/ QPageLayout {
  pub fn maximumMargins_0<RetType, T: QPageLayout_maximumMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumMargins_0(self);
    // return 1;
  }
}
pub trait QPageLayout_maximumMargins_0<RetType> {
  fn maximumMargins_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_maximumMargins_0<usize> for () {
  fn maximumMargins_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout14maximumMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:126
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF fullRect() const

/*
Returns the full page rectangle in the current layout units.

The page rectangle takes into account the page size and page orientation, but not the page margins.

See also paintRect() and units().
*/
impl /*struct*/ QPageLayout {
  pub fn fullRect_0<RetType, T: QPageLayout_fullRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fullRect_0(self);
    // return 1;
  }
}
pub trait QPageLayout_fullRect_0<RetType> {
  fn fullRect_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_fullRect_0<usize> for () {
  fn fullRect_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout8fullRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:127
// index:1
// Public Visibility=Default Availability=Available
// [32] QRectF fullRect(QPageLayout::Unit) const

/*
Returns the full page rectangle in the current layout units.

The page rectangle takes into account the page size and page orientation, but not the page margins.

See also paintRect() and units().
*/
impl /*struct*/ QPageLayout {
  pub fn fullRect_1<RetType, T: QPageLayout_fullRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fullRect_1(self);
    // return 1;
  }
}
pub trait QPageLayout_fullRect_1<RetType> {
  fn fullRect_1(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_fullRect_1<usize> for (i32) {
  fn fullRect_1(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout8fullRectENS_4UnitE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:128
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect fullRectPoints() const

/*
Returns the full page rectangle in Postscript Points (1/72 of an inch).

The page rectangle takes into account the page size and page orientation, but not the page margins.

See also paintRect().
*/
impl /*struct*/ QPageLayout {
  pub fn fullRectPoints_0<RetType, T: QPageLayout_fullRectPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fullRectPoints_0(self);
    // return 1;
  }
}
pub trait QPageLayout_fullRectPoints_0<RetType> {
  fn fullRectPoints_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_fullRectPoints_0<usize> for () {
  fn fullRectPoints_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout14fullRectPointsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:129
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect fullRectPixels(int) const

/*
Returns the full page rectangle in device pixels for the given resolution.

The page rectangle takes into account the page size and page orientation, but not the page margins.

See also paintRect().
*/
impl /*struct*/ QPageLayout {
  pub fn fullRectPixels_0<RetType, T: QPageLayout_fullRectPixels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fullRectPixels_0(self);
    // return 1;
  }
}
pub trait QPageLayout_fullRectPixels_0<RetType> {
  fn fullRectPixels_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_fullRectPixels_0<usize> for (i32) {
  fn fullRectPixels_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout14fullRectPixelsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:131
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF paintRect() const

/*
Returns the page rectangle in the current layout units.

The paintable rectangle takes into account the page size, orientation and margins.

If the FullPageMode mode is set then the fullRect() is returned and the margins must be manually managed.
*/
impl /*struct*/ QPageLayout {
  pub fn paintRect_0<RetType, T: QPageLayout_paintRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintRect_0(self);
    // return 1;
  }
}
pub trait QPageLayout_paintRect_0<RetType> {
  fn paintRect_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_paintRect_0<usize> for () {
  fn paintRect_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout9paintRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:132
// index:1
// Public Visibility=Default Availability=Available
// [32] QRectF paintRect(QPageLayout::Unit) const

/*
Returns the page rectangle in the current layout units.

The paintable rectangle takes into account the page size, orientation and margins.

If the FullPageMode mode is set then the fullRect() is returned and the margins must be manually managed.
*/
impl /*struct*/ QPageLayout {
  pub fn paintRect_1<RetType, T: QPageLayout_paintRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintRect_1(self);
    // return 1;
  }
}
pub trait QPageLayout_paintRect_1<RetType> {
  fn paintRect_1(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_paintRect_1<usize> for (i32) {
  fn paintRect_1(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout9paintRectENS_4UnitE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:133
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect paintRectPoints() const

/*
Returns the paintable rectangle in rounded Postscript Points (1/72 of an inch).

The paintable rectangle takes into account the page size, orientation and margins.

If the FullPageMode mode is set then the fullRect() is returned and the margins must be manually managed.
*/
impl /*struct*/ QPageLayout {
  pub fn paintRectPoints_0<RetType, T: QPageLayout_paintRectPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintRectPoints_0(self);
    // return 1;
  }
}
pub trait QPageLayout_paintRectPoints_0<RetType> {
  fn paintRectPoints_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_paintRectPoints_0<usize> for () {
  fn paintRectPoints_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout15paintRectPointsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagelayout.h:134
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect paintRectPixels(int) const

/*
Returns the paintable rectangle in rounded device pixels for the given resolution.

The paintable rectangle takes into account the page size, orientation and margins.

If the FullPageMode mode is set then the fullRect() is returned and the margins must be manually managed.
*/
impl /*struct*/ QPageLayout {
  pub fn paintRectPixels_0<RetType, T: QPageLayout_paintRectPixels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintRectPixels_0(self);
    // return 1;
  }
}
pub trait QPageLayout_paintRectPixels_0<RetType> {
  fn paintRectPixels_0(self , rsthis: & QPageLayout) -> RetType;
}
impl<'a> /*trait*/ QPageLayout_paintRectPixels_0<usize> for (i32) {
  fn paintRectPixels_0(self , rsthis: & QPageLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPageLayout15paintRectPixelsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum type is used to specify the measurement unit for page layout and margins.


*/
pub type QPageLayout__Unit = i32;
//  
pub const QPageLayout__Millimeter :QPageLayout__Unit = 0;
// 
pub const QPageLayout__Point :QPageLayout__Unit = 1;
//  
pub const QPageLayout__Inch :QPageLayout__Unit = 2;
// 
pub const QPageLayout__Pica :QPageLayout__Unit = 3;
// 
pub const QPageLayout__Didot :QPageLayout__Unit = 4;
// 
pub const QPageLayout__Cicero :QPageLayout__Unit = 5;
pub fn QPageLayout_UnitItemName(val: i32) ->String {
  match val {
     QPageLayout__Millimeter => // 0
     {return String::from("Millimeter");}
     QPageLayout__Point => // 1
     {return String::from("Point");}
     QPageLayout__Inch => // 2
     {return String::from("Inch");}
     QPageLayout__Pica => // 3
     {return String::from("Pica");}
     QPageLayout__Didot => // 4
     {return String::from("Didot");}
     QPageLayout__Cicero => // 5
     {return String::from("Cicero");}
  _ => {return format!("{}", val);}
}
}
pub fn QPageLayout_UnitItemName_s(val: i32) ->String {
  //var nilthis *QPageLayout
  //return nilthis.UnitItemName(val);
  return QPageLayout_UnitItemName(val);
}


/*
This enum type defines the page orientation



Note that some standard page sizes are defined with a width larger than their height, hence the orientation is defined relative to the standard page size and not using the relative page dimensions.

*/
pub type QPageLayout__Orientation = i32;
// The page size is used in its default orientation
pub const QPageLayout__Portrait :QPageLayout__Orientation = 0;
// 
pub const QPageLayout__Landscape :QPageLayout__Orientation = 1;
pub fn QPageLayout_OrientationItemName(val: i32) ->String {
  match val {
     QPageLayout__Portrait => // 0
     {return String::from("Portrait");}
     QPageLayout__Landscape => // 1
     {return String::from("Landscape");}
  _ => {return format!("{}", val);}
}
}
pub fn QPageLayout_OrientationItemName_s(val: i32) ->String {
  //var nilthis *QPageLayout
  //return nilthis.OrientationItemName(val);
  return QPageLayout_OrientationItemName(val);
}


/*
Defines the page layout mode


*/
pub type QPageLayout__Mode = i32;
// Paint Rect includes margins, margins must fall between the minimum and maximum.
pub const QPageLayout__StandardMode :QPageLayout__Mode = 0;
// Paint Rect excludes margins, margins can be any value and must be managed manually.
pub const QPageLayout__FullPageMode :QPageLayout__Mode = 1;
pub fn QPageLayout_ModeItemName(val: i32) ->String {
  match val {
     QPageLayout__StandardMode => // 0
     {return String::from("StandardMode");}
     QPageLayout__FullPageMode => // 1
     {return String::from("FullPageMode");}
  _ => {return format!("{}", val);}
}
}
pub fn QPageLayout_ModeItemName_s(val: i32) ->String {
  //var nilthis *QPageLayout
  //return nilthis.ModeItemName(val);
  return QPageLayout_ModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
