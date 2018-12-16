

// mod ::widgets::QItemDelegate
// package qtwidgets
// /usr/include/qt/QtWidgets/qitemdelegate.h
// #include <qitemdelegate.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 60
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

// void drawDisplay(QPainter *, const QStyleOptionViewItem &, const QRect &, const QString &)
// func (this *QItemDelegate) InheritDrawDisplay(f func(painter *qtgui.QPainter/*777 QPainter **/, option *QStyleOptionViewItem, rect *qtcore.QRect, text string) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawDisplay", f)
// }

// void drawDecoration(QPainter *, const QStyleOptionViewItem &, const QRect &, const QPixmap &)
// func (this *QItemDelegate) InheritDrawDecoration(f func(painter *qtgui.QPainter/*777 QPainter **/, option *QStyleOptionViewItem, rect *qtcore.QRect, pixmap *qtgui.QPixmap) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawDecoration", f)
// }

// void drawFocus(QPainter *, const QStyleOptionViewItem &, const QRect &)
// func (this *QItemDelegate) InheritDrawFocus(f func(painter *qtgui.QPainter/*777 QPainter **/, option *QStyleOptionViewItem, rect *qtcore.QRect) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawFocus", f)
// }

// void drawCheck(QPainter *, const QStyleOptionViewItem &, const QRect &, Qt::CheckState)
// func (this *QItemDelegate) InheritDrawCheck(f func(painter *qtgui.QPainter/*777 QPainter **/, option *QStyleOptionViewItem, rect *qtcore.QRect, state int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawCheck", f)
// }

// void drawBackground(QPainter *, const QStyleOptionViewItem &, const QModelIndex &)
// func (this *QItemDelegate) InheritDrawBackground(f func(painter *qtgui.QPainter/*777 QPainter **/, option *QStyleOptionViewItem, index *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawBackground", f)
// }

// void doLayout(const QStyleOptionViewItem &, QRect *, QRect *, QRect *, bool)
// func (this *QItemDelegate) InheritDoLayout(f func(option *QStyleOptionViewItem, checkRect *qtcore.QRect/*777 QRect **/, iconRect *qtcore.QRect/*777 QRect **/, textRect *qtcore.QRect/*777 QRect **/, hint bool) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "doLayout", f)
// }

// QRect rect(const QStyleOptionViewItem &, const QModelIndex &, int)
// func (this *QItemDelegate) InheritRect(f func(option *QStyleOptionViewItem, index *qtcore.QModelIndex, role int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "rect", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QItemDelegate) InheritEventFilter(f func(object *qtcore.QObject/*777 QObject **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// bool editorEvent(QEvent *, QAbstractItemModel *, const QStyleOptionViewItem &, const QModelIndex &)
// func (this *QItemDelegate) InheritEditorEvent(f func(event *qtcore.QEvent/*777 QEvent **/, model *qtcore.QAbstractItemModel/*777 QAbstractItemModel **/, option *QStyleOptionViewItem, index *qtcore.QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "editorEvent", f)
// }

// QStyleOptionViewItem setOptions(const QModelIndex &, const QStyleOptionViewItem &)
// func (this *QItemDelegate) InheritSetOptions(f func(index *qtcore.QModelIndex, option *QStyleOptionViewItem) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "setOptions", f)
// }

// QPixmap decoration(const QStyleOptionViewItem &, const QVariant &)
// func (this *QItemDelegate) InheritDecoration(f func(option *QStyleOptionViewItem, variant *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "decoration", f)
// }

// QPixmap * selected(const QPixmap &, const QPalette &, bool)
// func (this *QItemDelegate) InheritSelected(f func(pixmap *qtgui.QPixmap, palette *qtgui.QPalette, enabled bool) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "selected", f)
// }

// QRect doCheck(const QStyleOptionViewItem &, const QRect &, const QVariant &)
// func (this *QItemDelegate) InheritDoCheck(f func(option *QStyleOptionViewItem, bounding *qtcore.QRect, variant *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "doCheck", f)
// }

