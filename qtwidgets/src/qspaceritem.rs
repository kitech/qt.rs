

// mod ::widgets::QSpacerItem
// package qtwidgets
// /usr/include/qt/QtWidgets/qlayoutitem.h
// #include <qlayoutitem.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
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
#[derive(Default)] // class sizeof(QSpacerItem)=40
pub struct QSpacerItem {
  qbase: QLayoutItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSpacerItem_ITF interface {
//    QLayoutItem_ITF
//    QSpacerItem_PTR() *QSpacerItem
//}
//func (ptr *QSpacerItem) QSpacerItem_PTR() *QSpacerItem { return ptr }

impl /*struct*/ QSpacerItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSpacerItem {
    return QSpacerItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSpacerItem {
//  type Target = QSpacerItemBASE;
//
//  fn deref(&self) -> &QSpacerItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSpacerItemBASE> for QSpacerItem {
//  fn as_ref(& self) -> & QSpacerItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlayoutitem.h:95
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QSpacerItem(int, int, QSizePolicy::Policy, QSizePolicy::Policy)

/*

*/
// QSpacerItem(int, int, QSizePolicy::Policy, QSizePolicy::Policy) ctx.fn_proto_cpp
impl /*struct*/ QSpacerItem {
  pub fn QSpacerItem_0<T: QSpacerItem_QSpacerItem_0>(value: T) -> QSpacerItem {
    let rsthis = value.QSpacerItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSpacerItem_QSpacerItem_0 {
  fn QSpacerItem_0(self) -> QSpacerItem;
}
// QSpacerItem(int, int, QSizePolicy::Policy, QSizePolicy::Policy) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSpacerItem_QSpacerItem_0 for (i32,i32,i32,i32) {
  fn QSpacerItem_0(self) -> QSpacerItem {
    // unsafe{_ZN11QSpacerItemC2EiiN11QSizePolicy6PolicyES1_()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QSpacerItemC2EiiN11QSizePolicy6PolicyES1_", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSpacerItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSpacerItem()

/*

*/
pub fn DeleteQSpacerItem(this :*mut QSpacerItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QSpacerItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlayoutitem.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void changeSize(int, int, QSizePolicy::Policy, QSizePolicy::Policy)

/*

*/
impl /*struct*/ QSpacerItem {
  pub fn changeSize_0<RetType, T: QSpacerItem_changeSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeSize_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_changeSize_0<RetType> {
  fn changeSize_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_changeSize_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn changeSize_0(self , rsthis: & QSpacerItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QSpacerItem10changeSizeEiiN11QSizePolicy6PolicyES1_", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:104
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Implemented in subclasses to return the preferred size of this item.
*/
impl /*struct*/ QSpacerItem {
  pub fn sizeHint_0<RetType, T: QSpacerItem_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QSpacerItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSpacerItem8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:105
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Implemented in subclasses to return the minimum size of this item.
*/
impl /*struct*/ QSpacerItem {
  pub fn minimumSize_0<RetType, T: QSpacerItem_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QSpacerItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSpacerItem11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:106
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*
Implemented in subclasses to return the maximum size of this item.
*/
impl /*struct*/ QSpacerItem {
  pub fn maximumSize_0<RetType, T: QSpacerItem_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QSpacerItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSpacerItem11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:107
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::Orientations expandingDirections() const

/*
Returns whether this layout item can make use of more space than sizeHint(). A value of Qt::Vertical or Qt::Horizontal means that it wants to grow in only one dimension, whereas Qt::Vertical | Qt::Horizontal means that it wants to grow in both dimensions.
*/
impl /*struct*/ QSpacerItem {
  pub fn expandingDirections_0<RetType, T: QSpacerItem_expandingDirections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandingDirections_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_expandingDirections_0<RetType> {
  fn expandingDirections_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_expandingDirections_0<i32> for () {
  fn expandingDirections_0(self , rsthis: & QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSpacerItem19expandingDirectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:108
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Implemented in subclasses to return whether this item is empty, i.e. whether it contains any widgets.
*/
impl /*struct*/ QSpacerItem {
  pub fn isEmpty_0<RetType, T: QSpacerItem_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QSpacerItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSpacerItem7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:109
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Implemented in subclasses to set this item's geometry to r.

See also geometry().
*/
impl /*struct*/ QSpacerItem {
  pub fn setGeometry_0<RetType, T: QSpacerItem_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QSpacerItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QSpacerItem11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:110
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect geometry() const

/*
Returns the rectangle covered by this layout item.

See also setGeometry().
*/
impl /*struct*/ QSpacerItem {
  pub fn geometry_0<RetType, T: QSpacerItem_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QSpacerItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSpacerItem8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:111
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSpacerItem * spacerItem()

/*
If this item is a QSpacerItem, it is returned as a QSpacerItem; otherwise 0 is returned. This function provides type-safe casting.

See also layout() and widget().
*/
impl /*struct*/ QSpacerItem {
  pub fn spacerItem_0<RetType, T: QSpacerItem_spacerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacerItem_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_spacerItem_0<RetType> {
  fn spacerItem_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_spacerItem_0<usize> for () {
  fn spacerItem_0(self , rsthis: & QSpacerItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QSpacerItem10spacerItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QSizePolicy sizePolicy() const

/*

*/
impl /*struct*/ QSpacerItem {
  pub fn sizePolicy_0<RetType, T: QSpacerItem_sizePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy_0(self);
    // return 1;
  }
}
pub trait QSpacerItem_sizePolicy_0<RetType> {
  fn sizePolicy_0(self , rsthis: & QSpacerItem) -> RetType;
}
impl<'a> /*trait*/ QSpacerItem_sizePolicy_0<usize> for () {
  fn sizePolicy_0(self , rsthis: & QSpacerItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSpacerItem10sizePolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
