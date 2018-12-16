

// mod ::widgets::QAbstractItemDelegate
// package qtwidgets
// /usr/include/qt/QtWidgets/qabstractitemdelegate.h
// #include <qabstractitemdelegate.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QAbstractItemDelegate)=16
pub struct QAbstractItemDelegate {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractItemDelegate_ITF interface {
//    qtcore.QObject_ITF
//    QAbstractItemDelegate_PTR() *QAbstractItemDelegate
//}
//func (ptr *QAbstractItemDelegate) QAbstractItemDelegate_PTR() *QAbstractItemDelegate { return ptr }

impl /*struct*/ QAbstractItemDelegate {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractItemDelegate {
    return QAbstractItemDelegate{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractItemDelegate {
//  type Target = QAbstractItemDelegateBASE;
//
//  fn deref(&self) -> &QAbstractItemDelegateBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractItemDelegateBASE> for QAbstractItemDelegate {
//  fn as_ref(& self) -> & QAbstractItemDelegateBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn metaObject_0<RetType, T: QAbstractItemDelegate_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QAbstractItemDelegate10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractItemDelegate(QObject *)

/*
Creates a new abstract item delegate with the given parent.
*/
// QAbstractItemDelegate(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractItemDelegate {
  pub fn QAbstractItemDelegate_0<T: QAbstractItemDelegate_QAbstractItemDelegate_0>(value: T) -> QAbstractItemDelegate {
    let rsthis = value.QAbstractItemDelegate_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractItemDelegate_QAbstractItemDelegate_0 {
  fn QAbstractItemDelegate_0(self) -> QAbstractItemDelegate;
}
// QAbstractItemDelegate(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractItemDelegate_QAbstractItemDelegate_0 for (usize) {
  fn QAbstractItemDelegate_0(self) -> QAbstractItemDelegate {
    // unsafe{_ZN21QAbstractItemDelegateC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QAbstractItemDelegateC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractItemDelegate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractItemDelegate()

/*

*/
pub fn DeleteQAbstractItemDelegate(this :*mut QAbstractItemDelegate) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QAbstractItemDelegateD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:76
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
This pure abstract function must be reimplemented if you want to provide custom rendering. Use the painter and style option to render the item specified by the item index.

If you reimplement this you must also reimplement sizeHint().
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn paint_0<RetType, T: QAbstractItemDelegate_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_paint_0<RetType> {
  fn paint_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QAbstractItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK21QAbstractItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:80
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QSize sizeHint(const QStyleOptionViewItem &, const QModelIndex &) const

/*
This pure abstract function must be reimplemented if you want to provide custom rendering. The options are specified by option and the model item by index.

If you reimplement this you must also reimplement paint().
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn sizeHint_0<RetType, T: QAbstractItemDelegate_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_sizeHint_0<usize> for (usize,usize) {
  fn sizeHint_0(self , rsthis: & QAbstractItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QAbstractItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWidget * createEditor(QWidget *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Returns the editor to be used for editing the data item with the given index. Note that the index contains information about the model being used. The editor's parent widget is specified by parent, and the item options by option.

The base implementation returns 0. If you want custom editing you will need to reimplement this function.

The returned editor widget should have Qt::StrongFocus; otherwise, QMouseEvents received by the widget will propagate to the view. The view's background will shine through unless the editor paints its own background (e.g., with setAutoFillBackground()).

See also destroyEditor(), setModelData(), and setEditorData().
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn createEditor_0<RetType, T: QAbstractItemDelegate_createEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createEditor_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_createEditor_0<RetType> {
  fn createEditor_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_createEditor_0<usize> for (usize,usize,usize) {
  fn createEditor_0(self , rsthis: & QAbstractItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QAbstractItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void destroyEditor(QWidget *, const QModelIndex &) const

/*
Called when the editor is no longer needed for editing the data item with the given index and should be destroyed. The default behavior is a call to deleteLater on the editor. It is possible e.g. to avoid this delete by reimplementing this function.

This function was introduced in  Qt 5.0.

See also createEditor().
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn destroyEditor_0<RetType, T: QAbstractItemDelegate_destroyEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.destroyEditor_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_destroyEditor_0<RetType> {
  fn destroyEditor_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_destroyEditor_0<(/*void*/)> for (usize,usize) {
  fn destroyEditor_0(self , rsthis: & QAbstractItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK21QAbstractItemDelegate13destroyEditorEP7QWidgetRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:90
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setEditorData(QWidget *, const QModelIndex &) const

/*
Sets the contents of the given editor to the data for the item at the given index. Note that the index contains information about the model being used.

The base implementation does nothing. If you want custom editing you will need to reimplement this function.

See also setModelData().
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn setEditorData_0<RetType, T: QAbstractItemDelegate_setEditorData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEditorData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_setEditorData_0<RetType> {
  fn setEditorData_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_setEditorData_0<(/*void*/)> for (usize,usize) {
  fn setEditorData_0(self , rsthis: & QAbstractItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK21QAbstractItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:92
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setModelData(QWidget *, QAbstractItemModel *, const QModelIndex &) const

/*
Sets the data for the item at the given index in the model to the contents of the given editor.

The base implementation does nothing. If you want custom editing you will need to reimplement this function.

See also setEditorData().
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn setModelData_0<RetType, T: QAbstractItemDelegate_setModelData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModelData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_setModelData_0<RetType> {
  fn setModelData_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_setModelData_0<(/*void*/)> for (usize,usize,usize) {
  fn setModelData_0(self , rsthis: & QAbstractItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK21QAbstractItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void updateEditorGeometry(QWidget *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Updates the geometry of the editor for the item with the given index, according to the rectangle specified in the option. If the item has an internal layout, the editor will be laid out accordingly. Note that the index contains information about the model being used.

The base implementation does nothing. If you want custom editing you must reimplement this function.
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn updateEditorGeometry_0<RetType, T: QAbstractItemDelegate_updateEditorGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_updateEditorGeometry_0<RetType> {
  fn updateEditorGeometry_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_updateEditorGeometry_0<(/*void*/)> for (usize,usize,usize) {
  fn updateEditorGeometry_0(self , rsthis: & QAbstractItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK21QAbstractItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:101
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool editorEvent(QEvent *, QAbstractItemModel *, const QStyleOptionViewItem &, const QModelIndex &)

/*
When editing of an item starts, this function is called with the event that triggered the editing, the model, the index of the item, and the option used for rendering the item.

Mouse events are sent to editorEvent() even if they don't start editing of the item. This can, for instance, be useful if you wish to open a context menu when the right mouse button is pressed on an item.

The base implementation returns false (indicating that it has not handled the event).
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn editorEvent_0<RetType, T: QAbstractItemDelegate_editorEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editorEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_editorEvent_0<RetType> {
  fn editorEvent_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_editorEvent_0<bool> for (usize,usize,usize,usize) {
  fn editorEvent_0(self , rsthis: & QAbstractItemDelegate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QAbstractItemDelegate11editorEventEP6QEventP18QAbstractItemModelRK20QStyleOptionViewItemRK11QModelIndex", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:106
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString elidedText(const QFontMetrics &, int, Qt::TextElideMode, const QString &)

/*

*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn elidedText_0<RetType, T: QAbstractItemDelegate_elidedText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.elidedText_0();
    // return 1;
  }
}
pub trait QAbstractItemDelegate_elidedText_0<RetType> {
  fn elidedText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_elidedText_0<usize> for (usize,i32,i32,usize) {
  fn elidedText_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QAbstractItemDelegate10elidedTextERK12QFontMetricsiN2Qt13TextElideModeERK7QString", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:109
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool helpEvent(QHelpEvent *, QAbstractItemView *, const QStyleOptionViewItem &, const QModelIndex &)

/*
Whenever a help event occurs, this function is called with the event view option and the index that corresponds to the item where the event occurs.

Returns true if the delegate can handle the event; otherwise returns false. A return value of true indicates that the data obtained using the index had the required role.

For QEvent::ToolTip and QEvent::WhatsThis events that were handled successfully, the relevant popup may be shown depending on the user's system configuration.

This function was introduced in  Qt 4.3.

See also QHelpEvent.
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn helpEvent_0<RetType, T: QAbstractItemDelegate_helpEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.helpEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_helpEvent_0<RetType> {
  fn helpEvent_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_helpEvent_0<bool> for (usize,usize,usize,usize) {
  fn helpEvent_0(self , rsthis: & QAbstractItemDelegate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QAbstractItemDelegate9helpEventEP10QHelpEventP17QAbstractItemViewRK20QStyleOptionViewItemRK11QModelIndex", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void commitData(QWidget *)

/*
This signal must be emitted when the editor widget has completed editing the data, and wants to write it back into the model.
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn commitData_0<RetType, T: QAbstractItemDelegate_commitData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commitData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_commitData_0<RetType> {
  fn commitData_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_commitData_0<(/*void*/)> for (usize) {
  fn commitData_0(self , rsthis: & QAbstractItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QAbstractItemDelegate10commitDataEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closeEditor(QWidget *, QAbstractItemDelegate::EndEditHint)

/*
This signal is emitted when the user has finished editing an item using the specified editor.

The hint provides a way for the delegate to influence how the model and view behave after editing is completed. It indicates to these components what action should be performed next to provide a comfortable editing experience for the user. For example, if EditNextItem is specified, the view should use a delegate to open an editor on the next item in the model.

See also EndEditHint.
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn closeEditor_0<RetType, T: QAbstractItemDelegate_closeEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEditor_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_closeEditor_0<RetType> {
  fn closeEditor_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_closeEditor_0<(/*void*/)> for (usize,i32) {
  fn closeEditor_0(self , rsthis: & QAbstractItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QAbstractItemDelegate11closeEditorEP7QWidgetNS_11EndEditHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemdelegate.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sizeHintChanged(const QModelIndex &)

/*
This signal must be emitted when the sizeHint() of index changed.

Views automatically connect to this signal and relayout items as necessary.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QAbstractItemDelegate {
  pub fn sizeHintChanged_0<RetType, T: QAbstractItemDelegate_sizeHintChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHintChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractItemDelegate_sizeHintChanged_0<RetType> {
  fn sizeHintChanged_0(self , rsthis: & QAbstractItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemDelegate_sizeHintChanged_0<(/*void*/)> for (usize) {
  fn sizeHintChanged_0(self , rsthis: & QAbstractItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the different hints that the delegate can give to the model and view components to make editing data in a model a comfortable experience for the user.



These hints let the delegate influence the behavior of the view:



Note that custom views may interpret the concepts of next and previous differently.

The following hints are most useful when models are used that cache data, such as those that manipulate data locally in order to increase performance or conserve network bandwidth.



Although models and views should respond to these hints in appropriate ways, custom components may ignore any or all of them if they are not relevant.

*/
pub type QAbstractItemDelegate__EndEditHint = i32;
// There is no recommended action to be performed.
pub const QAbstractItemDelegate__NoHint :QAbstractItemDelegate__EndEditHint = 0;
// The view should use the delegate to open an editor on the next item in the view.
pub const QAbstractItemDelegate__EditNextItem :QAbstractItemDelegate__EndEditHint = 1;
// The view should use the delegate to open an editor on the previous item in the view.
pub const QAbstractItemDelegate__EditPreviousItem :QAbstractItemDelegate__EndEditHint = 2;
// If the model caches data, it should write out cached data to the underlying data store.
pub const QAbstractItemDelegate__SubmitModelCache :QAbstractItemDelegate__EndEditHint = 3;
// If the model caches data, it should discard cached data and replace it with data from the underlying data store.
pub const QAbstractItemDelegate__RevertModelCache :QAbstractItemDelegate__EndEditHint = 4;
pub fn QAbstractItemDelegate_EndEditHintItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemDelegate", val);
}
pub fn QAbstractItemDelegate_EndEditHintItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemDelegate
  //return nilthis.EndEditHintItemName(val);
  return QAbstractItemDelegate_EndEditHintItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
