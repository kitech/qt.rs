

// mod ::widgets::QGraphicsItem
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
// extern C begin: 7
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

// void updateMicroFocus()
// func (this *QGraphicsItem) InheritUpdateMicroFocus(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateMicroFocus", f)
// }

// bool sceneEventFilter(QGraphicsItem *, QEvent *)
// func (this *QGraphicsItem) InheritSceneEventFilter(f func(watched *QGraphicsItem/*777 QGraphicsItem **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "sceneEventFilter", f)
// }

// bool sceneEvent(QEvent *)
// func (this *QGraphicsItem) InheritSceneEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "sceneEvent", f)
// }

// void contextMenuEvent(QGraphicsSceneContextMenuEvent *)
// func (this *QGraphicsItem) InheritContextMenuEvent(f func(event *QGraphicsSceneContextMenuEvent/*777 QGraphicsSceneContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void dragEnterEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsItem) InheritDragEnterEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragLeaveEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsItem) InheritDragLeaveEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dragMoveEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsItem) InheritDragMoveEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dropEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsItem) InheritDropEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QGraphicsItem) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QGraphicsItem) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void hoverEnterEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsItem) InheritHoverEnterEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverEnterEvent", f)
// }

// void hoverMoveEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsItem) InheritHoverMoveEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverMoveEvent", f)
// }

// void hoverLeaveEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsItem) InheritHoverLeaveEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverLeaveEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QGraphicsItem) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QGraphicsItem) InheritKeyReleaseEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void mousePressEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsItem) InheritMousePressEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsItem) InheritMouseMoveEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsItem) InheritMouseReleaseEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsItem) InheritMouseDoubleClickEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void wheelEvent(QGraphicsSceneWheelEvent *)
// func (this *QGraphicsItem) InheritWheelEvent(f func(event *QGraphicsSceneWheelEvent/*777 QGraphicsSceneWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QGraphicsItem) InheritInputMethodEvent(f func(event *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// QVariant inputMethodQuery(Qt::InputMethodQuery)
// func (this *QGraphicsItem) InheritInputMethodQuery(f func(query int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "inputMethodQuery", f)
// }

// QVariant itemChange(QGraphicsItem::GraphicsItemChange, const QVariant &)
// func (this *QGraphicsItem) InheritItemChange(f func(change int, value *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "itemChange", f)
// }

// bool supportsExtension(QGraphicsItem::Extension)
// func (this *QGraphicsItem) InheritSupportsExtension(f func(extension int) bool) {
//  qtrt.SetAllInheritCallback(this, "supportsExtension", f)
// }

// void setExtension(QGraphicsItem::Extension, const QVariant &)
// func (this *QGraphicsItem) InheritSetExtension(f func(extension int, variant *qtcore.QVariant) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setExtension", f)
// }

// QVariant extension(const QVariant &)
// func (this *QGraphicsItem) InheritExtension(f func(variant *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "extension", f)
// }

// void addToIndex()
// func (this *QGraphicsItem) InheritAddToIndex(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "addToIndex", f)
// }

// void removeFromIndex()
// func (this *QGraphicsItem) InheritRemoveFromIndex(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "removeFromIndex", f)
// }

