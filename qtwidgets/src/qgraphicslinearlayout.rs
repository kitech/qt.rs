

// mod ::widgets::QGraphicsLinearLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h
// #include <qgraphicslinearlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 26
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
#[derive(Default)] // class sizeof(QGraphicsLinearLayout)=16
pub struct QGraphicsLinearLayout {
  qbase: QGraphicsLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsLinearLayout_ITF interface {
//    QGraphicsLayout_ITF
//    QGraphicsLinearLayout_PTR() *QGraphicsLinearLayout
//}
//func (ptr *QGraphicsLinearLayout) QGraphicsLinearLayout_PTR() *QGraphicsLinearLayout { return ptr }

impl /*struct*/ QGraphicsLinearLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsLinearLayout {
    return QGraphicsLinearLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsLinearLayout {
//  type Target = QGraphicsLinearLayoutBASE;
//
//  fn deref(&self) -> &QGraphicsLinearLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsLinearLayoutBASE> for QGraphicsLinearLayout {
//  fn as_ref(& self) -> & QGraphicsLinearLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsLinearLayout(QGraphicsLayoutItem *)

/*
Constructs a QGraphicsLinearLayout instance using Qt::Horizontal orientation. parent is passed to QGraphicsLayout's constructor.
*/
// QGraphicsLinearLayout(QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsLinearLayout {
  pub fn QGraphicsLinearLayout_0<T: QGraphicsLinearLayout_QGraphicsLinearLayout_0>(value: T) -> QGraphicsLinearLayout {
    let rsthis = value.QGraphicsLinearLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_QGraphicsLinearLayout_0 {
  fn QGraphicsLinearLayout_0(self) -> QGraphicsLinearLayout;
}
// QGraphicsLinearLayout(QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsLinearLayout_QGraphicsLinearLayout_0 for (usize) {
  fn QGraphicsLinearLayout_0(self) -> QGraphicsLinearLayout {
    // unsafe{_ZN21QGraphicsLinearLayoutC2EP19QGraphicsLayoutItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayoutC2EP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsLinearLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:57
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsLinearLayout(Qt::Orientation, QGraphicsLayoutItem *)

/*
Constructs a QGraphicsLinearLayout instance using Qt::Horizontal orientation. parent is passed to QGraphicsLayout's constructor.
*/
// QGraphicsLinearLayout(Qt::Orientation, QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsLinearLayout {
  pub fn QGraphicsLinearLayout_1<T: QGraphicsLinearLayout_QGraphicsLinearLayout_1>(value: T) -> QGraphicsLinearLayout {
    let rsthis = value.QGraphicsLinearLayout_1();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_QGraphicsLinearLayout_1 {
  fn QGraphicsLinearLayout_1(self) -> QGraphicsLinearLayout;
}
// QGraphicsLinearLayout(Qt::Orientation, QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsLinearLayout_QGraphicsLinearLayout_1 for (i32,usize) {
  fn QGraphicsLinearLayout_1(self) -> QGraphicsLinearLayout {
    // unsafe{_ZN21QGraphicsLinearLayoutC2EN2Qt11OrientationEP19QGraphicsLayoutItem()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayoutC2EN2Qt11OrientationEP19QGraphicsLayoutItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsLinearLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsLinearLayout()

/*

*/
pub fn DeleteQGraphicsLinearLayout(this :*mut QGraphicsLinearLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*
Change the layout orientation to orientation. Changing the layout orientation will automatically invalidate the layout.

See also orientation().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setOrientation_0<RetType, T: QGraphicsLinearLayout_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:61
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*
Returns the layout orientation.

See also setOrientation().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn orientation_0<RetType, T: QGraphicsLinearLayout_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void addItem(QGraphicsLayoutItem *)

/*
This convenience function is equivalent to calling insertItem(-1, item).
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn addItem_0<RetType, T: QGraphicsLinearLayout_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_addItem_0<(/*void*/)> for (usize) {
  fn addItem_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void addStretch(int)

/*
This convenience function is equivalent to calling insertStretch(-1, stretch).
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn addStretch_0<RetType, T: QGraphicsLinearLayout_addStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addStretch_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_addStretch_0<RetType> {
  fn addStretch_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_addStretch_0<(/*void*/)> for (i32) {
  fn addStretch_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout10addStretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertItem(int, QGraphicsLayoutItem *)

/*
Inserts item into the layout at index, or before any item that is currently at index.

See also addItem(), itemAt(), insertStretch(), and setItemSpacing().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn insertItem_0<RetType, T: QGraphicsLinearLayout_insertItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_insertItem_0<RetType> {
  fn insertItem_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_insertItem_0<(/*void*/)> for (i32,usize) {
  fn insertItem_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertStretch(int, int)

/*
Inserts a stretch of stretch at index, or before any item that is currently at index.

See also addStretch(), setStretchFactor(), setItemSpacing(), and insertItem().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn insertStretch_0<RetType, T: QGraphicsLinearLayout_insertStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertStretch_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_insertStretch_0<RetType> {
  fn insertStretch_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_insertStretch_0<(/*void*/)> for (i32,i32) {
  fn insertStretch_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout13insertStretchEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeItem(QGraphicsLayoutItem *)

/*
Removes item from the layout without destroying it. Ownership of item is transferred to the caller.

See also removeAt() and insertItem().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn removeItem_0<RetType, T: QGraphicsLinearLayout_removeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_removeItem_0<RetType> {
  fn removeItem_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_removeItem_0<(/*void*/)> for (usize) {
  fn removeItem_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void removeAt(int)

/*
Reimplemented from QGraphicsLayout::removeAt().

Removes the item at index without destroying it. Ownership of the item is transferred to the caller.

See also removeItem() and insertItem().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn removeAt_0<RetType, T: QGraphicsLinearLayout_removeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_removeAt_0<RetType> {
  fn removeAt_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_removeAt_0<(/*void*/)> for (i32) {
  fn removeAt_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout8removeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(qreal)

/*
Sets the layout's spacing to spacing. Spacing refers to the vertical and horizontal distances between items.

See also spacing(), setItemSpacing(), setStretchFactor(), and QGraphicsGridLayout::setSpacing().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setSpacing_0<RetType, T: QGraphicsLinearLayout_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_setSpacing_0<(/*void*/)> for (f64) {
  fn setSpacing_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout10setSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal spacing() const

/*
Returns the layout's spacing. Spacing refers to the vertical and horizontal distances between items.

See also setSpacing().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn spacing_0<RetType, T: QGraphicsLinearLayout_spacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_spacing_0<RetType> {
  fn spacing_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_spacing_0<f64> for () {
  fn spacing_0(self , rsthis: & QGraphicsLinearLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout7spacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemSpacing(int, qreal)

/*
Sets the spacing after item at index to spacing.

See also itemSpacing().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setItemSpacing_0<RetType, T: QGraphicsLinearLayout_setItemSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_setItemSpacing_0<RetType> {
  fn setItemSpacing_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_setItemSpacing_0<(/*void*/)> for (i32,f64) {
  fn setItemSpacing_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout14setItemSpacingEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal itemSpacing(int) const

/*
Returns the spacing after item at index.

See also setItemSpacing().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn itemSpacing_0<RetType, T: QGraphicsLinearLayout_itemSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_itemSpacing_0<RetType> {
  fn itemSpacing_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_itemSpacing_0<f64> for (i32) {
  fn itemSpacing_0(self , rsthis: & QGraphicsLinearLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout11itemSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStretchFactor(QGraphicsLayoutItem *, int)

/*
Sets the stretch factor for item to stretch. If an item's stretch factor changes, this function will invalidate the layout.

Setting stretch to 0 removes the stretch factor from the item, and is effectively equivalent to setting stretch to 1.

See also stretchFactor().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setStretchFactor_0<RetType, T: QGraphicsLinearLayout_setStretchFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStretchFactor_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_setStretchFactor_0<RetType> {
  fn setStretchFactor_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_setStretchFactor_0<(/*void*/)> for (usize,i32) {
  fn setStretchFactor_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] int stretchFactor(QGraphicsLayoutItem *) const

/*
Returns the stretch factor for item. The default stretch factor is 0, meaning that the item has no assigned stretch factor.

See also setStretchFactor().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn stretchFactor_0<RetType, T: QGraphicsLinearLayout_stretchFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stretchFactor_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_stretchFactor_0<RetType> {
  fn stretchFactor_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_stretchFactor_0<i32> for (usize) {
  fn stretchFactor_0(self , rsthis: & QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(QGraphicsLayoutItem *, Qt::Alignment)

/*
Sets the alignment of item to alignment. If item's alignment changes, the layout is automatically invalidated.

See also alignment() and invalidate().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setAlignment_0<RetType, T: QGraphicsLinearLayout_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_setAlignment_0<(/*void*/)> for (usize,i32) {
  fn setAlignment_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout12setAlignmentEP19QGraphicsLayoutItem6QFlagsIN2Qt13AlignmentFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment(QGraphicsLayoutItem *) const

/*
Returns the alignment for item. The default alignment is Qt::AlignTop | Qt::AlignLeft.

The alignment decides how the item is positioned within its assigned space in the case where there's more space available in the layout than the widgets can occupy.

See also setAlignment().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn alignment_0<RetType, T: QGraphicsLinearLayout_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_alignment_0<i32> for (usize) {
  fn alignment_0(self , rsthis: & QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout9alignmentEP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRectF &)

/*
Reimplemented from QGraphicsLayoutItem::setGeometry().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setGeometry_0<RetType, T: QGraphicsLinearLayout_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout11setGeometryERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int count() const

/*
Reimplemented from QGraphicsLayout::count().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn count_0<RetType, T: QGraphicsLinearLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QGraphicsLayoutItem * itemAt(int) const

/*
Reimplemented from QGraphicsLayout::itemAt().

When iterating from 0 and up, it will return the items in the visual arranged order.
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn itemAt_0<RetType, T: QGraphicsLinearLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_itemAt_0<usize> for (i32) {
  fn itemAt_0(self , rsthis: & QGraphicsLinearLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Reimplemented from QGraphicsLayout::invalidate().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn invalidate_0<RetType, T: QGraphicsLinearLayout_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN21QGraphicsLinearLayout10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QSizeF sizeHint(Qt::SizeHint, const QSizeF &) const

/*
Reimplemented from QGraphicsLayoutItem::sizeHint().
*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn sizeHint_0<RetType, T: QGraphicsLinearLayout_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_sizeHint_0<usize> for (i32,usize) {
  fn sizeHint_0(self , rsthis: & QGraphicsLinearLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout8sizeHintEN2Qt8SizeHintERK6QSizeF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslinearlayout.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void dump(int) const

/*

*/
impl /*struct*/ QGraphicsLinearLayout {
  pub fn dump_0<RetType, T: QGraphicsLinearLayout_dump_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dump_0(self);
    // return 1;
  }
}
pub trait QGraphicsLinearLayout_dump_0<RetType> {
  fn dump_0(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLinearLayout_dump_0<(/*void*/)> for (i32) {
  fn dump_0(self , rsthis: & QGraphicsLinearLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK21QGraphicsLinearLayout4dumpEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
