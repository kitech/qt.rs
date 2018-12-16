

// mod ::widgets::QListWidgetItem
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
// extern C begin: 68
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
#[derive(Default)] // class sizeof(QListWidgetItem)=48
pub struct QListWidgetItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QListWidgetItem_ITF interface {
//    QListWidgetItem_PTR() *QListWidgetItem
//}
//func (ptr *QListWidgetItem) QListWidgetItem_PTR() *QListWidgetItem { return ptr }

impl /*struct*/ QListWidgetItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QListWidgetItem {
    return QListWidgetItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QListWidgetItem {
//  type Target = QListWidgetItemBASE;
//
//  fn deref(&self) -> &QListWidgetItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QListWidgetItemBASE> for QListWidgetItem {
//  fn as_ref(& self) -> & QListWidgetItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlistwidget.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QListWidgetItem(QListWidget *, int)

/*

*/
// QListWidgetItem(QListWidget *, int) ctx.fn_proto_cpp
impl /*struct*/ QListWidgetItem {
  pub fn QListWidgetItem_0<T: QListWidgetItem_QListWidgetItem_0>(value: T) -> QListWidgetItem {
    let rsthis = value.QListWidgetItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidgetItem_QListWidgetItem_0 {
  fn QListWidgetItem_0(self) -> QListWidgetItem;
}
// QListWidgetItem(QListWidget *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QListWidgetItem_QListWidgetItem_0 for (usize,i32) {
  fn QListWidgetItem_0(self) -> QListWidgetItem {
    // unsafe{_ZN15QListWidgetItemC2EP11QListWidgeti()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QListWidgetItemC2EP11QListWidgeti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QListWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:65
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QListWidgetItem(const QString &, QListWidget *, int)

/*

*/
// QListWidgetItem(const QString &, QListWidget *, int) ctx.fn_proto_cpp
impl /*struct*/ QListWidgetItem {
  pub fn QListWidgetItem_1<T: QListWidgetItem_QListWidgetItem_1>(value: T) -> QListWidgetItem {
    let rsthis = value.QListWidgetItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidgetItem_QListWidgetItem_1 {
  fn QListWidgetItem_1(self) -> QListWidgetItem;
}
// QListWidgetItem(const QString &, QListWidget *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QListWidgetItem_QListWidgetItem_1 for (usize,usize,i32) {
  fn QListWidgetItem_1(self) -> QListWidgetItem {
    // unsafe{_ZN15QListWidgetItemC2ERK7QStringP11QListWidgeti()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QListWidgetItemC2ERK7QStringP11QListWidgeti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QListWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:66
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QListWidgetItem(const QIcon &, const QString &, QListWidget *, int)

/*

*/
// QListWidgetItem(const QIcon &, const QString &, QListWidget *, int) ctx.fn_proto_cpp
impl /*struct*/ QListWidgetItem {
  pub fn QListWidgetItem_2<T: QListWidgetItem_QListWidgetItem_2>(value: T) -> QListWidgetItem {
    let rsthis = value.QListWidgetItem_2();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidgetItem_QListWidgetItem_2 {
  fn QListWidgetItem_2(self) -> QListWidgetItem;
}
// QListWidgetItem(const QIcon &, const QString &, QListWidget *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QListWidgetItem_QListWidgetItem_2 for (usize,usize,usize,i32) {
  fn QListWidgetItem_2(self) -> QListWidgetItem {
    // unsafe{_ZN15QListWidgetItemC2ERK5QIconRK7QStringP11QListWidgeti()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QListWidgetItemC2ERK5QIconRK7QStringP11QListWidgeti", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QListWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QListWidgetItem()

/*

*/
pub fn DeleteQListWidgetItem(this :*mut QListWidgetItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QListWidgetItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlistwidget.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QListWidgetItem * clone() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn clone_0<RetType, T: QListWidgetItem_clone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clone_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_clone_0<RetType> {
  fn clone_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_clone_0<usize> for () {
  fn clone_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem5cloneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QListWidget * listWidget() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn listWidget_0<RetType, T: QListWidgetItem_listWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.listWidget_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_listWidget_0<RetType> {
  fn listWidget_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_listWidget_0<usize> for () {
  fn listWidget_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem10listWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSelected(bool)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setSelected_0<RetType, T: QListWidgetItem_setSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelected_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setSelected_0<RetType> {
  fn setSelected_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setSelected_0<(/*void*/)> for (bool) {
  fn setSelected_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem11setSelectedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSelected() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn isSelected_0<RetType, T: QListWidgetItem_isSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSelected_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_isSelected_0<RetType> {
  fn isSelected_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_isSelected_0<bool> for () {
  fn isSelected_0(self , rsthis: & QListWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem10isSelectedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHidden(bool)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setHidden_0<RetType, T: QListWidgetItem_setHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHidden_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setHidden_0<RetType> {
  fn setHidden_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setHidden_0<(/*void*/)> for (bool) {
  fn setHidden_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem9setHiddenEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isHidden() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn isHidden_0<RetType, T: QListWidgetItem_isHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isHidden_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_isHidden_0<RetType> {
  fn isHidden_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_isHidden_0<bool> for () {
  fn isHidden_0(self , rsthis: & QListWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem8isHiddenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn flags_0<RetType, T: QListWidgetItem_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_flags_0<RetType> {
  fn flags_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(Qt::ItemFlags)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setFlags_0<RetType, T: QListWidgetItem_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem8setFlagsE6QFlagsIN2Qt8ItemFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn text_0<RetType, T: QListWidgetItem_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_text_0<RetType> {
  fn text_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_text_0<usize> for () {
  fn text_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setText_0<RetType, T: QListWidgetItem_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setText_0<RetType> {
  fn setText_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:88
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QIcon icon() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn icon_0<RetType, T: QListWidgetItem_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_icon_0<RetType> {
  fn icon_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_icon_0<usize> for () {
  fn icon_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:90
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setIcon(const QIcon &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setIcon_0<RetType, T: QListWidgetItem_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setIcon_0<(/*void*/)> for (usize) {
  fn setIcon_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem7setIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString statusTip() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn statusTip_0<RetType, T: QListWidgetItem_statusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.statusTip_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_statusTip_0<RetType> {
  fn statusTip_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_statusTip_0<usize> for () {
  fn statusTip_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem9statusTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:94
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setStatusTip(const QString &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setStatusTip_0<RetType, T: QListWidgetItem_setStatusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setStatusTip_0<RetType> {
  fn setStatusTip_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setStatusTip_0<(/*void*/)> for (usize) {
  fn setStatusTip_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem12setStatusTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:97
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toolTip() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn toolTip_0<RetType, T: QListWidgetItem_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_toolTip_0<usize> for () {
  fn toolTip_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem7toolTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:99
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setToolTip(const QString &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setToolTip_0<RetType, T: QListWidgetItem_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setToolTip_0<(/*void*/)> for (usize) {
  fn setToolTip_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem10setToolTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:103
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString whatsThis() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn whatsThis_0<RetType, T: QListWidgetItem_whatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.whatsThis_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_whatsThis_0<RetType> {
  fn whatsThis_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_whatsThis_0<usize> for () {
  fn whatsThis_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem9whatsThisEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWhatsThis(const QString &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setWhatsThis_0<RetType, T: QListWidgetItem_setWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setWhatsThis_0<RetType> {
  fn setWhatsThis_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setWhatsThis_0<(/*void*/)> for (usize) {
  fn setWhatsThis_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem12setWhatsThisERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn font_0<RetType, T: QListWidgetItem_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_font_0<RetType> {
  fn font_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_font_0<usize> for () {
  fn font_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:110
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setFont_0<RetType, T: QListWidgetItem_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int textAlignment() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn textAlignment_0<RetType, T: QListWidgetItem_textAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textAlignment_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_textAlignment_0<RetType> {
  fn textAlignment_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_textAlignment_0<i32> for () {
  fn textAlignment_0(self , rsthis: & QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem13textAlignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:114
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextAlignment(int)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setTextAlignment_0<RetType, T: QListWidgetItem_setTextAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextAlignment_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setTextAlignment_0<RetType> {
  fn setTextAlignment_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setTextAlignment_0<(/*void*/)> for (i32) {
  fn setTextAlignment_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem16setTextAlignmentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:117
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QColor backgroundColor() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn backgroundColor_0<RetType, T: QListWidgetItem_backgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_backgroundColor_0<RetType> {
  fn backgroundColor_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_backgroundColor_0<usize> for () {
  fn backgroundColor_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem15backgroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:119
// index:0
// Public inline virtual Visibility=Default Availability=Available
// [-2] void setBackgroundColor(const QColor &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setBackgroundColor_0<RetType, T: QListWidgetItem_setBackgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundColor_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setBackgroundColor_0<RetType> {
  fn setBackgroundColor_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setBackgroundColor_0<(/*void*/)> for (usize) {
  fn setBackgroundColor_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem18setBackgroundColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush background() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn background_0<RetType, T: QListWidgetItem_background_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.background_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_background_0<RetType> {
  fn background_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_background_0<usize> for () {
  fn background_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem10backgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:124
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBackground(const QBrush &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setBackground_0<RetType, T: QListWidgetItem_setBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackground_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setBackground_0<RetType> {
  fn setBackground_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setBackground_0<(/*void*/)> for (usize) {
  fn setBackground_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem13setBackgroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QColor textColor() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn textColor_0<RetType, T: QListWidgetItem_textColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textColor_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_textColor_0<RetType> {
  fn textColor_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_textColor_0<usize> for () {
  fn textColor_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem9textColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:129
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextColor(const QColor &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setTextColor_0<RetType, T: QListWidgetItem_setTextColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextColor_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setTextColor_0<RetType> {
  fn setTextColor_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setTextColor_0<(/*void*/)> for (usize) {
  fn setTextColor_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem12setTextColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:132
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush foreground() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn foreground_0<RetType, T: QListWidgetItem_foreground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foreground_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_foreground_0<RetType> {
  fn foreground_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_foreground_0<usize> for () {
  fn foreground_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem10foregroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:134
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setForeground(const QBrush &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setForeground_0<RetType, T: QListWidgetItem_setForeground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setForeground_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setForeground_0<RetType> {
  fn setForeground_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setForeground_0<(/*void*/)> for (usize) {
  fn setForeground_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem13setForegroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:137
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::CheckState checkState() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn checkState_0<RetType, T: QListWidgetItem_checkState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkState_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_checkState_0<RetType> {
  fn checkState_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_checkState_0<i32> for () {
  fn checkState_0(self , rsthis: & QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem10checkStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:139
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCheckState(Qt::CheckState)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setCheckState_0<RetType, T: QListWidgetItem_setCheckState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckState_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setCheckState_0<RetType> {
  fn setCheckState_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setCheckState_0<(/*void*/)> for (i32) {
  fn setCheckState_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem13setCheckStateEN2Qt10CheckStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:142
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn sizeHint_0<RetType, T: QListWidgetItem_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:144
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSizeHint(const QSize &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setSizeHint_0<RetType, T: QListWidgetItem_setSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeHint_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setSizeHint_0<RetType> {
  fn setSizeHint_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setSizeHint_0<(/*void*/)> for (usize) {
  fn setSizeHint_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem11setSizeHintERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:147
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(int) const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn data_0<RetType, T: QListWidgetItem_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_data_0<RetType> {
  fn data_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_data_0<usize> for (i32) {
  fn data_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem4dataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:148
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setData(int, const QVariant &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn setData_0<RetType, T: QListWidgetItem_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_setData_0<RetType> {
  fn setData_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_setData_0<(/*void*/)> for (i32,usize) {
  fn setData_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem7setDataEiRK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:150
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool operator<(const QListWidgetItem &) const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn operator_less_than_0<RetType, T: QListWidgetItem_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QListWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItemltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:153
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void read(QDataStream &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn read_0<RetType, T: QListWidgetItem_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_read_0<RetType> {
  fn read_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_read_0<(/*void*/)> for (usize) {
  fn read_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QListWidgetItem4readER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:154
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void write(QDataStream &) const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn write_0<RetType, T: QListWidgetItem_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_write_0<RetType> {
  fn write_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_write_0<(/*void*/)> for (usize) {
  fn write_0(self , rsthis: & QListWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem5writeER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:156
// index:0
// Public Visibility=Default Availability=Available
// [48] QListWidgetItem & operator=(const QListWidgetItem &)

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn operator_equal_0<RetType, T: QListWidgetItem_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QListWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QListWidgetItemaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistwidget.h:158
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int type() const

/*

*/
impl /*struct*/ QListWidgetItem {
  pub fn type__0<RetType, T: QListWidgetItem_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QListWidgetItem_type__0<RetType> {
  fn type__0(self , rsthis: & QListWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QListWidgetItem_type__0<i32> for () {
  fn type__0(self , rsthis: & QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QListWidgetItem4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QListWidgetItem__ItemType = i32;
// 
pub const QListWidgetItem__Type :QListWidgetItem__ItemType = 0;
// 
pub const QListWidgetItem__UserType :QListWidgetItem__ItemType = 1000;
pub fn QListWidgetItem_ItemTypeItemName(val: i32) ->String {
  match val {
     QListWidgetItem__Type => // 0
     {return String::from("Type");}
     QListWidgetItem__UserType => // 1000
     {return String::from("UserType");}
  _ => {return format!("{}", val);}
}
}
pub fn QListWidgetItem_ItemTypeItemName_s(val: i32) ->String {
  //var nilthis *QListWidgetItem
  //return nilthis.ItemTypeItemName(val);
  return QListWidgetItem_ItemTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
