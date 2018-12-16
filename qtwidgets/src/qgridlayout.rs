

// mod ::widgets::QGridLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qgridlayout.h
// #include <qgridlayout.h>
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

// void addItem(QLayoutItem *)
// func (this *QGridLayout) InheritAddItem(f func(arg0 *QLayoutItem/*777 QLayoutItem **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "addItem", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGridLayout)=32
pub struct QGridLayout {
  qbase: QLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGridLayout_ITF interface {
//    QLayout_ITF
//    QGridLayout_PTR() *QGridLayout
//}
//func (ptr *QGridLayout) QGridLayout_PTR() *QGridLayout { return ptr }

impl /*struct*/ QGridLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGridLayout {
    return QGridLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGridLayout {
//  type Target = QGridLayoutBASE;
//
//  fn deref(&self) -> &QGridLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGridLayoutBASE> for QGridLayout {
//  fn as_ref(& self) -> & QGridLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgridlayout.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGridLayout {
  pub fn metaObject_0<RetType, T: QGridLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGridLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGridLayout(QWidget *)

/*
Constructs a new QGridLayout with parent widget, parent. The layout has one row and one column initially, and will expand when new items are inserted.
*/
// QGridLayout(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QGridLayout {
  pub fn QGridLayout_0<T: QGridLayout_QGridLayout_0>(value: T) -> QGridLayout {
    let rsthis = value.QGridLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGridLayout_QGridLayout_0 {
  fn QGridLayout_0(self) -> QGridLayout;
}
// QGridLayout(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGridLayout_QGridLayout_0 for (usize) {
  fn QGridLayout_0(self) -> QGridLayout {
    // unsafe{_ZN11QGridLayoutC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QGridLayoutC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGridLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:65
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QGridLayout()

/*
Constructs a new QGridLayout with parent widget, parent. The layout has one row and one column initially, and will expand when new items are inserted.
*/
// QGridLayout() ctx.fn_proto_cpp
impl /*struct*/ QGridLayout {
  pub fn QGridLayout_1<T: QGridLayout_QGridLayout_1>(value: T) -> QGridLayout {
    let rsthis = value.QGridLayout_1();
    return rsthis;
    // return 1;
  }
}

pub trait QGridLayout_QGridLayout_1 {
  fn QGridLayout_1(self) -> QGridLayout;
}
// QGridLayout() ctx.fn_proto_cpp
impl<'a> /*trait*/ QGridLayout_QGridLayout_1 for () {
  fn QGridLayout_1(self) -> QGridLayout {
    // unsafe{_ZN11QGridLayoutC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QGridLayoutC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGridLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGridLayout()

/*

*/
pub fn DeleteQGridLayout(this :*mut QGridLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QGridLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgridlayout.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QLayoutItem::sizeHint().
*/
impl /*struct*/ QGridLayout {
  pub fn sizeHint_0<RetType, T: QGridLayout_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QGridLayout_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Reimplemented from QLayoutItem::minimumSize().
*/
impl /*struct*/ QGridLayout {
  pub fn minimumSize_0<RetType, T: QGridLayout_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QGridLayout_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*
Reimplemented from QLayoutItem::maximumSize().
*/
impl /*struct*/ QGridLayout {
  pub fn maximumSize_0<RetType, T: QGridLayout_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QGridLayout_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalSpacing(int)

/*

*/
impl /*struct*/ QGridLayout {
  pub fn setHorizontalSpacing_0<RetType, T: QGridLayout_setHorizontalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setHorizontalSpacing_0<RetType> {
  fn setHorizontalSpacing_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setHorizontalSpacing_0<(/*void*/)> for (i32) {
  fn setHorizontalSpacing_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout20setHorizontalSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:74
// index:0
// Public Visibility=Default Availability=Available
// [4] int horizontalSpacing() const

/*

*/
impl /*struct*/ QGridLayout {
  pub fn horizontalSpacing_0<RetType, T: QGridLayout_horizontalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing_0(self);
    // return 1;
  }
}
pub trait QGridLayout_horizontalSpacing_0<RetType> {
  fn horizontalSpacing_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_horizontalSpacing_0<i32> for () {
  fn horizontalSpacing_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout17horizontalSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalSpacing(int)

/*

*/
impl /*struct*/ QGridLayout {
  pub fn setVerticalSpacing_0<RetType, T: QGridLayout_setVerticalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setVerticalSpacing_0<RetType> {
  fn setVerticalSpacing_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setVerticalSpacing_0<(/*void*/)> for (i32) {
  fn setVerticalSpacing_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout18setVerticalSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int verticalSpacing() const

/*

*/
impl /*struct*/ QGridLayout {
  pub fn verticalSpacing_0<RetType, T: QGridLayout_verticalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing_0(self);
    // return 1;
  }
}
pub trait QGridLayout_verticalSpacing_0<RetType> {
  fn verticalSpacing_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_verticalSpacing_0<i32> for () {
  fn verticalSpacing_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout15verticalSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(int)

/*
This function sets both the vertical and horizontal spacing to spacing.

See also spacing(), setVerticalSpacing(), and setHorizontalSpacing().
*/
impl /*struct*/ QGridLayout {
  pub fn setSpacing_0<RetType, T: QGridLayout_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setSpacing_0<(/*void*/)> for (i32) {
  fn setSpacing_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout10setSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] int spacing() const

/*
If the vertical spacing is equal to the horizontal spacing, this function returns that value; otherwise it return -1.

See also setSpacing(), verticalSpacing(), and horizontalSpacing().
*/
impl /*struct*/ QGridLayout {
  pub fn spacing_0<RetType, T: QGridLayout_spacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacing_0(self);
    // return 1;
  }
}
pub trait QGridLayout_spacing_0<RetType> {
  fn spacing_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_spacing_0<i32> for () {
  fn spacing_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout7spacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowStretch(int, int)

/*
Sets the stretch factor of row row to stretch. The first row is number 0.

The stretch factor is relative to the other rows in this grid. Rows with a higher stretch factor take more of the available space.

The default stretch factor is 0. If the stretch factor is 0 and no other row in this table can grow at all, the row may still grow.

See also rowStretch(), setRowMinimumHeight(), and setColumnStretch().
*/
impl /*struct*/ QGridLayout {
  pub fn setRowStretch_0<RetType, T: QGridLayout_setRowStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowStretch_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setRowStretch_0<RetType> {
  fn setRowStretch_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setRowStretch_0<(/*void*/)> for (i32,i32) {
  fn setRowStretch_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout13setRowStretchEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnStretch(int, int)

/*
Sets the stretch factor of column column to stretch. The first column is number 0.

The stretch factor is relative to the other columns in this grid. Columns with a higher stretch factor take more of the available space.

The default stretch factor is 0. If the stretch factor is 0 and no other column in this table can grow at all, the column may still grow.

An alternative approach is to add spacing using addItem() with a QSpacerItem.

See also columnStretch() and setRowStretch().
*/
impl /*struct*/ QGridLayout {
  pub fn setColumnStretch_0<RetType, T: QGridLayout_setColumnStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnStretch_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setColumnStretch_0<RetType> {
  fn setColumnStretch_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setColumnStretch_0<(/*void*/)> for (i32,i32) {
  fn setColumnStretch_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout16setColumnStretchEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowStretch(int) const

/*
Returns the stretch factor for row row.

See also setRowStretch().
*/
impl /*struct*/ QGridLayout {
  pub fn rowStretch_0<RetType, T: QGridLayout_rowStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowStretch_0(self);
    // return 1;
  }
}
pub trait QGridLayout_rowStretch_0<RetType> {
  fn rowStretch_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_rowStretch_0<i32> for (i32) {
  fn rowStretch_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout10rowStretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnStretch(int) const

/*
Returns the stretch factor for column column.

See also setColumnStretch().
*/
impl /*struct*/ QGridLayout {
  pub fn columnStretch_0<RetType, T: QGridLayout_columnStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnStretch_0(self);
    // return 1;
  }
}
pub trait QGridLayout_columnStretch_0<RetType> {
  fn columnStretch_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_columnStretch_0<i32> for (i32) {
  fn columnStretch_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout13columnStretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowMinimumHeight(int, int)

/*
Sets the minimum height of row row to minSize pixels.

See also rowMinimumHeight() and setColumnMinimumWidth().
*/
impl /*struct*/ QGridLayout {
  pub fn setRowMinimumHeight_0<RetType, T: QGridLayout_setRowMinimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowMinimumHeight_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setRowMinimumHeight_0<RetType> {
  fn setRowMinimumHeight_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setRowMinimumHeight_0<(/*void*/)> for (i32,i32) {
  fn setRowMinimumHeight_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout19setRowMinimumHeightEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnMinimumWidth(int, int)

/*
Sets the minimum width of column column to minSize pixels.

See also columnMinimumWidth() and setRowMinimumHeight().
*/
impl /*struct*/ QGridLayout {
  pub fn setColumnMinimumWidth_0<RetType, T: QGridLayout_setColumnMinimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnMinimumWidth_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setColumnMinimumWidth_0<RetType> {
  fn setColumnMinimumWidth_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setColumnMinimumWidth_0<(/*void*/)> for (i32,i32) {
  fn setColumnMinimumWidth_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout21setColumnMinimumWidthEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowMinimumHeight(int) const

/*
Returns the minimum width set for row row.

See also setRowMinimumHeight().
*/
impl /*struct*/ QGridLayout {
  pub fn rowMinimumHeight_0<RetType, T: QGridLayout_rowMinimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowMinimumHeight_0(self);
    // return 1;
  }
}
pub trait QGridLayout_rowMinimumHeight_0<RetType> {
  fn rowMinimumHeight_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_rowMinimumHeight_0<i32> for (i32) {
  fn rowMinimumHeight_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout16rowMinimumHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnMinimumWidth(int) const

/*
Returns the column spacing for column column.

See also setColumnMinimumWidth().
*/
impl /*struct*/ QGridLayout {
  pub fn columnMinimumWidth_0<RetType, T: QGridLayout_columnMinimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnMinimumWidth_0(self);
    // return 1;
  }
}
pub trait QGridLayout_columnMinimumWidth_0<RetType> {
  fn columnMinimumWidth_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_columnMinimumWidth_0<i32> for (i32) {
  fn columnMinimumWidth_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout18columnMinimumWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnCount() const

/*
Returns the number of columns in this grid.
*/
impl /*struct*/ QGridLayout {
  pub fn columnCount_0<RetType, T: QGridLayout_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QGridLayout_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:91
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowCount() const

/*
Returns the number of rows in this grid.
*/
impl /*struct*/ QGridLayout {
  pub fn rowCount_0<RetType, T: QGridLayout_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QGridLayout_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_rowCount_0<i32> for () {
  fn rowCount_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout8rowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:93
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect cellRect(int, int) const

/*
Returns the geometry of the cell with row row and column column in the grid. Returns an invalid rectangle if row or column is outside the grid.

Warning: in the current version of Qt this function does not return valid results until setGeometry() has been called, i.e. after the parentWidget() is visible.
*/
impl /*struct*/ QGridLayout {
  pub fn cellRect_0<RetType, T: QGridLayout_cellRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellRect_0(self);
    // return 1;
  }
}
pub trait QGridLayout_cellRect_0<RetType> {
  fn cellRect_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_cellRect_0<usize> for (i32,i32) {
  fn cellRect_0(self , rsthis: & QGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout8cellRectEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Reimplemented from QLayoutItem::hasHeightForWidth().
*/
impl /*struct*/ QGridLayout {
  pub fn hasHeightForWidth_0<RetType, T: QGridLayout_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QGridLayout_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QGridLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Reimplemented from QLayoutItem::heightForWidth().
*/
impl /*struct*/ QGridLayout {
  pub fn heightForWidth_0<RetType, T: QGridLayout_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QGridLayout_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:97
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int minimumHeightForWidth(int) const

/*
Reimplemented from QLayoutItem::minimumHeightForWidth().
*/
impl /*struct*/ QGridLayout {
  pub fn minimumHeightForWidth_0<RetType, T: QGridLayout_minimumHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QGridLayout_minimumHeightForWidth_0<RetType> {
  fn minimumHeightForWidth_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_minimumHeightForWidth_0<i32> for (i32) {
  fn minimumHeightForWidth_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout21minimumHeightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::Orientations expandingDirections() const

/*
Reimplemented from QLayoutItem::expandingDirections().
*/
impl /*struct*/ QGridLayout {
  pub fn expandingDirections_0<RetType, T: QGridLayout_expandingDirections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandingDirections_0(self);
    // return 1;
  }
}
pub trait QGridLayout_expandingDirections_0<RetType> {
  fn expandingDirections_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_expandingDirections_0<i32> for () {
  fn expandingDirections_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout19expandingDirectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:100
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Reimplemented from QLayoutItem::invalidate().
*/
impl /*struct*/ QGridLayout {
  pub fn invalidate_0<RetType, T: QGridLayout_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QGridLayout_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QGridLayout10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:102
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void addWidget(QWidget *)

/*
Adds the given widget to the cell grid at row, column. The top-left position is (0, 0) by default.

The alignment is specified by alignment. The default alignment is 0, which means that the widget fills the entire cell.
*/
impl /*struct*/ QGridLayout {
  pub fn addWidget_0<RetType, T: QGridLayout_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QGridLayout_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_addWidget_0<(/*void*/)> for (usize) {
  fn addWidget_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout9addWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:103
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addWidget(QWidget *, int, int, Qt::Alignment)

/*
Adds the given widget to the cell grid at row, column. The top-left position is (0, 0) by default.

The alignment is specified by alignment. The default alignment is 0, which means that the widget fills the entire cell.
*/
impl /*struct*/ QGridLayout {
  pub fn addWidget_1<RetType, T: QGridLayout_addWidget_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_1(self);
    // return 1;
  }
}
pub trait QGridLayout_addWidget_1<RetType> {
  fn addWidget_1(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_addWidget_1<(/*void*/)> for (usize,i32,i32,i32) {
  fn addWidget_1(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout9addWidgetEP7QWidgetii6QFlagsIN2Qt13AlignmentFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:104
// index:2
// Public Visibility=Default Availability=Available
// [-2] void addWidget(QWidget *, int, int, int, int, Qt::Alignment)

/*
Adds the given widget to the cell grid at row, column. The top-left position is (0, 0) by default.

The alignment is specified by alignment. The default alignment is 0, which means that the widget fills the entire cell.
*/
impl /*struct*/ QGridLayout {
  pub fn addWidget_2<RetType, T: QGridLayout_addWidget_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_2(self);
    // return 1;
  }
}
pub trait QGridLayout_addWidget_2<RetType> {
  fn addWidget_2(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_addWidget_2<(/*void*/)> for (usize,i32,i32,i32,i32,i32) {
  fn addWidget_2(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout9addWidgetEP7QWidgetiiii6QFlagsIN2Qt13AlignmentFlagEE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addLayout(QLayout *, int, int, Qt::Alignment)

/*
Places the layout at position (row, column) in the grid. The top-left position is (0, 0).

The alignment is specified by alignment. The default alignment is 0, which means that the widget fills the entire cell.

A non-zero alignment indicates that the layout should not grow to fill the available space but should be sized according to sizeHint().

layout becomes a child of the grid layout.
*/
impl /*struct*/ QGridLayout {
  pub fn addLayout_0<RetType, T: QGridLayout_addLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addLayout_0(self);
    // return 1;
  }
}
pub trait QGridLayout_addLayout_0<RetType> {
  fn addLayout_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_addLayout_0<(/*void*/)> for (usize,i32,i32,i32) {
  fn addLayout_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout9addLayoutEP7QLayoutii6QFlagsIN2Qt13AlignmentFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:106
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addLayout(QLayout *, int, int, int, int, Qt::Alignment)

/*
Places the layout at position (row, column) in the grid. The top-left position is (0, 0).

The alignment is specified by alignment. The default alignment is 0, which means that the widget fills the entire cell.

A non-zero alignment indicates that the layout should not grow to fill the available space but should be sized according to sizeHint().

layout becomes a child of the grid layout.
*/
impl /*struct*/ QGridLayout {
  pub fn addLayout_1<RetType, T: QGridLayout_addLayout_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addLayout_1(self);
    // return 1;
  }
}
pub trait QGridLayout_addLayout_1<RetType> {
  fn addLayout_1(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_addLayout_1<(/*void*/)> for (usize,i32,i32,i32,i32,i32) {
  fn addLayout_1(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout9addLayoutEP7QLayoutiiii6QFlagsIN2Qt13AlignmentFlagEE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOriginCorner(Qt::Corner)

/*
Sets the grid's origin corner, i.e. position (0, 0), to corner.

See also originCorner().
*/
impl /*struct*/ QGridLayout {
  pub fn setOriginCorner_0<RetType, T: QGridLayout_setOriginCorner_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOriginCorner_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setOriginCorner_0<RetType> {
  fn setOriginCorner_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setOriginCorner_0<(/*void*/)> for (i32) {
  fn setOriginCorner_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout15setOriginCornerEN2Qt6CornerE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:109
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Corner originCorner() const

/*
Returns the corner that's used for the grid's origin, i.e. for position (0, 0).

See also setOriginCorner().
*/
impl /*struct*/ QGridLayout {
  pub fn originCorner_0<RetType, T: QGridLayout_originCorner_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.originCorner_0(self);
    // return 1;
  }
}
pub trait QGridLayout_originCorner_0<RetType> {
  fn originCorner_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_originCorner_0<i32> for () {
  fn originCorner_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout12originCornerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:111
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayoutItem * itemAt(int) const

/*
Reimplemented from QLayout::itemAt().
*/
impl /*struct*/ QGridLayout {
  pub fn itemAt_0<RetType, T: QGridLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QGridLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_itemAt_0<usize> for (i32) {
  fn itemAt_0(self , rsthis: & QGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] QLayoutItem * itemAtPosition(int, int) const

/*
Returns the layout item that occupies cell (row, column), or 0 if the cell is empty.

This function was introduced in  Qt 4.4.

See also getItemPosition() and indexOf().
*/
impl /*struct*/ QGridLayout {
  pub fn itemAtPosition_0<RetType, T: QGridLayout_itemAtPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAtPosition_0(self);
    // return 1;
  }
}
pub trait QGridLayout_itemAtPosition_0<RetType> {
  fn itemAtPosition_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_itemAtPosition_0<usize> for (i32,i32) {
  fn itemAtPosition_0(self , rsthis: & QGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout14itemAtPositionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:113
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayoutItem * takeAt(int)

/*
Reimplemented from QLayout::takeAt().
*/
impl /*struct*/ QGridLayout {
  pub fn takeAt_0<RetType, T: QGridLayout_takeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeAt_0(self);
    // return 1;
  }
}
pub trait QGridLayout_takeAt_0<RetType> {
  fn takeAt_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_takeAt_0<usize> for (i32) {
  fn takeAt_0(self , rsthis: & QGridLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QGridLayout6takeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:114
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int count() const

/*
Reimplemented from QLayout::count().
*/
impl /*struct*/ QGridLayout {
  pub fn count_0<RetType, T: QGridLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QGridLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QGridLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:115
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Reimplemented from QLayoutItem::setGeometry().
*/
impl /*struct*/ QGridLayout {
  pub fn setGeometry_0<RetType, T: QGridLayout_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addItem(QLayoutItem *, int, int, int, int, Qt::Alignment)

/*
Adds item at position row, column, spanning rowSpan rows and columnSpan columns, and aligns it according to alignment. If rowSpan and/or columnSpan is -1, then the item will extend to the bottom and/or right edge, respectively. The layout takes ownership of the item.

Warning: Do not use this function to add child layouts or child widget items. Use addLayout() or addWidget() instead.
*/
impl /*struct*/ QGridLayout {
  pub fn addItem_0<RetType, T: QGridLayout_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QGridLayout_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_addItem_0<(/*void*/)> for (usize,i32,i32,i32,i32,i32) {
  fn addItem_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout7addItemEP11QLayoutItemiiii6QFlagsIN2Qt13AlignmentFlagEE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:123
// index:1
// Protected virtual Visibility=Default Availability=Available
// [-2] void addItem(QLayoutItem *)

/*
Adds item at position row, column, spanning rowSpan rows and columnSpan columns, and aligns it according to alignment. If rowSpan and/or columnSpan is -1, then the item will extend to the bottom and/or right edge, respectively. The layout takes ownership of the item.

Warning: Do not use this function to add child layouts or child widget items. Use addLayout() or addWidget() instead.
*/
impl /*struct*/ QGridLayout {
  pub fn addItem_1<RetType, T: QGridLayout_addItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_1(self);
    // return 1;
  }
}
pub trait QGridLayout_addItem_1<RetType> {
  fn addItem_1(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_addItem_1<(/*void*/)> for (usize) {
  fn addItem_1(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout7addItemEP11QLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultPositioning(int, Qt::Orientation)

/*

*/
impl /*struct*/ QGridLayout {
  pub fn setDefaultPositioning_0<RetType, T: QGridLayout_setDefaultPositioning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultPositioning_0(self);
    // return 1;
  }
}
pub trait QGridLayout_setDefaultPositioning_0<RetType> {
  fn setDefaultPositioning_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_setDefaultPositioning_0<(/*void*/)> for (i32,i32) {
  fn setDefaultPositioning_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QGridLayout21setDefaultPositioningEiN2Qt11OrientationE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgridlayout.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getItemPosition(int, int *, int *, int *, int *) const

/*
Returns the position information of the item with the given index.

The variables passed as row and column are updated with the position of the item in the layout, and the rowSpan and columnSpan variables are updated with the vertical and horizontal spans of the item.

See also itemAtPosition() and itemAt().
*/
impl /*struct*/ QGridLayout {
  pub fn getItemPosition_0<RetType, T: QGridLayout_getItemPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getItemPosition_0(self);
    // return 1;
  }
}
pub trait QGridLayout_getItemPosition_0<RetType> {
  fn getItemPosition_0(self , rsthis: & QGridLayout) -> RetType;
}
impl<'a> /*trait*/ QGridLayout_getItemPosition_0<(/*void*/)> for (i32,usize,usize,usize,usize) {
  fn getItemPosition_0(self , rsthis: & QGridLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let arg4 = (&self.4) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_", 5,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
