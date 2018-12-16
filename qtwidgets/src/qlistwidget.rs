

// mod ::widgets::QListWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qlistwidget.h
// #include <qlistwidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 45
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
// func (this *QListWidget) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// QStringList mimeTypes()
// func (this *QListWidget) InheritMimeTypes(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "mimeTypes", f)
// }

// bool dropMimeData(int, const QMimeData *, Qt::DropAction)
// func (this *QListWidget) InheritDropMimeData(f func(index int, data *qtcore.QMimeData/*777 const QMimeData **/, action int) bool) {
//  qtrt.SetAllInheritCallback(this, "dropMimeData", f)
// }

// Qt::DropActions supportedDropActions()
// func (this *QListWidget) InheritSupportedDropActions(f func() int) {
//  qtrt.SetAllInheritCallback(this, "supportedDropActions", f)
// }

// QModelIndex indexFromItem(QListWidgetItem *)
// func (this *QListWidget) InheritIndexFromItem(f func(item *QListWidgetItem/*777 QListWidgetItem **/) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "indexFromItem", f)
// }

// QListWidgetItem * itemFromIndex(const QModelIndex &)
// func (this *QListWidget) InheritItemFromIndex(f func(index *qtcore.QModelIndex) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "itemFromIndex", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QListWidget)=48
pub struct QListWidget {
  qbase: QListView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QListWidget_ITF interface {
//    QListView_ITF
//    QListWidget_PTR() *QListWidget
//}
//func (ptr *QListWidget) QListWidget_PTR() *QListWidget { return ptr }

impl /*struct*/ QListWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QListWidget {
    return QListWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QListWidget {
//  type Target = QListWidgetBASE;
//
//  fn deref(&self) -> &QListWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QListWidgetBASE> for QListWidget {
//  fn as_ref(& self) -> & QListWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlistwidget.h:199
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QListWidget {
  pub fn metaObject_0<RetType, T: QListWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QListWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QListWidget(QWidget *)

/*
Constructs an empty QListWidget with the given parent.
*/
// QListWidget(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QListWidget {
  pub fn QListWidget_0<T: QListWidget_QListWidget_0>(value: T) -> QListWidget {
    let rsthis = value.QListWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidget_QListWidget_0 {
  fn QListWidget_0(self) -> QListWidget;
}
// QListWidget(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QListWidget_QListWidget_0 for (usize) {
  fn QListWidget_0(self) -> QListWidget {
    // unsafe{_ZN11QListWidgetC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QListWidgetC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QListWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:208
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QListWidget()

/*

*/
pub fn DeleteQListWidget(this :*mut QListWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QListWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlistwidget.h:210
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSelectionModel(QItemSelectionModel *)

/*
Reimplemented from QAbstractItemView::setSelectionModel().
*/
impl /*struct*/ QListWidget {
  pub fn setSelectionModel_0<RetType, T: QListWidget_setSelectionModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel_0(self);
    // return 1;
  }
}
pub trait QListWidget_setSelectionModel_0<RetType> {
  fn setSelectionModel_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setSelectionModel_0<(/*void*/)> for (usize) {
  fn setSelectionModel_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget17setSelectionModelEP19QItemSelectionModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:212
// index:0
// Public Visibility=Default Availability=Available
// [8] QListWidgetItem * item(int) const

/*
Returns the item that occupies the given row in the list if one has been set; otherwise returns 0.

See also row().
*/
impl /*struct*/ QListWidget {
  pub fn item_0<RetType, T: QListWidget_item_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.item_0(self);
    // return 1;
  }
}
pub trait QListWidget_item_0<RetType> {
  fn item_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_item_0<usize> for (i32) {
  fn item_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget4itemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:213
// index:0
// Public Visibility=Default Availability=Available
// [4] int row(const QListWidgetItem *) const

/*
Returns the row containing the given item.

See also item().
*/
impl /*struct*/ QListWidget {
  pub fn row_0<RetType, T: QListWidget_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QListWidget_row_0<RetType> {
  fn row_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_row_0<i32> for (usize) {
  fn row_0(self , rsthis: & QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget3rowEPK15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:214
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertItem(int, QListWidgetItem *)

/*
Inserts the item at the position in the list given by row.

See also addItem().
*/
impl /*struct*/ QListWidget {
  pub fn insertItem_0<RetType, T: QListWidget_insertItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItem_0(self);
    // return 1;
  }
}
pub trait QListWidget_insertItem_0<RetType> {
  fn insertItem_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_insertItem_0<(/*void*/)> for (i32,usize) {
  fn insertItem_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget10insertItemEiP15QListWidgetItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:215
// index:1
// Public Visibility=Default Availability=Available
// [-2] void insertItem(int, const QString &)

/*
Inserts the item at the position in the list given by row.

See also addItem().
*/
impl /*struct*/ QListWidget {
  pub fn insertItem_1<RetType, T: QListWidget_insertItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItem_1(self);
    // return 1;
  }
}
pub trait QListWidget_insertItem_1<RetType> {
  fn insertItem_1(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_insertItem_1<(/*void*/)> for (i32,usize) {
  fn insertItem_1(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget10insertItemEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:216
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertItems(int, const QStringList &)

/*
Inserts items from the list of labels into the list, starting at the given row.

See also insertItem() and addItem().
*/
impl /*struct*/ QListWidget {
  pub fn insertItems_0<RetType, T: QListWidget_insertItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItems_0(self);
    // return 1;
  }
}
pub trait QListWidget_insertItems_0<RetType> {
  fn insertItems_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_insertItems_0<(/*void*/)> for (i32,usize) {
  fn insertItems_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget11insertItemsEiRK11QStringList", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:217
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void addItem(const QString &)

/*
Inserts an item with the text label at the end of the list widget.
*/
impl /*struct*/ QListWidget {
  pub fn addItem_0<RetType, T: QListWidget_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QListWidget_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_addItem_0<(/*void*/)> for (usize) {
  fn addItem_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget7addItemERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:218
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void addItem(QListWidgetItem *)

/*
Inserts an item with the text label at the end of the list widget.
*/
impl /*struct*/ QListWidget {
  pub fn addItem_1<RetType, T: QListWidget_addItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_1(self);
    // return 1;
  }
}
pub trait QListWidget_addItem_1<RetType> {
  fn addItem_1(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_addItem_1<(/*void*/)> for (usize) {
  fn addItem_1(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget7addItemEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:219
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void addItems(const QStringList &)

/*
Inserts items with the text labels at the end of the list widget.

See also insertItems().
*/
impl /*struct*/ QListWidget {
  pub fn addItems_0<RetType, T: QListWidget_addItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItems_0(self);
    // return 1;
  }
}
pub trait QListWidget_addItems_0<RetType> {
  fn addItems_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_addItems_0<(/*void*/)> for (usize) {
  fn addItems_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget8addItemsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:220
// index:0
// Public Visibility=Default Availability=Available
// [8] QListWidgetItem * takeItem(int)

/*
Removes and returns the item from the given row in the list widget; otherwise returns 0.

Items removed from a list widget will not be managed by Qt, and will need to be deleted manually.

See also insertItem() and addItem().
*/
impl /*struct*/ QListWidget {
  pub fn takeItem_0<RetType, T: QListWidget_takeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeItem_0(self);
    // return 1;
  }
}
pub trait QListWidget_takeItem_0<RetType> {
  fn takeItem_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_takeItem_0<usize> for (i32) {
  fn takeItem_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QListWidget8takeItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:221
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*

*/
impl /*struct*/ QListWidget {
  pub fn count_0<RetType, T: QListWidget_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QListWidget_count_0<RetType> {
  fn count_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_count_0<i32> for () {
  fn count_0(self , rsthis: & QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:223
// index:0
// Public Visibility=Default Availability=Available
// [8] QListWidgetItem * currentItem() const

/*
Returns the current item.

See also setCurrentItem().
*/
impl /*struct*/ QListWidget {
  pub fn currentItem_0<RetType, T: QListWidget_currentItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentItem_0(self);
    // return 1;
  }
}
pub trait QListWidget_currentItem_0<RetType> {
  fn currentItem_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_currentItem_0<usize> for () {
  fn currentItem_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget11currentItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:224
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentItem(QListWidgetItem *)

/*
Sets the current item to item.

Unless the selection mode is NoSelection, the item is also selected.

See also currentItem().
*/
impl /*struct*/ QListWidget {
  pub fn setCurrentItem_0<RetType, T: QListWidget_setCurrentItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem_0(self);
    // return 1;
  }
}
pub trait QListWidget_setCurrentItem_0<RetType> {
  fn setCurrentItem_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setCurrentItem_0<(/*void*/)> for (usize) {
  fn setCurrentItem_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget14setCurrentItemEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:225
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setCurrentItem(QListWidgetItem *, QItemSelectionModel::SelectionFlags)

/*
Sets the current item to item.

Unless the selection mode is NoSelection, the item is also selected.

See also currentItem().
*/
impl /*struct*/ QListWidget {
  pub fn setCurrentItem_1<RetType, T: QListWidget_setCurrentItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem_1(self);
    // return 1;
  }
}
pub trait QListWidget_setCurrentItem_1<RetType> {
  fn setCurrentItem_1(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setCurrentItem_1<(/*void*/)> for (usize,i32) {
  fn setCurrentItem_1(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget14setCurrentItemEP15QListWidgetItem6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:227
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentRow() const

/*

*/
impl /*struct*/ QListWidget {
  pub fn currentRow_0<RetType, T: QListWidget_currentRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentRow_0(self);
    // return 1;
  }
}
pub trait QListWidget_currentRow_0<RetType> {
  fn currentRow_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_currentRow_0<i32> for () {
  fn currentRow_0(self , rsthis: & QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget10currentRowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:228
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentRow(int)

/*

*/
impl /*struct*/ QListWidget {
  pub fn setCurrentRow_0<RetType, T: QListWidget_setCurrentRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentRow_0(self);
    // return 1;
  }
}
pub trait QListWidget_setCurrentRow_0<RetType> {
  fn setCurrentRow_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setCurrentRow_0<(/*void*/)> for (i32) {
  fn setCurrentRow_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget13setCurrentRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:229
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setCurrentRow(int, QItemSelectionModel::SelectionFlags)

/*

*/
impl /*struct*/ QListWidget {
  pub fn setCurrentRow_1<RetType, T: QListWidget_setCurrentRow_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentRow_1(self);
    // return 1;
  }
}
pub trait QListWidget_setCurrentRow_1<RetType> {
  fn setCurrentRow_1(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setCurrentRow_1<(/*void*/)> for (i32,i32) {
  fn setCurrentRow_1(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget13setCurrentRowEi6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:231
// index:0
// Public Visibility=Default Availability=Available
// [8] QListWidgetItem * itemAt(const QPoint &) const

/*
Returns a pointer to the item at the coordinates p. The coordinates are relative to the list widget's viewport().
*/
impl /*struct*/ QListWidget {
  pub fn itemAt_0<RetType, T: QListWidget_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemAt_0<usize> for (usize) {
  fn itemAt_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget6itemAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:232
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QListWidgetItem * itemAt(int, int) const

/*
Returns a pointer to the item at the coordinates p. The coordinates are relative to the list widget's viewport().
*/
impl /*struct*/ QListWidget {
  pub fn itemAt_1<RetType, T: QListWidget_itemAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_1(self);
    // return 1;
  }
}
pub trait QListWidget_itemAt_1<RetType> {
  fn itemAt_1(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemAt_1<usize> for (i32,i32) {
  fn itemAt_1(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget6itemAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:233
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect visualItemRect(const QListWidgetItem *) const

/*
Returns the rectangle on the viewport occupied by the item at item.
*/
impl /*struct*/ QListWidget {
  pub fn visualItemRect_0<RetType, T: QListWidget_visualItemRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualItemRect_0(self);
    // return 1;
  }
}
pub trait QListWidget_visualItemRect_0<RetType> {
  fn visualItemRect_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_visualItemRect_0<usize> for (usize) {
  fn visualItemRect_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:235
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sortItems(Qt::SortOrder)

/*
Sorts all the items in the list widget according to the specified order.
*/
impl /*struct*/ QListWidget {
  pub fn sortItems_0<RetType, T: QListWidget_sortItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortItems_0(self);
    // return 1;
  }
}
pub trait QListWidget_sortItems_0<RetType> {
  fn sortItems_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_sortItems_0<(/*void*/)> for (i32) {
  fn sortItems_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget9sortItemsEN2Qt9SortOrderE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:236
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortingEnabled(bool)

/*

*/
impl /*struct*/ QListWidget {
  pub fn setSortingEnabled_0<RetType, T: QListWidget_setSortingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortingEnabled_0(self);
    // return 1;
  }
}
pub trait QListWidget_setSortingEnabled_0<RetType> {
  fn setSortingEnabled_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setSortingEnabled_0<(/*void*/)> for (bool) {
  fn setSortingEnabled_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget17setSortingEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:237
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSortingEnabled() const

/*

*/
impl /*struct*/ QListWidget {
  pub fn isSortingEnabled_0<RetType, T: QListWidget_isSortingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSortingEnabled_0(self);
    // return 1;
  }
}
pub trait QListWidget_isSortingEnabled_0<RetType> {
  fn isSortingEnabled_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_isSortingEnabled_0<bool> for () {
  fn isSortingEnabled_0(self , rsthis: & QListWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget16isSortingEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:239
// index:0
// Public Visibility=Default Availability=Available
// [-2] void editItem(QListWidgetItem *)

/*
Starts editing the item if it is editable.
*/
impl /*struct*/ QListWidget {
  pub fn editItem_0<RetType, T: QListWidget_editItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editItem_0(self);
    // return 1;
  }
}
pub trait QListWidget_editItem_0<RetType> {
  fn editItem_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_editItem_0<(/*void*/)> for (usize) {
  fn editItem_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget8editItemEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:240
// index:0
// Public Visibility=Default Availability=Available
// [-2] void openPersistentEditor(QListWidgetItem *)

/*
Opens an editor for the given item. The editor remains open after editing.

See also closePersistentEditor() and isPersistentEditorOpen().
*/
impl /*struct*/ QListWidget {
  pub fn openPersistentEditor_0<RetType, T: QListWidget_openPersistentEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor_0(self);
    // return 1;
  }
}
pub trait QListWidget_openPersistentEditor_0<RetType> {
  fn openPersistentEditor_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_openPersistentEditor_0<(/*void*/)> for (usize) {
  fn openPersistentEditor_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closePersistentEditor(QListWidgetItem *)

/*
Closes the persistent editor for the given item.

See also openPersistentEditor() and isPersistentEditorOpen().
*/
impl /*struct*/ QListWidget {
  pub fn closePersistentEditor_0<RetType, T: QListWidget_closePersistentEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor_0(self);
    // return 1;
  }
}
pub trait QListWidget_closePersistentEditor_0<RetType> {
  fn closePersistentEditor_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_closePersistentEditor_0<(/*void*/)> for (usize) {
  fn closePersistentEditor_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:243
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPersistentEditorOpen(QListWidgetItem *) const

/*
Returns whether a persistent editor is open for item item.

This function was introduced in  Qt 5.10.

See also openPersistentEditor() and closePersistentEditor().
*/
impl /*struct*/ QListWidget {
  pub fn isPersistentEditorOpen_0<RetType, T: QListWidget_isPersistentEditorOpen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPersistentEditorOpen_0(self);
    // return 1;
  }
}
pub trait QListWidget_isPersistentEditorOpen_0<RetType> {
  fn isPersistentEditorOpen_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_isPersistentEditorOpen_0<bool> for (usize) {
  fn isPersistentEditorOpen_0(self , rsthis: & QListWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget22isPersistentEditorOpenEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:245
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * itemWidget(QListWidgetItem *) const

/*
Returns the widget displayed in the given item.

This function was introduced in  Qt 4.1.

See also setItemWidget() and removeItemWidget().
*/
impl /*struct*/ QListWidget {
  pub fn itemWidget_0<RetType, T: QListWidget_itemWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemWidget_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemWidget_0<RetType> {
  fn itemWidget_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemWidget_0<usize> for (usize) {
  fn itemWidget_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget10itemWidgetEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:246
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemWidget(QListWidgetItem *, QWidget *)

/*
Sets the widget to be displayed in the given item.

This function should only be used to display static content in the place of a list widget item. If you want to display custom dynamic content or implement a custom editor widget, use QListView and subclass QItemDelegate instead.

This function was introduced in  Qt 4.1.

See also itemWidget(), removeItemWidget(), and Delegate Classes.
*/
impl /*struct*/ QListWidget {
  pub fn setItemWidget_0<RetType, T: QListWidget_setItemWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemWidget_0(self);
    // return 1;
  }
}
pub trait QListWidget_setItemWidget_0<RetType> {
  fn setItemWidget_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setItemWidget_0<(/*void*/)> for (usize,usize) {
  fn setItemWidget_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:247
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void removeItemWidget(QListWidgetItem *)

/*
Removes the widget set on the given item.

To remove an item (row) from the list entirely, either delete the item or use takeItem().

This function was introduced in  Qt 4.3.

See also itemWidget() and setItemWidget().
*/
impl /*struct*/ QListWidget {
  pub fn removeItemWidget_0<RetType, T: QListWidget_removeItemWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItemWidget_0(self);
    // return 1;
  }
}
pub trait QListWidget_removeItemWidget_0<RetType> {
  fn removeItemWidget_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_removeItemWidget_0<(/*void*/)> for (usize) {
  fn removeItemWidget_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:249
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isItemSelected(const QListWidgetItem *) const

/*

*/
impl /*struct*/ QListWidget {
  pub fn isItemSelected_0<RetType, T: QListWidget_isItemSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isItemSelected_0(self);
    // return 1;
  }
}
pub trait QListWidget_isItemSelected_0<RetType> {
  fn isItemSelected_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_isItemSelected_0<bool> for (usize) {
  fn isItemSelected_0(self , rsthis: & QListWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:250
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemSelected(const QListWidgetItem *, bool)

/*

*/
impl /*struct*/ QListWidget {
  pub fn setItemSelected_0<RetType, T: QListWidget_setItemSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemSelected_0(self);
    // return 1;
  }
}
pub trait QListWidget_setItemSelected_0<RetType> {
  fn setItemSelected_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setItemSelected_0<(/*void*/)> for (usize,bool) {
  fn setItemSelected_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:254
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isItemHidden(const QListWidgetItem *) const

/*

*/
impl /*struct*/ QListWidget {
  pub fn isItemHidden_0<RetType, T: QListWidget_isItemHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isItemHidden_0(self);
    // return 1;
  }
}
pub trait QListWidget_isItemHidden_0<RetType> {
  fn isItemHidden_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_isItemHidden_0<bool> for (usize) {
  fn isItemHidden_0(self , rsthis: & QListWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:255
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemHidden(const QListWidgetItem *, bool)

/*

*/
impl /*struct*/ QListWidget {
  pub fn setItemHidden_0<RetType, T: QListWidget_setItemHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemHidden_0(self);
    // return 1;
  }
}
pub trait QListWidget_setItemHidden_0<RetType> {
  fn setItemHidden_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_setItemHidden_0<(/*void*/)> for (usize,bool) {
  fn setItemHidden_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:260
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().
*/
impl /*struct*/ QListWidget {
  pub fn dropEvent_0<RetType, T: QListWidget_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QListWidget_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:263
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scrollToItem(const QListWidgetItem *, QAbstractItemView::ScrollHint)

/*
Scrolls the view if necessary to ensure that the item is visible.

hint specifies where the item should be located after the operation.
*/
impl /*struct*/ QListWidget {
  pub fn scrollToItem_0<RetType, T: QListWidget_scrollToItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollToItem_0(self);
    // return 1;
  }
}
pub trait QListWidget_scrollToItem_0<RetType> {
  fn scrollToItem_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_scrollToItem_0<(/*void*/)> for (usize,i32) {
  fn scrollToItem_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget12scrollToItemEPK15QListWidgetItemN17QAbstractItemView10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:264
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all items and selections in the view.

Warning: All items will be permanently deleted.
*/
impl /*struct*/ QListWidget {
  pub fn clear_0<RetType, T: QListWidget_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QListWidget_clear_0<RetType> {
  fn clear_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QListWidget5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:267
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemPressed(QListWidgetItem *)

/*
This signal is emitted with the specified item when a mouse button is pressed on an item in the widget.

See also itemClicked() and itemDoubleClicked().
*/
impl /*struct*/ QListWidget {
  pub fn itemPressed_0<RetType, T: QListWidget_itemPressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemPressed_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemPressed_0<RetType> {
  fn itemPressed_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemPressed_0<(/*void*/)> for (usize) {
  fn itemPressed_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget11itemPressedEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemClicked(QListWidgetItem *)

/*
This signal is emitted with the specified item when a mouse button is clicked on an item in the widget.

See also itemPressed() and itemDoubleClicked().
*/
impl /*struct*/ QListWidget {
  pub fn itemClicked_0<RetType, T: QListWidget_itemClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemClicked_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemClicked_0<RetType> {
  fn itemClicked_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemClicked_0<(/*void*/)> for (usize) {
  fn itemClicked_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget11itemClickedEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:269
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemDoubleClicked(QListWidgetItem *)

/*
This signal is emitted with the specified item when a mouse button is double clicked on an item in the widget.

See also itemClicked() and itemPressed().
*/
impl /*struct*/ QListWidget {
  pub fn itemDoubleClicked_0<RetType, T: QListWidget_itemDoubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDoubleClicked_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemDoubleClicked_0<RetType> {
  fn itemDoubleClicked_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemDoubleClicked_0<(/*void*/)> for (usize) {
  fn itemDoubleClicked_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:270
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemActivated(QListWidgetItem *)

/*
This signal is emitted when the item is activated. The item is activated when the user clicks or double clicks on it, depending on the system configuration. It is also activated when the user presses the activation key (on Windows and X11 this is the Return key, on Mac OS X it is Command+O).
*/
impl /*struct*/ QListWidget {
  pub fn itemActivated_0<RetType, T: QListWidget_itemActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemActivated_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemActivated_0<RetType> {
  fn itemActivated_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemActivated_0<(/*void*/)> for (usize) {
  fn itemActivated_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget13itemActivatedEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:271
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemEntered(QListWidgetItem *)

/*
This signal is emitted when the mouse cursor enters an item. The item is the item entered. This signal is only emitted when mouseTracking is turned on, or when a mouse button is pressed while moving into an item.

See also QWidget::setMouseTracking().
*/
impl /*struct*/ QListWidget {
  pub fn itemEntered_0<RetType, T: QListWidget_itemEntered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemEntered_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemEntered_0<RetType> {
  fn itemEntered_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemEntered_0<(/*void*/)> for (usize) {
  fn itemEntered_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget11itemEnteredEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:272
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemChanged(QListWidgetItem *)

/*
This signal is emitted whenever the data of item has changed.
*/
impl /*struct*/ QListWidget {
  pub fn itemChanged_0<RetType, T: QListWidget_itemChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemChanged_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemChanged_0<RetType> {
  fn itemChanged_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemChanged_0<(/*void*/)> for (usize) {
  fn itemChanged_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget11itemChangedEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:274
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentItemChanged(QListWidgetItem *, QListWidgetItem *)

/*
This signal is emitted whenever the current item changes.

previous is the item that previously had the focus; current is the new current item.
*/
impl /*struct*/ QListWidget {
  pub fn currentItemChanged_0<RetType, T: QListWidget_currentItemChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentItemChanged_0(self);
    // return 1;
  }
}
pub trait QListWidget_currentItemChanged_0<RetType> {
  fn currentItemChanged_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_currentItemChanged_0<(/*void*/)> for (usize,usize) {
  fn currentItemChanged_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:275
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentTextChanged(const QString &)

/*
This signal is emitted whenever the current item changes.

currentText is the text data in the current item. If there is no current item, the currentText is invalid.
*/
impl /*struct*/ QListWidget {
  pub fn currentTextChanged_0<RetType, T: QListWidget_currentTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentTextChanged_0(self);
    // return 1;
  }
}
pub trait QListWidget_currentTextChanged_0<RetType> {
  fn currentTextChanged_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_currentTextChanged_0<(/*void*/)> for (usize) {
  fn currentTextChanged_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget18currentTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:276
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentRowChanged(int)

/*
This signal is emitted whenever the current item changes.

currentRow is the row of the current item. If there is no current item, the currentRow is -1.

Note: Notifier signal for property currentRow.
*/
impl /*struct*/ QListWidget {
  pub fn currentRowChanged_0<RetType, T: QListWidget_currentRowChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentRowChanged_0(self);
    // return 1;
  }
}
pub trait QListWidget_currentRowChanged_0<RetType> {
  fn currentRowChanged_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_currentRowChanged_0<(/*void*/)> for (i32) {
  fn currentRowChanged_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QListWidget17currentRowChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:278
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemSelectionChanged()

/*
This signal is emitted whenever the selection changes.

See also selectedItems(), QListWidgetItem::isSelected(), and currentItemChanged().
*/
impl /*struct*/ QListWidget {
  pub fn itemSelectionChanged_0<RetType, T: QListWidget_itemSelectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemSelectionChanged_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemSelectionChanged_0<RetType> {
  fn itemSelectionChanged_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemSelectionChanged_0<(/*void*/)> for () {
  fn itemSelectionChanged_0(self , rsthis: & QListWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QListWidget20itemSelectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:281
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QListWidget {
  pub fn event_0<RetType, T: QListWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QListWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QListWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QListWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:282
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Returns a list of MIME types that can be used to describe a list of listwidget items.

See also mimeData().
*/
impl /*struct*/ QListWidget {
  pub fn mimeTypes_0<RetType, T: QListWidget_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QListWidget_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:289
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(int, const QMimeData *, Qt::DropAction)

/*
Handles data supplied by an external drag and drop operation that ended with the given action in the given index. Returns true if data and action can be handled by the model; otherwise returns false.

See also supportedDropActions().
*/
impl /*struct*/ QListWidget {
  pub fn dropMimeData_0<RetType, T: QListWidget_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QListWidget_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_dropMimeData_0<bool> for (i32,usize,i32) {
  fn dropMimeData_0(self , rsthis: & QListWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QListWidget12dropMimeDataEiPK9QMimeDataN2Qt10DropActionE", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:290
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Returns the drop actions supported by this view.

See also Qt::DropActions.
*/
impl /*struct*/ QListWidget {
  pub fn supportedDropActions_0<RetType, T: QListWidget_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QListWidget_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:300
// index:0
// Protected Visibility=Default Availability=Available
// [24] QModelIndex indexFromItem(QListWidgetItem *) const

/*
Returns the QModelIndex associated with the given item.
*/
impl /*struct*/ QListWidget {
  pub fn indexFromItem_0<RetType, T: QListWidget_indexFromItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexFromItem_0(self);
    // return 1;
  }
}
pub trait QListWidget_indexFromItem_0<RetType> {
  fn indexFromItem_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_indexFromItem_0<usize> for (usize) {
  fn indexFromItem_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget13indexFromItemEP15QListWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:301
// index:0
// Protected Visibility=Default Availability=Available
// [8] QListWidgetItem * itemFromIndex(const QModelIndex &) const

/*
Returns a pointer to the QListWidgetItem associated with the given index.
*/
impl /*struct*/ QListWidget {
  pub fn itemFromIndex_0<RetType, T: QListWidget_itemFromIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemFromIndex_0(self);
    // return 1;
  }
}
pub trait QListWidget_itemFromIndex_0<RetType> {
  fn itemFromIndex_0(self , rsthis: & QListWidget) -> RetType;
}
impl<'a> /*trait*/ QListWidget_itemFromIndex_0<usize> for (usize) {
  fn itemFromIndex_0(self , rsthis: & QListWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QListWidget13itemFromIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
