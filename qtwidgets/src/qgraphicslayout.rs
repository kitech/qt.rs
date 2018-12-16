

// mod ::widgets::QGraphicsLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicslayout.h
// #include <qgraphicslayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 40
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

// void addChildLayoutItem(QGraphicsLayoutItem *)
// func (this *QGraphicsLayout) InheritAddChildLayoutItem(f func(layoutItem *QGraphicsLayoutItem/*777 QGraphicsLayoutItem **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "addChildLayoutItem", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsLayout)=16
pub struct QGraphicsLayout {
  qbase: QGraphicsLayoutItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsLayout_ITF interface {
//    QGraphicsLayoutItem_ITF
//    QGraphicsLayout_PTR() *QGraphicsLayout
//}
//func (ptr *QGraphicsLayout) QGraphicsLayout_PTR() *QGraphicsLayout { return ptr }

impl /*struct*/ QGraphicsLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsLayout {
    return QGraphicsLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsLayout {
//  type Target = QGraphicsLayoutBASE;
//
//  fn deref(&self) -> &QGraphicsLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsLayoutBASE> for QGraphicsLayout {
//  fn as_ref(& self) -> & QGraphicsLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicslayout.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsLayout(QGraphicsLayoutItem *)

/*
Contructs a QGraphicsLayout object.

parent is passed to QGraphicsLayoutItem's constructor and the QGraphicsLayoutItem's isLayout argument is set to true.

If parent is a QGraphicsWidget the layout will be installed on that widget. (Note that installing a layout will delete the old one installed.)
*/
// QGraphicsLayout(QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsLayout {
  pub fn QGraphicsLayout_0<T: QGraphicsLayout_QGraphicsLayout_0>(value: T) -> QGraphicsLayout {
    let rsthis = value.QGraphicsLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLayout_QGraphicsLayout_0 {
  fn QGraphicsLayout_0(self) -> QGraphicsLayout;
}
// QGraphicsLayout(QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsLayout_QGraphicsLayout_0 for (usize) {
  fn QGraphicsLayout_0(self) -> QGraphicsLayout {
    // unsafe{_ZN15QGraphicsLayoutC2EP19QGraphicsLayoutItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QGraphicsLayoutC2EP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsLayout()

/*

*/
pub fn DeleteQGraphicsLayout(this :*mut QGraphicsLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QGraphicsLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicslayout.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContentsMargins(qreal, qreal, qreal, qreal)

/*
Sets the contents margins to left, top, right and bottom. The default contents margins for toplevel layouts are style dependent (by querying the pixelMetric for QStyle::PM_LayoutLeftMargin, QStyle::PM_LayoutTopMargin, QStyle::PM_LayoutRightMargin and QStyle::PM_LayoutBottomMargin).

For sublayouts the default margins are 0.

Changing the contents margins automatically invalidates the layout.

See also invalidate().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn setContentsMargins_0<RetType, T: QGraphicsLayout_setContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_setContentsMargins_0<RetType> {
  fn setContentsMargins_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_setContentsMargins_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setContentsMargins_0(self , rsthis: & QGraphicsLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout18setContentsMarginsEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void getContentsMargins(qreal *, qreal *, qreal *, qreal *) const

/*
Reimplemented from QGraphicsLayoutItem::getContentsMargins().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn getContentsMargins_0<RetType, T: QGraphicsLayout_getContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_getContentsMargins_0<RetType> {
  fn getContentsMargins_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_getContentsMargins_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getContentsMargins_0(self , rsthis: & QGraphicsLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activate()

/*
Activates the layout, causing all items in the layout to be immediately rearranged. This function is based on calling count() and itemAt(), and then calling setGeometry() on all items sequentially. When activated, the layout will adjust its geometry to its parent's contentsRect(). The parent will then invalidate any layout of its own.

If called in sequence or recursively, e.g., by one of the arranged items in response to being resized, this function will do nothing.

Note that the layout is free to use geometry caching to optimize this process. To forcefully invalidate any such cache, you can call invalidate() before calling activate().

See also invalidate().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn activate_0<RetType, T: QGraphicsLayout_activate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activate_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_activate_0<RetType> {
  fn activate_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_activate_0<(/*void*/)> for () {
  fn activate_0(self , rsthis: & QGraphicsLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout8activateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:64
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isActivated() const

/*
Returns true if the layout is currently being activated; otherwise, returns false. If the layout is being activated, this means that it is currently in the process of rearranging its items (i.e., the activate() function has been called, and has not yet returned).

See also activate() and invalidate().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn isActivated_0<RetType, T: QGraphicsLayout_isActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActivated_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_isActivated_0<RetType> {
  fn isActivated_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_isActivated_0<bool> for () {
  fn isActivated_0(self , rsthis: & QGraphicsLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsLayout11isActivatedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Clears any cached geometry and size hint information in the layout, and posts a LayoutRequest event to the managed parent QGraphicsLayoutItem.

See also activate() and setGeometry().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn invalidate_0<RetType, T: QGraphicsLayout_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QGraphicsLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void updateGeometry()

/*
Reimplemented from QGraphicsLayoutItem::updateGeometry().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn updateGeometry_0<RetType, T: QGraphicsLayout_updateGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_updateGeometry_0<RetType> {
  fn updateGeometry_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_updateGeometry_0<(/*void*/)> for () {
  fn updateGeometry_0(self , rsthis: & QGraphicsLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout14updateGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void widgetEvent(QEvent *)

/*
This virtual event handler receives all events for the managed widget. QGraphicsLayout uses this event handler to listen for layout related events such as geometry changes, layout changes or layout direction changes.

e is a pointer to the event.

You can reimplement this event handler to track similar events for your own custom layout.

See also QGraphicsWidget::event() and QGraphicsItem::sceneEvent().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn widgetEvent_0<RetType, T: QGraphicsLayout_widgetEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widgetEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_widgetEvent_0<RetType> {
  fn widgetEvent_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_widgetEvent_0<(/*void*/)> for (usize) {
  fn widgetEvent_0(self , rsthis: & QGraphicsLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout11widgetEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:70
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int count() const

/*
This pure virtual function must be reimplemented in a subclass of QGraphicsLayout to return the number of items in the layout.

The subclass is free to decide how to store the items.

See also itemAt() and removeAt().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn count_0<RetType, T: QGraphicsLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:71
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QGraphicsLayoutItem * itemAt(int) const

/*
This pure virtual function must be reimplemented in a subclass of QGraphicsLayout to return a pointer to the item at index i. The reimplementation can assume that i is valid (i.e., it respects the value of count()). Together with count(), it is provided as a means of iterating over all items in a layout.

The subclass is free to decide how to store the items, and the visual arrangement does not have to be reflected through this function.

See also count() and removeAt().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn itemAt_0<RetType, T: QGraphicsLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_itemAt_0<usize> for (i32) {
  fn itemAt_0(self , rsthis: & QGraphicsLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:72
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void removeAt(int)

/*
This pure virtual function must be reimplemented in a subclass of QGraphicsLayout to remove the item at index. The reimplementation can assume that index is valid (i.e., it respects the value of count()).

The implementation must ensure that the parentLayoutItem() of the removed item does not point to this layout, since the item is considered to be removed from the layout hierarchy.

If the layout is to be reused between applications, we recommend that the layout deletes the item, but the graphics view framework does not depend on this.

The subclass is free to decide how to store the items.

See also itemAt() and count().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn removeAt_0<RetType, T: QGraphicsLayout_removeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_removeAt_0<RetType> {
  fn removeAt_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_removeAt_0<(/*void*/)> for (i32) {
  fn removeAt_0(self , rsthis: & QGraphicsLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout8removeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:74
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setInstantInvalidatePropagation(bool)

/*

*/
impl /*struct*/ QGraphicsLayout {
  pub fn setInstantInvalidatePropagation_0<RetType, T: QGraphicsLayout_setInstantInvalidatePropagation_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setInstantInvalidatePropagation_0();
    // return 1;
  }
}
pub trait QGraphicsLayout_setInstantInvalidatePropagation_0<RetType> {
  fn setInstantInvalidatePropagation_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_setInstantInvalidatePropagation_0<(/*void*/)> for (bool) {
  fn setInstantInvalidatePropagation_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:75
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool instantInvalidatePropagation()

/*

*/
impl /*struct*/ QGraphicsLayout {
  pub fn instantInvalidatePropagation_0<RetType, T: QGraphicsLayout_instantInvalidatePropagation_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.instantInvalidatePropagation_0();
    // return 1;
  }
}
pub trait QGraphicsLayout_instantInvalidatePropagation_0<RetType> {
  fn instantInvalidatePropagation_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_instantInvalidatePropagation_0<bool> for () {
  fn instantInvalidatePropagation_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout28instantInvalidatePropagationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayout.h:78
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void addChildLayoutItem(QGraphicsLayoutItem *)

/*
This function is a convenience function provided for custom layouts, and will go through all items in the layout and reparent their graphics items to the closest QGraphicsWidget ancestor of the layout.

If layoutItem is already in a different layout, it will be removed from that layout.

If custom layouts want special behaviour they can ignore to use this function, and implement their own behaviour.

This function was introduced in  Qt 4.6.

See also graphicsItem().
*/
impl /*struct*/ QGraphicsLayout {
  pub fn addChildLayoutItem_0<RetType, T: QGraphicsLayout_addChildLayoutItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addChildLayoutItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayout_addChildLayoutItem_0<RetType> {
  fn addChildLayoutItem_0(self , rsthis: & QGraphicsLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayout_addChildLayoutItem_0<(/*void*/)> for (usize) {
  fn addChildLayoutItem_0(self , rsthis: & QGraphicsLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsLayout18addChildLayoutItemEP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
