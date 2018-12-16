

// mod ::widgets::QStyledItemDelegate
// package qtwidgets
// /usr/include/qt/QtWidgets/qstyleditemdelegate.h
// #include <qstyleditemdelegate.h>
// #include <QtWidgets>

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

// void initStyleOption(QStyleOptionViewItem *, const QModelIndex &)
// func (this *QStyledItemDelegate) InheritInitStyleOption(f func(option *QStyleOptionViewItem/*777 QStyleOptionViewItem **/, index *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QStyledItemDelegate) InheritEventFilter(f func(object *qtcore.QObject/*777 QObject **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// bool editorEvent(QEvent *, QAbstractItemModel *, const QStyleOptionViewItem &, const QModelIndex &)
// func (this *QStyledItemDelegate) InheritEditorEvent(f func(event *qtcore.QEvent/*777 QEvent **/, model *qtcore.QAbstractItemModel/*777 QAbstractItemModel **/, option *QStyleOptionViewItem, index *qtcore.QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "editorEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QStyledItemDelegate)=16
pub struct QStyledItemDelegate {
  qbase: QAbstractItemDelegate,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyledItemDelegate_ITF interface {
//    QAbstractItemDelegate_ITF
//    QStyledItemDelegate_PTR() *QStyledItemDelegate
//}
//func (ptr *QStyledItemDelegate) QStyledItemDelegate_PTR() *QStyledItemDelegate { return ptr }

impl /*struct*/ QStyledItemDelegate {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyledItemDelegate {
    return QStyledItemDelegate{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyledItemDelegate {
//  type Target = QStyledItemDelegateBASE;
//
//  fn deref(&self) -> &QStyledItemDelegateBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyledItemDelegateBASE> for QStyledItemDelegate {
//  fn as_ref(& self) -> & QStyledItemDelegateBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStyledItemDelegate {
  pub fn metaObject_0<RetType, T: QStyledItemDelegate_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStyledItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyledItemDelegate(QObject *)

/*
Constructs an item delegate with the given parent.
*/
// QStyledItemDelegate(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QStyledItemDelegate {
  pub fn QStyledItemDelegate_0<T: QStyledItemDelegate_QStyledItemDelegate_0>(value: T) -> QStyledItemDelegate {
    let rsthis = value.QStyledItemDelegate_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyledItemDelegate_QStyledItemDelegate_0 {
  fn QStyledItemDelegate_0(self) -> QStyledItemDelegate;
}
// QStyledItemDelegate(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyledItemDelegate_QStyledItemDelegate_0 for (usize) {
  fn QStyledItemDelegate_0(self) -> QStyledItemDelegate {
    // unsafe{_ZN19QStyledItemDelegateC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QStyledItemDelegateC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyledItemDelegate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStyledItemDelegate()

/*

*/
pub fn DeleteQStyledItemDelegate(this :*mut QStyledItemDelegate) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QStyledItemDelegateD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::paint().

Renders the delegate using the given painter and style option for the item specified by index.

This function paints the item using the view's QStyle.

When reimplementing paint in a subclass. Use the initStyleOption() to set up the option in the same way as the QStyledItemDelegate.

Whenever possible, use the option while painting. Especially its rect variable to decide where to draw and its state to determine if it is enabled or selected.

After painting, you should ensure that the painter is returned to the state it was supplied in when this function was called. For example, it may be useful to call QPainter::save() before painting and QPainter::restore() afterwards.

See also QItemDelegate::paint(), QStyle::drawControl(), and QStyle::CE_ItemViewItem.
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn paint_0<RetType, T: QStyledItemDelegate_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_paint_0<RetType> {
  fn paint_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QStyledItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint(const QStyleOptionViewItem &, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::sizeHint().

Returns the size needed by the delegate to display the item specified by index, taking into account the style information provided by option.

This function uses the view's QStyle to determine the size of the item.

See also QStyle::sizeFromContents() and QStyle::CT_ItemViewItem.
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn sizeHint_0<RetType, T: QStyledItemDelegate_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_sizeHint_0<usize> for (usize,usize) {
  fn sizeHint_0(self , rsthis: & QStyledItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWidget * createEditor(QWidget *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::createEditor().

Returns the widget used to edit the item specified by index for editing. The parent widget and style option are used to control how the editor widget appears.

See also QAbstractItemDelegate::createEditor().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn createEditor_0<RetType, T: QStyledItemDelegate_createEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createEditor_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_createEditor_0<RetType> {
  fn createEditor_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_createEditor_0<usize> for (usize,usize,usize) {
  fn createEditor_0(self , rsthis: & QStyledItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setEditorData(QWidget *, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::setEditorData().

Sets the data to be displayed and edited by the editor from the data model item specified by the model index.

The default implementation stores the data in the editor widget's user property.

See also QMetaProperty::isUser().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn setEditorData_0<RetType, T: QStyledItemDelegate_setEditorData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEditorData_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_setEditorData_0<RetType> {
  fn setEditorData_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_setEditorData_0<(/*void*/)> for (usize,usize) {
  fn setEditorData_0(self , rsthis: & QStyledItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setModelData(QWidget *, QAbstractItemModel *, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::setModelData().

Gets data from the editor widget and stores it in the specified model at the item index.

The default implementation gets the value to be stored in the data model from the editor widget's user property.

See also QMetaProperty::isUser().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn setModelData_0<RetType, T: QStyledItemDelegate_setModelData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModelData_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_setModelData_0<RetType> {
  fn setModelData_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_setModelData_0<(/*void*/)> for (usize,usize,usize) {
  fn setModelData_0(self , rsthis: & QStyledItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void updateEditorGeometry(QWidget *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::updateEditorGeometry().

Updates the editor for the item specified by index according to the style option given.
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn updateEditorGeometry_0<RetType, T: QStyledItemDelegate_updateEditorGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_updateEditorGeometry_0<RetType> {
  fn updateEditorGeometry_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_updateEditorGeometry_0<(/*void*/)> for (usize,usize,usize) {
  fn updateEditorGeometry_0(self , rsthis: & QStyledItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:85
// index:0
// Public Visibility=Default Availability=Available
// [8] QItemEditorFactory * itemEditorFactory() const

/*
Returns the editor factory used by the item delegate. If no editor factory is set, the function will return null.

See also setItemEditorFactory().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn itemEditorFactory_0<RetType, T: QStyledItemDelegate_itemEditorFactory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemEditorFactory_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_itemEditorFactory_0<RetType> {
  fn itemEditorFactory_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_itemEditorFactory_0<usize> for () {
  fn itemEditorFactory_0(self , rsthis: & QStyledItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate17itemEditorFactoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemEditorFactory(QItemEditorFactory *)

/*
Sets the editor factory to be used by the item delegate to be the factory specified. If no editor factory is set, the item delegate will use the default editor factory.

See also itemEditorFactory().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn setItemEditorFactory_0<RetType, T: QStyledItemDelegate_setItemEditorFactory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemEditorFactory_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_setItemEditorFactory_0<RetType> {
  fn setItemEditorFactory_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_setItemEditorFactory_0<(/*void*/)> for (usize) {
  fn setItemEditorFactory_0(self , rsthis: & QStyledItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString displayText(const QVariant &, const QLocale &) const

/*
This function returns the string that the delegate will use to display the Qt::DisplayRole of the model in locale. value is the value of the Qt::DisplayRole provided by the model.

The default implementation uses the QLocale::toString to convert value into a QString.

This function is not called for empty model indices, i.e., indices for which the model returns an invalid QVariant.

See also QAbstractItemModel::data().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn displayText_0<RetType, T: QStyledItemDelegate_displayText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.displayText_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_displayText_0<RetType> {
  fn displayText_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_displayText_0<usize> for (usize,usize) {
  fn displayText_0(self , rsthis: & QStyledItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:91
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionViewItem *, const QModelIndex &) const

/*
Initialize option with the values using the index index. This method is useful for subclasses when they need a QStyleOptionViewItem, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn initStyleOption_0<RetType, T: QStyledItemDelegate_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_initStyleOption_0<(/*void*/)> for (usize,usize) {
  fn initStyleOption_0(self , rsthis: & QStyledItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK19QStyledItemDelegate15initStyleOptionEP20QStyleOptionViewItemRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:94
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().

Returns true if the given editor is a valid QWidget and the given event is handled; otherwise returns false. The following key press events are handled by default:


Tab
Backtab
Enter
Return
Esc


If the editor's type is QTextEdit or QPlainTextEdit then Enter and Return keys are not handled.

In the case of Tab, Backtab, Enter and Return key press events, the editor's data is comitted to the model and the editor is closed. If the event is a Tab key press the view will open an editor on the next item in the view. Likewise, if the event is a Backtab key press the view will open an editor on the previous item in the view.

If the event is a Esc key press event, the editor is closed without committing its data.

See also commitData() and closeEditor().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn eventFilter_0<RetType, T: QStyledItemDelegate_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QStyledItemDelegate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QStyledItemDelegate11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyleditemdelegate.h:95
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool editorEvent(QEvent *, QAbstractItemModel *, const QStyleOptionViewItem &, const QModelIndex &)

/*
Reimplemented from QAbstractItemDelegate::editorEvent().
*/
impl /*struct*/ QStyledItemDelegate {
  pub fn editorEvent_0<RetType, T: QStyledItemDelegate_editorEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editorEvent_0(self);
    // return 1;
  }
}
pub trait QStyledItemDelegate_editorEvent_0<RetType> {
  fn editorEvent_0(self , rsthis: & QStyledItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QStyledItemDelegate_editorEvent_0<bool> for (usize,usize,usize,usize) {
  fn editorEvent_0(self , rsthis: & QStyledItemDelegate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QStyledItemDelegate11editorEventEP6QEventP18QAbstractItemModelRK20QStyleOptionViewItemRK11QModelIndex", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
