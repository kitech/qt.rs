

// mod ::widgets::QTreeWidgetItem
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
// extern C begin: 11
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

// void emitDataChanged()
// func (this *QTreeWidgetItem) InheritEmitDataChanged(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "emitDataChanged", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTreeWidgetItem)=64
pub struct QTreeWidgetItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTreeWidgetItem_ITF interface {
//    QTreeWidgetItem_PTR() *QTreeWidgetItem
//}
//func (ptr *QTreeWidgetItem) QTreeWidgetItem_PTR() *QTreeWidgetItem { return ptr }

impl /*struct*/ QTreeWidgetItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTreeWidgetItem {
    return QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTreeWidgetItem {
//  type Target = QTreeWidgetItemBASE;
//
//  fn deref(&self) -> &QTreeWidgetItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTreeWidgetItemBASE> for QTreeWidgetItem {
//  fn as_ref(& self) -> & QTreeWidgetItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtreewidget.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItem(int)

/*

*/
// QTreeWidgetItem(int) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItem {
  pub fn QTreeWidgetItem_0<T: QTreeWidgetItem_QTreeWidgetItem_0>(value: T) -> QTreeWidgetItem {
    let rsthis = value.QTreeWidgetItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_QTreeWidgetItem_0 {
  fn QTreeWidgetItem_0(self) -> QTreeWidgetItem;
}
// QTreeWidgetItem(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItem_QTreeWidgetItem_0 for (i32) {
  fn QTreeWidgetItem_0(self) -> QTreeWidgetItem {
    // unsafe{_ZN15QTreeWidgetItemC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:68
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItem(const QStringList &, int)

/*

*/
// QTreeWidgetItem(const QStringList &, int) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItem {
  pub fn QTreeWidgetItem_1<T: QTreeWidgetItem_QTreeWidgetItem_1>(value: T) -> QTreeWidgetItem {
    let rsthis = value.QTreeWidgetItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_QTreeWidgetItem_1 {
  fn QTreeWidgetItem_1(self) -> QTreeWidgetItem;
}
// QTreeWidgetItem(const QStringList &, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItem_QTreeWidgetItem_1 for (usize,i32) {
  fn QTreeWidgetItem_1(self) -> QTreeWidgetItem {
    // unsafe{_ZN15QTreeWidgetItemC2ERK11QStringListi()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemC2ERK11QStringListi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:69
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItem(QTreeWidget *, int)

/*

*/
// QTreeWidgetItem(QTreeWidget *, int) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItem {
  pub fn QTreeWidgetItem_2<T: QTreeWidgetItem_QTreeWidgetItem_2>(value: T) -> QTreeWidgetItem {
    let rsthis = value.QTreeWidgetItem_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_QTreeWidgetItem_2 {
  fn QTreeWidgetItem_2(self) -> QTreeWidgetItem;
}
// QTreeWidgetItem(QTreeWidget *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItem_QTreeWidgetItem_2 for (usize,i32) {
  fn QTreeWidgetItem_2(self) -> QTreeWidgetItem {
    // unsafe{_ZN15QTreeWidgetItemC2EP11QTreeWidgeti()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemC2EP11QTreeWidgeti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:70
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItem(QTreeWidget *, const QStringList &, int)

/*

*/
// QTreeWidgetItem(QTreeWidget *, const QStringList &, int) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItem {
  pub fn QTreeWidgetItem_3<T: QTreeWidgetItem_QTreeWidgetItem_3>(value: T) -> QTreeWidgetItem {
    let rsthis = value.QTreeWidgetItem_3();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_QTreeWidgetItem_3 {
  fn QTreeWidgetItem_3(self) -> QTreeWidgetItem;
}
// QTreeWidgetItem(QTreeWidget *, const QStringList &, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItem_QTreeWidgetItem_3 for (usize,usize,i32) {
  fn QTreeWidgetItem_3(self) -> QTreeWidgetItem {
    // unsafe{_ZN15QTreeWidgetItemC2EP11QTreeWidgetRK11QStringListi()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemC2EP11QTreeWidgetRK11QStringListi", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:71
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItem(QTreeWidget *, QTreeWidgetItem *, int)

/*

*/
// QTreeWidgetItem(QTreeWidget *, QTreeWidgetItem *, int) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItem {
  pub fn QTreeWidgetItem_4<T: QTreeWidgetItem_QTreeWidgetItem_4>(value: T) -> QTreeWidgetItem {
    let rsthis = value.QTreeWidgetItem_4();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_QTreeWidgetItem_4 {
  fn QTreeWidgetItem_4(self) -> QTreeWidgetItem;
}
// QTreeWidgetItem(QTreeWidget *, QTreeWidgetItem *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItem_QTreeWidgetItem_4 for (usize,usize,i32) {
  fn QTreeWidgetItem_4(self) -> QTreeWidgetItem {
    // unsafe{_ZN15QTreeWidgetItemC2EP11QTreeWidgetPS_i()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemC2EP11QTreeWidgetPS_i", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:72
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItem(QTreeWidgetItem *, int)

/*

*/
// QTreeWidgetItem(QTreeWidgetItem *, int) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItem {
  pub fn QTreeWidgetItem_5<T: QTreeWidgetItem_QTreeWidgetItem_5>(value: T) -> QTreeWidgetItem {
    let rsthis = value.QTreeWidgetItem_5();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_QTreeWidgetItem_5 {
  fn QTreeWidgetItem_5(self) -> QTreeWidgetItem;
}
// QTreeWidgetItem(QTreeWidgetItem *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItem_QTreeWidgetItem_5 for (usize,i32) {
  fn QTreeWidgetItem_5(self) -> QTreeWidgetItem {
    // unsafe{_ZN15QTreeWidgetItemC2EPS_i()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemC2EPS_i", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:73
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItem(QTreeWidgetItem *, const QStringList &, int)

/*

*/
// QTreeWidgetItem(QTreeWidgetItem *, const QStringList &, int) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItem {
  pub fn QTreeWidgetItem_6<T: QTreeWidgetItem_QTreeWidgetItem_6>(value: T) -> QTreeWidgetItem {
    let rsthis = value.QTreeWidgetItem_6();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_QTreeWidgetItem_6 {
  fn QTreeWidgetItem_6(self) -> QTreeWidgetItem;
}
// QTreeWidgetItem(QTreeWidgetItem *, const QStringList &, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItem_QTreeWidgetItem_6 for (usize,usize,i32) {
  fn QTreeWidgetItem_6(self) -> QTreeWidgetItem {
    // unsafe{_ZN15QTreeWidgetItemC2EPS_RK11QStringListi()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemC2EPS_RK11QStringListi", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:74
// index:7
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItem(QTreeWidgetItem *, QTreeWidgetItem *, int)

/*

*/
// QTreeWidgetItem(QTreeWidgetItem *, QTreeWidgetItem *, int) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItem {
  pub fn QTreeWidgetItem_7<T: QTreeWidgetItem_QTreeWidgetItem_7>(value: T) -> QTreeWidgetItem {
    let rsthis = value.QTreeWidgetItem_7();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_QTreeWidgetItem_7 {
  fn QTreeWidgetItem_7(self) -> QTreeWidgetItem;
}
// QTreeWidgetItem(QTreeWidgetItem *, QTreeWidgetItem *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItem_QTreeWidgetItem_7 for (usize,usize,i32) {
  fn QTreeWidgetItem_7(self) -> QTreeWidgetItem {
    // unsafe{_ZN15QTreeWidgetItemC2EPS_S0_i()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemC2EPS_S0_i", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTreeWidgetItem()

/*

*/
pub fn DeleteQTreeWidgetItem(this :*mut QTreeWidgetItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 64)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtreewidget.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QTreeWidgetItem * clone() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn clone_0<RetType, T: QTreeWidgetItem_clone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clone_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_clone_0<RetType> {
  fn clone_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_clone_0<usize> for () {
  fn clone_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem5cloneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QTreeWidget * treeWidget() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn treeWidget_0<RetType, T: QTreeWidgetItem_treeWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.treeWidget_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_treeWidget_0<RetType> {
  fn treeWidget_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_treeWidget_0<usize> for () {
  fn treeWidget_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem10treeWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSelected(bool)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setSelected_0<RetType, T: QTreeWidgetItem_setSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelected_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setSelected_0<RetType> {
  fn setSelected_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setSelected_0<(/*void*/)> for (bool) {
  fn setSelected_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem11setSelectedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSelected() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn isSelected_0<RetType, T: QTreeWidgetItem_isSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSelected_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_isSelected_0<RetType> {
  fn isSelected_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_isSelected_0<bool> for () {
  fn isSelected_0(self , rsthis: & QTreeWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem10isSelectedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:85
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHidden(bool)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setHidden_0<RetType, T: QTreeWidgetItem_setHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHidden_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setHidden_0<RetType> {
  fn setHidden_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setHidden_0<(/*void*/)> for (bool) {
  fn setHidden_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem9setHiddenEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isHidden() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn isHidden_0<RetType, T: QTreeWidgetItem_isHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isHidden_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_isHidden_0<RetType> {
  fn isHidden_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_isHidden_0<bool> for () {
  fn isHidden_0(self , rsthis: & QTreeWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem8isHiddenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:88
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setExpanded(bool)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setExpanded_0<RetType, T: QTreeWidgetItem_setExpanded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExpanded_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setExpanded_0<RetType> {
  fn setExpanded_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setExpanded_0<(/*void*/)> for (bool) {
  fn setExpanded_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem11setExpandedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isExpanded() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn isExpanded_0<RetType, T: QTreeWidgetItem_isExpanded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isExpanded_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_isExpanded_0<RetType> {
  fn isExpanded_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_isExpanded_0<bool> for () {
  fn isExpanded_0(self , rsthis: & QTreeWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem10isExpandedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFirstColumnSpanned(bool)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setFirstColumnSpanned_0<RetType, T: QTreeWidgetItem_setFirstColumnSpanned_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFirstColumnSpanned_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setFirstColumnSpanned_0<RetType> {
  fn setFirstColumnSpanned_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setFirstColumnSpanned_0<(/*void*/)> for (bool) {
  fn setFirstColumnSpanned_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem21setFirstColumnSpannedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isFirstColumnSpanned() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn isFirstColumnSpanned_0<RetType, T: QTreeWidgetItem_isFirstColumnSpanned_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFirstColumnSpanned_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_isFirstColumnSpanned_0<RetType> {
  fn isFirstColumnSpanned_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_isFirstColumnSpanned_0<bool> for () {
  fn isFirstColumnSpanned_0(self , rsthis: & QTreeWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem20isFirstColumnSpannedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:94
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setDisabled(bool)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setDisabled_0<RetType, T: QTreeWidgetItem_setDisabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDisabled_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setDisabled_0<RetType> {
  fn setDisabled_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setDisabled_0<(/*void*/)> for (bool) {
  fn setDisabled_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem11setDisabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:95
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDisabled() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn isDisabled_0<RetType, T: QTreeWidgetItem_isDisabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDisabled_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_isDisabled_0<RetType> {
  fn isDisabled_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_isDisabled_0<bool> for () {
  fn isDisabled_0(self , rsthis: & QTreeWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem10isDisabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChildIndicatorPolicy(QTreeWidgetItem::ChildIndicatorPolicy)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setChildIndicatorPolicy_0<RetType, T: QTreeWidgetItem_setChildIndicatorPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChildIndicatorPolicy_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setChildIndicatorPolicy_0<RetType> {
  fn setChildIndicatorPolicy_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setChildIndicatorPolicy_0<(/*void*/)> for (i32) {
  fn setChildIndicatorPolicy_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem23setChildIndicatorPolicyENS_20ChildIndicatorPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] QTreeWidgetItem::ChildIndicatorPolicy childIndicatorPolicy() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn childIndicatorPolicy_0<RetType, T: QTreeWidgetItem_childIndicatorPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childIndicatorPolicy_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_childIndicatorPolicy_0<RetType> {
  fn childIndicatorPolicy_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_childIndicatorPolicy_0<i32> for () {
  fn childIndicatorPolicy_0(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem20childIndicatorPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn flags_0<RetType, T: QTreeWidgetItem_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_flags_0<RetType> {
  fn flags_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(Qt::ItemFlags)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setFlags_0<RetType, T: QTreeWidgetItem_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem8setFlagsE6QFlagsIN2Qt8ItemFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:104
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString text(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn text_0<RetType, T: QTreeWidgetItem_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_text_0<RetType> {
  fn text_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_text_0<usize> for (i32) {
  fn text_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem4textEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:106
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setText(int, const QString &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setText_0<RetType, T: QTreeWidgetItem_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setText_0<RetType> {
  fn setText_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setText_0<(/*void*/)> for (i32,usize) {
  fn setText_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem7setTextEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QIcon icon(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn icon_0<RetType, T: QTreeWidgetItem_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_icon_0<RetType> {
  fn icon_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_icon_0<usize> for (i32) {
  fn icon_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem4iconEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:110
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setIcon(int, const QIcon &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setIcon_0<RetType, T: QTreeWidgetItem_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setIcon_0<(/*void*/)> for (i32,usize) {
  fn setIcon_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem7setIconEiRK5QIcon", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString statusTip(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn statusTip_0<RetType, T: QTreeWidgetItem_statusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.statusTip_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_statusTip_0<RetType> {
  fn statusTip_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_statusTip_0<usize> for (i32) {
  fn statusTip_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem9statusTipEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:114
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setStatusTip(int, const QString &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setStatusTip_0<RetType, T: QTreeWidgetItem_setStatusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setStatusTip_0<RetType> {
  fn setStatusTip_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setStatusTip_0<(/*void*/)> for (i32,usize) {
  fn setStatusTip_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem12setStatusTipEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:117
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toolTip(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn toolTip_0<RetType, T: QTreeWidgetItem_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_toolTip_0<usize> for (i32) {
  fn toolTip_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem7toolTipEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:119
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setToolTip(int, const QString &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setToolTip_0<RetType, T: QTreeWidgetItem_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setToolTip_0<(/*void*/)> for (i32,usize) {
  fn setToolTip_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem10setToolTipEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:123
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString whatsThis(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn whatsThis_0<RetType, T: QTreeWidgetItem_whatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.whatsThis_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_whatsThis_0<RetType> {
  fn whatsThis_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_whatsThis_0<usize> for (i32) {
  fn whatsThis_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem9whatsThisEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWhatsThis(int, const QString &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setWhatsThis_0<RetType, T: QTreeWidgetItem_setWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setWhatsThis_0<RetType> {
  fn setWhatsThis_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setWhatsThis_0<(/*void*/)> for (i32,usize) {
  fn setWhatsThis_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem12setWhatsThisEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QFont font(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn font_0<RetType, T: QTreeWidgetItem_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_font_0<RetType> {
  fn font_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_font_0<usize> for (i32) {
  fn font_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem4fontEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:130
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFont(int, const QFont &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setFont_0<RetType, T: QTreeWidgetItem_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setFont_0<(/*void*/)> for (i32,usize) {
  fn setFont_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem7setFontEiRK5QFont", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:132
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int textAlignment(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn textAlignment_0<RetType, T: QTreeWidgetItem_textAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textAlignment_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_textAlignment_0<RetType> {
  fn textAlignment_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_textAlignment_0<i32> for (i32) {
  fn textAlignment_0(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem13textAlignmentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:134
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextAlignment(int, int)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setTextAlignment_0<RetType, T: QTreeWidgetItem_setTextAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextAlignment_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setTextAlignment_0<RetType> {
  fn setTextAlignment_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setTextAlignment_0<(/*void*/)> for (i32,i32) {
  fn setTextAlignment_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem16setTextAlignmentEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:137
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QColor backgroundColor(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn backgroundColor_0<RetType, T: QTreeWidgetItem_backgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_backgroundColor_0<RetType> {
  fn backgroundColor_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_backgroundColor_0<usize> for (i32) {
  fn backgroundColor_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem15backgroundColorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:139
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBackgroundColor(int, const QColor &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setBackgroundColor_0<RetType, T: QTreeWidgetItem_setBackgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundColor_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setBackgroundColor_0<RetType> {
  fn setBackgroundColor_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setBackgroundColor_0<(/*void*/)> for (i32,usize) {
  fn setBackgroundColor_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem18setBackgroundColorEiRK6QColor", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:142
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush background(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn background_0<RetType, T: QTreeWidgetItem_background_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.background_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_background_0<RetType> {
  fn background_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_background_0<usize> for (i32) {
  fn background_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem10backgroundEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:144
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBackground(int, const QBrush &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setBackground_0<RetType, T: QTreeWidgetItem_setBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackground_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setBackground_0<RetType> {
  fn setBackground_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setBackground_0<(/*void*/)> for (i32,usize) {
  fn setBackground_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem13setBackgroundEiRK6QBrush", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:147
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QColor textColor(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn textColor_0<RetType, T: QTreeWidgetItem_textColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textColor_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_textColor_0<RetType> {
  fn textColor_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_textColor_0<usize> for (i32) {
  fn textColor_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem9textColorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:149
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextColor(int, const QColor &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setTextColor_0<RetType, T: QTreeWidgetItem_setTextColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextColor_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setTextColor_0<RetType> {
  fn setTextColor_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setTextColor_0<(/*void*/)> for (i32,usize) {
  fn setTextColor_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem12setTextColorEiRK6QColor", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:152
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush foreground(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn foreground_0<RetType, T: QTreeWidgetItem_foreground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foreground_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_foreground_0<RetType> {
  fn foreground_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_foreground_0<usize> for (i32) {
  fn foreground_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem10foregroundEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:154
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setForeground(int, const QBrush &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setForeground_0<RetType, T: QTreeWidgetItem_setForeground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setForeground_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setForeground_0<RetType> {
  fn setForeground_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setForeground_0<(/*void*/)> for (i32,usize) {
  fn setForeground_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem13setForegroundEiRK6QBrush", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:157
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::CheckState checkState(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn checkState_0<RetType, T: QTreeWidgetItem_checkState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkState_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_checkState_0<RetType> {
  fn checkState_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_checkState_0<i32> for (i32) {
  fn checkState_0(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem10checkStateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:159
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCheckState(int, Qt::CheckState)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setCheckState_0<RetType, T: QTreeWidgetItem_setCheckState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckState_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setCheckState_0<RetType> {
  fn setCheckState_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setCheckState_0<(/*void*/)> for (i32,i32) {
  fn setCheckState_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem13setCheckStateEiN2Qt10CheckStateE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:162
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize sizeHint(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn sizeHint_0<RetType, T: QTreeWidgetItem_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_sizeHint_0<usize> for (i32) {
  fn sizeHint_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem8sizeHintEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:164
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSizeHint(int, const QSize &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setSizeHint_0<RetType, T: QTreeWidgetItem_setSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeHint_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setSizeHint_0<RetType> {
  fn setSizeHint_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setSizeHint_0<(/*void*/)> for (i32,usize) {
  fn setSizeHint_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem11setSizeHintEiRK5QSize", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:167
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(int, int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn data_0<RetType, T: QTreeWidgetItem_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_data_0<RetType> {
  fn data_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_data_0<usize> for (i32,i32) {
  fn data_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem4dataEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:168
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setData(int, int, const QVariant &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn setData_0<RetType, T: QTreeWidgetItem_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_setData_0<RetType> {
  fn setData_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_setData_0<(/*void*/)> for (i32,i32,usize) {
  fn setData_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem7setDataEiiRK8QVariant", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:170
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool operator<(const QTreeWidgetItem &) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn operator_less_than_0<RetType, T: QTreeWidgetItem_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QTreeWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItemltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:173
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void read(QDataStream &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn read_0<RetType, T: QTreeWidgetItem_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_read_0<RetType> {
  fn read_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_read_0<(/*void*/)> for (usize) {
  fn read_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem4readER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:174
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void write(QDataStream &) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn write_0<RetType, T: QTreeWidgetItem_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_write_0<RetType> {
  fn write_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_write_0<(/*void*/)> for (usize) {
  fn write_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem5writeER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:176
// index:0
// Public Visibility=Default Availability=Available
// [64] QTreeWidgetItem & operator=(const QTreeWidgetItem &)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn operator_equal_0<RetType, T: QTreeWidgetItem_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItemaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:178
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QTreeWidgetItem * parent() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn parent_0<RetType, T: QTreeWidgetItem_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_parent_0<RetType> {
  fn parent_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:179
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QTreeWidgetItem * child(int) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn child_0<RetType, T: QTreeWidgetItem_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_child_0<RetType> {
  fn child_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_child_0<usize> for (i32) {
  fn child_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem5childEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:185
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int childCount() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn childCount_0<RetType, T: QTreeWidgetItem_childCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childCount_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_childCount_0<RetType> {
  fn childCount_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_childCount_0<i32> for () {
  fn childCount_0(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem10childCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:186
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int columnCount() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn columnCount_0<RetType, T: QTreeWidgetItem_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:187
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int indexOfChild(QTreeWidgetItem *) const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn indexOfChild_0<RetType, T: QTreeWidgetItem_indexOfChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfChild_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_indexOfChild_0<RetType> {
  fn indexOfChild_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_indexOfChild_0<i32> for (usize) {
  fn indexOfChild_0(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem12indexOfChildEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addChild(QTreeWidgetItem *)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn addChild_0<RetType, T: QTreeWidgetItem_addChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addChild_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_addChild_0<RetType> {
  fn addChild_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_addChild_0<(/*void*/)> for (usize) {
  fn addChild_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem8addChildEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertChild(int, QTreeWidgetItem *)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn insertChild_0<RetType, T: QTreeWidgetItem_insertChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertChild_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_insertChild_0<RetType> {
  fn insertChild_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_insertChild_0<(/*void*/)> for (i32,usize) {
  fn insertChild_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem11insertChildEiPS_", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeChild(QTreeWidgetItem *)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn removeChild_0<RetType, T: QTreeWidgetItem_removeChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeChild_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_removeChild_0<RetType> {
  fn removeChild_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_removeChild_0<(/*void*/)> for (usize) {
  fn removeChild_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem11removeChildEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:192
// index:0
// Public Visibility=Default Availability=Available
// [8] QTreeWidgetItem * takeChild(int)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn takeChild_0<RetType, T: QTreeWidgetItem_takeChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeChild_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_takeChild_0<RetType> {
  fn takeChild_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_takeChild_0<usize> for (i32) {
  fn takeChild_0(self , rsthis: & QTreeWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem9takeChildEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:198
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int type() const

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn type__0<RetType, T: QTreeWidgetItem_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_type__0<RetType> {
  fn type__0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_type__0<i32> for () {
  fn type__0(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTreeWidgetItem4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:199
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void sortChildren(int, Qt::SortOrder)

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn sortChildren_0<RetType, T: QTreeWidgetItem_sortChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortChildren_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_sortChildren_0<RetType> {
  fn sortChildren_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_sortChildren_0<(/*void*/)> for (i32,i32) {
  fn sortChildren_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem12sortChildrenEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidget.h:203
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void emitDataChanged()

/*

*/
impl /*struct*/ QTreeWidgetItem {
  pub fn emitDataChanged_0<RetType, T: QTreeWidgetItem_emitDataChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.emitDataChanged_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItem_emitDataChanged_0<RetType> {
  fn emitDataChanged_0(self , rsthis: & QTreeWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItem_emitDataChanged_0<(/*void*/)> for () {
  fn emitDataChanged_0(self , rsthis: & QTreeWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QTreeWidgetItem15emitDataChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QTreeWidgetItem__ItemType = i32;
// 
pub const QTreeWidgetItem__Type :QTreeWidgetItem__ItemType = 0;
// 
pub const QTreeWidgetItem__UserType :QTreeWidgetItem__ItemType = 1000;
pub fn QTreeWidgetItem_ItemTypeItemName(val: i32) ->String {
  match val {
     QTreeWidgetItem__Type => // 0
     {return String::from("Type");}
     QTreeWidgetItem__UserType => // 1000
     {return String::from("UserType");}
  _ => {return format!("{}", val);}
}
}
pub fn QTreeWidgetItem_ItemTypeItemName_s(val: i32) ->String {
  //var nilthis *QTreeWidgetItem
  //return nilthis.ItemTypeItemName(val);
  return QTreeWidgetItem_ItemTypeItemName(val);
}


/*


*/
pub type QTreeWidgetItem__ChildIndicatorPolicy = i32;
// 
pub const QTreeWidgetItem__ShowIndicator :QTreeWidgetItem__ChildIndicatorPolicy = 0;
// 
pub const QTreeWidgetItem__DontShowIndicator :QTreeWidgetItem__ChildIndicatorPolicy = 1;
// 
pub const QTreeWidgetItem__DontShowIndicatorWhenChildless :QTreeWidgetItem__ChildIndicatorPolicy = 2;
pub fn QTreeWidgetItem_ChildIndicatorPolicyItemName(val: i32) ->String {
  match val {
     QTreeWidgetItem__ShowIndicator => // 0
     {return String::from("ShowIndicator");}
     QTreeWidgetItem__DontShowIndicator => // 1
     {return String::from("DontShowIndicator");}
     QTreeWidgetItem__DontShowIndicatorWhenChildless => // 2
     {return String::from("DontShowIndicatorWhenChildless");}
  _ => {return format!("{}", val);}
}
}
pub fn QTreeWidgetItem_ChildIndicatorPolicyItemName_s(val: i32) ->String {
  //var nilthis *QTreeWidgetItem
  //return nilthis.ChildIndicatorPolicyItemName(val);
  return QTreeWidgetItem_ChildIndicatorPolicyItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
