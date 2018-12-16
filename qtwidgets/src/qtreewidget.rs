

// mod ::widgets::QTreeWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qtreewidget.h
// #include <qtreewidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 69
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

// bool event(QEvent *)
// func (this *QTreeWidget) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// QStringList mimeTypes()
// func (this *QTreeWidget) InheritMimeTypes(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "mimeTypes", f)
// }

// bool dropMimeData(QTreeWidgetItem *, int, const QMimeData *, Qt::DropAction)
// func (this *QTreeWidget) InheritDropMimeData(f func(parent *QTreeWidgetItem/*777 QTreeWidgetItem **/, index int, data *qtcore.QMimeData/*777 const QMimeData **/, action int) bool) {
//  qtrt.SetAllInheritCallback(this, "dropMimeData", f)
// }

// Qt::DropActions supportedDropActions()
// func (this *QTreeWidget) InheritSupportedDropActions(f func() int) {
//  qtrt.SetAllInheritCallback(this, "supportedDropActions", f)
// }

// QModelIndex indexFromItem(const QTreeWidgetItem *, int)
// func (this *QTreeWidget) InheritIndexFromItem(f func(item *QTreeWidgetItem/*777 const QTreeWidgetItem **/, column int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "indexFromItem", f)
// }

// QTreeWidgetItem * itemFromIndex(const QModelIndex &)
// func (this *QTreeWidget) InheritItemFromIndex(f func(index *qtcore.QModelIndex) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "itemFromIndex", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QTreeWidget) InheritDropEvent(f func(event *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTreeWidget)=48
pub struct QTreeWidget {
  qbase: QTreeView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTreeWidget_ITF interface {
//    QTreeView_ITF
//    QTreeWidget_PTR() *QTreeWidget
//}
//func (ptr *QTreeWidget) QTreeWidget_PTR() *QTreeWidget { return ptr }

impl /*struct*/ QTreeWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTreeWidget {
    return QTreeWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTreeWidget {
//  type Target = QTreeWidgetBASE;
//
//  fn deref(&self) -> &QTreeWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTreeWidgetBASE> for QTreeWidget {
//  fn as_ref(& self) -> & QTreeWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtreewidget.h:257
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn metaObject_0<RetType, T: QTreeWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:264
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidget(QWidget *)

/*
Constructs a tree widget with the given parent.
*/
// QTreeWidget(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidget {
  pub fn QTreeWidget_0<T: QTreeWidget_QTreeWidget_0>(value: T) -> QTreeWidget {
    let rsthis = value.QTreeWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidget_QTreeWidget_0 {
  fn QTreeWidget_0(self) -> QTreeWidget;
}
// QTreeWidget(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidget_QTreeWidget_0 for (usize) {
  fn QTreeWidget_0(self) -> QTreeWidget {
    // unsafe{_ZN11QTreeWidgetC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTreeWidgetC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:265
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTreeWidget()

/*

*/
pub fn DeleteQTreeWidget(this :*mut QTreeWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTreeWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtreewidget.h:267
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnCount() const

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn columnCount_0<RetType, T: QTreeWidget_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnCount(int)

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn setColumnCount_0<RetType, T: QTreeWidget_setColumnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setColumnCount_0<RetType> {
  fn setColumnCount_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setColumnCount_0<(/*void*/)> for (i32) {
  fn setColumnCount_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget14setColumnCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:270
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * invisibleRootItem() const

/*
Returns the tree widget's invisible root item.

The invisible root item provides access to the tree widget's top-level items through the QTreeWidgetItem API, making it possible to write functions that can treat top-level items and their children in a uniform way; for example, recursive functions.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTreeWidget {
  pub fn invisibleRootItem_0<RetType, T: QTreeWidget_invisibleRootItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invisibleRootItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_invisibleRootItem_0<RetType> {
  fn invisibleRootItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_invisibleRootItem_0<usize> for () {
  fn invisibleRootItem_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget17invisibleRootItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:271
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * topLevelItem(int) const

/*
Returns the top level item at the given index, or 0 if the item does not exist.

See also topLevelItemCount() and insertTopLevelItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn topLevelItem_0<RetType, T: QTreeWidget_topLevelItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLevelItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_topLevelItem_0<RetType> {
  fn topLevelItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_topLevelItem_0<usize> for (i32) {
  fn topLevelItem_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget12topLevelItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:272
// index:0
// Public Visibility=Default Availability=Available
// [4] int topLevelItemCount() const

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn topLevelItemCount_0<RetType, T: QTreeWidget_topLevelItemCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLevelItemCount_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_topLevelItemCount_0<RetType> {
  fn topLevelItemCount_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_topLevelItemCount_0<i32> for () {
  fn topLevelItemCount_0(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget17topLevelItemCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:273
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertTopLevelItem(int, QTreeWidgetItem *)

/*
Inserts the item at index in the top level in the view.

If the item has already been inserted somewhere else it won't be inserted.

See also addTopLevelItem() and columnCount().
*/
impl /*struct*/ QTreeWidget {
  pub fn insertTopLevelItem_0<RetType, T: QTreeWidget_insertTopLevelItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertTopLevelItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_insertTopLevelItem_0<RetType> {
  fn insertTopLevelItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_insertTopLevelItem_0<(/*void*/)> for (i32,usize) {
  fn insertTopLevelItem_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:274
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addTopLevelItem(QTreeWidgetItem *)

/*
Appends the item as a top-level item in the widget.

This function was introduced in  Qt 4.1.

See also insertTopLevelItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn addTopLevelItem_0<RetType, T: QTreeWidget_addTopLevelItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTopLevelItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_addTopLevelItem_0<RetType> {
  fn addTopLevelItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_addTopLevelItem_0<(/*void*/)> for (usize) {
  fn addTopLevelItem_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:275
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * takeTopLevelItem(int)

/*
Removes the top-level item at the given index in the tree and returns it, otherwise returns 0;

See also insertTopLevelItem(), topLevelItem(), and topLevelItemCount().
*/
impl /*struct*/ QTreeWidget {
  pub fn takeTopLevelItem_0<RetType, T: QTreeWidget_takeTopLevelItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeTopLevelItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_takeTopLevelItem_0<RetType> {
  fn takeTopLevelItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_takeTopLevelItem_0<usize> for (i32) {
  fn takeTopLevelItem_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTreeWidget16takeTopLevelItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:276
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfTopLevelItem(QTreeWidgetItem *) const

/*
Returns the index of the given top-level item, or -1 if the item cannot be found.

See also sortItems() and topLevelItemCount().
*/
impl /*struct*/ QTreeWidget {
  pub fn indexOfTopLevelItem_0<RetType, T: QTreeWidget_indexOfTopLevelItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfTopLevelItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_indexOfTopLevelItem_0<RetType> {
  fn indexOfTopLevelItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_indexOfTopLevelItem_0<i32> for (usize) {
  fn indexOfTopLevelItem_0(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:281
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * headerItem() const

/*
Returns the item used for the tree widget's header.

See also setHeaderItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn headerItem_0<RetType, T: QTreeWidget_headerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_headerItem_0<RetType> {
  fn headerItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_headerItem_0<usize> for () {
  fn headerItem_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget10headerItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:282
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHeaderItem(QTreeWidgetItem *)

/*
Sets the header item for the tree widget. The label for each column in the header is supplied by the corresponding label in the item.

The tree widget takes ownership of the item.

See also headerItem() and setHeaderLabels().
*/
impl /*struct*/ QTreeWidget {
  pub fn setHeaderItem_0<RetType, T: QTreeWidget_setHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setHeaderItem_0<RetType> {
  fn setHeaderItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setHeaderItem_0<(/*void*/)> for (usize) {
  fn setHeaderItem_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:283
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHeaderLabels(const QStringList &)

/*
Adds a column in the header for each item in the labels list, and sets the label for each column.

Note that setHeaderLabels() won't remove existing columns.

See also setHeaderItem() and setHeaderLabel().
*/
impl /*struct*/ QTreeWidget {
  pub fn setHeaderLabels_0<RetType, T: QTreeWidget_setHeaderLabels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderLabels_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setHeaderLabels_0<RetType> {
  fn setHeaderLabels_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setHeaderLabels_0<(/*void*/)> for (usize) {
  fn setHeaderLabels_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget15setHeaderLabelsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:284
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeaderLabel(const QString &)

/*
Same as setHeaderLabels(QStringList(label)).

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTreeWidget {
  pub fn setHeaderLabel_0<RetType, T: QTreeWidget_setHeaderLabel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderLabel_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setHeaderLabel_0<RetType> {
  fn setHeaderLabel_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setHeaderLabel_0<(/*void*/)> for (usize) {
  fn setHeaderLabel_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget14setHeaderLabelERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:286
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * currentItem() const

/*
Returns the current item in the tree widget.

See also setCurrentItem() and currentItemChanged().
*/
impl /*struct*/ QTreeWidget {
  pub fn currentItem_0<RetType, T: QTreeWidget_currentItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_currentItem_0<RetType> {
  fn currentItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_currentItem_0<usize> for () {
  fn currentItem_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget11currentItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:287
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentColumn() const

/*
Returns the current column in the tree widget.

This function was introduced in  Qt 4.1.

See also setCurrentItem() and columnCount().
*/
impl /*struct*/ QTreeWidget {
  pub fn currentColumn_0<RetType, T: QTreeWidget_currentColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentColumn_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_currentColumn_0<RetType> {
  fn currentColumn_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_currentColumn_0<i32> for () {
  fn currentColumn_0(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget13currentColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:288
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentItem(QTreeWidgetItem *)

/*
Sets the current item in the tree widget.

Unless the selection mode is NoSelection, the item is also selected.

See also currentItem() and currentItemChanged().
*/
impl /*struct*/ QTreeWidget {
  pub fn setCurrentItem_0<RetType, T: QTreeWidget_setCurrentItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setCurrentItem_0<RetType> {
  fn setCurrentItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setCurrentItem_0<(/*void*/)> for (usize) {
  fn setCurrentItem_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:289
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setCurrentItem(QTreeWidgetItem *, int)

/*
Sets the current item in the tree widget.

Unless the selection mode is NoSelection, the item is also selected.

See also currentItem() and currentItemChanged().
*/
impl /*struct*/ QTreeWidget {
  pub fn setCurrentItem_1<RetType, T: QTreeWidget_setCurrentItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem_1(self);
    // return 1;
  }
}
pub trait QTreeWidget_setCurrentItem_1<RetType> {
  fn setCurrentItem_1(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setCurrentItem_1<(/*void*/)> for (usize,i32) {
  fn setCurrentItem_1(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:290
// index:2
// Public Visibility=Default Availability=Available
// [-2] void setCurrentItem(QTreeWidgetItem *, int, QItemSelectionModel::SelectionFlags)

/*
Sets the current item in the tree widget.

Unless the selection mode is NoSelection, the item is also selected.

See also currentItem() and currentItemChanged().
*/
impl /*struct*/ QTreeWidget {
  pub fn setCurrentItem_2<RetType, T: QTreeWidget_setCurrentItem_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem_2(self);
    // return 1;
  }
}
pub trait QTreeWidget_setCurrentItem_2<RetType> {
  fn setCurrentItem_2(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setCurrentItem_2<(/*void*/)> for (usize,i32,i32) {
  fn setCurrentItem_2(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:292
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * itemAt(const QPoint &) const

/*
Returns a pointer to the item at the coordinates p. The coordinates are relative to the tree widget's viewport().

See also visualItemRect().
*/
impl /*struct*/ QTreeWidget {
  pub fn itemAt_0<RetType, T: QTreeWidget_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemAt_0<usize> for (usize) {
  fn itemAt_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget6itemAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:293
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QTreeWidgetItem * itemAt(int, int) const

/*
Returns a pointer to the item at the coordinates p. The coordinates are relative to the tree widget's viewport().

See also visualItemRect().
*/
impl /*struct*/ QTreeWidget {
  pub fn itemAt_1<RetType, T: QTreeWidget_itemAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_1(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemAt_1<RetType> {
  fn itemAt_1(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemAt_1<usize> for (i32,i32) {
  fn itemAt_1(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget6itemAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:294
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect visualItemRect(const QTreeWidgetItem *) const

/*
Returns the rectangle on the viewport occupied by the item at item.

See also itemAt().
*/
impl /*struct*/ QTreeWidget {
  pub fn visualItemRect_0<RetType, T: QTreeWidget_visualItemRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualItemRect_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_visualItemRect_0<RetType> {
  fn visualItemRect_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_visualItemRect_0<usize> for (usize) {
  fn visualItemRect_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:296
// index:0
// Public Visibility=Default Availability=Available
// [4] int sortColumn() const

/*
Returns the column used to sort the contents of the widget.

This function was introduced in  Qt 4.1.

See also sortItems().
*/
impl /*struct*/ QTreeWidget {
  pub fn sortColumn_0<RetType, T: QTreeWidget_sortColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortColumn_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_sortColumn_0<RetType> {
  fn sortColumn_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_sortColumn_0<i32> for () {
  fn sortColumn_0(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget10sortColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:297
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sortItems(int, Qt::SortOrder)

/*
Sorts the items in the widget in the specified order by the values in the given column.

See also sortColumn().
*/
impl /*struct*/ QTreeWidget {
  pub fn sortItems_0<RetType, T: QTreeWidget_sortItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortItems_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_sortItems_0<RetType> {
  fn sortItems_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_sortItems_0<(/*void*/)> for (i32,i32) {
  fn sortItems_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget9sortItemsEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:299
// index:0
// Public Visibility=Default Availability=Available
// [-2] void editItem(QTreeWidgetItem *, int)

/*
Starts editing the item in the given column if it is editable.
*/
impl /*struct*/ QTreeWidget {
  pub fn editItem_0<RetType, T: QTreeWidget_editItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_editItem_0<RetType> {
  fn editItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_editItem_0<(/*void*/)> for (usize,i32) {
  fn editItem_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget8editItemEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:300
// index:0
// Public Visibility=Default Availability=Available
// [-2] void openPersistentEditor(QTreeWidgetItem *, int)

/*
Opens a persistent editor for the item in the given column.

See also closePersistentEditor() and isPersistentEditorOpen().
*/
impl /*struct*/ QTreeWidget {
  pub fn openPersistentEditor_0<RetType, T: QTreeWidget_openPersistentEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_openPersistentEditor_0<RetType> {
  fn openPersistentEditor_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_openPersistentEditor_0<(/*void*/)> for (usize,i32) {
  fn openPersistentEditor_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:301
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closePersistentEditor(QTreeWidgetItem *, int)

/*
Closes the persistent editor for the item in the given column.

This function has no effect if no persistent editor is open for this combination of item and column.

See also openPersistentEditor() and isPersistentEditorOpen().
*/
impl /*struct*/ QTreeWidget {
  pub fn closePersistentEditor_0<RetType, T: QTreeWidget_closePersistentEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_closePersistentEditor_0<RetType> {
  fn closePersistentEditor_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_closePersistentEditor_0<(/*void*/)> for (usize,i32) {
  fn closePersistentEditor_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:303
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPersistentEditorOpen(QTreeWidgetItem *, int) const

/*
Returns whether a persistent editor is open for item item in column column.

This function was introduced in  Qt 5.10.

See also openPersistentEditor() and closePersistentEditor().
*/
impl /*struct*/ QTreeWidget {
  pub fn isPersistentEditorOpen_0<RetType, T: QTreeWidget_isPersistentEditorOpen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPersistentEditorOpen_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_isPersistentEditorOpen_0<RetType> {
  fn isPersistentEditorOpen_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_isPersistentEditorOpen_0<bool> for (usize,i32) {
  fn isPersistentEditorOpen_0(self , rsthis: & QTreeWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget22isPersistentEditorOpenEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:305
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * itemWidget(QTreeWidgetItem *, int) const

/*
Returns the widget displayed in the cell specified by item and the given column.

This function was introduced in  Qt 4.1.

See also setItemWidget().
*/
impl /*struct*/ QTreeWidget {
  pub fn itemWidget_0<RetType, T: QTreeWidget_itemWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemWidget_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemWidget_0<RetType> {
  fn itemWidget_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemWidget_0<usize> for (usize,i32) {
  fn itemWidget_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget10itemWidgetEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:306
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemWidget(QTreeWidgetItem *, int, QWidget *)

/*
Sets the given widget to be displayed in the cell specified by the given item and column.

The given widget's autoFillBackground property must be set to true, otherwise the widget's background will be transparent, showing both the model data and the tree widget item.

This function should only be used to display static content in the place of a tree widget item. If you want to display custom dynamic content or implement a custom editor widget, use QTreeView and subclass QItemDelegate instead.

This function cannot be called before the item hierarchy has been set up, i.e., the QTreeWidgetItem that will hold widget must have been added to the view before widget is set.

Note: The tree takes ownership of the widget.

This function was introduced in  Qt 4.1.

See also itemWidget() and Delegate Classes.
*/
impl /*struct*/ QTreeWidget {
  pub fn setItemWidget_0<RetType, T: QTreeWidget_setItemWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemWidget_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setItemWidget_0<RetType> {
  fn setItemWidget_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setItemWidget_0<(/*void*/)> for (usize,i32,usize) {
  fn setItemWidget_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:307
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void removeItemWidget(QTreeWidgetItem *, int)

/*
Removes the widget set in the given item in the given column.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QTreeWidget {
  pub fn removeItemWidget_0<RetType, T: QTreeWidget_removeItemWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItemWidget_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_removeItemWidget_0<RetType> {
  fn removeItemWidget_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_removeItemWidget_0<(/*void*/)> for (usize,i32) {
  fn removeItemWidget_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:309
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isItemSelected(const QTreeWidgetItem *) const

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn isItemSelected_0<RetType, T: QTreeWidget_isItemSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isItemSelected_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_isItemSelected_0<RetType> {
  fn isItemSelected_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_isItemSelected_0<bool> for (usize) {
  fn isItemSelected_0(self , rsthis: & QTreeWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:310
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemSelected(const QTreeWidgetItem *, bool)

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn setItemSelected_0<RetType, T: QTreeWidget_setItemSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemSelected_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setItemSelected_0<RetType> {
  fn setItemSelected_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setItemSelected_0<(/*void*/)> for (usize,bool) {
  fn setItemSelected_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:315
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isItemHidden(const QTreeWidgetItem *) const

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn isItemHidden_0<RetType, T: QTreeWidget_isItemHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isItemHidden_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_isItemHidden_0<RetType> {
  fn isItemHidden_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_isItemHidden_0<bool> for (usize) {
  fn isItemHidden_0(self , rsthis: & QTreeWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:316
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemHidden(const QTreeWidgetItem *, bool)

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn setItemHidden_0<RetType, T: QTreeWidget_setItemHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemHidden_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setItemHidden_0<RetType> {
  fn setItemHidden_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setItemHidden_0<(/*void*/)> for (usize,bool) {
  fn setItemHidden_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:318
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isItemExpanded(const QTreeWidgetItem *) const

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn isItemExpanded_0<RetType, T: QTreeWidget_isItemExpanded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isItemExpanded_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_isItemExpanded_0<RetType> {
  fn isItemExpanded_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_isItemExpanded_0<bool> for (usize) {
  fn isItemExpanded_0(self , rsthis: & QTreeWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:319
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemExpanded(const QTreeWidgetItem *, bool)

/*

*/
impl /*struct*/ QTreeWidget {
  pub fn setItemExpanded_0<RetType, T: QTreeWidget_setItemExpanded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemExpanded_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setItemExpanded_0<RetType> {
  fn setItemExpanded_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setItemExpanded_0<(/*void*/)> for (usize,bool) {
  fn setItemExpanded_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:321
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFirstItemColumnSpanned(const QTreeWidgetItem *) const

/*
Returns true if the given item is set to show only one section over all columns; otherwise returns false.

This function was introduced in  Qt 4.3.

See also setFirstItemColumnSpanned().
*/
impl /*struct*/ QTreeWidget {
  pub fn isFirstItemColumnSpanned_0<RetType, T: QTreeWidget_isFirstItemColumnSpanned_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFirstItemColumnSpanned_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_isFirstItemColumnSpanned_0<RetType> {
  fn isFirstItemColumnSpanned_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_isFirstItemColumnSpanned_0<bool> for (usize) {
  fn isFirstItemColumnSpanned_0(self , rsthis: & QTreeWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:322
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFirstItemColumnSpanned(const QTreeWidgetItem *, bool)

/*
Sets the given item to only show one section for all columns if span is true; otherwise the item will show one section per column.

This function was introduced in  Qt 4.3.

See also isFirstItemColumnSpanned().
*/
impl /*struct*/ QTreeWidget {
  pub fn setFirstItemColumnSpanned_0<RetType, T: QTreeWidget_setFirstItemColumnSpanned_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFirstItemColumnSpanned_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setFirstItemColumnSpanned_0<RetType> {
  fn setFirstItemColumnSpanned_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setFirstItemColumnSpanned_0<(/*void*/)> for (usize,bool) {
  fn setFirstItemColumnSpanned_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:324
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * itemAbove(const QTreeWidgetItem *) const

/*
Returns the item above the given item.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QTreeWidget {
  pub fn itemAbove_0<RetType, T: QTreeWidget_itemAbove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAbove_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemAbove_0<RetType> {
  fn itemAbove_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemAbove_0<usize> for (usize) {
  fn itemAbove_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:325
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * itemBelow(const QTreeWidgetItem *) const

/*
Returns the item visually below the given item.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QTreeWidget {
  pub fn itemBelow_0<RetType, T: QTreeWidget_itemBelow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemBelow_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemBelow_0<RetType> {
  fn itemBelow_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemBelow_0<usize> for (usize) {
  fn itemBelow_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:327
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSelectionModel(QItemSelectionModel *)

/*
Reimplemented from QAbstractItemView::setSelectionModel().
*/
impl /*struct*/ QTreeWidget {
  pub fn setSelectionModel_0<RetType, T: QTreeWidget_setSelectionModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_setSelectionModel_0<RetType> {
  fn setSelectionModel_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_setSelectionModel_0<(/*void*/)> for (usize) {
  fn setSelectionModel_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:330
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scrollToItem(const QTreeWidgetItem *, QAbstractItemView::ScrollHint)

/*
Ensures that the item is visible, scrolling the view if necessary using the specified hint.

See also currentItem(), itemAt(), and topLevelItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn scrollToItem_0<RetType, T: QTreeWidget_scrollToItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollToItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_scrollToItem_0<RetType> {
  fn scrollToItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_scrollToItem_0<(/*void*/)> for (usize,i32) {
  fn scrollToItem_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget12scrollToItemEPK15QTreeWidgetItemN17QAbstractItemView10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:332
// index:0
// Public Visibility=Default Availability=Available
// [-2] void expandItem(const QTreeWidgetItem *)

/*
Expands the item. This causes the tree containing the item's children to be expanded.

See also collapseItem(), currentItem(), itemAt(), topLevelItem(), and itemExpanded().
*/
impl /*struct*/ QTreeWidget {
  pub fn expandItem_0<RetType, T: QTreeWidget_expandItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_expandItem_0<RetType> {
  fn expandItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_expandItem_0<(/*void*/)> for (usize) {
  fn expandItem_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:333
// index:0
// Public Visibility=Default Availability=Available
// [-2] void collapseItem(const QTreeWidgetItem *)

/*
Closes the item. This causes the tree containing the item's children to be collapsed.

See also expandItem(), currentItem(), itemAt(), and topLevelItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn collapseItem_0<RetType, T: QTreeWidget_collapseItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.collapseItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_collapseItem_0<RetType> {
  fn collapseItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_collapseItem_0<(/*void*/)> for (usize) {
  fn collapseItem_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:334
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the tree widget by removing all of its items and selections.

Note: Since each item is removed from the tree widget before being deleted, the return value of QTreeWidgetItem::treeWidget() will be invalid when called from an item's destructor.

See also takeTopLevelItem(), topLevelItemCount(), and columnCount().
*/
impl /*struct*/ QTreeWidget {
  pub fn clear_0<RetType, T: QTreeWidget_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_clear_0<RetType> {
  fn clear_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:337
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemPressed(QTreeWidgetItem *, int)

/*
This signal is emitted when the user presses a mouse button inside the widget.

The specified item is the item that was clicked, or 0 if no item was clicked. The column is the item's column that was clicked, or -1 if no item was clicked.
*/
impl /*struct*/ QTreeWidget {
  pub fn itemPressed_0<RetType, T: QTreeWidget_itemPressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemPressed_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemPressed_0<RetType> {
  fn itemPressed_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemPressed_0<(/*void*/)> for (usize,i32) {
  fn itemPressed_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:338
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemClicked(QTreeWidgetItem *, int)

/*
This signal is emitted when the user clicks inside the widget.

The specified item is the item that was clicked. The column is the item's column that was clicked. If no item was clicked, no signal will be emitted.
*/
impl /*struct*/ QTreeWidget {
  pub fn itemClicked_0<RetType, T: QTreeWidget_itemClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemClicked_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemClicked_0<RetType> {
  fn itemClicked_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemClicked_0<(/*void*/)> for (usize,i32) {
  fn itemClicked_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:339
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemDoubleClicked(QTreeWidgetItem *, int)

/*
This signal is emitted when the user double clicks inside the widget.

The specified item is the item that was clicked, or 0 if no item was clicked. The column is the item's column that was clicked. If no item was double clicked, no signal will be emitted.
*/
impl /*struct*/ QTreeWidget {
  pub fn itemDoubleClicked_0<RetType, T: QTreeWidget_itemDoubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDoubleClicked_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemDoubleClicked_0<RetType> {
  fn itemDoubleClicked_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemDoubleClicked_0<(/*void*/)> for (usize,i32) {
  fn itemDoubleClicked_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:340
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemActivated(QTreeWidgetItem *, int)

/*
This signal is emitted when the user activates an item by single- or double-clicking (depending on the platform, i.e. on the QStyle::SH_ItemView_ActivateItemOnSingleClick style hint) or pressing a special key (e.g., Enter).

The specified item is the item that was clicked, or 0 if no item was clicked. The column is the item's column that was clicked, or -1 if no item was clicked.
*/
impl /*struct*/ QTreeWidget {
  pub fn itemActivated_0<RetType, T: QTreeWidget_itemActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemActivated_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemActivated_0<RetType> {
  fn itemActivated_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemActivated_0<(/*void*/)> for (usize,i32) {
  fn itemActivated_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:341
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemEntered(QTreeWidgetItem *, int)

/*
This signal is emitted when the mouse cursor enters an item over the specified column. QTreeWidget mouse tracking needs to be enabled for this feature to work.
*/
impl /*struct*/ QTreeWidget {
  pub fn itemEntered_0<RetType, T: QTreeWidget_itemEntered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemEntered_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemEntered_0<RetType> {
  fn itemEntered_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemEntered_0<(/*void*/)> for (usize,i32) {
  fn itemEntered_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:342
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemChanged(QTreeWidgetItem *, int)

/*
This signal is emitted when the contents of the column in the specified item changes.
*/
impl /*struct*/ QTreeWidget {
  pub fn itemChanged_0<RetType, T: QTreeWidget_itemChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemChanged_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemChanged_0<RetType> {
  fn itemChanged_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemChanged_0<(/*void*/)> for (usize,i32) {
  fn itemChanged_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:343
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemExpanded(QTreeWidgetItem *)

/*
This signal is emitted when the specified item is expanded so that all of its children are displayed.

Note: This signal will not be emitted if an item changes its state when expandAll() is invoked.

See also setItemExpanded(), QTreeWidgetItem::isExpanded(), itemCollapsed(), and expandItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn itemExpanded_0<RetType, T: QTreeWidget_itemExpanded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemExpanded_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemExpanded_0<RetType> {
  fn itemExpanded_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemExpanded_0<(/*void*/)> for (usize) {
  fn itemExpanded_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:344
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemCollapsed(QTreeWidgetItem *)

/*
This signal is emitted when the specified item is collapsed so that none of its children are displayed.

Note: This signal will not be emitted if an item changes its state when collapseAll() is invoked.

See also QTreeWidgetItem::isExpanded(), itemExpanded(), and collapseItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn itemCollapsed_0<RetType, T: QTreeWidget_itemCollapsed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemCollapsed_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemCollapsed_0<RetType> {
  fn itemCollapsed_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemCollapsed_0<(/*void*/)> for (usize) {
  fn itemCollapsed_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:345
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentItemChanged(QTreeWidgetItem *, QTreeWidgetItem *)

/*
This signal is emitted when the current item changes. The current item is specified by current, and this replaces the previous current item.

See also setCurrentItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn currentItemChanged_0<RetType, T: QTreeWidget_currentItemChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentItemChanged_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_currentItemChanged_0<RetType> {
  fn currentItemChanged_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_currentItemChanged_0<(/*void*/)> for (usize,usize) {
  fn currentItemChanged_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:346
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemSelectionChanged()

/*
This signal is emitted when the selection changes in the tree widget. The current selection can be found with selectedItems().
*/
impl /*struct*/ QTreeWidget {
  pub fn itemSelectionChanged_0<RetType, T: QTreeWidget_itemSelectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemSelectionChanged_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemSelectionChanged_0<RetType> {
  fn itemSelectionChanged_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemSelectionChanged_0<(/*void*/)> for () {
  fn itemSelectionChanged_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget20itemSelectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:349
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QTreeWidget {
  pub fn event_0<RetType, T: QTreeWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QTreeWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTreeWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:350
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Returns a list of MIME types that can be used to describe a list of treewidget items.

See also mimeData().
*/
impl /*struct*/ QTreeWidget {
  pub fn mimeTypes_0<RetType, T: QTreeWidget_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:356
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(QTreeWidgetItem *, int, const QMimeData *, Qt::DropAction)

/*
Handles the data supplied by a drag and drop operation that ended with the given action in the index in the given parent item.

The default implementation returns true if the drop was successfully handled by decoding the mime data and inserting it into the model; otherwise it returns false.

See also supportedDropActions().
*/
impl /*struct*/ QTreeWidget {
  pub fn dropMimeData_0<RetType, T: QTreeWidget_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_dropMimeData_0<bool> for (usize,i32,usize,i32) {
  fn dropMimeData_0(self , rsthis: & QTreeWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTreeWidget12dropMimeDataEP15QTreeWidgetItemiPK9QMimeDataN2Qt10DropActionE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:358
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Returns the drop actions supported by this view.

See also Qt::DropActions.
*/
impl /*struct*/ QTreeWidget {
  pub fn supportedDropActions_0<RetType, T: QTreeWidget_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:367
// index:0
// Protected Visibility=Default Availability=Available
// [24] QModelIndex indexFromItem(const QTreeWidgetItem *, int) const

/*
Returns the QModelIndex associated with the given item in the given column.

Note: In Qt versions prior to 5.7, this function took a non-const item.

See also itemFromIndex() and topLevelItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn indexFromItem_0<RetType, T: QTreeWidget_indexFromItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexFromItem_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_indexFromItem_0<RetType> {
  fn indexFromItem_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_indexFromItem_0<usize> for (usize,i32) {
  fn indexFromItem_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget13indexFromItemEPK15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:368
// index:1
// Protected Visibility=Default Availability=Available
// [24] QModelIndex indexFromItem(QTreeWidgetItem *, int) const

/*
Returns the QModelIndex associated with the given item in the given column.

Note: In Qt versions prior to 5.7, this function took a non-const item.

See also itemFromIndex() and topLevelItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn indexFromItem_1<RetType, T: QTreeWidget_indexFromItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexFromItem_1(self);
    // return 1;
  }
}
pub trait QTreeWidget_indexFromItem_1<RetType> {
  fn indexFromItem_1(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_indexFromItem_1<usize> for (usize,i32) {
  fn indexFromItem_1(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget13indexFromItemEP15QTreeWidgetItemi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:369
// index:0
// Protected Visibility=Default Availability=Available
// [8] QTreeWidgetItem * itemFromIndex(const QModelIndex &) const

/*
Returns a pointer to the QTreeWidgetItem associated with the given index.

See also indexFromItem().
*/
impl /*struct*/ QTreeWidget {
  pub fn itemFromIndex_0<RetType, T: QTreeWidget_itemFromIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemFromIndex_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_itemFromIndex_0<RetType> {
  fn itemFromIndex_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_itemFromIndex_0<usize> for (usize) {
  fn itemFromIndex_0(self , rsthis: & QTreeWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTreeWidget13itemFromIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:373
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().
*/
impl /*struct*/ QTreeWidget {
  pub fn dropEvent_0<RetType, T: QTreeWidget_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QTreeWidget_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QTreeWidget) -> RetType;
}
impl<'a> /*trait*/ QTreeWidget_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QTreeWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTreeWidget9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
