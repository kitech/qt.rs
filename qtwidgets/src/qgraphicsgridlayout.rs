

// mod ::widgets::QGraphicsGridLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h
// #include <qgraphicsgridlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 10
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
#[derive(Default)] // class sizeof(QGraphicsGridLayout)=16
pub struct QGraphicsGridLayout {
  qbase: QGraphicsLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsGridLayout_ITF interface {
//    QGraphicsLayout_ITF
//    QGraphicsGridLayout_PTR() *QGraphicsGridLayout
//}
//func (ptr *QGraphicsGridLayout) QGraphicsGridLayout_PTR() *QGraphicsGridLayout { return ptr }

impl /*struct*/ QGraphicsGridLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsGridLayout {
    return QGraphicsGridLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsGridLayout {
//  type Target = QGraphicsGridLayoutBASE;
//
//  fn deref(&self) -> &QGraphicsGridLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsGridLayoutBASE> for QGraphicsGridLayout {
//  fn as_ref(& self) -> & QGraphicsGridLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsGridLayout(QGraphicsLayoutItem *)

/*
Constructs a QGraphicsGridLayout instance. parent is passed to QGraphicsLayout's constructor.
*/
// QGraphicsGridLayout(QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsGridLayout {
  pub fn QGraphicsGridLayout_0<T: QGraphicsGridLayout_QGraphicsGridLayout_0>(value: T) -> QGraphicsGridLayout {
    let rsthis = value.QGraphicsGridLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsGridLayout_QGraphicsGridLayout_0 {
  fn QGraphicsGridLayout_0(self) -> QGraphicsGridLayout;
}
// QGraphicsGridLayout(QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsGridLayout_QGraphicsGridLayout_0 for (usize) {
  fn QGraphicsGridLayout_0(self) -> QGraphicsGridLayout {
    // unsafe{_ZN19QGraphicsGridLayoutC2EP19QGraphicsLayoutItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayoutC2EP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsGridLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsGridLayout()

/*

*/
pub fn DeleteQGraphicsGridLayout(this :*mut QGraphicsGridLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addItem(QGraphicsLayoutItem *, int, int, int, int, Qt::Alignment)

/*
Adds item to the grid on row and column. You can specify a rowSpan and columnSpan and an optional alignment.
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn addItem_0<RetType, T: QGraphicsGridLayout_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_addItem_0<(/*void*/)> for (usize,i32,i32,i32,i32,i32) {
  fn addItem_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout7addItemEP19QGraphicsLayoutItemiiii6QFlagsIN2Qt13AlignmentFlagEE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:61
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void addItem(QGraphicsLayoutItem *, int, int, Qt::Alignment)

/*
Adds item to the grid on row and column. You can specify a rowSpan and columnSpan and an optional alignment.
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn addItem_1<RetType, T: QGraphicsGridLayout_addItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_1(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_addItem_1<RetType> {
  fn addItem_1(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_addItem_1<(/*void*/)> for (usize,i32,i32,i32) {
  fn addItem_1(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout7addItemEP19QGraphicsLayoutItemii6QFlagsIN2Qt13AlignmentFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalSpacing(qreal)

/*
Sets the default horizontal spacing for the grid layout to spacing.

See also horizontalSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setHorizontalSpacing_0<RetType, T: QGraphicsGridLayout_setHorizontalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setHorizontalSpacing_0<RetType> {
  fn setHorizontalSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setHorizontalSpacing_0<(/*void*/)> for (f64) {
  fn setHorizontalSpacing_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout20setHorizontalSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:64
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal horizontalSpacing() const

/*
Returns the default horizontal spacing for the grid layout.

See also setHorizontalSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn horizontalSpacing_0<RetType, T: QGraphicsGridLayout_horizontalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_horizontalSpacing_0<RetType> {
  fn horizontalSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_horizontalSpacing_0<f64> for () {
  fn horizontalSpacing_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout17horizontalSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalSpacing(qreal)

/*
Sets the default vertical spacing for the grid layout to spacing.

See also verticalSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setVerticalSpacing_0<RetType, T: QGraphicsGridLayout_setVerticalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setVerticalSpacing_0<RetType> {
  fn setVerticalSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setVerticalSpacing_0<(/*void*/)> for (f64) {
  fn setVerticalSpacing_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout18setVerticalSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal verticalSpacing() const

/*
Returns the default vertical spacing for the grid layout.

See also setVerticalSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn verticalSpacing_0<RetType, T: QGraphicsGridLayout_verticalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_verticalSpacing_0<RetType> {
  fn verticalSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_verticalSpacing_0<f64> for () {
  fn verticalSpacing_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout15verticalSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(qreal)

/*
Sets the grid layout's default spacing, both vertical and horizontal, to spacing.

See also rowSpacing() and columnSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setSpacing_0<RetType, T: QGraphicsGridLayout_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setSpacing_0<(/*void*/)> for (f64) {
  fn setSpacing_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout10setSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowSpacing(int, qreal)

/*
Sets the spacing for row to spacing.

See also rowSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowSpacing_0<RetType, T: QGraphicsGridLayout_setRowSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setRowSpacing_0<RetType> {
  fn setRowSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setRowSpacing_0<(/*void*/)> for (i32,f64) {
  fn setRowSpacing_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout13setRowSpacingEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rowSpacing(int) const

/*
Returns the row spacing for row.

See also setRowSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowSpacing_0<RetType, T: QGraphicsGridLayout_rowSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_rowSpacing_0<RetType> {
  fn rowSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_rowSpacing_0<f64> for (i32) {
  fn rowSpacing_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout10rowSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnSpacing(int, qreal)

/*
Sets the spacing for column to spacing.

See also columnSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnSpacing_0<RetType, T: QGraphicsGridLayout_setColumnSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setColumnSpacing_0<RetType> {
  fn setColumnSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnSpacing_0<(/*void*/)> for (i32,f64) {
  fn setColumnSpacing_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout16setColumnSpacingEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal columnSpacing(int) const

/*
Returns the column spacing for column.

See also setColumnSpacing().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnSpacing_0<RetType, T: QGraphicsGridLayout_columnSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_columnSpacing_0<RetType> {
  fn columnSpacing_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_columnSpacing_0<f64> for (i32) {
  fn columnSpacing_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout13columnSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowStretchFactor(int, int)

/*
Sets the stretch factor for row to stretch.

See also rowStretchFactor().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowStretchFactor_0<RetType, T: QGraphicsGridLayout_setRowStretchFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowStretchFactor_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setRowStretchFactor_0<RetType> {
  fn setRowStretchFactor_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setRowStretchFactor_0<(/*void*/)> for (i32,i32) {
  fn setRowStretchFactor_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout19setRowStretchFactorEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:75
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowStretchFactor(int) const

/*
Returns the stretch factor for row.

See also setRowStretchFactor().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowStretchFactor_0<RetType, T: QGraphicsGridLayout_rowStretchFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowStretchFactor_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_rowStretchFactor_0<RetType> {
  fn rowStretchFactor_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_rowStretchFactor_0<i32> for (i32) {
  fn rowStretchFactor_0(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout16rowStretchFactorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnStretchFactor(int, int)

/*
Sets the stretch factor for column to stretch.

See also columnStretchFactor().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnStretchFactor_0<RetType, T: QGraphicsGridLayout_setColumnStretchFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnStretchFactor_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setColumnStretchFactor_0<RetType> {
  fn setColumnStretchFactor_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnStretchFactor_0<(/*void*/)> for (i32,i32) {
  fn setColumnStretchFactor_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout22setColumnStretchFactorEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnStretchFactor(int) const

/*
Returns the stretch factor for column.

See also setColumnStretchFactor().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnStretchFactor_0<RetType, T: QGraphicsGridLayout_columnStretchFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnStretchFactor_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_columnStretchFactor_0<RetType> {
  fn columnStretchFactor_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_columnStretchFactor_0<i32> for (i32) {
  fn columnStretchFactor_0(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout19columnStretchFactorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowMinimumHeight(int, qreal)

/*
Sets the minimum height for row, row, to height.

See also rowMinimumHeight().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowMinimumHeight_0<RetType, T: QGraphicsGridLayout_setRowMinimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowMinimumHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setRowMinimumHeight_0<RetType> {
  fn setRowMinimumHeight_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setRowMinimumHeight_0<(/*void*/)> for (i32,f64) {
  fn setRowMinimumHeight_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout19setRowMinimumHeightEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:80
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rowMinimumHeight(int) const

/*
Returns the minimum height for row, row.

See also setRowMinimumHeight().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowMinimumHeight_0<RetType, T: QGraphicsGridLayout_rowMinimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowMinimumHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_rowMinimumHeight_0<RetType> {
  fn rowMinimumHeight_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_rowMinimumHeight_0<f64> for (i32) {
  fn rowMinimumHeight_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout16rowMinimumHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowPreferredHeight(int, qreal)

/*
Sets the preferred height for row, row, to height.

See also rowPreferredHeight().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowPreferredHeight_0<RetType, T: QGraphicsGridLayout_setRowPreferredHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowPreferredHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setRowPreferredHeight_0<RetType> {
  fn setRowPreferredHeight_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setRowPreferredHeight_0<(/*void*/)> for (i32,f64) {
  fn setRowPreferredHeight_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout21setRowPreferredHeightEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rowPreferredHeight(int) const

/*
Returns the preferred height for row, row.

See also setRowPreferredHeight().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowPreferredHeight_0<RetType, T: QGraphicsGridLayout_rowPreferredHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowPreferredHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_rowPreferredHeight_0<RetType> {
  fn rowPreferredHeight_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_rowPreferredHeight_0<f64> for (i32) {
  fn rowPreferredHeight_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout18rowPreferredHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowMaximumHeight(int, qreal)

/*
Sets the maximum height for row, row, to height.

See also rowMaximumHeight().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowMaximumHeight_0<RetType, T: QGraphicsGridLayout_setRowMaximumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowMaximumHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setRowMaximumHeight_0<RetType> {
  fn setRowMaximumHeight_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setRowMaximumHeight_0<(/*void*/)> for (i32,f64) {
  fn setRowMaximumHeight_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout19setRowMaximumHeightEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rowMaximumHeight(int) const

/*
Returns the maximum height for row, row.

See also setRowMaximumHeight().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowMaximumHeight_0<RetType, T: QGraphicsGridLayout_rowMaximumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowMaximumHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_rowMaximumHeight_0<RetType> {
  fn rowMaximumHeight_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_rowMaximumHeight_0<f64> for (i32) {
  fn rowMaximumHeight_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout16rowMaximumHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowFixedHeight(int, qreal)

/*
Sets the fixed height for row, row, to height.
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowFixedHeight_0<RetType, T: QGraphicsGridLayout_setRowFixedHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowFixedHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setRowFixedHeight_0<RetType> {
  fn setRowFixedHeight_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setRowFixedHeight_0<(/*void*/)> for (i32,f64) {
  fn setRowFixedHeight_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout17setRowFixedHeightEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnMinimumWidth(int, qreal)

/*
Sets the minimum width for column to width.

See also columnMinimumWidth().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnMinimumWidth_0<RetType, T: QGraphicsGridLayout_setColumnMinimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnMinimumWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setColumnMinimumWidth_0<RetType> {
  fn setColumnMinimumWidth_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnMinimumWidth_0<(/*void*/)> for (i32,f64) {
  fn setColumnMinimumWidth_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout21setColumnMinimumWidthEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal columnMinimumWidth(int) const

/*
Returns the minimum width for column.

See also setColumnMinimumWidth().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnMinimumWidth_0<RetType, T: QGraphicsGridLayout_columnMinimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnMinimumWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_columnMinimumWidth_0<RetType> {
  fn columnMinimumWidth_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_columnMinimumWidth_0<f64> for (i32) {
  fn columnMinimumWidth_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout18columnMinimumWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnPreferredWidth(int, qreal)

/*
Sets the preferred width for column to width.

See also columnPreferredWidth().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnPreferredWidth_0<RetType, T: QGraphicsGridLayout_setColumnPreferredWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnPreferredWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setColumnPreferredWidth_0<RetType> {
  fn setColumnPreferredWidth_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnPreferredWidth_0<(/*void*/)> for (i32,f64) {
  fn setColumnPreferredWidth_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout23setColumnPreferredWidthEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal columnPreferredWidth(int) const

/*
Returns the preferred width for column.

See also setColumnPreferredWidth().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnPreferredWidth_0<RetType, T: QGraphicsGridLayout_columnPreferredWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnPreferredWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_columnPreferredWidth_0<RetType> {
  fn columnPreferredWidth_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_columnPreferredWidth_0<f64> for (i32) {
  fn columnPreferredWidth_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout20columnPreferredWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnMaximumWidth(int, qreal)

/*
Sets the maximum width of column to width.

See also columnMaximumWidth().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnMaximumWidth_0<RetType, T: QGraphicsGridLayout_setColumnMaximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnMaximumWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setColumnMaximumWidth_0<RetType> {
  fn setColumnMaximumWidth_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnMaximumWidth_0<(/*void*/)> for (i32,f64) {
  fn setColumnMaximumWidth_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout21setColumnMaximumWidthEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:92
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal columnMaximumWidth(int) const

/*
Returns the maximum width for column.

See also setColumnMaximumWidth().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnMaximumWidth_0<RetType, T: QGraphicsGridLayout_columnMaximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnMaximumWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_columnMaximumWidth_0<RetType> {
  fn columnMaximumWidth_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_columnMaximumWidth_0<f64> for (i32) {
  fn columnMaximumWidth_0(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout18columnMaximumWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnFixedWidth(int, qreal)

/*
Sets the fixed width of column to width.
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnFixedWidth_0<RetType, T: QGraphicsGridLayout_setColumnFixedWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnFixedWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setColumnFixedWidth_0<RetType> {
  fn setColumnFixedWidth_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnFixedWidth_0<(/*void*/)> for (i32,f64) {
  fn setColumnFixedWidth_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout19setColumnFixedWidthEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowAlignment(int, Qt::Alignment)

/*
Sets the alignment of row to alignment.

See also rowAlignment().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowAlignment_0<RetType, T: QGraphicsGridLayout_setRowAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowAlignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setRowAlignment_0<RetType> {
  fn setRowAlignment_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setRowAlignment_0<(/*void*/)> for (i32,i32) {
  fn setRowAlignment_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout15setRowAlignmentEi6QFlagsIN2Qt13AlignmentFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:96
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment rowAlignment(int) const

/*
Returns the alignment of row.

See also setRowAlignment().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowAlignment_0<RetType, T: QGraphicsGridLayout_rowAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowAlignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_rowAlignment_0<RetType> {
  fn rowAlignment_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_rowAlignment_0<i32> for (i32) {
  fn rowAlignment_0(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout12rowAlignmentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnAlignment(int, Qt::Alignment)

/*
Sets the alignment for column to alignment.

See also columnAlignment().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnAlignment_0<RetType, T: QGraphicsGridLayout_setColumnAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnAlignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setColumnAlignment_0<RetType> {
  fn setColumnAlignment_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnAlignment_0<(/*void*/)> for (i32,i32) {
  fn setColumnAlignment_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout18setColumnAlignmentEi6QFlagsIN2Qt13AlignmentFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:98
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment columnAlignment(int) const

/*
Returns the alignment for column.

See also setColumnAlignment().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnAlignment_0<RetType, T: QGraphicsGridLayout_columnAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnAlignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_columnAlignment_0<RetType> {
  fn columnAlignment_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_columnAlignment_0<i32> for (i32) {
  fn columnAlignment_0(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout15columnAlignmentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(QGraphicsLayoutItem *, Qt::Alignment)

/*
Sets the alignment for item to alignment.

See also alignment().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setAlignment_0<RetType, T: QGraphicsGridLayout_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setAlignment_0<(/*void*/)> for (usize,i32) {
  fn setAlignment_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout12setAlignmentEP19QGraphicsLayoutItem6QFlagsIN2Qt13AlignmentFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment(QGraphicsLayoutItem *) const

/*
Returns the alignment for item.

See also setAlignment().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn alignment_0<RetType, T: QGraphicsGridLayout_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_alignment_0<i32> for (usize) {
  fn alignment_0(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout9alignmentEP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowCount() const

/*
Returns the number of rows in the grid layout. This is always one more than the index of the last row that is occupied by a layout item (empty rows are counted except for those at the end).
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowCount_0<RetType, T: QGraphicsGridLayout_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_rowCount_0<i32> for () {
  fn rowCount_0(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout8rowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:104
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnCount() const

/*
Returns the number of columns in the grid layout. This is always one more than the index of the last column that is occupied by a layout item (empty columns are counted except for those at the end).
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnCount_0<RetType, T: QGraphicsGridLayout_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:106
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsLayoutItem * itemAt(int, int) const

/*
Returns a pointer to the layout item at (row, column).
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn itemAt_0<RetType, T: QGraphicsGridLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_itemAt_0<usize> for (i32,i32) {
  fn itemAt_0(self , rsthis: & QGraphicsGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout6itemAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:110
// index:1
// Public virtual Visibility=Default Availability=Available
// [8] QGraphicsLayoutItem * itemAt(int) const

/*
Returns a pointer to the layout item at (row, column).
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn itemAt_1<RetType, T: QGraphicsGridLayout_itemAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_1(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_itemAt_1<RetType> {
  fn itemAt_1(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_itemAt_1<usize> for (i32) {
  fn itemAt_1(self , rsthis: & QGraphicsGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:109
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int count() const

/*
Reimplemented from QGraphicsLayout::count().

Returns the number of layout items in this grid layout.
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn count_0<RetType, T: QGraphicsGridLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:111
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void removeAt(int)

/*
Reimplemented from QGraphicsLayout::removeAt().

Removes the layout item at index without destroying it. Ownership of the item is transferred to the caller.

See also addItem().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn removeAt_0<RetType, T: QGraphicsGridLayout_removeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_removeAt_0<RetType> {
  fn removeAt_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_removeAt_0<(/*void*/)> for (i32) {
  fn removeAt_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout8removeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeItem(QGraphicsLayoutItem *)

/*
Removes the layout item item without destroying it. Ownership of the item is transferred to the caller.

See also addItem().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn removeItem_0<RetType, T: QGraphicsGridLayout_removeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_removeItem_0<RetType> {
  fn removeItem_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_removeItem_0<(/*void*/)> for (usize) {
  fn removeItem_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:114
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Reimplemented from QGraphicsLayout::invalidate().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn invalidate_0<RetType, T: QGraphicsGridLayout_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:117
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRectF &)

/*
Reimplemented from QGraphicsLayoutItem::setGeometry().

Sets the bounding geometry of the grid layout to rect.
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn setGeometry_0<RetType, T: QGraphicsGridLayout_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QGraphicsGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsGridLayout11setGeometryERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsgridlayout.h:118
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QSizeF sizeHint(Qt::SizeHint, const QSizeF &) const

/*
Reimplemented from QGraphicsLayoutItem::sizeHint().
*/
impl /*struct*/ QGraphicsGridLayout {
  pub fn sizeHint_0<RetType, T: QGraphicsGridLayout_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsGridLayout_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QGraphicsGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsGridLayout_sizeHint_0<usize> for (i32,usize) {
  fn sizeHint_0(self , rsthis: & QGraphicsGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsGridLayout8sizeHintEN2Qt8SizeHintERK6QSizeF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