// QRect textRectangle(QPainter *, const QRect &, const QFont &, const QString &)
// func (this *QItemDelegate) InheritTextRectangle(f func(painter *qtgui.QPainter/*777 QPainter **/, rect *qtcore.QRect, font *qtgui.QFont, text string) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "textRectangle", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QItemDelegate)=16
pub struct QItemDelegate {
  qbase: QAbstractItemDelegate,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QItemDelegate_ITF interface {
//    QAbstractItemDelegate_ITF
//    QItemDelegate_PTR() *QItemDelegate
//}
//func (ptr *QItemDelegate) QItemDelegate_PTR() *QItemDelegate { return ptr }

impl /*struct*/ QItemDelegate {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QItemDelegate {
    return QItemDelegate{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QItemDelegate {
//  type Target = QItemDelegateBASE;
//
//  fn deref(&self) -> &QItemDelegateBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QItemDelegateBASE> for QItemDelegate {
//  fn as_ref(& self) -> & QItemDelegateBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qitemdelegate.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn metaObject_0<RetType, T: QItemDelegate_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QItemDelegate(QObject *)

/*
Constructs an item delegate with the given parent.
*/
// QItemDelegate(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QItemDelegate {
  pub fn QItemDelegate_0<T: QItemDelegate_QItemDelegate_0>(value: T) -> QItemDelegate {
    let rsthis = value.QItemDelegate_0();
    return rsthis;
    // return 1;
  }
}

pub trait QItemDelegate_QItemDelegate_0 {
  fn QItemDelegate_0(self) -> QItemDelegate;
}
// QItemDelegate(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemDelegate_QItemDelegate_0 for (usize) {
  fn QItemDelegate_0(self) -> QItemDelegate {
    // unsafe{_ZN13QItemDelegateC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QItemDelegateC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemDelegate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QItemDelegate()

/*

*/
pub fn DeleteQItemDelegate(this :*mut QItemDelegate) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QItemDelegateD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qitemdelegate.h:65
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasClipping() const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn hasClipping_0<RetType, T: QItemDelegate_hasClipping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasClipping_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_hasClipping_0<RetType> {
  fn hasClipping_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_hasClipping_0<bool> for () {
  fn hasClipping_0(self , rsthis: & QItemDelegate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate11hasClippingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setClipping(bool)

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn setClipping_0<RetType, T: QItemDelegate_setClipping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClipping_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_setClipping_0<RetType> {
  fn setClipping_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_setClipping_0<(/*void*/)> for (bool) {
  fn setClipping_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QItemDelegate11setClippingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::paint().

Renders the delegate using the given painter and style option for the item specified by index.

When reimplementing this function in a subclass, you should update the area held by the option's rect variable, using the option's state variable to determine the state of the item to be displayed, and adjust the way it is painted accordingly.

For example, a selected item may need to be displayed differently to unselected items, as shown in the following code:


      if (option.state & QStyle::State_Selected)
          painter->fillRect(option.rect, option.palette.highlight());

      int size = qMin(option.rect.width(), option.rect.height());
      int brightness = index.model()->data(index, Qt::DisplayRole).toInt();
      double radius = (size / 2.0) - (brightness / 255.0 * size / 2.0);
      if (radius == 0.0)
          return;

      painter->save();
      painter->setRenderHint(QPainter::Antialiasing, true);
      painter->setPen(Qt::NoPen);
      if (option.state & QStyle::State_Selected)
          painter->setBrush(option.palette.highlightedText());
      else
      ...



After painting, you should ensure that the painter is returned to its the state it was supplied in when this function was called. For example, it may be useful to call QPainter::save() before painting and QPainter::restore() afterwards.

See also QStyle::State.
*/
impl /*struct*/ QItemDelegate {
  pub fn paint_0<RetType, T: QItemDelegate_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_paint_0<RetType> {
  fn paint_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint(const QStyleOptionViewItem &, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::sizeHint().

Returns the size needed by the delegate to display the item specified by index, taking into account the style information provided by option.

When reimplementing this function, note that in case of text items, QItemDelegate adds a margin (i.e. 2 * QStyle::PM_FocusFrameHMargin) to the length of the text.
*/
impl /*struct*/ QItemDelegate {
  pub fn sizeHint_0<RetType, T: QItemDelegate_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_sizeHint_0<usize> for (usize,usize) {
  fn sizeHint_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWidget * createEditor(QWidget *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::createEditor().

Returns the widget used to edit the item specified by index for editing. The parent widget and style option are used to control how the editor widget appears.

See also QAbstractItemDelegate::createEditor().
*/
impl /*struct*/ QItemDelegate {
  pub fn createEditor_0<RetType, T: QItemDelegate_createEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createEditor_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_createEditor_0<RetType> {
  fn createEditor_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_createEditor_0<usize> for (usize,usize,usize) {
  fn createEditor_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setEditorData(QWidget *, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::setEditorData().

Sets the data to be displayed and edited by the editor from the data model item specified by the model index.

The default implementation stores the data in the editor widget's user property.

See also QMetaProperty::isUser().
*/
impl /*struct*/ QItemDelegate {
  pub fn setEditorData_0<RetType, T: QItemDelegate_setEditorData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEditorData_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_setEditorData_0<RetType> {
  fn setEditorData_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_setEditorData_0<(/*void*/)> for (usize,usize) {
  fn setEditorData_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setModelData(QWidget *, QAbstractItemModel *, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::setModelData().

Gets data from the editor widget and stores it in the specified model at the item index.

The default implementation gets the value to be stored in the data model from the editor widget's user property.

See also QMetaProperty::isUser().
*/
impl /*struct*/ QItemDelegate {
  pub fn setModelData_0<RetType, T: QItemDelegate_setModelData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModelData_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_setModelData_0<RetType> {
  fn setModelData_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_setModelData_0<(/*void*/)> for (usize,usize,usize) {
  fn setModelData_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void updateEditorGeometry(QWidget *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Reimplemented from QAbstractItemDelegate::updateEditorGeometry().

Updates the editor for the item specified by index according to the style option given.
*/
impl /*struct*/ QItemDelegate {
  pub fn updateEditorGeometry_0<RetType, T: QItemDelegate_updateEditorGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_updateEditorGeometry_0<RetType> {
  fn updateEditorGeometry_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_updateEditorGeometry_0<(/*void*/)> for (usize,usize,usize) {
  fn updateEditorGeometry_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QItemEditorFactory * itemEditorFactory() const

/*
Returns the editor factory used by the item delegate. If no editor factory is set, the function will return null.

See also setItemEditorFactory().
*/
impl /*struct*/ QItemDelegate {
  pub fn itemEditorFactory_0<RetType, T: QItemDelegate_itemEditorFactory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemEditorFactory_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_itemEditorFactory_0<RetType> {
  fn itemEditorFactory_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_itemEditorFactory_0<usize> for () {
  fn itemEditorFactory_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate17itemEditorFactoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemEditorFactory(QItemEditorFactory *)

/*
Sets the editor factory to be used by the item delegate to be the factory specified. If no editor factory is set, the item delegate will use the default editor factory.

See also itemEditorFactory().
*/
impl /*struct*/ QItemDelegate {
  pub fn setItemEditorFactory_0<RetType, T: QItemDelegate_setItemEditorFactory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemEditorFactory_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_setItemEditorFactory_0<RetType> {
  fn setItemEditorFactory_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_setItemEditorFactory_0<(/*void*/)> for (usize) {
  fn setItemEditorFactory_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:92
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawDisplay(QPainter *, const QStyleOptionViewItem &, const QRect &, const QString &) const

/*
Renders the item view text within the rectangle specified by rect using the given painter and style option.
*/
impl /*struct*/ QItemDelegate {
  pub fn drawDisplay_0<RetType, T: QItemDelegate_drawDisplay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawDisplay_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_drawDisplay_0<RetType> {
  fn drawDisplay_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_drawDisplay_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn drawDisplay_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate11drawDisplayEP8QPainterRK20QStyleOptionViewItemRK5QRectRK7QString", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:94
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawDecoration(QPainter *, const QStyleOptionViewItem &, const QRect &, const QPixmap &) const

/*
Renders the decoration pixmap within the rectangle specified by rect using the given painter and style option.
*/
impl /*struct*/ QItemDelegate {
  pub fn drawDecoration_0<RetType, T: QItemDelegate_drawDecoration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawDecoration_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_drawDecoration_0<RetType> {
  fn drawDecoration_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_drawDecoration_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn drawDecoration_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate14drawDecorationEP8QPainterRK20QStyleOptionViewItemRK5QRectRK7QPixmap", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:96
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawFocus(QPainter *, const QStyleOptionViewItem &, const QRect &) const

/*
Renders the region within the rectangle specified by rect, indicating that it has the focus, using the given painter and style option.
*/
impl /*struct*/ QItemDelegate {
  pub fn drawFocus_0<RetType, T: QItemDelegate_drawFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawFocus_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_drawFocus_0<RetType> {
  fn drawFocus_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_drawFocus_0<(/*void*/)> for (usize,usize,usize) {
  fn drawFocus_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate9drawFocusEP8QPainterRK20QStyleOptionViewItemRK5QRect", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:98
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawCheck(QPainter *, const QStyleOptionViewItem &, const QRect &, Qt::CheckState) const

/*
Renders a check indicator within the rectangle specified by rect, using the given painter and style option, using the given state.
*/
impl /*struct*/ QItemDelegate {
  pub fn drawCheck_0<RetType, T: QItemDelegate_drawCheck_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawCheck_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_drawCheck_0<RetType> {
  fn drawCheck_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_drawCheck_0<(/*void*/)> for (usize,usize,usize,i32) {
  fn drawCheck_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate9drawCheckEP8QPainterRK20QStyleOptionViewItemRK5QRectN2Qt10CheckStateE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:100
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void drawBackground(QPainter *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Renders the item background for the given index, using the given painter and style option.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QItemDelegate {
  pub fn drawBackground_0<RetType, T: QItemDelegate_drawBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawBackground_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_drawBackground_0<RetType> {
  fn drawBackground_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_drawBackground_0<(/*void*/)> for (usize,usize,usize) {
  fn drawBackground_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate14drawBackgroundEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:103
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void doLayout(const QStyleOptionViewItem &, QRect *, QRect *, QRect *, bool) const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn doLayout_0<RetType, T: QItemDelegate_doLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doLayout_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_doLayout_0<RetType> {
  fn doLayout_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_doLayout_0<(/*void*/)> for (usize,usize,usize,usize,bool) {
  fn doLayout_0(self , rsthis: & QItemDelegate) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZNK13QItemDelegate8doLayoutERK20QStyleOptionViewItemP5QRectS4_S4_b", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:106
// index:0
// Protected Visibility=Default Availability=Available
// [16] QRect rect(const QStyleOptionViewItem &, const QModelIndex &, int) const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn rect_0<RetType, T: QItemDelegate_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_rect_0<RetType> {
  fn rect_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_rect_0<usize> for (usize,usize,i32) {
  fn rect_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate4rectERK20QStyleOptionViewItemRK11QModelIndexi", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:108
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


In the case of Tab, Backtab, Enter and Return key press events, the editor's data is comitted to the model and the editor is closed. If the event is a Tab key press the view will open an editor on the next item in the view. Likewise, if the event is a Backtab key press the view will open an editor on the previous item in the view.

If the event is a Esc key press event, the editor is closed without committing its data.

See also commitData() and closeEditor().
*/
impl /*struct*/ QItemDelegate {
  pub fn eventFilter_0<RetType, T: QItemDelegate_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QItemDelegate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QItemDelegate11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:109
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool editorEvent(QEvent *, QAbstractItemModel *, const QStyleOptionViewItem &, const QModelIndex &)

/*
Reimplemented from QAbstractItemDelegate::editorEvent().
*/
impl /*struct*/ QItemDelegate {
  pub fn editorEvent_0<RetType, T: QItemDelegate_editorEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editorEvent_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_editorEvent_0<RetType> {
  fn editorEvent_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_editorEvent_0<bool> for (usize,usize,usize,usize) {
  fn editorEvent_0(self , rsthis: & QItemDelegate) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QItemDelegate11editorEventEP6QEventP18QAbstractItemModelRK20QStyleOptionViewItemRK11QModelIndex", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:112
// index:0
// Protected Visibility=Default Availability=Available
// [192] QStyleOptionViewItem setOptions(const QModelIndex &, const QStyleOptionViewItem &) const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn setOptions_0<RetType, T: QItemDelegate_setOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptions_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_setOptions_0<RetType> {
  fn setOptions_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_setOptions_0<usize> for (usize,usize) {
  fn setOptions_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate10setOptionsERK11QModelIndexRK20QStyleOptionViewItem", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:115
// index:0
// Protected Visibility=Default Availability=Available
// [32] QPixmap decoration(const QStyleOptionViewItem &, const QVariant &) const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn decoration_0<RetType, T: QItemDelegate_decoration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decoration_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_decoration_0<RetType> {
  fn decoration_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_decoration_0<usize> for (usize,usize) {
  fn decoration_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate10decorationERK20QStyleOptionViewItemRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:116
// index:0
// Protected Visibility=Default Availability=Available
// [8] QPixmap * selected(const QPixmap &, const QPalette &, bool) const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn selected_0<RetType, T: QItemDelegate_selected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selected_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_selected_0<RetType> {
  fn selected_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_selected_0<usize> for (usize,usize,bool) {
  fn selected_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate8selectedERK7QPixmapRK8QPaletteb", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:118
// index:0
// Protected Visibility=Default Availability=Available
// [16] QRect doCheck(const QStyleOptionViewItem &, const QRect &, const QVariant &) const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn doCheck_0<RetType, T: QItemDelegate_doCheck_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doCheck_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_doCheck_0<RetType> {
  fn doCheck_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_doCheck_0<usize> for (usize,usize,usize) {
  fn doCheck_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate7doCheckERK20QStyleOptionViewItemRK5QRectRK8QVariant", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemdelegate.h:120
// index:0
// Protected Visibility=Default Availability=Available
// [16] QRect textRectangle(QPainter *, const QRect &, const QFont &, const QString &) const

/*

*/
impl /*struct*/ QItemDelegate {
  pub fn textRectangle_0<RetType, T: QItemDelegate_textRectangle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textRectangle_0(self);
    // return 1;
  }
}
pub trait QItemDelegate_textRectangle_0<RetType> {
  fn textRectangle_0(self , rsthis: & QItemDelegate) -> RetType;
}
impl<'a> /*trait*/ QItemDelegate_textRectangle_0<usize> for (usize,usize,usize,usize) {
  fn textRectangle_0(self , rsthis: & QItemDelegate) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QItemDelegate13textRectangleEP8QPainterRK5QRectRK5QFontRK7QString", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
