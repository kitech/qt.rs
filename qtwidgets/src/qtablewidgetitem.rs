

// mod ::widgets::QTableWidgetItem
// package qtwidgets
// /usr/include/qt/QtWidgets/qtablewidget.h
// #include <qtablewidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QTableWidgetItem)=48
pub struct QTableWidgetItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTableWidgetItem_ITF interface {
//    QTableWidgetItem_PTR() *QTableWidgetItem
//}
//func (ptr *QTableWidgetItem) QTableWidgetItem_PTR() *QTableWidgetItem { return ptr }

impl /*struct*/ QTableWidgetItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTableWidgetItem {
    return QTableWidgetItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTableWidgetItem {
//  type Target = QTableWidgetItemBASE;
//
//  fn deref(&self) -> &QTableWidgetItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTableWidgetItemBASE> for QTableWidgetItem {
//  fn as_ref(& self) -> & QTableWidgetItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtablewidget.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTableWidgetItem(int)

/*

*/
// QTableWidgetItem(int) ctx.fn_proto_cpp
impl /*struct*/ QTableWidgetItem {
  pub fn QTableWidgetItem_0<T: QTableWidgetItem_QTableWidgetItem_0>(value: T) -> QTableWidgetItem {
    let rsthis = value.QTableWidgetItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetItem_QTableWidgetItem_0 {
  fn QTableWidgetItem_0(self) -> QTableWidgetItem;
}
// QTableWidgetItem(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTableWidgetItem_QTableWidgetItem_0 for (i32) {
  fn QTableWidgetItem_0(self) -> QTableWidgetItem {
    // unsafe{_ZN16QTableWidgetItemC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTableWidgetItemC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTableWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:83
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTableWidgetItem(const QString &, int)

/*

*/
// QTableWidgetItem(const QString &, int) ctx.fn_proto_cpp
impl /*struct*/ QTableWidgetItem {
  pub fn QTableWidgetItem_1<T: QTableWidgetItem_QTableWidgetItem_1>(value: T) -> QTableWidgetItem {
    let rsthis = value.QTableWidgetItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetItem_QTableWidgetItem_1 {
  fn QTableWidgetItem_1(self) -> QTableWidgetItem;
}
// QTableWidgetItem(const QString &, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTableWidgetItem_QTableWidgetItem_1 for (usize,i32) {
  fn QTableWidgetItem_1(self) -> QTableWidgetItem {
    // unsafe{_ZN16QTableWidgetItemC2ERK7QStringi()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTableWidgetItemC2ERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTableWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:84
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTableWidgetItem(const QIcon &, const QString &, int)

/*

*/
// QTableWidgetItem(const QIcon &, const QString &, int) ctx.fn_proto_cpp
impl /*struct*/ QTableWidgetItem {
  pub fn QTableWidgetItem_2<T: QTableWidgetItem_QTableWidgetItem_2>(value: T) -> QTableWidgetItem {
    let rsthis = value.QTableWidgetItem_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetItem_QTableWidgetItem_2 {
  fn QTableWidgetItem_2(self) -> QTableWidgetItem;
}
// QTableWidgetItem(const QIcon &, const QString &, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTableWidgetItem_QTableWidgetItem_2 for (usize,usize,i32) {
  fn QTableWidgetItem_2(self) -> QTableWidgetItem {
    // unsafe{_ZN16QTableWidgetItemC2ERK5QIconRK7QStringi()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTableWidgetItemC2ERK5QIconRK7QStringi", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTableWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTableWidgetItem()

/*

*/
pub fn DeleteQTableWidgetItem(this :*mut QTableWidgetItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QTableWidgetItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtablewidget.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QTableWidgetItem * clone() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn clone_0<RetType, T: QTableWidgetItem_clone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clone_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_clone_0<RetType> {
  fn clone_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_clone_0<usize> for () {
  fn clone_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem5cloneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:90
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QTableWidget * tableWidget() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn tableWidget_0<RetType, T: QTableWidgetItem_tableWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tableWidget_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_tableWidget_0<RetType> {
  fn tableWidget_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_tableWidget_0<usize> for () {
  fn tableWidget_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem11tableWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int row() const

/*
Returns the row for the item.
*/
impl /*struct*/ QTableWidgetItem {
  pub fn row_0<RetType, T: QTableWidgetItem_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_row_0<RetType> {
  fn row_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_row_0<i32> for () {
  fn row_0(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem3rowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int column() const

/*
Returns the column for the item.
*/
impl /*struct*/ QTableWidgetItem {
  pub fn column_0<RetType, T: QTableWidgetItem_column_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.column_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_column_0<RetType> {
  fn column_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_column_0<i32> for () {
  fn column_0(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem6columnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:95
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSelected(bool)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setSelected_0<RetType, T: QTableWidgetItem_setSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelected_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setSelected_0<RetType> {
  fn setSelected_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setSelected_0<(/*void*/)> for (bool) {
  fn setSelected_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem11setSelectedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:96
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSelected() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn isSelected_0<RetType, T: QTableWidgetItem_isSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSelected_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_isSelected_0<RetType> {
  fn isSelected_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_isSelected_0<bool> for () {
  fn isSelected_0(self , rsthis: & QTableWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem10isSelectedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:98
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn flags_0<RetType, T: QTableWidgetItem_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_flags_0<RetType> {
  fn flags_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(Qt::ItemFlags)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setFlags_0<RetType, T: QTableWidgetItem_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem8setFlagsE6QFlagsIN2Qt8ItemFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:101
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn text_0<RetType, T: QTableWidgetItem_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_text_0<RetType> {
  fn text_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_text_0<usize> for () {
  fn text_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:103
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setText_0<RetType, T: QTableWidgetItem_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setText_0<RetType> {
  fn setText_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QIcon icon() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn icon_0<RetType, T: QTableWidgetItem_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_icon_0<RetType> {
  fn icon_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_icon_0<usize> for () {
  fn icon_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:107
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setIcon(const QIcon &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setIcon_0<RetType, T: QTableWidgetItem_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setIcon_0<(/*void*/)> for (usize) {
  fn setIcon_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem7setIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:109
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString statusTip() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn statusTip_0<RetType, T: QTableWidgetItem_statusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.statusTip_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_statusTip_0<RetType> {
  fn statusTip_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_statusTip_0<usize> for () {
  fn statusTip_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem9statusTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:111
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setStatusTip(const QString &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setStatusTip_0<RetType, T: QTableWidgetItem_setStatusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setStatusTip_0<RetType> {
  fn setStatusTip_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setStatusTip_0<(/*void*/)> for (usize) {
  fn setStatusTip_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem12setStatusTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:114
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toolTip() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn toolTip_0<RetType, T: QTableWidgetItem_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_toolTip_0<usize> for () {
  fn toolTip_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem7toolTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:116
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setToolTip(const QString &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setToolTip_0<RetType, T: QTableWidgetItem_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setToolTip_0<(/*void*/)> for (usize) {
  fn setToolTip_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem10setToolTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString whatsThis() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn whatsThis_0<RetType, T: QTableWidgetItem_whatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.whatsThis_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_whatsThis_0<RetType> {
  fn whatsThis_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_whatsThis_0<usize> for () {
  fn whatsThis_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem9whatsThisEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWhatsThis(const QString &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setWhatsThis_0<RetType, T: QTableWidgetItem_setWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setWhatsThis_0<RetType> {
  fn setWhatsThis_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setWhatsThis_0<(/*void*/)> for (usize) {
  fn setWhatsThis_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem12setWhatsThisERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn font_0<RetType, T: QTableWidgetItem_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_font_0<RetType> {
  fn font_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_font_0<usize> for () {
  fn font_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setFont_0<RetType, T: QTableWidgetItem_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:129
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int textAlignment() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn textAlignment_0<RetType, T: QTableWidgetItem_textAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textAlignment_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_textAlignment_0<RetType> {
  fn textAlignment_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_textAlignment_0<i32> for () {
  fn textAlignment_0(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem13textAlignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:131
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextAlignment(int)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setTextAlignment_0<RetType, T: QTableWidgetItem_setTextAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextAlignment_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setTextAlignment_0<RetType> {
  fn setTextAlignment_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setTextAlignment_0<(/*void*/)> for (i32) {
  fn setTextAlignment_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem16setTextAlignmentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:134
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QColor backgroundColor() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn backgroundColor_0<RetType, T: QTableWidgetItem_backgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_backgroundColor_0<RetType> {
  fn backgroundColor_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_backgroundColor_0<usize> for () {
  fn backgroundColor_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem15backgroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:136
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBackgroundColor(const QColor &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setBackgroundColor_0<RetType, T: QTableWidgetItem_setBackgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundColor_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setBackgroundColor_0<RetType> {
  fn setBackgroundColor_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setBackgroundColor_0<(/*void*/)> for (usize) {
  fn setBackgroundColor_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem18setBackgroundColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:139
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush background() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn background_0<RetType, T: QTableWidgetItem_background_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.background_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_background_0<RetType> {
  fn background_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_background_0<usize> for () {
  fn background_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem10backgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:141
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBackground(const QBrush &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setBackground_0<RetType, T: QTableWidgetItem_setBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackground_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setBackground_0<RetType> {
  fn setBackground_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setBackground_0<(/*void*/)> for (usize) {
  fn setBackground_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem13setBackgroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:144
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QColor textColor() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn textColor_0<RetType, T: QTableWidgetItem_textColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textColor_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_textColor_0<RetType> {
  fn textColor_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_textColor_0<usize> for () {
  fn textColor_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem9textColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:146
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextColor(const QColor &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setTextColor_0<RetType, T: QTableWidgetItem_setTextColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextColor_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setTextColor_0<RetType> {
  fn setTextColor_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setTextColor_0<(/*void*/)> for (usize) {
  fn setTextColor_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem12setTextColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:149
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush foreground() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn foreground_0<RetType, T: QTableWidgetItem_foreground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foreground_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_foreground_0<RetType> {
  fn foreground_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_foreground_0<usize> for () {
  fn foreground_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem10foregroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:151
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setForeground(const QBrush &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setForeground_0<RetType, T: QTableWidgetItem_setForeground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setForeground_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setForeground_0<RetType> {
  fn setForeground_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setForeground_0<(/*void*/)> for (usize) {
  fn setForeground_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem13setForegroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:154
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::CheckState checkState() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn checkState_0<RetType, T: QTableWidgetItem_checkState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkState_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_checkState_0<RetType> {
  fn checkState_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_checkState_0<i32> for () {
  fn checkState_0(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem10checkStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:156
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCheckState(Qt::CheckState)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setCheckState_0<RetType, T: QTableWidgetItem_setCheckState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckState_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setCheckState_0<RetType> {
  fn setCheckState_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setCheckState_0<(/*void*/)> for (i32) {
  fn setCheckState_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem13setCheckStateEN2Qt10CheckStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:159
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn sizeHint_0<RetType, T: QTableWidgetItem_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:161
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSizeHint(const QSize &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setSizeHint_0<RetType, T: QTableWidgetItem_setSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeHint_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setSizeHint_0<RetType> {
  fn setSizeHint_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setSizeHint_0<(/*void*/)> for (usize) {
  fn setSizeHint_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem11setSizeHintERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:164
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(int) const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn data_0<RetType, T: QTableWidgetItem_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_data_0<RetType> {
  fn data_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_data_0<usize> for (i32) {
  fn data_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem4dataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:165
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setData(int, const QVariant &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn setData_0<RetType, T: QTableWidgetItem_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_setData_0<RetType> {
  fn setData_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_setData_0<(/*void*/)> for (i32,usize) {
  fn setData_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem7setDataEiRK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:167
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool operator<(const QTableWidgetItem &) const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn operator_less_than_0<RetType, T: QTableWidgetItem_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QTableWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItemltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:170
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void read(QDataStream &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn read_0<RetType, T: QTableWidgetItem_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_read_0<RetType> {
  fn read_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_read_0<(/*void*/)> for (usize) {
  fn read_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTableWidgetItem4readER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:171
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void write(QDataStream &) const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn write_0<RetType, T: QTableWidgetItem_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_write_0<RetType> {
  fn write_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_write_0<(/*void*/)> for (usize) {
  fn write_0(self , rsthis: & QTableWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem5writeER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:173
// index:0
// Public Visibility=Default Availability=Available
// [48] QTableWidgetItem & operator=(const QTableWidgetItem &)

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn operator_equal_0<RetType, T: QTableWidgetItem_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTableWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QTableWidgetItemaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:175
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int type() const

/*

*/
impl /*struct*/ QTableWidgetItem {
  pub fn type__0<RetType, T: QTableWidgetItem_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QTableWidgetItem_type__0<RetType> {
  fn type__0(self , rsthis: & QTableWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTableWidgetItem_type__0<i32> for () {
  fn type__0(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTableWidgetItem4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QTableWidgetItem__ItemType = i32;
// 
pub const QTableWidgetItem__Type :QTableWidgetItem__ItemType = 0;
// 
pub const QTableWidgetItem__UserType :QTableWidgetItem__ItemType = 1000;
pub fn QTableWidgetItem_ItemTypeItemName(val: i32) ->String {
  match val {
     QTableWidgetItem__Type => // 0
     {return String::from("Type");}
     QTableWidgetItem__UserType => // 1000
     {return String::from("UserType");}
  _ => {return format!("{}", val);}
}
}
pub fn QTableWidgetItem_ItemTypeItemName_s(val: i32) ->String {
  //var nilthis *QTableWidgetItem
  //return nilthis.ItemTypeItemName(val);
  return QTableWidgetItem_ItemTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
