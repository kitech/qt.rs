

// mod ::widgets::QAbstractGraphicsShapeItem
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsitem.h
// #include <qgraphicsitem.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 209
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
#[derive(Default)] // class sizeof(QAbstractGraphicsShapeItem)=16
pub struct QAbstractGraphicsShapeItem {
  qbase: QGraphicsItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractGraphicsShapeItem_ITF interface {
//    QGraphicsItem_ITF
//    QAbstractGraphicsShapeItem_PTR() *QAbstractGraphicsShapeItem
//}
//func (ptr *QAbstractGraphicsShapeItem) QAbstractGraphicsShapeItem_PTR() *QAbstractGraphicsShapeItem { return ptr }

impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractGraphicsShapeItem {
    return QAbstractGraphicsShapeItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractGraphicsShapeItem {
//  type Target = QAbstractGraphicsShapeItemBASE;
//
//  fn deref(&self) -> &QAbstractGraphicsShapeItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractGraphicsShapeItemBASE> for QAbstractGraphicsShapeItem {
//  fn as_ref(& self) -> & QAbstractGraphicsShapeItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:603
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractGraphicsShapeItem(QGraphicsItem *)

/*

*/
// QAbstractGraphicsShapeItem(QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn QAbstractGraphicsShapeItem_0<T: QAbstractGraphicsShapeItem_QAbstractGraphicsShapeItem_0>(value: T) -> QAbstractGraphicsShapeItem {
    let rsthis = value.QAbstractGraphicsShapeItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_QAbstractGraphicsShapeItem_0 {
  fn QAbstractGraphicsShapeItem_0(self) -> QAbstractGraphicsShapeItem;
}
// QAbstractGraphicsShapeItem(QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_QAbstractGraphicsShapeItem_0 for (usize) {
  fn QAbstractGraphicsShapeItem_0(self) -> QAbstractGraphicsShapeItem {
    // unsafe{_ZN26QAbstractGraphicsShapeItemC2EP13QGraphicsItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAbstractGraphicsShapeItemC2EP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractGraphicsShapeItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:604
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractGraphicsShapeItem()

/*

*/
pub fn DeleteQAbstractGraphicsShapeItem(this :*mut QAbstractGraphicsShapeItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN26QAbstractGraphicsShapeItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:606
// index:0
// Public Visibility=Default Availability=Available
// [8] QPen pen() const

/*

*/
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn pen_0<RetType, T: QAbstractGraphicsShapeItem_pen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pen_0(self);
    // return 1;
  }
}
pub trait QAbstractGraphicsShapeItem_pen_0<RetType> {
  fn pen_0(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_pen_0<usize> for () {
  fn pen_0(self , rsthis: & QAbstractGraphicsShapeItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAbstractGraphicsShapeItem3penEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:607
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPen(const QPen &)

/*

*/
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn setPen_0<RetType, T: QAbstractGraphicsShapeItem_setPen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPen_0(self);
    // return 1;
  }
}
pub trait QAbstractGraphicsShapeItem_setPen_0<RetType> {
  fn setPen_0(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_setPen_0<(/*void*/)> for (usize) {
  fn setPen_0(self , rsthis: & QAbstractGraphicsShapeItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN26QAbstractGraphicsShapeItem6setPenERK4QPen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:609
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush brush() const

/*

*/
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn brush_0<RetType, T: QAbstractGraphicsShapeItem_brush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brush_0(self);
    // return 1;
  }
}
pub trait QAbstractGraphicsShapeItem_brush_0<RetType> {
  fn brush_0(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_brush_0<usize> for () {
  fn brush_0(self , rsthis: & QAbstractGraphicsShapeItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAbstractGraphicsShapeItem5brushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:610
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBrush(const QBrush &)

/*

*/
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn setBrush_0<RetType, T: QAbstractGraphicsShapeItem_setBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrush_0(self);
    // return 1;
  }
}
pub trait QAbstractGraphicsShapeItem_setBrush_0<RetType> {
  fn setBrush_0(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_setBrush_0<(/*void*/)> for (usize) {
  fn setBrush_0(self , rsthis: & QAbstractGraphicsShapeItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN26QAbstractGraphicsShapeItem8setBrushERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:612
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isObscuredBy(const QGraphicsItem *) const

/*
Returns true if this item's bounding rect is completely obscured by the opaque shape of item.

The base implementation maps item's opaqueArea() to this item's coordinate system, and then checks if this item's boundingRect() is fully contained within the mapped shape.

You can reimplement this function to provide a custom algorithm for determining whether this item is obscured by item.

See also opaqueArea() and isObscured().
*/
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn isObscuredBy_0<RetType, T: QAbstractGraphicsShapeItem_isObscuredBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy_0(self);
    // return 1;
  }
}
pub trait QAbstractGraphicsShapeItem_isObscuredBy_0<RetType> {
  fn isObscuredBy_0(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_isObscuredBy_0<bool> for (usize) {
  fn isObscuredBy_0(self , rsthis: & QAbstractGraphicsShapeItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAbstractGraphicsShapeItem12isObscuredByEPK13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:613
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath opaqueArea() const

/*
This virtual function returns a shape representing the area where this item is opaque. An area is opaque if it is filled using an opaque brush or color (i.e., not transparent).

This function is used by isObscuredBy(), which is called by underlying items to determine if they are obscured by this item.

The default implementation returns an empty QPainterPath, indicating that this item is completely transparent and does not obscure any other items.

See also isObscuredBy(), isObscured(), and shape().
*/
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn opaqueArea_0<RetType, T: QAbstractGraphicsShapeItem_opaqueArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea_0(self);
    // return 1;
  }
}
pub trait QAbstractGraphicsShapeItem_opaqueArea_0<RetType> {
  fn opaqueArea_0(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_opaqueArea_0<usize> for () {
  fn opaqueArea_0(self , rsthis: & QAbstractGraphicsShapeItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAbstractGraphicsShapeItem10opaqueAreaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