// void prepareGeometryChange()
// func (this *QGraphicsItem) InheritPrepareGeometryChange(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "prepareGeometryChange", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsItem)=16
pub struct QGraphicsItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsItem_ITF interface {
//    QGraphicsItem_PTR() *QGraphicsItem
//}
//func (ptr *QGraphicsItem) QGraphicsItem_PTR() *QGraphicsItem { return ptr }

impl /*struct*/ QGraphicsItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsItem {
    return QGraphicsItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsItem {
//  type Target = QGraphicsItemBASE;
//
//  fn deref(&self) -> &QGraphicsItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsItemBASE> for QGraphicsItem {
//  fn as_ref(& self) -> & QGraphicsItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsItem(QGraphicsItem *)

/*
Constructs a QGraphicsItem with the given parent item. It does not modify the parent object returned by QObject::parent().

If parent is 0, you can add the item to a scene by calling QGraphicsScene::addItem(). The item will then become a top-level item.

See also QGraphicsScene::addItem() and setParentItem().
*/
// QGraphicsItem(QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsItem {
  pub fn QGraphicsItem_0<T: QGraphicsItem_QGraphicsItem_0>(value: T) -> QGraphicsItem {
    let rsthis = value.QGraphicsItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItem_QGraphicsItem_0 {
  fn QGraphicsItem_0(self) -> QGraphicsItem;
}
// QGraphicsItem(QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsItem_QGraphicsItem_0 for (usize) {
  fn QGraphicsItem_0(self) -> QGraphicsItem {
    // unsafe{_ZN13QGraphicsItemC2EPS_()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QGraphicsItemC2EPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:162
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsItem()

/*

*/
pub fn DeleteQGraphicsItem(this :*mut QGraphicsItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QGraphicsItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:164
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsScene * scene() const

/*
Returns the current scene for the item, or 0 if the item is not stored in a scene.

To add or move an item to a scene, call QGraphicsScene::addItem().
*/
impl /*struct*/ QGraphicsItem {
  pub fn scene_0<RetType, T: QGraphicsItem_scene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scene_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_scene_0<RetType> {
  fn scene_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_scene_0<usize> for () {
  fn scene_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem5sceneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:166
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * parentItem() const

/*
Returns a pointer to this item's parent item. If this item does not have a parent, 0 is returned.

See also setParentItem() and childItems().
*/
impl /*struct*/ QGraphicsItem {
  pub fn parentItem_0<RetType, T: QGraphicsItem_parentItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_parentItem_0<RetType> {
  fn parentItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_parentItem_0<usize> for () {
  fn parentItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10parentItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:167
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * topLevelItem() const

/*
Returns this item's top-level item. The top-level item is the item's topmost ancestor item whose parent is 0. If an item has no parent, its own pointer is returned (i.e., a top-level item is its own top-level item).

See also parentItem().
*/
impl /*struct*/ QGraphicsItem {
  pub fn topLevelItem_0<RetType, T: QGraphicsItem_topLevelItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLevelItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_topLevelItem_0<RetType> {
  fn topLevelItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_topLevelItem_0<usize> for () {
  fn topLevelItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12topLevelItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:169
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsWidget * parentWidget() const

/*
Returns a pointer to the item's parent widget. The item's parent widget is the closest parent item that is a widget.

This function was introduced in  Qt 4.4.

See also parentItem() and childItems().
*/
impl /*struct*/ QGraphicsItem {
  pub fn parentWidget_0<RetType, T: QGraphicsItem_parentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_parentWidget_0<RetType> {
  fn parentWidget_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_parentWidget_0<usize> for () {
  fn parentWidget_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12parentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:170
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsWidget * topLevelWidget() const

/*
Returns a pointer to the item's top level widget (i.e., the item's ancestor whose parent is 0, or whose parent is not a widget), or 0 if this item does not have a top level widget. If the item is its own top level widget, this function returns a pointer to the item itself.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QGraphicsItem {
  pub fn topLevelWidget_0<RetType, T: QGraphicsItem_topLevelWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLevelWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_topLevelWidget_0<RetType> {
  fn topLevelWidget_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_topLevelWidget_0<usize> for () {
  fn topLevelWidget_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem14topLevelWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:171
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsWidget * window() const

/*
Returns the item's window, or 0 if this item does not have a window. If the item is a window, it will return itself. Otherwise it will return the closest ancestor that is a window.

This function was introduced in  Qt 4.4.

See also QGraphicsWidget::isWindow().
*/
impl /*struct*/ QGraphicsItem {
  pub fn window_0<RetType, T: QGraphicsItem_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_window_0<RetType> {
  fn window_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_window_0<usize> for () {
  fn window_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:172
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * panel() const

/*
Returns the item's panel, or 0 if this item does not have a panel. If the item is a panel, it will return itself. Otherwise it will return the closest ancestor that is a panel.

This function was introduced in  Qt 4.6.

See also isPanel() and ItemIsPanel.
*/
impl /*struct*/ QGraphicsItem {
  pub fn panel_0<RetType, T: QGraphicsItem_panel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.panel_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_panel_0<RetType> {
  fn panel_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_panel_0<usize> for () {
  fn panel_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem5panelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setParentItem(QGraphicsItem *)

/*
Sets this item's parent item to newParent. If this item already has a parent, it is first removed from the previous parent. If newParent is 0, this item will become a top-level item.

Note that this implicitly adds this graphics item to the scene of the parent. You should not add the item to the scene yourself.

The behavior when calling this function on an item that is an ancestor of newParent is undefined.

See also parentItem() and childItems().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setParentItem_0<RetType, T: QGraphicsItem_setParentItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setParentItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setParentItem_0<RetType> {
  fn setParentItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setParentItem_0<(/*void*/)> for (usize) {
  fn setParentItem_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem13setParentItemEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:177
// index:0
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> childItems() const

/*
Returns a list of this item's children.

The items are sorted by stacking order. This takes into account both the items' insertion order and their Z-values.

This function was introduced in  Qt 4.4.

See also setParentItem(), zValue(), and Sorting.
*/
impl /*struct*/ QGraphicsItem {
  pub fn childItems_0<RetType, T: QGraphicsItem_childItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childItems_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_childItems_0<RetType> {
  fn childItems_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_childItems_0<usize> for () {
  fn childItems_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10childItemsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:178
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWidget() const

/*
Returns true if this item is a widget (i.e., QGraphicsWidget); otherwise, returns false.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QGraphicsItem {
  pub fn isWidget_0<RetType, T: QGraphicsItem_isWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isWidget_0<RetType> {
  fn isWidget_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isWidget_0<bool> for () {
  fn isWidget_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem8isWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:179
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWindow() const

/*
Returns true if the item is a QGraphicsWidget window, otherwise returns false.

This function was introduced in  Qt 4.4.

See also QGraphicsWidget::windowFlags().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isWindow_0<RetType, T: QGraphicsItem_isWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWindow_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isWindow_0<RetType> {
  fn isWindow_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isWindow_0<bool> for () {
  fn isWindow_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem8isWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:180
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPanel() const

/*
Returns true if the item is a panel; otherwise returns false.

This function was introduced in  Qt 4.6.

See also QGraphicsItem::panel() and ItemIsPanel.
*/
impl /*struct*/ QGraphicsItem {
  pub fn isPanel_0<RetType, T: QGraphicsItem_isPanel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPanel_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isPanel_0<RetType> {
  fn isPanel_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isPanel_0<bool> for () {
  fn isPanel_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem7isPanelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:185
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItemGroup * group() const

/*
Returns a pointer to this item's item group, or 0 if this item is not member of a group.

See also setGroup(), QGraphicsItemGroup, and QGraphicsScene::createItemGroup().
*/
impl /*struct*/ QGraphicsItem {
  pub fn group_0<RetType, T: QGraphicsItem_group_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.group_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_group_0<RetType> {
  fn group_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_group_0<usize> for () {
  fn group_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem5groupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGroup(QGraphicsItemGroup *)

/*
Adds this item to the item group group. If group is 0, this item is removed from any current group and added as a child of the previous group's parent.

See also group() and QGraphicsScene::createItemGroup().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setGroup_0<RetType, T: QGraphicsItem_setGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGroup_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setGroup_0<RetType> {
  fn setGroup_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setGroup_0<(/*void*/)> for (usize) {
  fn setGroup_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:188
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsItem::GraphicsItemFlags flags() const

/*
Returns this item's flags. The flags describe what configurable features of the item are enabled and not. For example, if the flags include ItemIsFocusable, the item can accept input focus.

By default, no flags are enabled.

See also setFlags() and setFlag().
*/
impl /*struct*/ QGraphicsItem {
  pub fn flags_0<RetType, T: QGraphicsItem_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_flags_0<RetType> {
  fn flags_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlag(QGraphicsItem::GraphicsItemFlag, bool)

/*
If enabled is true, the item flag flag is enabled; otherwise, it is disabled.

See also flags() and setFlags().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setFlag_0<RetType, T: QGraphicsItem_setFlag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlag_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setFlag_0<RetType> {
  fn setFlag_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setFlag_0<(/*void*/)> for (i32,bool) {
  fn setFlag_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem7setFlagENS_16GraphicsItemFlagEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(QGraphicsItem::GraphicsItemFlags)

/*
Sets the item flags to flags. All flags in flags are enabled; all flags not in flags are disabled.

If the item had focus and flags does not enable ItemIsFocusable, the item loses focus as a result of calling this function. Similarly, if the item was selected, and flags does not enabled ItemIsSelectable, the item is automatically unselected.

By default, no flags are enabled. (QGraphicsWidget enables the ItemSendsGeometryChanges flag by default in order to track position changes.)

See also flags() and setFlag().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setFlags_0<RetType, T: QGraphicsItem_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem8setFlagsE6QFlagsINS_16GraphicsItemFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:192
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsItem::CacheMode cacheMode() const

/*
Returns the cache mode for this item. The default mode is NoCache (i.e., cache is disabled and all painting is immediate).

This function was introduced in  Qt 4.4.

See also setCacheMode().
*/
impl /*struct*/ QGraphicsItem {
  pub fn cacheMode_0<RetType, T: QGraphicsItem_cacheMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cacheMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_cacheMode_0<RetType> {
  fn cacheMode_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_cacheMode_0<i32> for () {
  fn cacheMode_0(self , rsthis: & QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9cacheModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:193
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCacheMode(QGraphicsItem::CacheMode, const QSize &)

/*
Sets the item's cache mode to mode.

The optional logicalCacheSize argument is used only by ItemCoordinateCache mode, and describes the resolution of the cache buffer; if logicalCacheSize is (100, 100), QGraphicsItem will fit the item into 100x100 pixels in graphics memory, regardless of the logical size of the item itself. By default QGraphicsItem uses the size of boundingRect(). For all other cache modes than ItemCoordinateCache, logicalCacheSize is ignored.

Caching can speed up rendering if your item spends a significant time redrawing itself. In some cases the cache can also slow down rendering, in particular when the item spends less time redrawing than QGraphicsItem spends redrawing from the cache.

When caching is enabled, an item's paint() function will generally draw into an offscreen pixmap cache; for any subsequent repaint requests, the Graphics View framework will redraw from the cache. This approach works particularly well with QGLWidget, which stores all the cache as OpenGL textures.

Be aware that QPixmapCache's cache limit may need to be changed to obtain optimal performance.

You can read more about the different cache modes in the CacheMode documentation.

Note: Enabling caching does not imply that the item's paint() function will be called only in response to an explicit update() call. For instance, under memory pressure, Qt may decide to drop some of the cache information; in such cases an item's paint() function will be called even if there was no update() call (that is, exactly as if there were no caching enabled).

This function was introduced in  Qt 4.4.

See also cacheMode(), CacheMode, and QPixmapCache::setCacheLimit().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setCacheMode_0<RetType, T: QGraphicsItem_setCacheMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCacheMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setCacheMode_0<RetType> {
  fn setCacheMode_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setCacheMode_0<(/*void*/)> for (i32,usize) {
  fn setCacheMode_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem12setCacheModeENS_9CacheModeERK5QSize", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:195
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsItem::PanelModality panelModality() const

/*
Returns the modality for this item.

This function was introduced in  Qt 4.6.

See also setPanelModality().
*/
impl /*struct*/ QGraphicsItem {
  pub fn panelModality_0<RetType, T: QGraphicsItem_panelModality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.panelModality_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_panelModality_0<RetType> {
  fn panelModality_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_panelModality_0<i32> for () {
  fn panelModality_0(self , rsthis: & QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13panelModalityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:196
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPanelModality(QGraphicsItem::PanelModality)

/*
Sets the modality for this item to panelModality.

Changing the modality of a visible item takes effect immediately.

This function was introduced in  Qt 4.6.

See also panelModality().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setPanelModality_0<RetType, T: QGraphicsItem_setPanelModality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPanelModality_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setPanelModality_0<RetType> {
  fn setPanelModality_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setPanelModality_0<(/*void*/)> for (i32) {
  fn setPanelModality_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem16setPanelModalityENS_13PanelModalityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:197
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isBlockedByModalPanel(QGraphicsItem **) const

/*
Returns true if this item is blocked by a modal panel, false otherwise. If blockingPanel is non-zero, blockingPanel will be set to the modal panel that is blocking this item. If this item is not blocked, blockingPanel will not be set by this function.

This function always returns false for items not in a scene.

This function was introduced in  Qt 4.6.

See also panelModality(), setPanelModality(), and PanelModality.
*/
impl /*struct*/ QGraphicsItem {
  pub fn isBlockedByModalPanel_0<RetType, T: QGraphicsItem_isBlockedByModalPanel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isBlockedByModalPanel_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isBlockedByModalPanel_0<RetType> {
  fn isBlockedByModalPanel_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isBlockedByModalPanel_0<bool> for (usize) {
  fn isBlockedByModalPanel_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:200
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toolTip() const

/*
Returns the item's tool tip, or an empty QString if no tool tip has been set.

See also setToolTip() and QToolTip.
*/
impl /*struct*/ QGraphicsItem {
  pub fn toolTip_0<RetType, T: QGraphicsItem_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_toolTip_0<usize> for () {
  fn toolTip_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem7toolTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:201
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolTip(const QString &)

/*
Sets the item's tool tip to toolTip. If toolTip is empty, the item's tool tip is cleared.

See also toolTip() and QToolTip.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setToolTip_0<RetType, T: QGraphicsItem_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setToolTip_0<(/*void*/)> for (usize) {
  fn setToolTip_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10setToolTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:205
// index:0
// Public Visibility=Default Availability=Available
// [8] QCursor cursor() const

/*
Returns the current cursor shape for the item. The mouse cursor will assume this shape when it's over this item. See the list of predefined cursor objects for a range of useful shapes.

An editor item might want to use an I-beam cursor:


  item->setCursor(Qt::IBeamCursor);



If no cursor has been set, the cursor of the item beneath is used.

See also setCursor(), hasCursor(), unsetCursor(), QWidget::cursor, and QApplication::overrideCursor().
*/
impl /*struct*/ QGraphicsItem {
  pub fn cursor_0<RetType, T: QGraphicsItem_cursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursor_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_cursor_0<RetType> {
  fn cursor_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_cursor_0<usize> for () {
  fn cursor_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem6cursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:206
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursor(const QCursor &)

/*
Sets the current cursor shape for the item to cursor. The mouse cursor will assume this shape when it's over this item. See the list of predefined cursor objects for a range of useful shapes.

An editor item might want to use an I-beam cursor:


  item->setCursor(Qt::IBeamCursor);



If no cursor has been set, the cursor of the item beneath is used.

See also cursor(), hasCursor(), unsetCursor(), QWidget::cursor, and QApplication::overrideCursor().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setCursor_0<RetType, T: QGraphicsItem_setCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursor_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setCursor_0<RetType> {
  fn setCursor_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setCursor_0<(/*void*/)> for (usize) {
  fn setCursor_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem9setCursorERK7QCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:207
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasCursor() const

/*
Returns true if this item has a cursor set; otherwise, false is returned.

By default, items don't have any cursor set. cursor() will return a standard pointing arrow cursor.

See also unsetCursor().
*/
impl /*struct*/ QGraphicsItem {
  pub fn hasCursor_0<RetType, T: QGraphicsItem_hasCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasCursor_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_hasCursor_0<RetType> {
  fn hasCursor_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_hasCursor_0<bool> for () {
  fn hasCursor_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9hasCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetCursor()

/*
Clears the cursor from this item.

See also hasCursor() and setCursor().
*/
impl /*struct*/ QGraphicsItem {
  pub fn unsetCursor_0<RetType, T: QGraphicsItem_unsetCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_unsetCursor_0<RetType> {
  fn unsetCursor_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_unsetCursor_0<(/*void*/)> for () {
  fn unsetCursor_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem11unsetCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:211
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisible() const

/*
Returns true if the item is visible; otherwise, false is returned.

Note that the item's general visibility is unrelated to whether or not it is actually being visualized by a QGraphicsView.

See also setVisible().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isVisible_0<RetType, T: QGraphicsItem_isVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisible_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isVisible_0<RetType> {
  fn isVisible_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isVisible_0<bool> for () {
  fn isVisible_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:212
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisibleTo(const QGraphicsItem *) const

/*
Returns true if the item is visible to parent; otherwise, false is returned. parent can be 0, in which case this function will return whether the item is visible to the scene or not.

An item may not be visible to its ancestors even if isVisible() is true. It may also be visible to its ancestors even if isVisible() is false. If any ancestor is hidden, the item itself will be implicitly hidden, in which case this function will return false.

This function was introduced in  Qt 4.4.

See also isVisible() and setVisible().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isVisibleTo_0<RetType, T: QGraphicsItem_isVisibleTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisibleTo_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isVisibleTo_0<RetType> {
  fn isVisibleTo_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isVisibleTo_0<bool> for (usize) {
  fn isVisibleTo_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11isVisibleToEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:213
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
If visible is true, the item is made visible. Otherwise, the item is made invisible. Invisible items are not painted, nor do they receive any events. In particular, mouse events pass right through invisible items, and are delivered to any item that may be behind. Invisible items are also unselectable, they cannot take input focus, and are not detected by QGraphicsScene's item location functions.

If an item becomes invisible while grabbing the mouse, (i.e., while it is receiving mouse events,) it will automatically lose the mouse grab, and the grab is not regained by making the item visible again; it must receive a new mouse press to regain the mouse grab.

Similarly, an invisible item cannot have focus, so if the item has focus when it becomes invisible, it will lose focus, and the focus is not regained by simply making the item visible again.

If you hide a parent item, all its children will also be hidden. If you show a parent item, all children will be shown, unless they have been explicitly hidden (i.e., if you call setVisible(false) on a child, it will not be reshown even if its parent is hidden, and then shown again).

Items are visible by default; it is unnecessary to call setVisible() on a new item.

Note: An item with opacity set to 0 will still be considered visible, although it will be treated like an invisible item: mouse events will pass through it, it will not be included in the items returned by QGraphicsView::items(), and so on. However, the item will retain the focus.

See also isVisible(), show(), hide(), and setOpacity().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setVisible_0<RetType, T: QGraphicsItem_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:214
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void hide()

/*
Hides the item (items are visible by default).

This convenience function is equivalent to calling setVisible(false).

See also show() and setVisible().
*/
impl /*struct*/ QGraphicsItem {
  pub fn hide_0<RetType, T: QGraphicsItem_hide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hide_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_hide_0<RetType> {
  fn hide_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_hide_0<(/*void*/)> for () {
  fn hide_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem4hideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:215
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void show()

/*
Shows the item (items are visible by default).

This convenience function is equivalent to calling setVisible(true).

See also hide() and setVisible().
*/
impl /*struct*/ QGraphicsItem {
  pub fn show_0<RetType, T: QGraphicsItem_show_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.show_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_show_0<RetType> {
  fn show_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_show_0<(/*void*/)> for () {
  fn show_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem4showEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:217
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*
Returns true if the item is enabled; otherwise, false is returned.

See also setEnabled().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isEnabled_0<RetType, T: QGraphicsItem_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:218
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*
If enabled is true, the item is enabled; otherwise, it is disabled.

Disabled items are visible, but they do not receive any events, and cannot take focus nor be selected. Mouse events are discarded; they are not propagated unless the item is also invisible, or if it does not accept mouse events (see acceptedMouseButtons()). A disabled item cannot become the mouse grabber, and as a result of this, an item loses the grab if it becomes disabled when grabbing the mouse, just like it loses focus if it had focus when it was disabled.

Disabled items are traditionally drawn using grayed-out colors (see QPalette::Disabled).

If you disable a parent item, all its children will also be disabled. If you enable a parent item, all children will be enabled, unless they have been explicitly disabled (i.e., if you call setEnabled(false) on a child, it will not be reenabled if its parent is disabled, and then enabled again).

Items are enabled by default.

Note: If you install an event filter, you can still intercept events before they are delivered to items; this mechanism disregards the item's enabled state.

See also isEnabled().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setEnabled_0<RetType, T: QGraphicsItem_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:220
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSelected() const

/*
Returns true if this item is selected; otherwise, false is returned.

Items that are in a group inherit the group's selected state.

Items are not selected by default.

See also setSelected() and QGraphicsScene::setSelectionArea().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isSelected_0<RetType, T: QGraphicsItem_isSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSelected_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isSelected_0<RetType> {
  fn isSelected_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isSelected_0<bool> for () {
  fn isSelected_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10isSelectedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelected(bool)

/*
If selected is true and this item is selectable, this item is selected; otherwise, it is unselected.

If the item is in a group, the whole group's selected state is toggled by this function. If the group is selected, all items in the group are also selected, and if the group is not selected, no item in the group is selected.

Only visible, enabled, selectable items can be selected. If selected is true and this item is either invisible or disabled or unselectable, this function does nothing.

By default, items cannot be selected. To enable selection, set the ItemIsSelectable flag.

This function is provided for convenience, allowing individual toggling of the selected state of an item. However, a more common way of selecting items is to call QGraphicsScene::setSelectionArea(), which will call this function for all visible, enabled, and selectable items within a specified area on the scene.

See also isSelected() and QGraphicsScene::selectedItems().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setSelected_0<RetType, T: QGraphicsItem_setSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelected_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setSelected_0<RetType> {
  fn setSelected_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setSelected_0<(/*void*/)> for (bool) {
  fn setSelected_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem11setSelectedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:223
// index:0
// Public Visibility=Default Availability=Available
// [1] bool acceptDrops() const

/*
Returns true if this item can accept drag and drop events; otherwise, returns false. By default, items do not accept drag and drop events; items are transparent to drag and drop.

See also setAcceptDrops().
*/
impl /*struct*/ QGraphicsItem {
  pub fn acceptDrops_0<RetType, T: QGraphicsItem_acceptDrops_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptDrops_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_acceptDrops_0<RetType> {
  fn acceptDrops_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_acceptDrops_0<bool> for () {
  fn acceptDrops_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11acceptDropsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:224
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAcceptDrops(bool)

/*
If on is true, this item will accept drag and drop events; otherwise, it is transparent for drag and drop events. By default, items do not accept drag and drop events.

See also acceptDrops().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptDrops_0<RetType, T: QGraphicsItem_setAcceptDrops_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAcceptDrops_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setAcceptDrops_0<RetType> {
  fn setAcceptDrops_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setAcceptDrops_0<(/*void*/)> for (bool) {
  fn setAcceptDrops_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem14setAcceptDropsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:226
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal opacity() const

/*
Returns this item's local opacity, which is between 0.0 (transparent) and 1.0 (opaque). This value is combined with parent and ancestor values into the effectiveOpacity(). The effective opacity decides how the item is rendered and also affects its visibility when queried by functions such as QGraphicsView::items().

The opacity property decides the state of the painter passed to the paint() function. If the item is cached, i.e., ItemCoordinateCache or DeviceCoordinateCache, the effective property will be applied to the item's cache as it is rendered.

The default opacity is 1.0; fully opaque.

This function was introduced in  Qt 4.5.

See also setOpacity(), paint(), ItemIgnoresParentOpacity, and ItemDoesntPropagateOpacityToChildren.
*/
impl /*struct*/ QGraphicsItem {
  pub fn opacity_0<RetType, T: QGraphicsItem_opacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opacity_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_opacity_0<RetType> {
  fn opacity_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_opacity_0<f64> for () {
  fn opacity_0(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem7opacityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:227
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal effectiveOpacity() const

/*
Returns this item's effective opacity, which is between 0.0 (transparent) and 1.0 (opaque). This value is a combination of this item's local opacity, and its parent and ancestors' opacities. The effective opacity decides how the item is rendered.

This function was introduced in  Qt 4.5.

See also opacity(), setOpacity(), paint(), ItemIgnoresParentOpacity, and ItemDoesntPropagateOpacityToChildren.
*/
impl /*struct*/ QGraphicsItem {
  pub fn effectiveOpacity_0<RetType, T: QGraphicsItem_effectiveOpacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.effectiveOpacity_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_effectiveOpacity_0<RetType> {
  fn effectiveOpacity_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_effectiveOpacity_0<f64> for () {
  fn effectiveOpacity_0(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem16effectiveOpacityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:228
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpacity(qreal)

/*
Sets this item's local opacity, between 0.0 (transparent) and 1.0 (opaque). The item's local opacity is combined with parent and ancestor opacities into the effectiveOpacity().

By default, opacity propagates from parent to child, so if a parent's opacity is 0.5 and the child is also 0.5, the child's effective opacity will be 0.25.

The opacity property decides the state of the painter passed to the paint() function. If the item is cached, i.e., ItemCoordinateCache or DeviceCoordinateCache, the effective property will be applied to the item's cache as it is rendered.

There are two item flags that affect how the item's opacity is combined with the parent: ItemIgnoresParentOpacity and ItemDoesntPropagateOpacityToChildren.

Note: Setting the opacity of an item to 0 will not make the item invisible (according to isVisible()), but the item will be treated like an invisible one. See the documentation of setVisible() for more information.

This function was introduced in  Qt 4.5.

See also opacity(), effectiveOpacity(), and setVisible().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setOpacity_0<RetType, T: QGraphicsItem_setOpacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpacity_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setOpacity_0<RetType> {
  fn setOpacity_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setOpacity_0<(/*void*/)> for (f64) {
  fn setOpacity_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10setOpacityEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:232
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsEffect * graphicsEffect() const

/*
Returns a pointer to this item's effect if it has one; otherwise 0.

This function was introduced in  Qt 4.6.

See also setGraphicsEffect().
*/
impl /*struct*/ QGraphicsItem {
  pub fn graphicsEffect_0<RetType, T: QGraphicsItem_graphicsEffect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.graphicsEffect_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_graphicsEffect_0<RetType> {
  fn graphicsEffect_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_graphicsEffect_0<usize> for () {
  fn graphicsEffect_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem14graphicsEffectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:233
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGraphicsEffect(QGraphicsEffect *)

/*
Sets effect as the item's effect. If there already is an effect installed on this item, QGraphicsItem will delete the existing effect before installing the new effect. You can delete an existing effect by calling setGraphicsEffect(0).

If effect is the installed effect on a different item, setGraphicsEffect() will remove the effect from the item and install it on this item.

QGraphicsItem takes ownership of effect.

Note: This function will apply the effect on itself and all its children.

This function was introduced in  Qt 4.6.

See also graphicsEffect().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setGraphicsEffect_0<RetType, T: QGraphicsItem_setGraphicsEffect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGraphicsEffect_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setGraphicsEffect_0<RetType> {
  fn setGraphicsEffect_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setGraphicsEffect_0<(/*void*/)> for (usize) {
  fn setGraphicsEffect_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:236
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseButtons acceptedMouseButtons() const

/*
Returns the mouse buttons that this item accepts mouse events for. By default, all mouse buttons are accepted.

If an item accepts a mouse button, it will become the mouse grabber item when a mouse press event is delivered for that mouse button. However, if the item does not accept the button, QGraphicsScene will forward the mouse events to the first item beneath it that does.

See also setAcceptedMouseButtons() and mousePressEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn acceptedMouseButtons_0<RetType, T: QGraphicsItem_acceptedMouseButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptedMouseButtons_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_acceptedMouseButtons_0<RetType> {
  fn acceptedMouseButtons_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_acceptedMouseButtons_0<i32> for () {
  fn acceptedMouseButtons_0(self , rsthis: & QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem20acceptedMouseButtonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:237
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAcceptedMouseButtons(Qt::MouseButtons)

/*
Sets the mouse buttons that this item accepts mouse events for.

By default, all mouse buttons are accepted. If an item accepts a mouse button, it will become the mouse grabber item when a mouse press event is delivered for that button. However, if the item does not accept the mouse button, QGraphicsScene will forward the mouse events to the first item beneath it that does.

To disable mouse events for an item (i.e., make it transparent for mouse events), call setAcceptedMouseButtons(0).

See also acceptedMouseButtons() and mousePressEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptedMouseButtons_0<RetType, T: QGraphicsItem_setAcceptedMouseButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAcceptedMouseButtons_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setAcceptedMouseButtons_0<RetType> {
  fn setAcceptedMouseButtons_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setAcceptedMouseButtons_0<(/*void*/)> for (i32) {
  fn setAcceptedMouseButtons_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem23setAcceptedMouseButtonsE6QFlagsIN2Qt11MouseButtonEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:242
// index:0
// Public Visibility=Default Availability=Available
// [1] bool acceptHoverEvents() const

/*
Returns true if an item accepts hover events (QGraphicsSceneHoverEvent); otherwise, returns false. By default, items do not accept hover events.

This function was introduced in  Qt 4.4.

See also setAcceptHoverEvents() and setAcceptedMouseButtons().
*/
impl /*struct*/ QGraphicsItem {
  pub fn acceptHoverEvents_0<RetType, T: QGraphicsItem_acceptHoverEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptHoverEvents_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_acceptHoverEvents_0<RetType> {
  fn acceptHoverEvents_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_acceptHoverEvents_0<bool> for () {
  fn acceptHoverEvents_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem17acceptHoverEventsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:243
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAcceptHoverEvents(bool)

/*
If enabled is true, this item will accept hover events; otherwise, it will ignore them. By default, items do not accept hover events.

Hover events are delivered when there is no current mouse grabber item. They are sent when the mouse cursor enters an item, when it moves around inside the item, and when the cursor leaves an item. Hover events are commonly used to highlight an item when it's entered, and for tracking the mouse cursor as it hovers over the item (equivalent to QWidget::mouseTracking).

Parent items receive hover enter events before their children, and leave events after their children. The parent does not receive a hover leave event if the cursor enters a child, though; the parent stays "hovered" until the cursor leaves its area, including its children's areas.

If a parent item handles child events, it will receive hover move, drag move, and drop events as the cursor passes through its children, but it does not receive hover enter and hover leave, nor drag enter and drag leave events on behalf of its children.

A QGraphicsWidget with window decorations will accept hover events regardless of the value of acceptHoverEvents().

This function was introduced in  Qt 4.4.

See also acceptHoverEvents(), hoverEnterEvent(), hoverMoveEvent(), and hoverLeaveEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptHoverEvents_0<RetType, T: QGraphicsItem_setAcceptHoverEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAcceptHoverEvents_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setAcceptHoverEvents_0<RetType> {
  fn setAcceptHoverEvents_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setAcceptHoverEvents_0<(/*void*/)> for (bool) {
  fn setAcceptHoverEvents_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem20setAcceptHoverEventsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:244
// index:0
// Public Visibility=Default Availability=Available
// [1] bool acceptTouchEvents() const

/*
Returns true if an item accepts touch events; otherwise, returns false. By default, items do not accept touch events.

This function was introduced in  Qt 4.6.

See also setAcceptTouchEvents().
*/
impl /*struct*/ QGraphicsItem {
  pub fn acceptTouchEvents_0<RetType, T: QGraphicsItem_acceptTouchEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptTouchEvents_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_acceptTouchEvents_0<RetType> {
  fn acceptTouchEvents_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_acceptTouchEvents_0<bool> for () {
  fn acceptTouchEvents_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem17acceptTouchEventsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:245
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAcceptTouchEvents(bool)

/*
If enabled is true, this item will accept touch events; otherwise, it will ignore them. By default, items do not accept touch events.

This function was introduced in  Qt 4.6.

See also acceptTouchEvents().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptTouchEvents_0<RetType, T: QGraphicsItem_setAcceptTouchEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAcceptTouchEvents_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setAcceptTouchEvents_0<RetType> {
  fn setAcceptTouchEvents_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setAcceptTouchEvents_0<(/*void*/)> for (bool) {
  fn setAcceptTouchEvents_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem20setAcceptTouchEventsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:247
// index:0
// Public Visibility=Default Availability=Available
// [1] bool filtersChildEvents() const

/*
Returns true if this item filters child events (i.e., all events intended for any of its children are instead sent to this item); otherwise, false is returned.

The default value is false; child events are not filtered.

This function was introduced in  Qt 4.6.

See also setFiltersChildEvents().
*/
impl /*struct*/ QGraphicsItem {
  pub fn filtersChildEvents_0<RetType, T: QGraphicsItem_filtersChildEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filtersChildEvents_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_filtersChildEvents_0<RetType> {
  fn filtersChildEvents_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_filtersChildEvents_0<bool> for () {
  fn filtersChildEvents_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem18filtersChildEventsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:248
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFiltersChildEvents(bool)

/*
If enabled is true, this item is set to filter all events for all its children (i.e., all events intented for any of its children are instead sent to this item); otherwise, if enabled is false, this item will only handle its own events. The default value is false.

This function was introduced in  Qt 4.6.

See also filtersChildEvents().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setFiltersChildEvents_0<RetType, T: QGraphicsItem_setFiltersChildEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFiltersChildEvents_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setFiltersChildEvents_0<RetType> {
  fn setFiltersChildEvents_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setFiltersChildEvents_0<(/*void*/)> for (bool) {
  fn setFiltersChildEvents_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem21setFiltersChildEventsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:250
// index:0
// Public Visibility=Default Availability=Available
// [1] bool handlesChildEvents() const

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn handlesChildEvents_0<RetType, T: QGraphicsItem_handlesChildEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.handlesChildEvents_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_handlesChildEvents_0<RetType> {
  fn handlesChildEvents_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_handlesChildEvents_0<bool> for () {
  fn handlesChildEvents_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem18handlesChildEventsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:251
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHandlesChildEvents(bool)

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn setHandlesChildEvents_0<RetType, T: QGraphicsItem_setHandlesChildEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHandlesChildEvents_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setHandlesChildEvents_0<RetType> {
  fn setHandlesChildEvents_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setHandlesChildEvents_0<(/*void*/)> for (bool) {
  fn setHandlesChildEvents_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem21setHandlesChildEventsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:253
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isActive() const

/*
Returns true if this item is active; otherwise returns false.

An item can only be active if the scene is active. An item is active if it is, or is a descendent of, an active panel. Items in non-active panels are not active.

Items that are not part of a panel follow scene activation when the scene has no active panel.

Only active items can gain input focus.

This function was introduced in  Qt 4.6.

See also QGraphicsScene::isActive(), QGraphicsScene::activePanel(), panel(), and isPanel().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isActive_0<RetType, T: QGraphicsItem_isActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActive_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isActive_0<RetType> {
  fn isActive_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isActive_0<bool> for () {
  fn isActive_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:254
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActive(bool)

/*
If active is true, and the scene is active, this item's panel will be activated. Otherwise, the panel is deactivated.

If the item is not part of an active scene, active will decide what happens to the panel when the scene becomes active or the item is added to the scene. If true, the item's panel will be activated when the item is either added to the scene or the scene is activated. Otherwise, the item will stay inactive independent of the scene's activated state.

This function was introduced in  Qt 4.6.

See also isPanel(), QGraphicsScene::setActivePanel(), and QGraphicsScene::isActive().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setActive_0<RetType, T: QGraphicsItem_setActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActive_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setActive_0<RetType> {
  fn setActive_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setActive_0<(/*void*/)> for (bool) {
  fn setActive_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem9setActiveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:256
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFocus() const

/*
Returns true if this item is active, and it or its focus proxy has keyboard input focus; otherwise, returns false.

See also focusItem(), setFocus(), QGraphicsScene::setFocusItem(), and isActive().
*/
impl /*struct*/ QGraphicsItem {
  pub fn hasFocus_0<RetType, T: QGraphicsItem_hasFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_hasFocus_0<RetType> {
  fn hasFocus_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_hasFocus_0<bool> for () {
  fn hasFocus_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem8hasFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:257
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocus(Qt::FocusReason)

/*
Gives keyboard input focus to this item. The focusReason argument will be passed into any focus event generated by this function; it is used to give an explanation of what caused the item to get focus.

Only enabled items that set the ItemIsFocusable flag can accept keyboard focus.

If this item is not visible, not active, or not associated with a scene, it will not gain immediate input focus. However, it will be registered as the preferred focus item for its subtree of items, should it later become visible.

As a result of calling this function, this item will receive a focus in event with focusReason. If another item already has focus, that item will first receive a focus out event indicating that it has lost input focus.

See also clearFocus(), hasFocus(), focusItem(), and focusProxy().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setFocus_0<RetType, T: QGraphicsItem_setFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setFocus_0<RetType> {
  fn setFocus_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setFocus_0<(/*void*/)> for (i32) {
  fn setFocus_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem8setFocusEN2Qt11FocusReasonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:258
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearFocus()

/*
Takes keyboard input focus from the item.

If it has focus, a focus out event is sent to this item to tell it that it is about to lose the focus.

Only items that set the ItemIsFocusable flag, or widgets that set an appropriate focus policy, can accept keyboard focus.

See also setFocus(), hasFocus(), and QGraphicsWidget::focusPolicy.
*/
impl /*struct*/ QGraphicsItem {
  pub fn clearFocus_0<RetType, T: QGraphicsItem_clearFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_clearFocus_0<RetType> {
  fn clearFocus_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_clearFocus_0<(/*void*/)> for () {
  fn clearFocus_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10clearFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:260
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * focusProxy() const

/*
Returns this item's focus proxy, or 0 if this item has no focus proxy.

This function was introduced in  Qt 4.6.

See also setFocusProxy(), setFocus(), and hasFocus().
*/
impl /*struct*/ QGraphicsItem {
  pub fn focusProxy_0<RetType, T: QGraphicsItem_focusProxy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusProxy_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_focusProxy_0<RetType> {
  fn focusProxy_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_focusProxy_0<usize> for () {
  fn focusProxy_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10focusProxyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:261
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocusProxy(QGraphicsItem *)

/*
Sets the item's focus proxy to item.

If an item has a focus proxy, the focus proxy will receive input focus when the item gains input focus. The item itself will still have focus (i.e., hasFocus() will return true), but only the focus proxy will receive the keyboard input.

A focus proxy can itself have a focus proxy, and so on. In such case, keyboard input will be handled by the outermost focus proxy.

The focus proxy item must belong to the same scene as this item.

This function was introduced in  Qt 4.6.

See also focusProxy(), setFocus(), and hasFocus().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setFocusProxy_0<RetType, T: QGraphicsItem_setFocusProxy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocusProxy_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setFocusProxy_0<RetType> {
  fn setFocusProxy_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setFocusProxy_0<(/*void*/)> for (usize) {
  fn setFocusProxy_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem13setFocusProxyEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:263
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * focusItem() const

/*
If this item, a child or descendant of this item currently has input focus, this function will return a pointer to that item. If no descendant has input focus, 0 is returned.

This function was introduced in  Qt 4.6.

See also hasFocus(), setFocus(), and QWidget::focusWidget().
*/
impl /*struct*/ QGraphicsItem {
  pub fn focusItem_0<RetType, T: QGraphicsItem_focusItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_focusItem_0<RetType> {
  fn focusItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_focusItem_0<usize> for () {
  fn focusItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9focusItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:264
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * focusScopeItem() const

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn focusScopeItem_0<RetType, T: QGraphicsItem_focusScopeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusScopeItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_focusScopeItem_0<RetType> {
  fn focusScopeItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_focusScopeItem_0<usize> for () {
  fn focusScopeItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem14focusScopeItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:266
// index:0
// Public Visibility=Default Availability=Available
// [-2] void grabMouse()

/*
Grabs the mouse input.

This item will receive all mouse events for the scene until any of the following events occurs:


The item becomes invisible
The item is removed from the scene
The item is deleted
The item call ungrabMouse()
Another item calls grabMouse(); the item will regain the mouse grab when the other item calls ungrabMouse().


When an item gains the mouse grab, it receives a QEvent::GrabMouse event. When it loses the mouse grab, it receives a QEvent::UngrabMouse event. These events can be used to detect when your item gains or loses the mouse grab through other means than receiving mouse button events.

It is almost never necessary to explicitly grab the mouse in Qt, as Qt grabs and releases it sensibly. In particular, Qt grabs the mouse when you press a mouse button, and keeps the mouse grabbed until you release the last mouse button. Also, Qt::Popup widgets implicitly call grabMouse() when shown, and ungrabMouse() when hidden.

Note that only visible items can grab mouse input. Calling grabMouse() on an invisible item has no effect.

Keyboard events are not affected.

This function was introduced in  Qt 4.4.

See also QGraphicsScene::mouseGrabberItem(), ungrabMouse(), and grabKeyboard().
*/
impl /*struct*/ QGraphicsItem {
  pub fn grabMouse_0<RetType, T: QGraphicsItem_grabMouse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabMouse_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_grabMouse_0<RetType> {
  fn grabMouse_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_grabMouse_0<(/*void*/)> for () {
  fn grabMouse_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem9grabMouseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:267
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ungrabMouse()

/*
Releases the mouse grab.

This function was introduced in  Qt 4.4.

See also grabMouse() and ungrabKeyboard().
*/
impl /*struct*/ QGraphicsItem {
  pub fn ungrabMouse_0<RetType, T: QGraphicsItem_ungrabMouse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ungrabMouse_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_ungrabMouse_0<RetType> {
  fn ungrabMouse_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_ungrabMouse_0<(/*void*/)> for () {
  fn ungrabMouse_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem11ungrabMouseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void grabKeyboard()

/*
Grabs the keyboard input.

The item will receive all keyboard input to the scene until one of the following events occur:


The item becomes invisible
The item is removed from the scene
The item is deleted
The item calls ungrabKeyboard()
Another item calls grabKeyboard(); the item will regain the keyboard grab when the other item calls ungrabKeyboard().


When an item gains the keyboard grab, it receives a QEvent::GrabKeyboard event. When it loses the keyboard grab, it receives a QEvent::UngrabKeyboard event. These events can be used to detect when your item gains or loses the keyboard grab through other means than gaining input focus.

It is almost never necessary to explicitly grab the keyboard in Qt, as Qt grabs and releases it sensibly. In particular, Qt grabs the keyboard when your item gains input focus, and releases it when your item loses input focus, or when the item is hidden.

Note that only visible items can grab keyboard input. Calling grabKeyboard() on an invisible item has no effect.

Keyboard events are not affected.

This function was introduced in  Qt 4.4.

See also ungrabKeyboard(), grabMouse(), and setFocus().
*/
impl /*struct*/ QGraphicsItem {
  pub fn grabKeyboard_0<RetType, T: QGraphicsItem_grabKeyboard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabKeyboard_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_grabKeyboard_0<RetType> {
  fn grabKeyboard_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_grabKeyboard_0<(/*void*/)> for () {
  fn grabKeyboard_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem12grabKeyboardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:269
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ungrabKeyboard()

/*
Releases the keyboard grab.

This function was introduced in  Qt 4.4.

See also grabKeyboard() and ungrabMouse().
*/
impl /*struct*/ QGraphicsItem {
  pub fn ungrabKeyboard_0<RetType, T: QGraphicsItem_ungrabKeyboard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ungrabKeyboard_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_ungrabKeyboard_0<RetType> {
  fn ungrabKeyboard_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_ungrabKeyboard_0<(/*void*/)> for () {
  fn ungrabKeyboard_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem14ungrabKeyboardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:272
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF pos() const

/*
Returns the position of the item in parent coordinates. If the item has no parent, its position is given in scene coordinates.

The position of the item describes its origin (local coordinate (0, 0)) in parent coordinates; this function returns the same as mapToParent(0, 0).

For convenience, you can also call scenePos() to determine the item's position in scene coordinates, regardless of its parent.

See also x(), y(), setPos(), transform(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn pos_0<RetType, T: QGraphicsItem_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_pos_0<RetType> {
  fn pos_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:273
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal x() const

/*
This convenience function is equivalent to calling pos().x().

See also setX() and y().
*/
impl /*struct*/ QGraphicsItem {
  pub fn x_0<RetType, T: QGraphicsItem_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_x_0<RetType> {
  fn x_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_x_0<f64> for () {
  fn x_0(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:274
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setX(qreal)

/*
Set's the x coordinate of the item's position. Equivalent to calling setPos(x, y()).

This function was introduced in  Qt 4.6.

See also x() and setPos().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setX_0<RetType, T: QGraphicsItem_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setX_0<RetType> {
  fn setX_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setX_0<(/*void*/)> for (f64) {
  fn setX_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem4setXEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:275
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal y() const

/*
This convenience function is equivalent to calling pos().y().

See also setY() and x().
*/
impl /*struct*/ QGraphicsItem {
  pub fn y_0<RetType, T: QGraphicsItem_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_y_0<RetType> {
  fn y_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_y_0<f64> for () {
  fn y_0(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:276
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setY(qreal)

/*
Set's the y coordinate of the item's position. Equivalent to calling setPos(x(), y).

This function was introduced in  Qt 4.6.

See also y(), x(), and setPos().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setY_0<RetType, T: QGraphicsItem_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setY_0<RetType> {
  fn setY_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setY_0<(/*void*/)> for (f64) {
  fn setY_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem4setYEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:277
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF scenePos() const

/*
Returns the item's position in scene coordinates. This is equivalent to calling mapToScene(0, 0).

See also pos(), sceneTransform(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn scenePos_0<RetType, T: QGraphicsItem_scenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_scenePos_0<RetType> {
  fn scenePos_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_scenePos_0<usize> for () {
  fn scenePos_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem8scenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:278
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPos(const QPointF &)

/*
Sets the position of the item to pos, which is in parent coordinates. For items with no parent, pos is in scene coordinates.

The position of the item describes its origin (local coordinate (0, 0)) in parent coordinates.

See also pos(), scenePos(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setPos_0<RetType, T: QGraphicsItem_setPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setPos_0<RetType> {
  fn setPos_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setPos_0<(/*void*/)> for (usize) {
  fn setPos_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem6setPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:279
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setPos(qreal, qreal)

/*
Sets the position of the item to pos, which is in parent coordinates. For items with no parent, pos is in scene coordinates.

The position of the item describes its origin (local coordinate (0, 0)) in parent coordinates.

See also pos(), scenePos(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setPos_1<RetType, T: QGraphicsItem_setPos_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPos_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setPos_1<RetType> {
  fn setPos_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setPos_1<(/*void*/)> for (f64,f64) {
  fn setPos_1(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem6setPosEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:280
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveBy(qreal, qreal)

/*
Moves the item by dx points horizontally, and dy point vertically. This function is equivalent to calling setPos(pos() + QPointF(dx, dy)).
*/
impl /*struct*/ QGraphicsItem {
  pub fn moveBy_0<RetType, T: QGraphicsItem_moveBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveBy_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_moveBy_0<RetType> {
  fn moveBy_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_moveBy_0<(/*void*/)> for (f64,f64) {
  fn moveBy_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem6moveByEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:282
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensureVisible(const QRectF &, int, int)

/*
If this item is part of a scene that is viewed by a QGraphicsView, this convenience function will attempt to scroll the view to ensure that rect is visible inside the view's viewport. If rect is a null rect (the default), QGraphicsItem will default to the item's bounding rect. xmargin and ymargin are the number of pixels the view should use for margins.

If the specified rect cannot be reached, the contents are scrolled to the nearest valid position.

If this item is not viewed by a QGraphicsView, this function does nothing.

See also QGraphicsView::ensureVisible().
*/
impl /*struct*/ QGraphicsItem {
  pub fn ensureVisible_0<RetType, T: QGraphicsItem_ensureVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_ensureVisible_0<RetType> {
  fn ensureVisible_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_ensureVisible_0<(/*void*/)> for (usize,i32,i32) {
  fn ensureVisible_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem13ensureVisibleERK6QRectFii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:283
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void ensureVisible(qreal, qreal, qreal, qreal, int, int)

/*
If this item is part of a scene that is viewed by a QGraphicsView, this convenience function will attempt to scroll the view to ensure that rect is visible inside the view's viewport. If rect is a null rect (the default), QGraphicsItem will default to the item's bounding rect. xmargin and ymargin are the number of pixels the view should use for margins.

If the specified rect cannot be reached, the contents are scrolled to the nearest valid position.

If this item is not viewed by a QGraphicsView, this function does nothing.

See also QGraphicsView::ensureVisible().
*/
impl /*struct*/ QGraphicsItem {
  pub fn ensureVisible_1<RetType, T: QGraphicsItem_ensureVisible_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_ensureVisible_1<RetType> {
  fn ensureVisible_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_ensureVisible_1<(/*void*/)> for (f64,f64,f64,f64,i32,i32) {
  fn ensureVisible_1(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem13ensureVisibleEddddii", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:286
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix matrix() const

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn matrix_0<RetType, T: QGraphicsItem_matrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matrix_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_matrix_0<RetType> {
  fn matrix_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_matrix_0<usize> for () {
  fn matrix_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem6matrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:287
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix sceneMatrix() const

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn sceneMatrix_0<RetType, T: QGraphicsItem_sceneMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneMatrix_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_sceneMatrix_0<RetType> {
  fn sceneMatrix_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_sceneMatrix_0<usize> for () {
  fn sceneMatrix_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11sceneMatrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:288
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMatrix(const QMatrix &, bool)

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn setMatrix_0<RetType, T: QGraphicsItem_setMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMatrix_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setMatrix_0<RetType> {
  fn setMatrix_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setMatrix_0<(/*void*/)> for (usize,bool) {
  fn setMatrix_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem9setMatrixERK7QMatrixb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:289
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetMatrix()

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn resetMatrix_0<RetType, T: QGraphicsItem_resetMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetMatrix_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_resetMatrix_0<RetType> {
  fn resetMatrix_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_resetMatrix_0<(/*void*/)> for () {
  fn resetMatrix_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem11resetMatrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:290
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform transform() const

/*
Returns this item's transformation matrix.

The transformation matrix is combined with the item's rotation(), scale() and transformations() into a combined transformations for the item.

The default transformation matrix is an identity matrix.

This function was introduced in  Qt 4.3.

See also setTransform() and sceneTransform().
*/
impl /*struct*/ QGraphicsItem {
  pub fn transform_0<RetType, T: QGraphicsItem_transform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transform_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_transform_0<RetType> {
  fn transform_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_transform_0<usize> for () {
  fn transform_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9transformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:291
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform sceneTransform() const

/*
Returns this item's scene transformation matrix. This matrix can be used to map coordinates and geometrical shapes from this item's local coordinate system to the scene's coordinate system. To map coordinates from the scene, you must first invert the returned matrix.

Example:


  QGraphicsRectItem rect;
  rect.setPos(100, 100);

  rect.sceneTransform().map(QPointF(0, 0));
  // returns QPointF(100, 100);

  rect.sceneTransform().inverted().map(QPointF(100, 100));
  // returns QPointF(0, 0);



Unlike transform(), which returns only an item's local transformation, this function includes the item's (and any parents') position, and all the transfomation properties.

This function was introduced in  Qt 4.3.

See also transform(), setTransform(), scenePos(), The Graphics View Coordinate System, and Transformations.
*/
impl /*struct*/ QGraphicsItem {
  pub fn sceneTransform_0<RetType, T: QGraphicsItem_sceneTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneTransform_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_sceneTransform_0<RetType> {
  fn sceneTransform_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_sceneTransform_0<usize> for () {
  fn sceneTransform_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem14sceneTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:292
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform deviceTransform(const QTransform &) const

/*
Returns this item's device transformation matrix, using viewportTransform to map from scene to device coordinates. This matrix can be used to map coordinates and geometrical shapes from this item's local coordinate system to the viewport's (or any device's) coordinate system. To map coordinates from the viewport, you must first invert the returned matrix.

Example:


  QGraphicsRectItem rect;
  rect.setPos(100, 100);

  rect.deviceTransform(view->viewportTransform()).map(QPointF(0, 0));
  // returns the item's (0, 0) point in view's viewport coordinates

  rect.deviceTransform(view->viewportTransform()).inverted().map(QPointF(100, 100));
  // returns view's viewport's (100, 100) coordinate in item coordinates



This function is the same as combining this item's scene transform with the view's viewport transform, but it also understands the ItemIgnoresTransformations flag. The device transform can be used to do accurate coordinate mapping (and collision detection) for untransformable items.

This function was introduced in  Qt 4.3.

See also transform(), setTransform(), scenePos(), The Graphics View Coordinate System, and itemTransform().
*/
impl /*struct*/ QGraphicsItem {
  pub fn deviceTransform_0<RetType, T: QGraphicsItem_deviceTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deviceTransform_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_deviceTransform_0<RetType> {
  fn deviceTransform_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_deviceTransform_0<usize> for (usize) {
  fn deviceTransform_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem15deviceTransformERK10QTransform", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:293
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform itemTransform(const QGraphicsItem *, bool *) const

/*
Returns a QTransform that maps coordinates from this item to other. If ok is not null, and if there is no such transform, the boolean pointed to by ok will be set to false; otherwise it will be set to true.

This transform provides an alternative to the mapToItem() or mapFromItem() functions, by returning the appropriate transform so that you can map shapes and coordinates yourself. It also helps you write more efficient code when repeatedly mapping between the same two items.

Note: In rare circumstances, there is no transform that maps between two items.

This function was introduced in  Qt 4.5.

See also mapToItem(), mapFromItem(), and deviceTransform().
*/
impl /*struct*/ QGraphicsItem {
  pub fn itemTransform_0<RetType, T: QGraphicsItem_itemTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemTransform_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_itemTransform_0<RetType> {
  fn itemTransform_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_itemTransform_0<usize> for (usize,usize) {
  fn itemTransform_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13itemTransformEPKS_Pb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:294
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransform(const QTransform &, bool)

/*
Sets the item's current transformation matrix to matrix.

If combine is true, then matrix is combined with the current matrix; otherwise, matrix replaces the current matrix. combine is false by default.

To simplify interaction with items using a transformed view, QGraphicsItem provides mapTo... and mapFrom... functions that can translate between items' and the scene's coordinates. For example, you can call mapToScene() to map an item coordiate to a scene coordinate, or mapFromScene() to map from scene coordinates to item coordinates.

The transformation matrix is combined with the item's rotation(), scale() and transformations() into a combined transformation that maps the item's coordinate system to its parent.

This function was introduced in  Qt 4.3.

See also transform(), setRotation(), setScale(), setTransformOriginPoint(), The Graphics View Coordinate System, and Transformations.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setTransform_0<RetType, T: QGraphicsItem_setTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransform_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setTransform_0<RetType> {
  fn setTransform_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setTransform_0<(/*void*/)> for (usize,bool) {
  fn setTransform_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem12setTransformERK10QTransformb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:295
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetTransform()

/*
Resets this item's transformation matrix to the identity matrix or all the transformation properties to their default values. This is equivalent to calling setTransform(QTransform()).

This function was introduced in  Qt 4.3.

See also setTransform() and transform().
*/
impl /*struct*/ QGraphicsItem {
  pub fn resetTransform_0<RetType, T: QGraphicsItem_resetTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetTransform_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_resetTransform_0<RetType> {
  fn resetTransform_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_resetTransform_0<(/*void*/)> for () {
  fn resetTransform_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem14resetTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:302
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRotation(qreal)

/*
Sets the clockwise rotation angle, in degrees, around the Z axis. The default value is 0 (i.e., the item is not rotated). Assigning a negative value will rotate the item counter-clockwise. Normally the rotation angle is in the range (-360, 360), but it's also possible to assign values outside of this range (e.g., a rotation of 370 degrees is the same as a rotation of 10 degrees).

The item is rotated around its transform origin point, which by default is (0, 0). You can select a different transformation origin by calling setTransformOriginPoint().

The rotation is combined with the item's scale(), transform() and transformations() to map the item's coordinate system to the parent item.

This function was introduced in  Qt 4.6.

See also rotation(), setTransformOriginPoint(), and Transformations.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setRotation_0<RetType, T: QGraphicsItem_setRotation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRotation_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setRotation_0<RetType> {
  fn setRotation_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setRotation_0<(/*void*/)> for (f64) {
  fn setRotation_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem11setRotationEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:303
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rotation() const

/*
Returns the clockwise rotation, in degrees, around the Z axis. The default value is 0 (i.e., the item is not rotated).

The rotation is combined with the item's scale(), transform() and transformations() to map the item's coordinate system to the parent item.

This function was introduced in  Qt 4.6.

See also setRotation(), transformOriginPoint(), and Transformations.
*/
impl /*struct*/ QGraphicsItem {
  pub fn rotation_0<RetType, T: QGraphicsItem_rotation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotation_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_rotation_0<RetType> {
  fn rotation_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_rotation_0<f64> for () {
  fn rotation_0(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem8rotationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:305
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScale(qreal)

/*
Sets the scale factor of the item. The default scale factor is 1.0 (i.e., the item is not scaled). A scale factor of 0.0 will collapse the item to a single point. If you provide a negative scale factor, the item will be flipped and mirrored (i.e., rotated 180 degrees).

The item is scaled around its transform origin point, which by default is (0, 0). You can select a different transformation origin by calling setTransformOriginPoint().

The scale is combined with the item's rotation(), transform() and transformations() to map the item's coordinate system to the parent item.

This function was introduced in  Qt 4.6.

See also scale(), setTransformOriginPoint(), and Transformations Example.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setScale_0<RetType, T: QGraphicsItem_setScale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScale_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setScale_0<RetType> {
  fn setScale_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setScale_0<(/*void*/)> for (f64) {
  fn setScale_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem8setScaleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:306
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal scale() const

/*
Returns the scale factor of the item. The default scale factor is 1.0 (i.e., the item is not scaled).

The scale is combined with the item's rotation(), transform() and transformations() to map the item's coordinate system to the parent item.

This function was introduced in  Qt 4.6.

See also setScale(), rotation(), and Transformations.
*/
impl /*struct*/ QGraphicsItem {
  pub fn scale_0<RetType, T: QGraphicsItem_scale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_scale_0<RetType> {
  fn scale_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_scale_0<f64> for () {
  fn scale_0(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem5scaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:311
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF transformOriginPoint() const

/*
Returns the origin point for the transformation in item coordinates.

The default is QPointF(0,0).

This function was introduced in  Qt 4.6.

See also setTransformOriginPoint() and Transformations.
*/
impl /*struct*/ QGraphicsItem {
  pub fn transformOriginPoint_0<RetType, T: QGraphicsItem_transformOriginPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformOriginPoint_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_transformOriginPoint_0<RetType> {
  fn transformOriginPoint_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_transformOriginPoint_0<usize> for () {
  fn transformOriginPoint_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem20transformOriginPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:312
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransformOriginPoint(const QPointF &)

/*
Sets the origin point for the transformation in item coordinates.

This function was introduced in  Qt 4.6.

See also transformOriginPoint() and Transformations.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setTransformOriginPoint_0<RetType, T: QGraphicsItem_setTransformOriginPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransformOriginPoint_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setTransformOriginPoint_0<RetType> {
  fn setTransformOriginPoint_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint_0<(/*void*/)> for (usize) {
  fn setTransformOriginPoint_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:313
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setTransformOriginPoint(qreal, qreal)

/*
Sets the origin point for the transformation in item coordinates.

This function was introduced in  Qt 4.6.

See also transformOriginPoint() and Transformations.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setTransformOriginPoint_1<RetType, T: QGraphicsItem_setTransformOriginPoint_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransformOriginPoint_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setTransformOriginPoint_1<RetType> {
  fn setTransformOriginPoint_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint_1<(/*void*/)> for (f64,f64) {
  fn setTransformOriginPoint_1(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem23setTransformOriginPointEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:316
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void advance(int)

/*
This virtual function is called twice for all items by the QGraphicsScene::advance() slot. In the first phase, all items are called with phase == 0, indicating that items on the scene are about to advance, and then all items are called with phase == 1. Reimplement this function to update your item if you need simple scene-controlled animation.

The default implementation does nothing.

This function is intended for animations. An alternative is to multiple-inherit from QObject and QGraphicsItem and use the Animation Framework.

See also QGraphicsScene::advance() and QTimeLine.
*/
impl /*struct*/ QGraphicsItem {
  pub fn advance_0<RetType, T: QGraphicsItem_advance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.advance_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_advance_0<RetType> {
  fn advance_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_advance_0<(/*void*/)> for (i32) {
  fn advance_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem7advanceEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:319
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal zValue() const

/*
Returns the Z-value of the item. The Z-value affects the stacking order of sibling (neighboring) items.

The default Z-value is 0.

See also setZValue(), Sorting, stackBefore(), and ItemStacksBehindParent.
*/
impl /*struct*/ QGraphicsItem {
  pub fn zValue_0<RetType, T: QGraphicsItem_zValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zValue_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_zValue_0<RetType> {
  fn zValue_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_zValue_0<f64> for () {
  fn zValue_0(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem6zValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:320
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setZValue(qreal)

/*
Sets the Z-value of the item to z. The Z value decides the stacking order of sibling (neighboring) items. A sibling item of high Z value will always be drawn on top of another sibling item with a lower Z value.

If you restore the Z value, the item's insertion order will decide its stacking order.

The Z-value does not affect the item's size in any way.

The default Z-value is 0.

See also zValue(), Sorting, stackBefore(), and ItemStacksBehindParent.
*/
impl /*struct*/ QGraphicsItem {
  pub fn setZValue_0<RetType, T: QGraphicsItem_setZValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setZValue_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setZValue_0<RetType> {
  fn setZValue_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setZValue_0<(/*void*/)> for (f64) {
  fn setZValue_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem9setZValueEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:321
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stackBefore(const QGraphicsItem *)

/*
Stacks this item before sibling, which must be a sibling item (i.e., the two items must share the same parent item, or must both be toplevel items). The sibling must have the same Z value as this item, otherwise calling this function will have no effect.

By default, all sibling items are stacked by insertion order (i.e., the first item you add is drawn before the next item you add). If two items' Z values are different, then the item with the highest Z value is drawn on top. When the Z values are the same, the insertion order will decide the stacking order.

This function was introduced in  Qt 4.6.

See also setZValue(), ItemStacksBehindParent, and Sorting.
*/
impl /*struct*/ QGraphicsItem {
  pub fn stackBefore_0<RetType, T: QGraphicsItem_stackBefore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stackBefore_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_stackBefore_0<RetType> {
  fn stackBefore_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_stackBefore_0<(/*void*/)> for (usize) {
  fn stackBefore_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem11stackBeforeEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:324
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [32] QRectF boundingRect() const

/*
This pure virtual function defines the outer bounds of the item as a rectangle; all painting must be restricted to inside an item's bounding rect. QGraphicsView uses this to determine whether the item requires redrawing.

Although the item's shape can be arbitrary, the bounding rect is always rectangular, and it is unaffected by the items' transformation.

If you want to change the item's bounding rectangle, you must first call prepareGeometryChange(). This notifies the scene of the imminent change, so that it can update its item geometry index; otherwise, the scene will be unaware of the item's new geometry, and the results are undefined (typically, rendering artifacts are left within the view).

Reimplement this function to let QGraphicsView determine what parts of the widget, if any, need to be redrawn.

Note: For shapes that paint an outline / stroke, it is important to include half the pen width in the bounding rect. It is not necessary to compensate for antialiasing, though.

Example:


  QRectF CircleItem::boundingRect() const
  {
      qreal penWidth = 1;
      return QRectF(-radius - penWidth / 2, -radius - penWidth / 2,
                    diameter + penWidth, diameter + penWidth);
  }



See also boundingRegion(), shape(), contains(), The Graphics View Coordinate System, and prepareGeometryChange().
*/
impl /*struct*/ QGraphicsItem {
  pub fn boundingRect_0<RetType, T: QGraphicsItem_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:325
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF childrenBoundingRect() const

/*
Returns the bounding rect of this item's descendants (i.e., its children, their children, etc.) in local coordinates. The rectangle will contain all descendants after they have been mapped to local coordinates. If the item has no children, this function returns an empty QRectF.

This does not include this item's own bounding rect; it only returns its descendants' accumulated bounding rect. If you need to include this item's bounding rect, you can add boundingRect() to childrenBoundingRect() using QRectF::operator|().

This function is linear in complexity; it determines the size of the returned bounding rect by iterating through all descendants.

See also boundingRect() and sceneBoundingRect().
*/
impl /*struct*/ QGraphicsItem {
  pub fn childrenBoundingRect_0<RetType, T: QGraphicsItem_childrenBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childrenBoundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_childrenBoundingRect_0<RetType> {
  fn childrenBoundingRect_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_childrenBoundingRect_0<usize> for () {
  fn childrenBoundingRect_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem20childrenBoundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:326
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF sceneBoundingRect() const

/*
Returns the bounding rect of this item in scene coordinates, by combining sceneTransform() with boundingRect().

See also boundingRect() and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn sceneBoundingRect_0<RetType, T: QGraphicsItem_sceneBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneBoundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_sceneBoundingRect_0<RetType> {
  fn sceneBoundingRect_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_sceneBoundingRect_0<usize> for () {
  fn sceneBoundingRect_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem17sceneBoundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:327
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath shape() const

/*
Returns the shape of this item as a QPainterPath in local coordinates. The shape is used for many things, including collision detection, hit tests, and for the QGraphicsScene::items() functions.

The default implementation calls boundingRect() to return a simple rectangular shape, but subclasses can reimplement this function to return a more accurate shape for non-rectangular items. For example, a round item may choose to return an elliptic shape for better collision detection. For example:


  QPainterPath RoundItem::shape() const
  {
      QPainterPath path;
      path.addEllipse(boundingRect());
      return path;
  }



The outline of a shape can vary depending on the width and style of the pen used when drawing. If you want to include this outline in the item's shape, you can create a shape from the stroke using QPainterPathStroker.

This function is called by the default implementations of contains() and collidesWithPath().

See also boundingRect(), contains(), prepareGeometryChange(), and QPainterPathStroker.
*/
impl /*struct*/ QGraphicsItem {
  pub fn shape_0<RetType, T: QGraphicsItem_shape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shape_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_shape_0<RetType> {
  fn shape_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_shape_0<usize> for () {
  fn shape_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem5shapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:328
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isClipped() const

/*
Returns true if this item is clipped. An item is clipped if it has either set the ItemClipsToShape flag, or if it or any of its ancestors has set the ItemClipsChildrenToShape flag.

Clipping affects the item's appearance (i.e., painting), as well as mouse and hover event delivery.

See also clipPath(), shape(), and setFlags().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isClipped_0<RetType, T: QGraphicsItem_isClipped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isClipped_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isClipped_0<RetType> {
  fn isClipped_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isClipped_0<bool> for () {
  fn isClipped_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9isClippedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:329
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath clipPath() const

/*
Returns this item's clip path, or an empty QPainterPath if this item is not clipped. The clip path constrains the item's appearance and interaction (i.e., restricts the area the item can draw within and receive events for).

You can enable clipping by setting the ItemClipsToShape or ItemClipsChildrenToShape flags. The item's clip path is calculated by intersecting all clipping ancestors' shapes. If the item sets ItemClipsToShape, the final clip is intersected with the item's own shape.

Note: Clipping introduces a performance penalty for all items involved; you should generally avoid using clipping if you can (e.g., if your items always draw inside boundingRect() or shape() boundaries, clipping is not necessary).

This function was introduced in  Qt 4.5.

See also isClipped(), shape(), and setFlags().
*/
impl /*struct*/ QGraphicsItem {
  pub fn clipPath_0<RetType, T: QGraphicsItem_clipPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clipPath_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_clipPath_0<RetType> {
  fn clipPath_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_clipPath_0<usize> for () {
  fn clipPath_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem8clipPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:330
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool contains(const QPointF &) const

/*
Returns true if this item contains point, which is in local coordinates; otherwise, false is returned. It is most often called from QGraphicsView to determine what item is under the cursor, and for that reason, the implementation of this function should be as light-weight as possible.

By default, this function calls shape(), but you can reimplement it in a subclass to provide a (perhaps more efficient) implementation.

See also shape(), boundingRect(), and collidesWithPath().
*/
impl /*struct*/ QGraphicsItem {
  pub fn contains_0<RetType, T: QGraphicsItem_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_contains_0<RetType> {
  fn contains_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem8containsERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:331
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool collidesWithItem(const QGraphicsItem *, Qt::ItemSelectionMode) const

/*
Returns true if this item collides with other; otherwise returns false.

The mode is applied to other, and the resulting shape or bounding rectangle is then compared to this item's shape. The default value for mode is Qt::IntersectsItemShape; other collides with this item if it either intersects, contains, or is contained by this item's shape (see Qt::ItemSelectionMode for details).

The default implementation is based on shape intersection, and it calls shape() on both items. Because the complexity of arbitrary shape-shape intersection grows with an order of magnitude when the shapes are complex, this operation can be noticably time consuming. You have the option of reimplementing this function in a subclass of QGraphicsItem to provide a custom algorithm. This allows you to make use of natural constraints in the shapes of your own items, in order to improve the performance of the collision detection. For instance, two untransformed perfectly circular items' collision can be determined very efficiently by comparing their positions and radii.

Keep in mind that when reimplementing this function and calling shape() or boundingRect() on other, the returned coordinates must be mapped to this item's coordinate system before any intersection can take place.

See also contains() and shape().
*/
impl /*struct*/ QGraphicsItem {
  pub fn collidesWithItem_0<RetType, T: QGraphicsItem_collidesWithItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.collidesWithItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_collidesWithItem_0<RetType> {
  fn collidesWithItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_collidesWithItem_0<bool> for (usize,i32) {
  fn collidesWithItem_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem16collidesWithItemEPKS_N2Qt17ItemSelectionModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:332
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool collidesWithPath(const QPainterPath &, Qt::ItemSelectionMode) const

/*
Returns true if this item collides with path.

The collision is determined by mode. The default value for mode is Qt::IntersectsItemShape; path collides with this item if it either intersects, contains, or is contained by this item's shape.

Note that this function checks whether the item's shape or bounding rectangle (depending on mode) is contained within path, and not whether path is contained within the items shape or bounding rectangle.

See also collidesWithItem(), contains(), and shape().
*/
impl /*struct*/ QGraphicsItem {
  pub fn collidesWithPath_0<RetType, T: QGraphicsItem_collidesWithPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.collidesWithPath_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_collidesWithPath_0<RetType> {
  fn collidesWithPath_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_collidesWithPath_0<bool> for (usize,i32) {
  fn collidesWithPath_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem16collidesWithPathERK12QPainterPathN2Qt17ItemSelectionModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:333
// index:0
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> collidingItems(Qt::ItemSelectionMode) const

/*
Returns a list of all items that collide with this item.

The way collisions are detected is determined by applying mode to items that are compared to this item, i.e., each item's shape or bounding rectangle is checked against this item's shape. The default value for mode is Qt::IntersectsItemShape.

See also collidesWithItem().
*/
impl /*struct*/ QGraphicsItem {
  pub fn collidingItems_0<RetType, T: QGraphicsItem_collidingItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.collidingItems_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_collidingItems_0<RetType> {
  fn collidingItems_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_collidingItems_0<usize> for (i32) {
  fn collidingItems_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem14collidingItemsEN2Qt17ItemSelectionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:334
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isObscured(const QRectF &) const

/*
This is an overloaded function.

Returns true if rect is completely obscured by the opaque shape of any of colliding items above it (i.e., with a higher Z value than this item).

This function was introduced in  Qt 4.3.

See also opaqueArea().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isObscured_0<RetType, T: QGraphicsItem_isObscured_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObscured_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isObscured_0<RetType> {
  fn isObscured_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isObscured_0<bool> for (usize) {
  fn isObscured_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10isObscuredERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:335
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool isObscured(qreal, qreal, qreal, qreal) const

/*
This is an overloaded function.

Returns true if rect is completely obscured by the opaque shape of any of colliding items above it (i.e., with a higher Z value than this item).

This function was introduced in  Qt 4.3.

See also opaqueArea().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isObscured_1<RetType, T: QGraphicsItem_isObscured_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObscured_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isObscured_1<RetType> {
  fn isObscured_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isObscured_1<bool> for (f64,f64,f64,f64) {
  fn isObscured_1(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10isObscuredEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:336
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isObscuredBy(const QGraphicsItem *) const

/*
Returns true if this item's bounding rect is completely obscured by the opaque shape of item.

The base implementation maps item's opaqueArea() to this item's coordinate system, and then checks if this item's boundingRect() is fully contained within the mapped shape.

You can reimplement this function to provide a custom algorithm for determining whether this item is obscured by item.

See also opaqueArea() and isObscured().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isObscuredBy_0<RetType, T: QGraphicsItem_isObscuredBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isObscuredBy_0<RetType> {
  fn isObscuredBy_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isObscuredBy_0<bool> for (usize) {
  fn isObscuredBy_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12isObscuredByEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:337
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath opaqueArea() const

/*
This virtual function returns a shape representing the area where this item is opaque. An area is opaque if it is filled using an opaque brush or color (i.e., not transparent).

This function is used by isObscuredBy(), which is called by underlying items to determine if they are obscured by this item.

The default implementation returns an empty QPainterPath, indicating that this item is completely transparent and does not obscure any other items.

See also isObscuredBy(), isObscured(), and shape().
*/
impl /*struct*/ QGraphicsItem {
  pub fn opaqueArea_0<RetType, T: QGraphicsItem_opaqueArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_opaqueArea_0<RetType> {
  fn opaqueArea_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_opaqueArea_0<usize> for () {
  fn opaqueArea_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10opaqueAreaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:339
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion boundingRegion(const QTransform &) const

/*
Returns the bounding region for this item. The coordinate space of the returned region depends on itemToDeviceTransform. If you pass an identity QTransform as a parameter, this function will return a local coordinate region.

The bounding region describes a coarse outline of the item's visual contents. Although it's expensive to calculate, it's also more precise than boundingRect(), and it can help to avoid unnecessary repainting when an item is updated. This is particularly efficient for thin items (e.g., lines or simple polygons). You can tune the granularity for the bounding region by calling setBoundingRegionGranularity(). The default granularity is 0; in which the item's bounding region is the same as its bounding rect.

itemToDeviceTransform is the transformation from item coordinates to device coordinates. If you want this function to return a QRegion in scene coordinates, you can pass sceneTransform() as an argument.

This function was introduced in  Qt 4.4.

See also boundingRegionGranularity().
*/
impl /*struct*/ QGraphicsItem {
  pub fn boundingRegion_0<RetType, T: QGraphicsItem_boundingRegion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRegion_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_boundingRegion_0<RetType> {
  fn boundingRegion_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_boundingRegion_0<usize> for (usize) {
  fn boundingRegion_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem14boundingRegionERK10QTransform", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:340
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal boundingRegionGranularity() const

/*
Returns the item's bounding region granularity; a value between and including 0 and 1. The default value is 0 (i.e., the lowest granularity, where the bounding region corresponds to the item's bounding rectangle).

This function was introduced in  Qt 4.4.

See also setBoundingRegionGranularity().
*/
impl /*struct*/ QGraphicsItem {
  pub fn boundingRegionGranularity_0<RetType, T: QGraphicsItem_boundingRegionGranularity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRegionGranularity_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_boundingRegionGranularity_0<RetType> {
  fn boundingRegionGranularity_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_boundingRegionGranularity_0<f64> for () {
  fn boundingRegionGranularity_0(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem25boundingRegionGranularityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:341
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBoundingRegionGranularity(qreal)

/*
Sets the bounding region granularity to granularity; a value between and including 0 and 1. The default value is 0 (i.e., the lowest granularity, where the bounding region corresponds to the item's bounding rectangle).

The granularity is used by boundingRegion() to calculate how fine the bounding region of the item should be. The highest achievable granularity is 1, where boundingRegion() will return the finest outline possible for the respective device (e.g., for a QGraphicsView viewport, this gives you a pixel-perfect bounding region). The lowest possible granularity is 0. The value of granularity describes the ratio between device resolution and the resolution of the bounding region (e.g., a value of 0.25 will provide a region where each chunk corresponds to 4x4 device units / pixels).

This function was introduced in  Qt 4.4.

See also boundingRegionGranularity().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setBoundingRegionGranularity_0<RetType, T: QGraphicsItem_setBoundingRegionGranularity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBoundingRegionGranularity_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setBoundingRegionGranularity_0<RetType> {
  fn setBoundingRegionGranularity_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setBoundingRegionGranularity_0<(/*void*/)> for (f64) {
  fn setBoundingRegionGranularity_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem28setBoundingRegionGranularityEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:344
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QStyleOptionGraphicsItem *, QWidget *)

/*
This function, which is usually called by QGraphicsView, paints the contents of an item in local coordinates.

Reimplement this function in a QGraphicsItem subclass to provide the item's painting implementation, using painter. The option parameter provides style options for the item, such as its state, exposed area and its level-of-detail hints. The widget argument is optional. If provided, it points to the widget that is being painted on; otherwise, it is 0. For cached painting, widget is always 0.


  void RoundRectItem::paint(QPainter *painter,
                            const QStyleOptionGraphicsItem *option,
                            QWidget *widget)
  {
      painter->drawRoundedRect(-10, -10, 20, 20, 5, 5);
  }



The painter's pen is 0-width by default, and its pen is initialized to the QPalette::Text brush from the paint device's palette. The brush is initialized to QPalette::Window.

Make sure to constrain all painting inside the boundaries of boundingRect() to avoid rendering artifacts (as QGraphicsView does not clip the painter for you). In particular, when QPainter renders the outline of a shape using an assigned QPen, half of the outline will be drawn outside, and half inside, the shape you're rendering (e.g., with a pen width of 2 units, you must draw outlines 1 unit inside boundingRect()). QGraphicsItem does not support use of cosmetic pens with a non-zero width.

All painting is done in local coordinates.

Note: It is mandatory that an item will always redraw itself in the exact same way, unless update() was called; otherwise visual artifacts may occur. In other words, two subsequent calls to paint() must always produce the same output, unless update() was called between them.

Note: Enabling caching for an item does not guarantee that paint() will be invoked only once by the Graphics View framework, even without any explicit call to update(). See the documentation of setCacheMode() for more details.

See also setCacheMode(), QPen::width(), Item Coordinates, and ItemUsesExtendedStyleOption.
*/
impl /*struct*/ QGraphicsItem {
  pub fn paint_0<RetType, T: QGraphicsItem_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_paint_0<RetType> {
  fn paint_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:345
// index:0
// Public Visibility=Default Availability=Available
// [-2] void update(const QRectF &)

/*
Schedules a redraw of the area covered by rect in this item. You can call this function whenever your item needs to be redrawn, such as if it changes appearance or size.

This function does not cause an immediate paint; instead it schedules a paint request that is processed by QGraphicsView after control reaches the event loop. The item will only be redrawn if it is visible in any associated view.

As a side effect of the item being repainted, other items that overlap the area rect may also be repainted.

If the item is invisible (i.e., isVisible() returns false), this function does nothing.

See also paint() and boundingRect().
*/
impl /*struct*/ QGraphicsItem {
  pub fn update_0<RetType, T: QGraphicsItem_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_update_0<RetType> {
  fn update_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_update_0<(/*void*/)> for (usize) {
  fn update_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem6updateERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:346
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void update(qreal, qreal, qreal, qreal)

/*
Schedules a redraw of the area covered by rect in this item. You can call this function whenever your item needs to be redrawn, such as if it changes appearance or size.

This function does not cause an immediate paint; instead it schedules a paint request that is processed by QGraphicsView after control reaches the event loop. The item will only be redrawn if it is visible in any associated view.

As a side effect of the item being repainted, other items that overlap the area rect may also be repainted.

If the item is invisible (i.e., isVisible() returns false), this function does nothing.

See also paint() and boundingRect().
*/
impl /*struct*/ QGraphicsItem {
  pub fn update_1<RetType, T: QGraphicsItem_update_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_update_1<RetType> {
  fn update_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_update_1<(/*void*/)> for (f64,f64,f64,f64) {
  fn update_1(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem6updateEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:347
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scroll(qreal, qreal, const QRectF &)

/*
Scrolls the contents of rect by dx, dy. If rect is a null rect (the default), the item's bounding rect is scrolled.

Scrolling provides a fast alternative to simply redrawing when the contents of the item (or parts of the item) are shifted vertically or horizontally. Depending on the current transformation and the capabilities of the paint device (i.e., the viewport), this operation may consist of simply moving pixels from one location to another using memmove(). In most cases this is faster than rerendering the entire area.

After scrolling, the item will issue an update for the newly exposed areas. If scrolling is not supported (e.g., you are rendering to an OpenGL viewport, which does not benefit from scroll optimizations), this function is equivalent to calling update(rect).

Note: Scrolling is only supported when QGraphicsItem::ItemCoordinateCache is enabled; in all other cases calling this function is equivalent to calling update(rect). If you for sure know that the item is opaque and not overlapped by other items, you can map the rect to viewport coordinates and scroll the viewport.


  QTransform xform = item->deviceTransform(view->viewportTransform());
  QRect deviceRect = xform.mapRect(rect).toAlignedRect();
  view->viewport()->scroll(dx, dy, deviceRect);



This function was introduced in  Qt 4.4.

See also boundingRect().
*/
impl /*struct*/ QGraphicsItem {
  pub fn scroll_0<RetType, T: QGraphicsItem_scroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scroll_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_scroll_0<RetType> {
  fn scroll_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_scroll_0<(/*void*/)> for (f64,f64,usize) {
  fn scroll_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem6scrollEddRK6QRectF", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:350
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF mapToItem(const QGraphicsItem *, const QPointF &) const

/*
Maps the point point, which is in this item's coordinate system, to item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapToScene().

See also itemTransform(), mapToParent(), mapToScene(), transform(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToItem_0<RetType, T: QGraphicsItem_mapToItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToItem_0<RetType> {
  fn mapToItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToItem_0<usize> for (usize,usize) {
  fn mapToItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:353
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapToItem(const QGraphicsItem *, const QRectF &) const

/*
Maps the point point, which is in this item's coordinate system, to item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapToScene().

See also itemTransform(), mapToParent(), mapToScene(), transform(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToItem_1<RetType, T: QGraphicsItem_mapToItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToItem_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToItem_1<RetType> {
  fn mapToItem_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToItem_1<usize> for (usize,usize) {
  fn mapToItem_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:359
// index:2
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapToItem(const QGraphicsItem *, const QPolygonF &) const

/*
Maps the point point, which is in this item's coordinate system, to item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapToScene().

See also itemTransform(), mapToParent(), mapToScene(), transform(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToItem_2<RetType, T: QGraphicsItem_mapToItem_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToItem_2(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToItem_2<RetType> {
  fn mapToItem_2(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToItem_2<usize> for (usize,usize) {
  fn mapToItem_2(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:362
// index:3
// Public Visibility=Default Availability=Available
// [8] QPainterPath mapToItem(const QGraphicsItem *, const QPainterPath &) const

/*
Maps the point point, which is in this item's coordinate system, to item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapToScene().

See also itemTransform(), mapToParent(), mapToScene(), transform(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToItem_3<RetType, T: QGraphicsItem_mapToItem_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToItem_3(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToItem_3<RetType> {
  fn mapToItem_3(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToItem_3<usize> for (usize,usize) {
  fn mapToItem_3(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:381
// index:4
// Public inline Visibility=Default Availability=Available
// [16] QPointF mapToItem(const QGraphicsItem *, qreal, qreal) const

/*
Maps the point point, which is in this item's coordinate system, to item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapToScene().

See also itemTransform(), mapToParent(), mapToScene(), transform(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToItem_4<RetType, T: QGraphicsItem_mapToItem_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToItem_4(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToItem_4<RetType> {
  fn mapToItem_4(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToItem_4<usize> for (usize,f64,f64) {
  fn mapToItem_4(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9mapToItemEPKS_dd", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:384
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF mapToItem(const QGraphicsItem *, qreal, qreal, qreal, qreal) const

/*
Maps the point point, which is in this item's coordinate system, to item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapToScene().

See also itemTransform(), mapToParent(), mapToScene(), transform(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToItem_5<RetType, T: QGraphicsItem_mapToItem_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToItem_5(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToItem_5<RetType> {
  fn mapToItem_5(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToItem_5<usize> for (usize,f64,f64,f64,f64) {
  fn mapToItem_5(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9mapToItemEPKS_dddd", 5,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:351
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF mapToParent(const QPointF &) const

/*
Maps the point point, which is in this item's coordinate system, to its parent's coordinate system, and returns the mapped coordinate. If the item has no parent, point will be mapped to the scene's coordinate system.

See also mapToItem(), mapToScene(), transform(), mapFromParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToParent_0<RetType, T: QGraphicsItem_mapToParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToParent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToParent_0<RetType> {
  fn mapToParent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToParent_0<usize> for (usize) {
  fn mapToParent_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapToParentERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:354
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapToParent(const QRectF &) const

/*
Maps the point point, which is in this item's coordinate system, to its parent's coordinate system, and returns the mapped coordinate. If the item has no parent, point will be mapped to the scene's coordinate system.

See also mapToItem(), mapToScene(), transform(), mapFromParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToParent_1<RetType, T: QGraphicsItem_mapToParent_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToParent_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToParent_1<RetType> {
  fn mapToParent_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToParent_1<usize> for (usize) {
  fn mapToParent_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapToParentERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:360
// index:2
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapToParent(const QPolygonF &) const

/*
Maps the point point, which is in this item's coordinate system, to its parent's coordinate system, and returns the mapped coordinate. If the item has no parent, point will be mapped to the scene's coordinate system.

See also mapToItem(), mapToScene(), transform(), mapFromParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToParent_2<RetType, T: QGraphicsItem_mapToParent_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToParent_2(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToParent_2<RetType> {
  fn mapToParent_2(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToParent_2<usize> for (usize) {
  fn mapToParent_2(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapToParentERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:363
// index:3
// Public Visibility=Default Availability=Available
// [8] QPainterPath mapToParent(const QPainterPath &) const

/*
Maps the point point, which is in this item's coordinate system, to its parent's coordinate system, and returns the mapped coordinate. If the item has no parent, point will be mapped to the scene's coordinate system.

See also mapToItem(), mapToScene(), transform(), mapFromParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToParent_3<RetType, T: QGraphicsItem_mapToParent_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToParent_3(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToParent_3<RetType> {
  fn mapToParent_3(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToParent_3<usize> for (usize) {
  fn mapToParent_3(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapToParentERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:382
// index:4
// Public inline Visibility=Default Availability=Available
// [16] QPointF mapToParent(qreal, qreal) const

/*
Maps the point point, which is in this item's coordinate system, to its parent's coordinate system, and returns the mapped coordinate. If the item has no parent, point will be mapped to the scene's coordinate system.

See also mapToItem(), mapToScene(), transform(), mapFromParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToParent_4<RetType, T: QGraphicsItem_mapToParent_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToParent_4(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToParent_4<RetType> {
  fn mapToParent_4(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToParent_4<usize> for (f64,f64) {
  fn mapToParent_4(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapToParentEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:385
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF mapToParent(qreal, qreal, qreal, qreal) const

/*
Maps the point point, which is in this item's coordinate system, to its parent's coordinate system, and returns the mapped coordinate. If the item has no parent, point will be mapped to the scene's coordinate system.

See also mapToItem(), mapToScene(), transform(), mapFromParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToParent_5<RetType, T: QGraphicsItem_mapToParent_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToParent_5(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToParent_5<RetType> {
  fn mapToParent_5(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToParent_5<usize> for (f64,f64,f64,f64) {
  fn mapToParent_5(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapToParentEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:352
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF mapToScene(const QPointF &) const

/*
Maps the point point, which is in this item's coordinate system, to the scene's coordinate system, and returns the mapped coordinate.

See also mapToItem(), mapToParent(), transform(), mapFromScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToScene_0<RetType, T: QGraphicsItem_mapToScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToScene_0<RetType> {
  fn mapToScene_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToScene_0<usize> for (usize) {
  fn mapToScene_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10mapToSceneERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:355
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapToScene(const QRectF &) const

/*
Maps the point point, which is in this item's coordinate system, to the scene's coordinate system, and returns the mapped coordinate.

See also mapToItem(), mapToParent(), transform(), mapFromScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToScene_1<RetType, T: QGraphicsItem_mapToScene_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToScene_1<RetType> {
  fn mapToScene_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToScene_1<usize> for (usize) {
  fn mapToScene_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10mapToSceneERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:361
// index:2
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapToScene(const QPolygonF &) const

/*
Maps the point point, which is in this item's coordinate system, to the scene's coordinate system, and returns the mapped coordinate.

See also mapToItem(), mapToParent(), transform(), mapFromScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToScene_2<RetType, T: QGraphicsItem_mapToScene_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_2(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToScene_2<RetType> {
  fn mapToScene_2(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToScene_2<usize> for (usize) {
  fn mapToScene_2(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10mapToSceneERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:364
// index:3
// Public Visibility=Default Availability=Available
// [8] QPainterPath mapToScene(const QPainterPath &) const

/*
Maps the point point, which is in this item's coordinate system, to the scene's coordinate system, and returns the mapped coordinate.

See also mapToItem(), mapToParent(), transform(), mapFromScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToScene_3<RetType, T: QGraphicsItem_mapToScene_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_3(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToScene_3<RetType> {
  fn mapToScene_3(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToScene_3<usize> for (usize) {
  fn mapToScene_3(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10mapToSceneERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:383
// index:4
// Public inline Visibility=Default Availability=Available
// [16] QPointF mapToScene(qreal, qreal) const

/*
Maps the point point, which is in this item's coordinate system, to the scene's coordinate system, and returns the mapped coordinate.

See also mapToItem(), mapToParent(), transform(), mapFromScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToScene_4<RetType, T: QGraphicsItem_mapToScene_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_4(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToScene_4<RetType> {
  fn mapToScene_4(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToScene_4<usize> for (f64,f64) {
  fn mapToScene_4(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10mapToSceneEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:386
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF mapToScene(qreal, qreal, qreal, qreal) const

/*
Maps the point point, which is in this item's coordinate system, to the scene's coordinate system, and returns the mapped coordinate.

See also mapToItem(), mapToParent(), transform(), mapFromScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapToScene_5<RetType, T: QGraphicsItem_mapToScene_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_5(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapToScene_5<RetType> {
  fn mapToScene_5(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapToScene_5<usize> for (f64,f64,f64,f64) {
  fn mapToScene_5(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem10mapToSceneEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:356
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF mapRectToItem(const QGraphicsItem *, const QRectF &) const

/*
Maps the rectangle rect, which is in this item's coordinate system, to item's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

If item is 0, this function returns the same as mapRectToScene().

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToItem_0<RetType, T: QGraphicsItem_mapRectToItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectToItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectToItem_0<RetType> {
  fn mapRectToItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem_0<usize> for (usize,usize) {
  fn mapRectToItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:387
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QRectF mapRectToItem(const QGraphicsItem *, qreal, qreal, qreal, qreal) const

/*
Maps the rectangle rect, which is in this item's coordinate system, to item's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

If item is 0, this function returns the same as mapRectToScene().

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToItem_1<RetType, T: QGraphicsItem_mapRectToItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectToItem_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectToItem_1<RetType> {
  fn mapRectToItem_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem_1<usize> for (usize,f64,f64,f64,f64) {
  fn mapRectToItem_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd", 5,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:357
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF mapRectToParent(const QRectF &) const

/*
Maps the rectangle rect, which is in this item's coordinate system, to its parent's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToParent_0<RetType, T: QGraphicsItem_mapRectToParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectToParent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectToParent_0<RetType> {
  fn mapRectToParent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent_0<usize> for (usize) {
  fn mapRectToParent_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem15mapRectToParentERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:388
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QRectF mapRectToParent(qreal, qreal, qreal, qreal) const

/*
Maps the rectangle rect, which is in this item's coordinate system, to its parent's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToParent_1<RetType, T: QGraphicsItem_mapRectToParent_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectToParent_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectToParent_1<RetType> {
  fn mapRectToParent_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent_1<usize> for (f64,f64,f64,f64) {
  fn mapRectToParent_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem15mapRectToParentEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:358
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF mapRectToScene(const QRectF &) const

/*
Maps the rectangle rect, which is in this item's coordinate system, to the scene coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToScene_0<RetType, T: QGraphicsItem_mapRectToScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectToScene_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectToScene_0<RetType> {
  fn mapRectToScene_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene_0<usize> for (usize) {
  fn mapRectToScene_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:389
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QRectF mapRectToScene(qreal, qreal, qreal, qreal) const

/*
Maps the rectangle rect, which is in this item's coordinate system, to the scene coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToScene_1<RetType, T: QGraphicsItem_mapRectToScene_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectToScene_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectToScene_1<RetType> {
  fn mapRectToScene_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene_1<usize> for (f64,f64,f64,f64) {
  fn mapRectToScene_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem14mapRectToSceneEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:365
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF mapFromItem(const QGraphicsItem *, const QPointF &) const

/*
Maps the point point, which is in item's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapFromScene().

See also itemTransform(), mapFromParent(), mapFromScene(), transform(), mapToItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem_0<RetType, T: QGraphicsItem_mapFromItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromItem_0<RetType> {
  fn mapFromItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromItem_0<usize> for (usize,usize) {
  fn mapFromItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:368
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapFromItem(const QGraphicsItem *, const QRectF &) const

/*
Maps the point point, which is in item's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapFromScene().

See also itemTransform(), mapFromParent(), mapFromScene(), transform(), mapToItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem_1<RetType, T: QGraphicsItem_mapFromItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromItem_1<RetType> {
  fn mapFromItem_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromItem_1<usize> for (usize,usize) {
  fn mapFromItem_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:374
// index:2
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapFromItem(const QGraphicsItem *, const QPolygonF &) const

/*
Maps the point point, which is in item's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapFromScene().

See also itemTransform(), mapFromParent(), mapFromScene(), transform(), mapToItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem_2<RetType, T: QGraphicsItem_mapFromItem_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem_2(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromItem_2<RetType> {
  fn mapFromItem_2(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromItem_2<usize> for (usize,usize) {
  fn mapFromItem_2(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:377
// index:3
// Public Visibility=Default Availability=Available
// [8] QPainterPath mapFromItem(const QGraphicsItem *, const QPainterPath &) const

/*
Maps the point point, which is in item's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapFromScene().

See also itemTransform(), mapFromParent(), mapFromScene(), transform(), mapToItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem_3<RetType, T: QGraphicsItem_mapFromItem_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem_3(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromItem_3<RetType> {
  fn mapFromItem_3(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromItem_3<usize> for (usize,usize) {
  fn mapFromItem_3(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:390
// index:4
// Public inline Visibility=Default Availability=Available
// [16] QPointF mapFromItem(const QGraphicsItem *, qreal, qreal) const

/*
Maps the point point, which is in item's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapFromScene().

See also itemTransform(), mapFromParent(), mapFromScene(), transform(), mapToItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem_4<RetType, T: QGraphicsItem_mapFromItem_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem_4(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromItem_4<RetType> {
  fn mapFromItem_4(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromItem_4<usize> for (usize,f64,f64) {
  fn mapFromItem_4(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapFromItemEPKS_dd", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:393
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF mapFromItem(const QGraphicsItem *, qreal, qreal, qreal, qreal) const

/*
Maps the point point, which is in item's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

If item is 0, this function returns the same as mapFromScene().

See also itemTransform(), mapFromParent(), mapFromScene(), transform(), mapToItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem_5<RetType, T: QGraphicsItem_mapFromItem_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem_5(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromItem_5<RetType> {
  fn mapFromItem_5(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromItem_5<usize> for (usize,f64,f64,f64,f64) {
  fn mapFromItem_5(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem11mapFromItemEPKS_dddd", 5,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:366
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF mapFromParent(const QPointF &) const

/*
Maps the point point, which is in this item's parent's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromScene(), transform(), mapToParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent_0<RetType, T: QGraphicsItem_mapFromParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromParent_0<RetType> {
  fn mapFromParent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromParent_0<usize> for (usize) {
  fn mapFromParent_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13mapFromParentERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:369
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapFromParent(const QRectF &) const

/*
Maps the point point, which is in this item's parent's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromScene(), transform(), mapToParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent_1<RetType, T: QGraphicsItem_mapFromParent_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromParent_1<RetType> {
  fn mapFromParent_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromParent_1<usize> for (usize) {
  fn mapFromParent_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13mapFromParentERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:375
// index:2
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapFromParent(const QPolygonF &) const

/*
Maps the point point, which is in this item's parent's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromScene(), transform(), mapToParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent_2<RetType, T: QGraphicsItem_mapFromParent_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent_2(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromParent_2<RetType> {
  fn mapFromParent_2(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromParent_2<usize> for (usize) {
  fn mapFromParent_2(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13mapFromParentERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:378
// index:3
// Public Visibility=Default Availability=Available
// [8] QPainterPath mapFromParent(const QPainterPath &) const

/*
Maps the point point, which is in this item's parent's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromScene(), transform(), mapToParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent_3<RetType, T: QGraphicsItem_mapFromParent_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent_3(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromParent_3<RetType> {
  fn mapFromParent_3(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromParent_3<usize> for (usize) {
  fn mapFromParent_3(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13mapFromParentERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:391
// index:4
// Public inline Visibility=Default Availability=Available
// [16] QPointF mapFromParent(qreal, qreal) const

/*
Maps the point point, which is in this item's parent's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromScene(), transform(), mapToParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent_4<RetType, T: QGraphicsItem_mapFromParent_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent_4(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromParent_4<RetType> {
  fn mapFromParent_4(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromParent_4<usize> for (f64,f64) {
  fn mapFromParent_4(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13mapFromParentEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:394
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF mapFromParent(qreal, qreal, qreal, qreal) const

/*
Maps the point point, which is in this item's parent's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromScene(), transform(), mapToParent(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent_5<RetType, T: QGraphicsItem_mapFromParent_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent_5(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromParent_5<RetType> {
  fn mapFromParent_5(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromParent_5<usize> for (f64,f64,f64,f64) {
  fn mapFromParent_5(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem13mapFromParentEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:367
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF mapFromScene(const QPointF &) const

/*
Maps the point point, which is in this item's scene's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromParent(), transform(), mapToScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene_0<RetType, T: QGraphicsItem_mapFromScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromScene_0<RetType> {
  fn mapFromScene_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromScene_0<usize> for (usize) {
  fn mapFromScene_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12mapFromSceneERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:370
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapFromScene(const QRectF &) const

/*
Maps the point point, which is in this item's scene's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromParent(), transform(), mapToScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene_1<RetType, T: QGraphicsItem_mapFromScene_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromScene_1<RetType> {
  fn mapFromScene_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromScene_1<usize> for (usize) {
  fn mapFromScene_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12mapFromSceneERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:376
// index:2
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapFromScene(const QPolygonF &) const

/*
Maps the point point, which is in this item's scene's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromParent(), transform(), mapToScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene_2<RetType, T: QGraphicsItem_mapFromScene_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_2(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromScene_2<RetType> {
  fn mapFromScene_2(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromScene_2<usize> for (usize) {
  fn mapFromScene_2(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:379
// index:3
// Public Visibility=Default Availability=Available
// [8] QPainterPath mapFromScene(const QPainterPath &) const

/*
Maps the point point, which is in this item's scene's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromParent(), transform(), mapToScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene_3<RetType, T: QGraphicsItem_mapFromScene_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_3(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromScene_3<RetType> {
  fn mapFromScene_3(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromScene_3<usize> for (usize) {
  fn mapFromScene_3(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:392
// index:4
// Public inline Visibility=Default Availability=Available
// [16] QPointF mapFromScene(qreal, qreal) const

/*
Maps the point point, which is in this item's scene's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromParent(), transform(), mapToScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene_4<RetType, T: QGraphicsItem_mapFromScene_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_4(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromScene_4<RetType> {
  fn mapFromScene_4(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromScene_4<usize> for (f64,f64) {
  fn mapFromScene_4(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12mapFromSceneEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:395
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF mapFromScene(qreal, qreal, qreal, qreal) const

/*
Maps the point point, which is in this item's scene's coordinate system, to this item's coordinate system, and returns the mapped coordinate.

See also mapFromItem(), mapFromParent(), transform(), mapToScene(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene_5<RetType, T: QGraphicsItem_mapFromScene_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_5(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapFromScene_5<RetType> {
  fn mapFromScene_5(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapFromScene_5<usize> for (f64,f64,f64,f64) {
  fn mapFromScene_5(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12mapFromSceneEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:371
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF mapRectFromItem(const QGraphicsItem *, const QRectF &) const

/*
Maps the rectangle rect, which is in item's coordinate system, to this item's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

If item is 0, this function returns the same as mapRectFromScene().

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromItem_0<RetType, T: QGraphicsItem_mapRectFromItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectFromItem_0<RetType> {
  fn mapRectFromItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem_0<usize> for (usize,usize) {
  fn mapRectFromItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:396
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QRectF mapRectFromItem(const QGraphicsItem *, qreal, qreal, qreal, qreal) const

/*
Maps the rectangle rect, which is in item's coordinate system, to this item's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

If item is 0, this function returns the same as mapRectFromScene().

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromItem_1<RetType, T: QGraphicsItem_mapRectFromItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromItem_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectFromItem_1<RetType> {
  fn mapRectFromItem_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem_1<usize> for (usize,f64,f64,f64,f64) {
  fn mapRectFromItem_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd", 5,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:372
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF mapRectFromParent(const QRectF &) const

/*
Maps the rectangle rect, which is in this item's parent's coordinate system, to this item's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromParent_0<RetType, T: QGraphicsItem_mapRectFromParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromParent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectFromParent_0<RetType> {
  fn mapRectFromParent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent_0<usize> for (usize) {
  fn mapRectFromParent_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:397
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QRectF mapRectFromParent(qreal, qreal, qreal, qreal) const

/*
Maps the rectangle rect, which is in this item's parent's coordinate system, to this item's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromParent_1<RetType, T: QGraphicsItem_mapRectFromParent_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromParent_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectFromParent_1<RetType> {
  fn mapRectFromParent_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent_1<usize> for (f64,f64,f64,f64) {
  fn mapRectFromParent_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem17mapRectFromParentEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:373
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF mapRectFromScene(const QRectF &) const

/*
Maps the rectangle rect, which is in scene coordinates, to this item's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromScene_0<RetType, T: QGraphicsItem_mapRectFromScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromScene_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectFromScene_0<RetType> {
  fn mapRectFromScene_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene_0<usize> for (usize) {
  fn mapRectFromScene_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:398
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QRectF mapRectFromScene(qreal, qreal, qreal, qreal) const

/*
Maps the rectangle rect, which is in scene coordinates, to this item's coordinate system, and returns the mapped rectangle as a new rectangle (i.e., the bounding rectangle of the resulting polygon).

This function was introduced in  Qt 4.5.

See also itemTransform(), mapToParent(), mapToScene(), mapFromItem(), and The Graphics View Coordinate System.
*/
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromScene_1<RetType, T: QGraphicsItem_mapRectFromScene_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromScene_1(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mapRectFromScene_1<RetType> {
  fn mapRectFromScene_1(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene_1<usize> for (f64,f64,f64,f64) {
  fn mapRectFromScene_1(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem16mapRectFromSceneEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:400
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAncestorOf(const QGraphicsItem *) const

/*
Returns true if this item is an ancestor of child (i.e., if this item is child's parent, or one of child's parent's ancestors).

See also parentItem().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isAncestorOf_0<RetType, T: QGraphicsItem_isAncestorOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAncestorOf_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isAncestorOf_0<RetType> {
  fn isAncestorOf_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isAncestorOf_0<bool> for (usize) {
  fn isAncestorOf_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12isAncestorOfEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:401
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * commonAncestorItem(const QGraphicsItem *) const

/*
Returns the closest common ancestor item of this item and other, or 0 if either other is 0, or there is no common ancestor.

This function was introduced in  Qt 4.4.

See also isAncestorOf().
*/
impl /*struct*/ QGraphicsItem {
  pub fn commonAncestorItem_0<RetType, T: QGraphicsItem_commonAncestorItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commonAncestorItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_commonAncestorItem_0<RetType> {
  fn commonAncestorItem_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_commonAncestorItem_0<usize> for (usize) {
  fn commonAncestorItem_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem18commonAncestorItemEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:402
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isUnderMouse() const

/*
Returns true if this item is currently under the mouse cursor in one of the views; otherwise, false is returned.

This function was introduced in  Qt 4.4.

See also QGraphicsScene::views() and QCursor::pos().
*/
impl /*struct*/ QGraphicsItem {
  pub fn isUnderMouse_0<RetType, T: QGraphicsItem_isUnderMouse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUnderMouse_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_isUnderMouse_0<RetType> {
  fn isUnderMouse_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_isUnderMouse_0<bool> for () {
  fn isUnderMouse_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem12isUnderMouseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:405
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant data(int) const

/*
Returns this item's custom data for the key key as a QVariant.

Custom item data is useful for storing arbitrary properties in any item. Example:


  static const int ObjectName = 0;

  QGraphicsItem *item = scene.itemAt(100, 50);
  if (item->data(ObjectName).toString().isEmpty()) {
      if (qgraphicsitem_cast<ButtonItem *>(item))
          item->setData(ObjectName, "Button");
  }



Qt does not use this feature for storing data; it is provided solely for the convenience of the user.

See also setData().
*/
impl /*struct*/ QGraphicsItem {
  pub fn data_0<RetType, T: QGraphicsItem_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_data_0<RetType> {
  fn data_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_data_0<usize> for (i32) {
  fn data_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem4dataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:406
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setData(int, const QVariant &)

/*
Sets this item's custom data for the key key to value.

Custom item data is useful for storing arbitrary properties for any item. Qt does not use this feature for storing data; it is provided solely for the convenience of the user.

See also data().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setData_0<RetType, T: QGraphicsItem_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setData_0<RetType> {
  fn setData_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setData_0<(/*void*/)> for (i32,usize) {
  fn setData_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem7setDataEiRK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:408
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::InputMethodHints inputMethodHints() const

/*
Returns the current input method hints of this item.

Input method hints are only relevant for input items. The hints are used by the input method to indicate how it should operate. For example, if the Qt::ImhNumbersOnly flag is set, the input method may change its visual components to reflect that only numbers can be entered.

The effect may vary between input method implementations.

This function was introduced in  Qt 4.6.

See also setInputMethodHints() and inputMethodQuery().
*/
impl /*struct*/ QGraphicsItem {
  pub fn inputMethodHints_0<RetType, T: QGraphicsItem_inputMethodHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodHints_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_inputMethodHints_0<RetType> {
  fn inputMethodHints_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_inputMethodHints_0<i32> for () {
  fn inputMethodHints_0(self , rsthis: & QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem16inputMethodHintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:409
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInputMethodHints(Qt::InputMethodHints)

/*
Sets the current input method hints of this item to hints.

This function was introduced in  Qt 4.6.

See also inputMethodHints() and inputMethodQuery().
*/
impl /*struct*/ QGraphicsItem {
  pub fn setInputMethodHints_0<RetType, T: QGraphicsItem_setInputMethodHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInputMethodHints_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setInputMethodHints_0<RetType> {
  fn setInputMethodHints_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setInputMethodHints_0<(/*void*/)> for (i32) {
  fn setInputMethodHints_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem19setInputMethodHintsE6QFlagsIN2Qt15InputMethodHintEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:415
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int type() const

/*
Returns the type of an item as an int. All standard graphicsitem classes are associated with a unique value; see QGraphicsItem::Type. This type information is used by qgraphicsitem_cast() to distinguish between types.

The default implementation (in QGraphicsItem) returns UserType.

To enable use of qgraphicsitem_cast() with a custom item, reimplement this function and declare a Type enum value equal to your custom item's type. Custom items must return a value larger than or equal to UserType (65536).

For example:


  class CustomItem : public QGraphicsItem
  {
  public:
     enum { Type = UserType + 1 };

     int type() const
     {
         // Enable the use of qgraphicsitem_cast with this item.
         return Type;
     }
     ...
  };



See also UserType.
*/
impl /*struct*/ QGraphicsItem {
  pub fn type__0<RetType, T: QGraphicsItem_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_type__0<RetType> {
  fn type__0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_type__0<i32> for () {
  fn type__0(self , rsthis: & QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:417
// index:0
// Public Visibility=Default Availability=Available
// [-2] void installSceneEventFilter(QGraphicsItem *)

/*
Installs an event filter for this item on filterItem, causing all events for this item to first pass through filterItem's sceneEventFilter() function.

To filter another item's events, install this item as an event filter for the other item. Example:


  QGraphicsScene scene;
  QGraphicsEllipseItem *ellipse = scene.addEllipse(QRectF(-10, -10, 20, 20));
  QGraphicsLineItem *line = scene.addLine(QLineF(-10, -10, 20, 20));

  line->installSceneEventFilter(ellipse);
  // line's events are filtered by ellipse's sceneEventFilter() function.

  ellipse->installSceneEventFilter(line);
  // ellipse's events are filtered by line's sceneEventFilter() function.



An item can only filter events for other items in the same scene. Also, an item cannot filter its own events; instead, you can reimplement sceneEvent() directly.

Items must belong to a scene for scene event filters to be installed and used.

See also removeSceneEventFilter(), sceneEventFilter(), and sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn installSceneEventFilter_0<RetType, T: QGraphicsItem_installSceneEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.installSceneEventFilter_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_installSceneEventFilter_0<RetType> {
  fn installSceneEventFilter_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_installSceneEventFilter_0<(/*void*/)> for (usize) {
  fn installSceneEventFilter_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem23installSceneEventFilterEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:418
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeSceneEventFilter(QGraphicsItem *)

/*
Removes an event filter on this item from filterItem.

See also installSceneEventFilter().
*/
impl /*struct*/ QGraphicsItem {
  pub fn removeSceneEventFilter_0<RetType, T: QGraphicsItem_removeSceneEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeSceneEventFilter_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_removeSceneEventFilter_0<RetType> {
  fn removeSceneEventFilter_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_removeSceneEventFilter_0<(/*void*/)> for (usize) {
  fn removeSceneEventFilter_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem22removeSceneEventFilterEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:421
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void updateMicroFocus()

/*
Updates the item's micro focus.

This function was introduced in  Qt 4.7.

See also QInputMethod.
*/
impl /*struct*/ QGraphicsItem {
  pub fn updateMicroFocus_0<RetType, T: QGraphicsItem_updateMicroFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateMicroFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_updateMicroFocus_0<RetType> {
  fn updateMicroFocus_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_updateMicroFocus_0<(/*void*/)> for () {
  fn updateMicroFocus_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem16updateMicroFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:422
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool sceneEventFilter(QGraphicsItem *, QEvent *)

/*
Filters events for the item watched. event is the filtered event.

Reimplementing this function in a subclass makes it possible for the item to be used as an event filter for other items, intercepting all the events sent to those items before they are able to respond.

Reimplementations must return true to prevent further processing of a given event, ensuring that it will not be delivered to the watched item, or return false to indicate that the event should be propagated further by the event system.

See also installSceneEventFilter().
*/
impl /*struct*/ QGraphicsItem {
  pub fn sceneEventFilter_0<RetType, T: QGraphicsItem_sceneEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneEventFilter_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_sceneEventFilter_0<RetType> {
  fn sceneEventFilter_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_sceneEventFilter_0<bool> for (usize,usize) {
  fn sceneEventFilter_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QGraphicsItem16sceneEventFilterEPS_P6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:423
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool sceneEvent(QEvent *)

/*
This virtual function receives events to this item. Reimplement this function to intercept events before they are dispatched to the specialized event handlers contextMenuEvent(), focusInEvent(), focusOutEvent(), hoverEnterEvent(), hoverMoveEvent(), hoverLeaveEvent(), keyPressEvent(), keyReleaseEvent(), mousePressEvent(), mouseReleaseEvent(), mouseMoveEvent(), and mouseDoubleClickEvent().

Returns true if the event was recognized and handled; otherwise, (e.g., if the event type was not recognized,) false is returned.

event is the intercepted event.
*/
impl /*struct*/ QGraphicsItem {
  pub fn sceneEvent_0<RetType, T: QGraphicsItem_sceneEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_sceneEvent_0<RetType> {
  fn sceneEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_sceneEvent_0<bool> for (usize) {
  fn sceneEvent_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10sceneEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:424
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QGraphicsSceneContextMenuEvent *)

/*
This event handler can be reimplemented in a subclass to process context menu events. The event parameter contains details about the event to be handled.

If you ignore the event (i.e., by calling QEvent::ignore()), event will propagate to any item beneath this item. If no items accept the event, it will be ignored by the scene and propagate to the view.

It's common to open a QMenu in response to receiving a context menu event. Example:


  void CustomItem::contextMenuEvent(QGraphicsSceneContextMenuEvent *event)
  {
      QMenu menu;
      QAction *removeAction = menu.addAction("Remove");
      QAction *markAction = menu.addAction("Mark");
      QAction *selectedAction = menu.exec(event->screenPos());
      // ...
  }



The default implementation ignores the event.

See also sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn contextMenuEvent_0<RetType, T: QGraphicsItem_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem16contextMenuEventEP30QGraphicsSceneContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:425
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented to receive drag enter events for this item. Drag enter events are generated as the cursor enters the item's area.

By accepting the event (i.e., by calling QEvent::accept()), the item will accept drop events, in addition to receiving drag move and drag leave. Otherwise, the event will be ignored and propagate to the item beneath. If the event is accepted, the item will receive a drag move event before control goes back to the event loop.

A common implementation of dragEnterEvent accepts or ignores event depending on the associated mime data in event. Example:


  CustomItem::CustomItem()
  {
      setAcceptDrops(true);
      ...
  }

  void CustomItem::dragEnterEvent(QGraphicsSceneDragDropEvent *event)
  {
      event->setAccepted(event->mimeData()->hasFormat("text/plain"));
  }



Items do not receive drag and drop events by default; to enable this feature, call setAcceptDrops(true).

The default implementation does nothing.

See also dropEvent(), dragMoveEvent(), and dragLeaveEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn dragEnterEvent_0<RetType, T: QGraphicsItem_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem14dragEnterEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:426
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented to receive drag leave events for this item. Drag leave events are generated as the cursor leaves the item's area. Most often you will not need to reimplement this function, but it can be useful for resetting state in your item (e.g., highlighting).

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

Items do not receive drag and drop events by default; to enable this feature, call setAcceptDrops(true).

The default implementation does nothing.

See also dragEnterEvent(), dropEvent(), and dragMoveEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn dragLeaveEvent_0<RetType, T: QGraphicsItem_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem14dragLeaveEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:427
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented to receive drag move events for this item. Drag move events are generated as the cursor moves around inside the item's area. Most often you will not need to reimplement this function; it is used to indicate that only parts of the item can accept drops.

Calling QEvent::ignore() or QEvent::accept() on event toggles whether or not the item will accept drops at the position from the event. By default, event is accepted, indicating that the item allows drops at the specified position.

Items do not receive drag and drop events by default; to enable this feature, call setAcceptDrops(true).

The default implementation does nothing.

See also dropEvent(), dragEnterEvent(), and dragLeaveEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn dragMoveEvent_0<RetType, T: QGraphicsItem_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem13dragMoveEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:428
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented to receive drop events for this item. Items can only receive drop events if the last drag move event was accepted.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

Items do not receive drag and drop events by default; to enable this feature, call setAcceptDrops(true).

The default implementation does nothing.

See also dragEnterEvent(), dragMoveEvent(), and dragLeaveEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn dropEvent_0<RetType, T: QGraphicsItem_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem9dropEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:429
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
This event handler, for event event, can be reimplemented to receive focus in events for this item. The default implementation calls ensureVisible().

See also focusOutEvent(), sceneEvent(), and setFocus().
*/
impl /*struct*/ QGraphicsItem {
  pub fn focusInEvent_0<RetType, T: QGraphicsItem_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:430
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
This event handler, for event event, can be reimplemented to receive focus out events for this item. The default implementation does nothing.

See also focusInEvent(), sceneEvent(), and setFocus().
*/
impl /*struct*/ QGraphicsItem {
  pub fn focusOutEvent_0<RetType, T: QGraphicsItem_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:431
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverEnterEvent(QGraphicsSceneHoverEvent *)

/*
This event handler, for event event, can be reimplemented to receive hover enter events for this item. The default implementation calls update(); otherwise it does nothing.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

See also hoverMoveEvent(), hoverLeaveEvent(), sceneEvent(), and setAcceptHoverEvents().
*/
impl /*struct*/ QGraphicsItem {
  pub fn hoverEnterEvent_0<RetType, T: QGraphicsItem_hoverEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverEnterEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_hoverEnterEvent_0<RetType> {
  fn hoverEnterEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_hoverEnterEvent_0<(/*void*/)> for (usize) {
  fn hoverEnterEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem15hoverEnterEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:432
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverMoveEvent(QGraphicsSceneHoverEvent *)

/*
This event handler, for event event, can be reimplemented to receive hover move events for this item. The default implementation does nothing.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

See also hoverEnterEvent(), hoverLeaveEvent(), sceneEvent(), and setAcceptHoverEvents().
*/
impl /*struct*/ QGraphicsItem {
  pub fn hoverMoveEvent_0<RetType, T: QGraphicsItem_hoverMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_hoverMoveEvent_0<RetType> {
  fn hoverMoveEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_hoverMoveEvent_0<(/*void*/)> for (usize) {
  fn hoverMoveEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem14hoverMoveEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:433
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverLeaveEvent(QGraphicsSceneHoverEvent *)

/*
This event handler, for event event, can be reimplemented to receive hover leave events for this item. The default implementation calls update(); otherwise it does nothing.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

See also hoverEnterEvent(), hoverMoveEvent(), sceneEvent(), and setAcceptHoverEvents().
*/
impl /*struct*/ QGraphicsItem {
  pub fn hoverLeaveEvent_0<RetType, T: QGraphicsItem_hoverLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_hoverLeaveEvent_0<RetType> {
  fn hoverLeaveEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_hoverLeaveEvent_0<(/*void*/)> for (usize) {
  fn hoverLeaveEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem15hoverLeaveEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:434
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
This event handler, for event event, can be reimplemented to receive key press events for this item. The default implementation ignores the event. If you reimplement this handler, the event will by default be accepted.

Note that key events are only received for items that set the ItemIsFocusable flag, and that have keyboard input focus.

See also keyReleaseEvent(), setFocus(), QGraphicsScene::setFocusItem(), and sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn keyPressEvent_0<RetType, T: QGraphicsItem_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:435
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
This event handler, for event event, can be reimplemented to receive key release events for this item. The default implementation ignores the event. If you reimplement this handler, the event will by default be accepted.

Note that key events are only received for items that set the ItemIsFocusable flag, and that have keyboard input focus.

See also keyPressEvent(), setFocus(), QGraphicsScene::setFocusItem(), and sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn keyReleaseEvent_0<RetType, T: QGraphicsItem_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:436
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event event, can be reimplemented to receive mouse press events for this item. Mouse press events are only delivered to items that accept the mouse button that is pressed. By default, an item accepts all mouse buttons, but you can change this by calling setAcceptedMouseButtons().

The mouse press event decides which item should become the mouse grabber (see QGraphicsScene::mouseGrabberItem()). If you do not reimplement this function, the press event will propagate to any topmost item beneath this item, and no other mouse events will be delivered to this item.

If you do reimplement this function, event will by default be accepted (see QEvent::accept()), and this item is then the mouse grabber. This allows the item to receive future move, release and doubleclick events. If you call QEvent::ignore() on event, this item will lose the mouse grab, and event will propagate to any topmost item beneath. No further mouse events will be delivered to this item unless a new mouse press event is received.

The default implementation handles basic item interaction, such as selection and moving. If you want to keep the base implementation when reimplementing this function, call QGraphicsItem::mousePressEvent() in your reimplementation.

The event is QEvent::ignore()d for items that are neither movable nor selectable.

See also mouseMoveEvent(), mouseReleaseEvent(), mouseDoubleClickEvent(), and sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn mousePressEvent_0<RetType, T: QGraphicsItem_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem15mousePressEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:437
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event event, can be reimplemented to receive mouse move events for this item. If you do receive this event, you can be certain that this item also received a mouse press event, and that this item is the current mouse grabber.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

The default implementation handles basic item interaction, such as selection and moving. If you want to keep the base implementation when reimplementing this function, call QGraphicsItem::mouseMoveEvent() in your reimplementation.

Please note that mousePressEvent() decides which graphics item it is that receives mouse events. See the mousePressEvent() description for details.

See also mousePressEvent(), mouseReleaseEvent(), mouseDoubleClickEvent(), and sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn mouseMoveEvent_0<RetType, T: QGraphicsItem_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem14mouseMoveEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:438
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event event, can be reimplemented to receive mouse release events for this item.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

The default implementation handles basic item interaction, such as selection and moving. If you want to keep the base implementation when reimplementing this function, call QGraphicsItem::mouseReleaseEvent() in your reimplementation.

Please note that mousePressEvent() decides which graphics item it is that receives mouse events. See the mousePressEvent() description for details.

See also mousePressEvent(), mouseMoveEvent(), mouseDoubleClickEvent(), and sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn mouseReleaseEvent_0<RetType, T: QGraphicsItem_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem17mouseReleaseEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:439
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event event, can be reimplemented to receive mouse doubleclick events for this item.

When doubleclicking an item, the item will first receive a mouse press event, followed by a release event (i.e., a click), then a doubleclick event, and finally a release event.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

The default implementation calls mousePressEvent(). If you want to keep the base implementation when reimplementing this function, call QGraphicsItem::mouseDoubleClickEvent() in your reimplementation.

Note that an item will not receive double click events if it is neither selectable nor movable (single mouse clicks are ignored in this case, and that stops the generation of double clicks).

See also mousePressEvent(), mouseMoveEvent(), mouseReleaseEvent(), and sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn mouseDoubleClickEvent_0<RetType, T: QGraphicsItem_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem21mouseDoubleClickEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:440
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QGraphicsSceneWheelEvent *)

/*
This event handler, for event event, can be reimplemented to receive wheel events for this item. If you reimplement this function, event will be accepted by default.

If you ignore the event, (i.e., by calling QEvent::ignore(),) it will propagate to any item beneath this item. If no items accept the event, it will be ignored by the scene, and propagate to the view (e.g., the view's vertical scroll bar).

The default implementation ignores the event.

See also sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn wheelEvent_0<RetType, T: QGraphicsItem_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10wheelEventEP24QGraphicsSceneWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:441
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
This event handler, for event event, can be reimplemented to receive input method events for this item. The default implementation ignores the event.

See also inputMethodQuery() and sceneEvent().
*/
impl /*struct*/ QGraphicsItem {
  pub fn inputMethodEvent_0<RetType, T: QGraphicsItem_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:442
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
This method is only relevant for input items. It is used by the input method to query a set of properties of the item to be able to support complex input method operations, such as support for surrounding text and reconversions. query specifies which property is queried.

See also inputMethodEvent() and QInputMethodEvent.
*/
impl /*struct*/ QGraphicsItem {
  pub fn inputMethodQuery_0<RetType, T: QGraphicsItem_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:444
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant itemChange(QGraphicsItem::GraphicsItemChange, const QVariant &)

/*
This virtual function is called by QGraphicsItem to notify custom items that some part of the item's state changes. By reimplementing this function, you can react to a change, and in some cases (depending on change), adjustments can be made.

change is the parameter of the item that is changing. value is the new value; the type of the value depends on change.

Example:


  QVariant Component::itemChange(GraphicsItemChange change, const QVariant &value)
  {
      if (change == ItemPositionChange && scene()) {
          // value is the new position.
          QPointF newPos = value.toPointF();
          QRectF rect = scene()->sceneRect();
          if (!rect.contains(newPos)) {
              // Keep the item inside the scene rect.
              newPos.setX(qMin(rect.right(), qMax(newPos.x(), rect.left())));
              newPos.setY(qMin(rect.bottom(), qMax(newPos.y(), rect.top())));
              return newPos;
          }
      }
      return QGraphicsItem::itemChange(change, value);
  }



The default implementation does nothing, and returns value.

Note: Certain QGraphicsItem functions cannot be called in a reimplementation of this function; see the GraphicsItemChange documentation for details.

See also GraphicsItemChange.
*/
impl /*struct*/ QGraphicsItem {
  pub fn itemChange_0<RetType, T: QGraphicsItem_itemChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemChange_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_itemChange_0<RetType> {
  fn itemChange_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_itemChange_0<usize> for (i32,usize) {
  fn itemChange_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10itemChangeENS_18GraphicsItemChangeERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:449
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool supportsExtension(QGraphicsItem::Extension) const

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn supportsExtension_0<RetType, T: QGraphicsItem_supportsExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsExtension_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_supportsExtension_0<RetType> {
  fn supportsExtension_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_supportsExtension_0<bool> for (i32) {
  fn supportsExtension_0(self , rsthis: & QGraphicsItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem17supportsExtensionENS_9ExtensionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:450
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setExtension(QGraphicsItem::Extension, const QVariant &)

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn setExtension_0<RetType, T: QGraphicsItem_setExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExtension_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_setExtension_0<RetType> {
  fn setExtension_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_setExtension_0<(/*void*/)> for (i32,usize) {
  fn setExtension_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem12setExtensionENS_9ExtensionERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:451
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant extension(const QVariant &) const

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn extension_0<RetType, T: QGraphicsItem_extension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.extension_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_extension_0<RetType> {
  fn extension_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_extension_0<usize> for (usize) {
  fn extension_0(self , rsthis: & QGraphicsItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsItem9extensionERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:457
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void addToIndex()

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn addToIndex_0<RetType, T: QGraphicsItem_addToIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addToIndex_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_addToIndex_0<RetType> {
  fn addToIndex_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_addToIndex_0<(/*void*/)> for () {
  fn addToIndex_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem10addToIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:458
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void removeFromIndex()

/*

*/
impl /*struct*/ QGraphicsItem {
  pub fn removeFromIndex_0<RetType, T: QGraphicsItem_removeFromIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeFromIndex_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_removeFromIndex_0<RetType> {
  fn removeFromIndex_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_removeFromIndex_0<(/*void*/)> for () {
  fn removeFromIndex_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem15removeFromIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:459
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void prepareGeometryChange()

/*
Prepares the item for a geometry change. Call this function before changing the bounding rect of an item to keep QGraphicsScene's index up to date.

prepareGeometryChange() will call update() if this is necessary.

Example:


  void CircleItem::setRadius(qreal newRadius)
  {
      if (radius != newRadius) {
          prepareGeometryChange();
          radius = newRadius;
      }
  }



See also boundingRect().
*/
impl /*struct*/ QGraphicsItem {
  pub fn prepareGeometryChange_0<RetType, T: QGraphicsItem_prepareGeometryChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.prepareGeometryChange_0(self);
    // return 1;
  }
}
pub trait QGraphicsItem_prepareGeometryChange_0<RetType> {
  fn prepareGeometryChange_0(self , rsthis: & QGraphicsItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItem_prepareGeometryChange_0<(/*void*/)> for () {
  fn prepareGeometryChange_0(self , rsthis: & QGraphicsItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsItem21prepareGeometryChangeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QGraphicsItem__GraphicsItemFlag = i32;
// 
pub const QGraphicsItem__ItemIsMovable :QGraphicsItem__GraphicsItemFlag = 1;
// 
pub const QGraphicsItem__ItemIsSelectable :QGraphicsItem__GraphicsItemFlag = 2;
// 
pub const QGraphicsItem__ItemIsFocusable :QGraphicsItem__GraphicsItemFlag = 4;
// 
pub const QGraphicsItem__ItemClipsToShape :QGraphicsItem__GraphicsItemFlag = 8;
// 
pub const QGraphicsItem__ItemClipsChildrenToShape :QGraphicsItem__GraphicsItemFlag = 16;
// 
pub const QGraphicsItem__ItemIgnoresTransformations :QGraphicsItem__GraphicsItemFlag = 32;
// 
pub const QGraphicsItem__ItemIgnoresParentOpacity :QGraphicsItem__GraphicsItemFlag = 64;
// 
pub const QGraphicsItem__ItemDoesntPropagateOpacityToChildren :QGraphicsItem__GraphicsItemFlag = 128;
// 
pub const QGraphicsItem__ItemStacksBehindParent :QGraphicsItem__GraphicsItemFlag = 256;
// 
pub const QGraphicsItem__ItemUsesExtendedStyleOption :QGraphicsItem__GraphicsItemFlag = 512;
// 
pub const QGraphicsItem__ItemHasNoContents :QGraphicsItem__GraphicsItemFlag = 1024;
// 
pub const QGraphicsItem__ItemSendsGeometryChanges :QGraphicsItem__GraphicsItemFlag = 2048;
// 
pub const QGraphicsItem__ItemAcceptsInputMethod :QGraphicsItem__GraphicsItemFlag = 4096;
// 
pub const QGraphicsItem__ItemNegativeZStacksBehindParent :QGraphicsItem__GraphicsItemFlag = 8192;
// 
pub const QGraphicsItem__ItemIsPanel :QGraphicsItem__GraphicsItemFlag = 16384;
// 
pub const QGraphicsItem__ItemIsFocusScope :QGraphicsItem__GraphicsItemFlag = 32768;
// 
pub const QGraphicsItem__ItemSendsScenePositionChanges :QGraphicsItem__GraphicsItemFlag = 65536;
// 
pub const QGraphicsItem__ItemStopsClickFocusPropagation :QGraphicsItem__GraphicsItemFlag = 131072;
// 
pub const QGraphicsItem__ItemStopsFocusHandling :QGraphicsItem__GraphicsItemFlag = 262144;
// 
pub const QGraphicsItem__ItemContainsChildrenInShape :QGraphicsItem__GraphicsItemFlag = 524288;
pub fn QGraphicsItem_GraphicsItemFlagItemName(val: i32) ->String {
  match val {
     QGraphicsItem__ItemIsMovable => // 1
     {return String::from("ItemIsMovable");}
     QGraphicsItem__ItemIsSelectable => // 2
     {return String::from("ItemIsSelectable");}
     QGraphicsItem__ItemIsFocusable => // 4
     {return String::from("ItemIsFocusable");}
     QGraphicsItem__ItemClipsToShape => // 8
     {return String::from("ItemClipsToShape");}
     QGraphicsItem__ItemClipsChildrenToShape => // 16
     {return String::from("ItemClipsChildrenToShape");}
     QGraphicsItem__ItemIgnoresTransformations => // 32
     {return String::from("ItemIgnoresTransformations");}
     QGraphicsItem__ItemIgnoresParentOpacity => // 64
     {return String::from("ItemIgnoresParentOpacity");}
     QGraphicsItem__ItemDoesntPropagateOpacityToChildren => // 128
     {return String::from("ItemDoesntPropagateOpacityToChildren");}
     QGraphicsItem__ItemStacksBehindParent => // 256
     {return String::from("ItemStacksBehindParent");}
     QGraphicsItem__ItemUsesExtendedStyleOption => // 512
     {return String::from("ItemUsesExtendedStyleOption");}
     QGraphicsItem__ItemHasNoContents => // 1024
     {return String::from("ItemHasNoContents");}
     QGraphicsItem__ItemSendsGeometryChanges => // 2048
     {return String::from("ItemSendsGeometryChanges");}
     QGraphicsItem__ItemAcceptsInputMethod => // 4096
     {return String::from("ItemAcceptsInputMethod");}
     QGraphicsItem__ItemNegativeZStacksBehindParent => // 8192
     {return String::from("ItemNegativeZStacksBehindParent");}
     QGraphicsItem__ItemIsPanel => // 16384
     {return String::from("ItemIsPanel");}
     QGraphicsItem__ItemIsFocusScope => // 32768
     {return String::from("ItemIsFocusScope");}
     QGraphicsItem__ItemSendsScenePositionChanges => // 65536
     {return String::from("ItemSendsScenePositionChanges");}
     QGraphicsItem__ItemStopsClickFocusPropagation => // 131072
     {return String::from("ItemStopsClickFocusPropagation");}
     QGraphicsItem__ItemStopsFocusHandling => // 262144
     {return String::from("ItemStopsFocusHandling");}
     QGraphicsItem__ItemContainsChildrenInShape => // 524288
     {return String::from("ItemContainsChildrenInShape");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsItem_GraphicsItemFlagItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsItem
  //return nilthis.GraphicsItemFlagItemName(val);
  return QGraphicsItem_GraphicsItemFlagItemName(val);
}


/*
This enum describes the state changes that are notified by QGraphicsItem::itemChange(). The notifications are sent as the state changes, and in some cases, adjustments can be made (see the documentation for each change for details).

Note: Be careful with calling functions on the QGraphicsItem itself inside itemChange(), as certain function calls can lead to unwanted recursion. For example, you cannot call setPos() in itemChange() on an ItemPositionChange notification, as the setPos() function will again call itemChange(ItemPositionChange). Instead, you can return the new, adjusted position from itemChange().


*/
pub type QGraphicsItem__GraphicsItemChange = i32;
// The item's position changes. This notification is sent if the ItemSendsGeometryChanges flag is enabled, and when the item's local position changes, relative to its parent (i.e., as a result of calling setPos() or moveBy()). The value argument is the new position (i.e., a QPointF). You can call pos() to get the original position. Do not call setPos() or moveBy() in itemChange() as this notification is delivered; instead, you can return the new, adjusted position from itemChange(). After this notification, QGraphicsItem immediately sends the ItemPositionHasChanged notification if the position changed.
pub const QGraphicsItem__ItemPositionChange :QGraphicsItem__GraphicsItemChange = 0;
// The item's affine transformation matrix is changing. This value is obsolete; you can use ItemTransformChange instead.
pub const QGraphicsItem__ItemMatrixChange :QGraphicsItem__GraphicsItemChange = 1;
// The item's visible state changes. If the item is presently visible, it will become invisible, and vice verca. The value argument is the new visible state (i.e., true or false). Do not call setVisible() in itemChange() as this notification is delivered; instead, you can return the new visible state from itemChange().
pub const QGraphicsItem__ItemVisibleChange :QGraphicsItem__GraphicsItemChange = 2;
// The item's enabled state changes. If the item is presently enabled, it will become disabled, and vice verca. The value argument is the new enabled state (i.e., true or false). Do not call setEnabled() in itemChange() as this notification is delivered. Instead, you can return the new state from itemChange().
pub const QGraphicsItem__ItemEnabledChange :QGraphicsItem__GraphicsItemChange = 3;
// The item's selected state changes. If the item is presently selected, it will become unselected, and vice verca. The value argument is the new selected state (i.e., true or false). Do not call setSelected() in itemChange() as this notification is delivered; instead, you can return the new selected state from itemChange().
pub const QGraphicsItem__ItemSelectedChange :QGraphicsItem__GraphicsItemChange = 4;
// The item's parent changes. The value argument is the new parent item (i.e., a QGraphicsItem pointer). Do not call setParentItem() in itemChange() as this notification is delivered; instead, you can return the new parent from itemChange().
pub const QGraphicsItem__ItemParentChange :QGraphicsItem__GraphicsItemChange = 5;
// A child is added to this item. The value argument is the new child item (i.e., a QGraphicsItem pointer). Do not pass this item to any item's setParentItem() function as this notification is delivered. The return value is unused; you cannot adjust anything in this notification. Note that the new child might not be fully constructed when this notification is sent; calling pure virtual functions on the child can lead to a crash.
pub const QGraphicsItem__ItemChildAddedChange :QGraphicsItem__GraphicsItemChange = 6;
// A child is removed from this item. The value argument is the child item that is about to be removed (i.e., a QGraphicsItem pointer). The return value is unused; you cannot adjust anything in this notification.
pub const QGraphicsItem__ItemChildRemovedChange :QGraphicsItem__GraphicsItemChange = 7;
// The item's transformation matrix changes. This notification is sent if the ItemSendsGeometryChanges flag is enabled, and when the item's local transformation matrix changes (i.e., as a result of calling setTransform(). The value argument is the new matrix (i.e., a QTransform); to get the old matrix, call transform(). Do not call setTransform() or set any of the transformation properties in itemChange() as this notification is delivered; instead, you can return the new matrix from itemChange(). This notification is not sent if you change the transformation properties.
pub const QGraphicsItem__ItemTransformChange :QGraphicsItem__GraphicsItemChange = 8;
// The item's position has changed. This notification is sent if the ItemSendsGeometryChanges flag is enabled, and after the item's local position, relative to its parent, has changed. The value argument is the new position (the same as pos()), and QGraphicsItem ignores the return value for this notification (i.e., a read-only notification).
pub const QGraphicsItem__ItemPositionHasChanged :QGraphicsItem__GraphicsItemChange = 9;
// 
pub const QGraphicsItem__ItemTransformHasChanged :QGraphicsItem__GraphicsItemChange = 10;
// 
pub const QGraphicsItem__ItemSceneChange :QGraphicsItem__GraphicsItemChange = 11;
// 
pub const QGraphicsItem__ItemVisibleHasChanged :QGraphicsItem__GraphicsItemChange = 12;
// 
pub const QGraphicsItem__ItemEnabledHasChanged :QGraphicsItem__GraphicsItemChange = 13;
// 
pub const QGraphicsItem__ItemSelectedHasChanged :QGraphicsItem__GraphicsItemChange = 14;
// 
pub const QGraphicsItem__ItemParentHasChanged :QGraphicsItem__GraphicsItemChange = 15;
// 
pub const QGraphicsItem__ItemSceneHasChanged :QGraphicsItem__GraphicsItemChange = 16;
// 
pub const QGraphicsItem__ItemCursorChange :QGraphicsItem__GraphicsItemChange = 17;
// 
pub const QGraphicsItem__ItemCursorHasChanged :QGraphicsItem__GraphicsItemChange = 18;
// 
pub const QGraphicsItem__ItemToolTipChange :QGraphicsItem__GraphicsItemChange = 19;
// 
pub const QGraphicsItem__ItemToolTipHasChanged :QGraphicsItem__GraphicsItemChange = 20;
// 
pub const QGraphicsItem__ItemFlagsChange :QGraphicsItem__GraphicsItemChange = 21;
// 
pub const QGraphicsItem__ItemFlagsHaveChanged :QGraphicsItem__GraphicsItemChange = 22;
// 
pub const QGraphicsItem__ItemZValueChange :QGraphicsItem__GraphicsItemChange = 23;
// 
pub const QGraphicsItem__ItemZValueHasChanged :QGraphicsItem__GraphicsItemChange = 24;
// 
pub const QGraphicsItem__ItemOpacityChange :QGraphicsItem__GraphicsItemChange = 25;
// 
pub const QGraphicsItem__ItemOpacityHasChanged :QGraphicsItem__GraphicsItemChange = 26;
// 
pub const QGraphicsItem__ItemScenePositionHasChanged :QGraphicsItem__GraphicsItemChange = 27;
// 
pub const QGraphicsItem__ItemRotationChange :QGraphicsItem__GraphicsItemChange = 28;
// 
pub const QGraphicsItem__ItemRotationHasChanged :QGraphicsItem__GraphicsItemChange = 29;
// 
pub const QGraphicsItem__ItemScaleChange :QGraphicsItem__GraphicsItemChange = 30;
// 
pub const QGraphicsItem__ItemScaleHasChanged :QGraphicsItem__GraphicsItemChange = 31;
// 
pub const QGraphicsItem__ItemTransformOriginPointChange :QGraphicsItem__GraphicsItemChange = 32;
// 
pub const QGraphicsItem__ItemTransformOriginPointHasChanged :QGraphicsItem__GraphicsItemChange = 33;
pub fn QGraphicsItem_GraphicsItemChangeItemName(val: i32) ->String {
  match val {
     QGraphicsItem__ItemPositionChange => // 0
     {return String::from("ItemPositionChange");}
     QGraphicsItem__ItemMatrixChange => // 1
     {return String::from("ItemMatrixChange");}
     QGraphicsItem__ItemVisibleChange => // 2
     {return String::from("ItemVisibleChange");}
     QGraphicsItem__ItemEnabledChange => // 3
     {return String::from("ItemEnabledChange");}
     QGraphicsItem__ItemSelectedChange => // 4
     {return String::from("ItemSelectedChange");}
     QGraphicsItem__ItemParentChange => // 5
     {return String::from("ItemParentChange");}
     QGraphicsItem__ItemChildAddedChange => // 6
     {return String::from("ItemChildAddedChange");}
     QGraphicsItem__ItemChildRemovedChange => // 7
     {return String::from("ItemChildRemovedChange");}
     QGraphicsItem__ItemTransformChange => // 8
     {return String::from("ItemTransformChange");}
     QGraphicsItem__ItemPositionHasChanged => // 9
     {return String::from("ItemPositionHasChanged");}
     QGraphicsItem__ItemTransformHasChanged => // 10
     {return String::from("ItemTransformHasChanged");}
     QGraphicsItem__ItemSceneChange => // 11
     {return String::from("ItemSceneChange");}
     QGraphicsItem__ItemVisibleHasChanged => // 12
     {return String::from("ItemVisibleHasChanged");}
     QGraphicsItem__ItemEnabledHasChanged => // 13
     {return String::from("ItemEnabledHasChanged");}
     QGraphicsItem__ItemSelectedHasChanged => // 14
     {return String::from("ItemSelectedHasChanged");}
     QGraphicsItem__ItemParentHasChanged => // 15
     {return String::from("ItemParentHasChanged");}
     QGraphicsItem__ItemSceneHasChanged => // 16
     {return String::from("ItemSceneHasChanged");}
     QGraphicsItem__ItemCursorChange => // 17
     {return String::from("ItemCursorChange");}
     QGraphicsItem__ItemCursorHasChanged => // 18
     {return String::from("ItemCursorHasChanged");}
     QGraphicsItem__ItemToolTipChange => // 19
     {return String::from("ItemToolTipChange");}
     QGraphicsItem__ItemToolTipHasChanged => // 20
     {return String::from("ItemToolTipHasChanged");}
     QGraphicsItem__ItemFlagsChange => // 21
     {return String::from("ItemFlagsChange");}
     QGraphicsItem__ItemFlagsHaveChanged => // 22
     {return String::from("ItemFlagsHaveChanged");}
     QGraphicsItem__ItemZValueChange => // 23
     {return String::from("ItemZValueChange");}
     QGraphicsItem__ItemZValueHasChanged => // 24
     {return String::from("ItemZValueHasChanged");}
     QGraphicsItem__ItemOpacityChange => // 25
     {return String::from("ItemOpacityChange");}
     QGraphicsItem__ItemOpacityHasChanged => // 26
     {return String::from("ItemOpacityHasChanged");}
     QGraphicsItem__ItemScenePositionHasChanged => // 27
     {return String::from("ItemScenePositionHasChanged");}
     QGraphicsItem__ItemRotationChange => // 28
     {return String::from("ItemRotationChange");}
     QGraphicsItem__ItemRotationHasChanged => // 29
     {return String::from("ItemRotationHasChanged");}
     QGraphicsItem__ItemScaleChange => // 30
     {return String::from("ItemScaleChange");}
     QGraphicsItem__ItemScaleHasChanged => // 31
     {return String::from("ItemScaleHasChanged");}
     QGraphicsItem__ItemTransformOriginPointChange => // 32
     {return String::from("ItemTransformOriginPointChange");}
     QGraphicsItem__ItemTransformOriginPointHasChanged => // 33
     {return String::from("ItemTransformOriginPointHasChanged");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsItem_GraphicsItemChangeItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsItem
  //return nilthis.GraphicsItemChangeItemName(val);
  return QGraphicsItem_GraphicsItemChangeItemName(val);
}


/*
This enum describes QGraphicsItem's cache modes. Caching is used to speed up rendering by allocating and rendering to an off-screen pixel buffer, which can be reused when the item requires redrawing. For some paint devices, the cache is stored directly in graphics memory, which makes rendering very quick.



This enum was introduced or modified in  Qt 4.4.

See also QGraphicsItem::setCacheMode().

*/
pub type QGraphicsItem__CacheMode = i32;
// The default; all item caching is disabled. QGraphicsItem::paint() is called every time the item needs redrawing.
pub const QGraphicsItem__NoCache :QGraphicsItem__CacheMode = 0;
// Caching is enabled for the item's logical (local) coordinate system. QGraphicsItem creates an off-screen pixel buffer with a configurable size / resolution that you can pass to QGraphicsItem::setCacheMode(). Rendering quality will typically degrade, depending on the resolution of the cache and the item transformation. The first time the item is redrawn, it will render itself into the cache, and the cache is then reused for every subsequent expose. The cache is also reused as the item is transformed. To adjust the resolution of the cache, you can call setCacheMode() again.
pub const QGraphicsItem__ItemCoordinateCache :QGraphicsItem__CacheMode = 1;
// Caching is enabled at the paint device level, in device coordinates. This mode is for items that can move, but are not rotated, scaled or sheared. If the item is transformed directly or indirectly, the cache will be regenerated automatically. Unlike ItemCoordinateCacheMode, DeviceCoordinateCache always renders at maximum quality.
pub const QGraphicsItem__DeviceCoordinateCache :QGraphicsItem__CacheMode = 2;
pub fn QGraphicsItem_CacheModeItemName(val: i32) ->String {
  match val {
     QGraphicsItem__NoCache => // 0
     {return String::from("NoCache");}
     QGraphicsItem__ItemCoordinateCache => // 1
     {return String::from("ItemCoordinateCache");}
     QGraphicsItem__DeviceCoordinateCache => // 2
     {return String::from("DeviceCoordinateCache");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsItem_CacheModeItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsItem
  //return nilthis.CacheModeItemName(val);
  return QGraphicsItem_CacheModeItemName(val);
}


/*
This enum specifies the behavior of a modal panel. A modal panel is one that blocks input to other panels. Note that items that are children of a modal panel are not blocked.

The values are:



This enum was introduced or modified in  Qt 4.6.

See also QGraphicsItem::setPanelModality(), QGraphicsItem::panelModality(), and QGraphicsItem::ItemIsPanel.

*/
pub type QGraphicsItem__PanelModality = i32;
// The panel is not modal and does not block input to other panels. This is the default value for panels.
pub const QGraphicsItem__NonModal :QGraphicsItem__PanelModality = 0;
// The panel is modal to a single item hierarchy and blocks input to its parent pane, all grandparent panels, and all siblings of its parent and grandparent panels.
pub const QGraphicsItem__PanelModal :QGraphicsItem__PanelModality = 1;
// The window is modal to the entire scene and blocks input to all panels.
pub const QGraphicsItem__SceneModal :QGraphicsItem__PanelModality = 2;
pub fn QGraphicsItem_PanelModalityItemName(val: i32) ->String {
  match val {
     QGraphicsItem__NonModal => // 0
     {return String::from("NonModal");}
     QGraphicsItem__PanelModal => // 1
     {return String::from("PanelModal");}
     QGraphicsItem__SceneModal => // 2
     {return String::from("SceneModal");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsItem_PanelModalityItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsItem
  //return nilthis.PanelModalityItemName(val);
  return QGraphicsItem_PanelModalityItemName(val);
}


/*


*/
pub type QGraphicsItem__ = i32;
// 
pub const QGraphicsItem__Type :QGraphicsItem__ = 1;
// 
pub const QGraphicsItem__UserType :QGraphicsItem__ = 65536;
pub fn QGraphicsItem_ItemName(val: i32) ->String {
  match val {
     QGraphicsItem__Type => // 1
     {return String::from("Type");}
     QGraphicsItem__UserType => // 65536
     {return String::from("UserType");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsItem_ItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsItem
  //return nilthis.ItemName(val);
  return QGraphicsItem_ItemName(val);
}


/*


*/
pub type QGraphicsItem__Extension = i32;
// 
pub const QGraphicsItem__UserExtension :QGraphicsItem__Extension = -2147483648;
pub fn QGraphicsItem_ExtensionItemName(val: i32) ->String {
  match val {
     QGraphicsItem__UserExtension => // -2147483648
     {return String::from("UserExtension");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsItem_ExtensionItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsItem
  //return nilthis.ExtensionItemName(val);
  return QGraphicsItem_ExtensionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
