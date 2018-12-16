

// mod ::gui::QStandardItem
// package qtgui
// /usr/include/qt/QtGui/qstandarditemmodel.h
// #include <qstandarditemmodel.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 17
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// QStandardItem & operator=(const QStandardItem &)
// func (this *QStandardItem) InheritOperator_equal(f func(other *QStandardItem) unsafe.Pointer/*555*/) {
//  qtrt.SetAllInheritCallback(this, "operator=", f)
// }

// void emitDataChanged()
// func (this *QStandardItem) InheritEmitDataChanged(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "emitDataChanged", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QStandardItem)=16
pub struct QStandardItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStandardItem_ITF interface {
//    QStandardItem_PTR() *QStandardItem
//}
//func (ptr *QStandardItem) QStandardItem_PTR() *QStandardItem { return ptr }

impl /*struct*/ QStandardItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStandardItem {
    return QStandardItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStandardItem {
//  type Target = QStandardItemBASE;
//
//  fn deref(&self) -> &QStandardItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStandardItemBASE> for QStandardItem {
//  fn as_ref(& self) -> & QStandardItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qstandarditemmodel.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStandardItem()

/*

*/
// QStandardItem() ctx.fn_proto_cpp
impl /*struct*/ QStandardItem {
  pub fn QStandardItem_0<T: QStandardItem_QStandardItem_0>(value: T) -> QStandardItem {
    let rsthis = value.QStandardItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItem_QStandardItem_0 {
  fn QStandardItem_0(self) -> QStandardItem;
}
// QStandardItem() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStandardItem_QStandardItem_0 for () {
  fn QStandardItem_0(self) -> QStandardItem {
    // unsafe{_ZN13QStandardItemC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStandardItemC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStandardItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:66
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QStandardItem(const QString &)

/*

*/
// QStandardItem(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QStandardItem {
  pub fn QStandardItem_1<T: QStandardItem_QStandardItem_1>(value: T) -> QStandardItem {
    let rsthis = value.QStandardItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItem_QStandardItem_1 {
  fn QStandardItem_1(self) -> QStandardItem;
}
// QStandardItem(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStandardItem_QStandardItem_1 for (usize) {
  fn QStandardItem_1(self) -> QStandardItem {
    // unsafe{_ZN13QStandardItemC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStandardItemC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStandardItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:67
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QStandardItem(const QIcon &, const QString &)

/*

*/
// QStandardItem(const QIcon &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QStandardItem {
  pub fn QStandardItem_2<T: QStandardItem_QStandardItem_2>(value: T) -> QStandardItem {
    let rsthis = value.QStandardItem_2();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItem_QStandardItem_2 {
  fn QStandardItem_2(self) -> QStandardItem;
}
// QStandardItem(const QIcon &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStandardItem_QStandardItem_2 for (usize,usize) {
  fn QStandardItem_2(self) -> QStandardItem {
    // unsafe{_ZN13QStandardItemC2ERK5QIconRK7QString()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStandardItemC2ERK5QIconRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStandardItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:68
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QStandardItem(int, int)

/*

*/
// QStandardItem(int, int) ctx.fn_proto_cpp
impl /*struct*/ QStandardItem {
  pub fn QStandardItem_3<T: QStandardItem_QStandardItem_3>(value: T) -> QStandardItem {
    let rsthis = value.QStandardItem_3();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItem_QStandardItem_3 {
  fn QStandardItem_3(self) -> QStandardItem;
}
// QStandardItem(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStandardItem_QStandardItem_3 for (i32,i32) {
  fn QStandardItem_3(self) -> QStandardItem {
    // unsafe{_ZN13QStandardItemC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStandardItemC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStandardItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStandardItem()

/*

*/
pub fn DeleteQStandardItem(this :*mut QStandardItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QStandardItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qstandarditemmodel.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(int) const

/*
Reimplemented from QAbstractItemModel::data().

See also setData().
*/
impl /*struct*/ QStandardItem {
  pub fn data_0<RetType, T: QStandardItem_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QStandardItem_data_0<RetType> {
  fn data_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_data_0<usize> for (i32) {
  fn data_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem4dataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setData(const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setData().

See also data().
*/
impl /*struct*/ QStandardItem {
  pub fn setData_0<RetType, T: QStandardItem_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setData_0<RetType> {
  fn setData_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setData_0<(/*void*/)> for (usize,i32) {
  fn setData_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem7setDataERK8QVarianti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn text_0<RetType, T: QStandardItem_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QStandardItem_text_0<RetType> {
  fn text_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_text_0<usize> for () {
  fn text_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setText_0<RetType, T: QStandardItem_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setText_0<RetType> {
  fn setText_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QIcon icon() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn icon_0<RetType, T: QStandardItem_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QStandardItem_icon_0<RetType> {
  fn icon_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_icon_0<usize> for () {
  fn icon_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setIcon(const QIcon &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setIcon_0<RetType, T: QStandardItem_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setIcon_0<(/*void*/)> for (usize) {
  fn setIcon_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem7setIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:85
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toolTip() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn toolTip_0<RetType, T: QStandardItem_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QStandardItem_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_toolTip_0<usize> for () {
  fn toolTip_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem7toolTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:88
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setToolTip(const QString &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setToolTip_0<RetType, T: QStandardItem_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setToolTip_0<(/*void*/)> for (usize) {
  fn setToolTip_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem10setToolTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString statusTip() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn statusTip_0<RetType, T: QStandardItem_statusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.statusTip_0(self);
    // return 1;
  }
}
pub trait QStandardItem_statusTip_0<RetType> {
  fn statusTip_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_statusTip_0<usize> for () {
  fn statusTip_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem9statusTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:95
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setStatusTip(const QString &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setStatusTip_0<RetType, T: QStandardItem_setStatusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setStatusTip_0<RetType> {
  fn setStatusTip_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setStatusTip_0<(/*void*/)> for (usize) {
  fn setStatusTip_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem12setStatusTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:99
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString whatsThis() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn whatsThis_0<RetType, T: QStandardItem_whatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.whatsThis_0(self);
    // return 1;
  }
}
pub trait QStandardItem_whatsThis_0<RetType> {
  fn whatsThis_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_whatsThis_0<usize> for () {
  fn whatsThis_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem9whatsThisEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:102
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWhatsThis(const QString &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setWhatsThis_0<RetType, T: QStandardItem_setWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setWhatsThis_0<RetType> {
  fn setWhatsThis_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setWhatsThis_0<(/*void*/)> for (usize) {
  fn setWhatsThis_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem12setWhatsThisERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn sizeHint_0<RetType, T: QStandardItem_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QStandardItem_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSizeHint(const QSize &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setSizeHint_0<RetType, T: QStandardItem_setSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeHint_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setSizeHint_0<RetType> {
  fn setSizeHint_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setSizeHint_0<(/*void*/)> for (usize) {
  fn setSizeHint_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem11setSizeHintERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:110
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn font_0<RetType, T: QStandardItem_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QStandardItem_font_0<RetType> {
  fn font_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_font_0<usize> for () {
  fn font_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:113
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setFont_0<RetType, T: QStandardItem_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:115
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::Alignment textAlignment() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn textAlignment_0<RetType, T: QStandardItem_textAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textAlignment_0(self);
    // return 1;
  }
}
pub trait QStandardItem_textAlignment_0<RetType> {
  fn textAlignment_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_textAlignment_0<i32> for () {
  fn textAlignment_0(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem13textAlignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:118
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setTextAlignment_0<RetType, T: QStandardItem_setTextAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextAlignment_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setTextAlignment_0<RetType> {
  fn setTextAlignment_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setTextAlignment_0<(/*void*/)> for (i32) {
  fn setTextAlignment_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem16setTextAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush background() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn background_0<RetType, T: QStandardItem_background_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.background_0(self);
    // return 1;
  }
}
pub trait QStandardItem_background_0<RetType> {
  fn background_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_background_0<usize> for () {
  fn background_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem10backgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:123
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBackground(const QBrush &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setBackground_0<RetType, T: QStandardItem_setBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackground_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setBackground_0<RetType> {
  fn setBackground_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setBackground_0<(/*void*/)> for (usize) {
  fn setBackground_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem13setBackgroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush foreground() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn foreground_0<RetType, T: QStandardItem_foreground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foreground_0(self);
    // return 1;
  }
}
pub trait QStandardItem_foreground_0<RetType> {
  fn foreground_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_foreground_0<usize> for () {
  fn foreground_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem10foregroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setForeground(const QBrush &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setForeground_0<RetType, T: QStandardItem_setForeground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setForeground_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setForeground_0<RetType> {
  fn setForeground_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setForeground_0<(/*void*/)> for (usize) {
  fn setForeground_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem13setForegroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:130
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::CheckState checkState() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn checkState_0<RetType, T: QStandardItem_checkState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkState_0(self);
    // return 1;
  }
}
pub trait QStandardItem_checkState_0<RetType> {
  fn checkState_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_checkState_0<i32> for () {
  fn checkState_0(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem10checkStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:133
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCheckState(Qt::CheckState)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setCheckState_0<RetType, T: QStandardItem_setCheckState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckState_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setCheckState_0<RetType> {
  fn setCheckState_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setCheckState_0<(/*void*/)> for (i32) {
  fn setCheckState_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem13setCheckStateEN2Qt10CheckStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:135
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString accessibleText() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn accessibleText_0<RetType, T: QStandardItem_accessibleText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accessibleText_0(self);
    // return 1;
  }
}
pub trait QStandardItem_accessibleText_0<RetType> {
  fn accessibleText_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_accessibleText_0<usize> for () {
  fn accessibleText_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem14accessibleTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:138
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAccessibleText(const QString &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setAccessibleText_0<RetType, T: QStandardItem_setAccessibleText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleText_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setAccessibleText_0<RetType> {
  fn setAccessibleText_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setAccessibleText_0<(/*void*/)> for (usize) {
  fn setAccessibleText_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem17setAccessibleTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:140
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString accessibleDescription() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn accessibleDescription_0<RetType, T: QStandardItem_accessibleDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accessibleDescription_0(self);
    // return 1;
  }
}
pub trait QStandardItem_accessibleDescription_0<RetType> {
  fn accessibleDescription_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_accessibleDescription_0<usize> for () {
  fn accessibleDescription_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem21accessibleDescriptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:143
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAccessibleDescription(const QString &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setAccessibleDescription_0<RetType, T: QStandardItem_setAccessibleDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleDescription_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setAccessibleDescription_0<RetType> {
  fn setAccessibleDescription_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setAccessibleDescription_0<(/*void*/)> for (usize) {
  fn setAccessibleDescription_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem24setAccessibleDescriptionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:145
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags() const

/*
Reimplemented from QAbstractItemModel::flags().
*/
impl /*struct*/ QStandardItem {
  pub fn flags_0<RetType, T: QStandardItem_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QStandardItem_flags_0<RetType> {
  fn flags_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(Qt::ItemFlags)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setFlags_0<RetType, T: QStandardItem_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem8setFlagsE6QFlagsIN2Qt8ItemFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:148
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isEnabled_0<RetType, T: QStandardItem_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setEnabled_0<RetType, T: QStandardItem_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:153
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEditable() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isEditable_0<RetType, T: QStandardItem_isEditable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEditable_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isEditable_0<RetType> {
  fn isEditable_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isEditable_0<bool> for () {
  fn isEditable_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem10isEditableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEditable(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setEditable_0<RetType, T: QStandardItem_setEditable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEditable_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setEditable_0<RetType> {
  fn setEditable_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setEditable_0<(/*void*/)> for (bool) {
  fn setEditable_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem11setEditableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:158
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSelectable() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isSelectable_0<RetType, T: QStandardItem_isSelectable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSelectable_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isSelectable_0<RetType> {
  fn isSelectable_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isSelectable_0<bool> for () {
  fn isSelectable_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem12isSelectableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectable(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setSelectable_0<RetType, T: QStandardItem_setSelectable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectable_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setSelectable_0<RetType> {
  fn setSelectable_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setSelectable_0<(/*void*/)> for (bool) {
  fn setSelectable_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem13setSelectableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:163
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isCheckable() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isCheckable_0<RetType, T: QStandardItem_isCheckable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCheckable_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isCheckable_0<RetType> {
  fn isCheckable_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isCheckable_0<bool> for () {
  fn isCheckable_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem11isCheckableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:166
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCheckable(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setCheckable_0<RetType, T: QStandardItem_setCheckable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckable_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setCheckable_0<RetType> {
  fn setCheckable_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setCheckable_0<(/*void*/)> for (bool) {
  fn setCheckable_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem12setCheckableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:168
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAutoTristate() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isAutoTristate_0<RetType, T: QStandardItem_isAutoTristate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAutoTristate_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isAutoTristate_0<RetType> {
  fn isAutoTristate_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isAutoTristate_0<bool> for () {
  fn isAutoTristate_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem14isAutoTristateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:171
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoTristate(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setAutoTristate_0<RetType, T: QStandardItem_setAutoTristate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoTristate_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setAutoTristate_0<RetType> {
  fn setAutoTristate_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setAutoTristate_0<(/*void*/)> for (bool) {
  fn setAutoTristate_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem15setAutoTristateEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:173
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isUserTristate() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isUserTristate_0<RetType, T: QStandardItem_isUserTristate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUserTristate_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isUserTristate_0<RetType> {
  fn isUserTristate_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isUserTristate_0<bool> for () {
  fn isUserTristate_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem14isUserTristateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUserTristate(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setUserTristate_0<RetType, T: QStandardItem_setUserTristate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUserTristate_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setUserTristate_0<RetType> {
  fn setUserTristate_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setUserTristate_0<(/*void*/)> for (bool) {
  fn setUserTristate_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem15setUserTristateEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:179
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isTristate() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isTristate_0<RetType, T: QStandardItem_isTristate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTristate_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isTristate_0<RetType> {
  fn isTristate_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isTristate_0<bool> for () {
  fn isTristate_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem10isTristateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:180
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTristate(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setTristate_0<RetType, T: QStandardItem_setTristate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTristate_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setTristate_0<RetType> {
  fn setTristate_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setTristate_0<(/*void*/)> for (bool) {
  fn setTristate_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem11setTristateEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:184
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDragEnabled() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isDragEnabled_0<RetType, T: QStandardItem_isDragEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDragEnabled_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isDragEnabled_0<RetType> {
  fn isDragEnabled_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isDragEnabled_0<bool> for () {
  fn isDragEnabled_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem13isDragEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDragEnabled(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setDragEnabled_0<RetType, T: QStandardItem_setDragEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDragEnabled_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setDragEnabled_0<RetType> {
  fn setDragEnabled_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setDragEnabled_0<(/*void*/)> for (bool) {
  fn setDragEnabled_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem14setDragEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:189
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDropEnabled() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn isDropEnabled_0<RetType, T: QStandardItem_isDropEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDropEnabled_0(self);
    // return 1;
  }
}
pub trait QStandardItem_isDropEnabled_0<RetType> {
  fn isDropEnabled_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_isDropEnabled_0<bool> for () {
  fn isDropEnabled_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem13isDropEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:192
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDropEnabled(bool)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setDropEnabled_0<RetType, T: QStandardItem_setDropEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDropEnabled_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setDropEnabled_0<RetType> {
  fn setDropEnabled_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setDropEnabled_0<(/*void*/)> for (bool) {
  fn setDropEnabled_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem14setDropEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:195
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * parent() const

/*
Reimplemented from QAbstractItemModel::parent().
*/
impl /*struct*/ QStandardItem {
  pub fn parent_0<RetType, T: QStandardItem_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QStandardItem_parent_0<RetType> {
  fn parent_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:196
// index:0
// Public Visibility=Default Availability=Available
// [4] int row() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn row_0<RetType, T: QStandardItem_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QStandardItem_row_0<RetType> {
  fn row_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_row_0<i32> for () {
  fn row_0(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem3rowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:197
// index:0
// Public Visibility=Default Availability=Available
// [4] int column() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn column_0<RetType, T: QStandardItem_column_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.column_0(self);
    // return 1;
  }
}
pub trait QStandardItem_column_0<RetType> {
  fn column_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_column_0<i32> for () {
  fn column_0(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem6columnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:198
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex index() const

/*
Reimplemented from QAbstractItemModel::index().
*/
impl /*struct*/ QStandardItem {
  pub fn index_0<RetType, T: QStandardItem_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QStandardItem_index_0<RetType> {
  fn index_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_index_0<usize> for () {
  fn index_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem5indexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:199
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItemModel * model() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn model_0<RetType, T: QStandardItem_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QStandardItem_model_0<RetType> {
  fn model_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_model_0<usize> for () {
  fn model_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:201
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowCount() const

/*
Reimplemented from QAbstractItemModel::rowCount().

See also setRowCount().
*/
impl /*struct*/ QStandardItem {
  pub fn rowCount_0<RetType, T: QStandardItem_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QStandardItem_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_rowCount_0<i32> for () {
  fn rowCount_0(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem8rowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:202
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowCount(int)

/*
Sets the number of rows in this model to rows. If this is less than rowCount(), the data in the unwanted rows is discarded.

This function was introduced in  Qt 4.2.

See also rowCount() and setColumnCount().
*/
impl /*struct*/ QStandardItem {
  pub fn setRowCount_0<RetType, T: QStandardItem_setRowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowCount_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setRowCount_0<RetType> {
  fn setRowCount_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setRowCount_0<(/*void*/)> for (i32) {
  fn setRowCount_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem11setRowCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:203
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnCount() const

/*
Reimplemented from QAbstractItemModel::columnCount().

See also setColumnCount().
*/
impl /*struct*/ QStandardItem {
  pub fn columnCount_0<RetType, T: QStandardItem_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QStandardItem_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:204
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnCount(int)

/*
Sets the number of columns in this model to columns. If this is less than columnCount(), the data in the unwanted columns is discarded.

This function was introduced in  Qt 4.2.

See also columnCount() and setRowCount().
*/
impl /*struct*/ QStandardItem {
  pub fn setColumnCount_0<RetType, T: QStandardItem_setColumnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setColumnCount_0<RetType> {
  fn setColumnCount_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setColumnCount_0<(/*void*/)> for (i32) {
  fn setColumnCount_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem14setColumnCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:206
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasChildren() const

/*
Reimplemented from QAbstractItemModel::hasChildren().
*/
impl /*struct*/ QStandardItem {
  pub fn hasChildren_0<RetType, T: QStandardItem_hasChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasChildren_0(self);
    // return 1;
  }
}
pub trait QStandardItem_hasChildren_0<RetType> {
  fn hasChildren_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_hasChildren_0<bool> for () {
  fn hasChildren_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem11hasChildrenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:207
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * child(int, int) const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn child_0<RetType, T: QStandardItem_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QStandardItem_child_0<RetType> {
  fn child_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_child_0<usize> for (i32,i32) {
  fn child_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem5childEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChild(int, int, QStandardItem *)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setChild_0<RetType, T: QStandardItem_setChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChild_0(self);
    // return 1;
  }
}
pub trait QStandardItem_setChild_0<RetType> {
  fn setChild_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setChild_0<(/*void*/)> for (i32,i32,usize) {
  fn setChild_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem8setChildEiiPS_", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:209
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setChild(int, QStandardItem *)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn setChild_1<RetType, T: QStandardItem_setChild_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChild_1(self);
    // return 1;
  }
}
pub trait QStandardItem_setChild_1<RetType> {
  fn setChild_1(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_setChild_1<(/*void*/)> for (i32,usize) {
  fn setChild_1(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem8setChildEiPS_", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:214
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertRows(int, int)

/*
Reimplemented from QAbstractItemModel::insertRows().
*/
impl /*struct*/ QStandardItem {
  pub fn insertRows_0<RetType, T: QStandardItem_insertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRows_0(self);
    // return 1;
  }
}
pub trait QStandardItem_insertRows_0<RetType> {
  fn insertRows_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_insertRows_0<(/*void*/)> for (i32,i32) {
  fn insertRows_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem10insertRowsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:215
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertColumns(int, int)

/*
Reimplemented from QAbstractItemModel::insertColumns().
*/
impl /*struct*/ QStandardItem {
  pub fn insertColumns_0<RetType, T: QStandardItem_insertColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumns_0(self);
    // return 1;
  }
}
pub trait QStandardItem_insertColumns_0<RetType> {
  fn insertColumns_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_insertColumns_0<(/*void*/)> for (i32,i32) {
  fn insertColumns_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem13insertColumnsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeRow(int)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn removeRow_0<RetType, T: QStandardItem_removeRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRow_0(self);
    // return 1;
  }
}
pub trait QStandardItem_removeRow_0<RetType> {
  fn removeRow_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_removeRow_0<(/*void*/)> for (i32) {
  fn removeRow_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem9removeRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:218
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeColumn(int)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn removeColumn_0<RetType, T: QStandardItem_removeColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumn_0(self);
    // return 1;
  }
}
pub trait QStandardItem_removeColumn_0<RetType> {
  fn removeColumn_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_removeColumn_0<(/*void*/)> for (i32) {
  fn removeColumn_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem12removeColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:219
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeRows(int, int)

/*
Reimplemented from QAbstractItemModel::removeRows().
*/
impl /*struct*/ QStandardItem {
  pub fn removeRows_0<RetType, T: QStandardItem_removeRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRows_0(self);
    // return 1;
  }
}
pub trait QStandardItem_removeRows_0<RetType> {
  fn removeRows_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_removeRows_0<(/*void*/)> for (i32,i32) {
  fn removeRows_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem10removeRowsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeColumns(int, int)

/*
Reimplemented from QAbstractItemModel::removeColumns().
*/
impl /*struct*/ QStandardItem {
  pub fn removeColumns_0<RetType, T: QStandardItem_removeColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumns_0(self);
    // return 1;
  }
}
pub trait QStandardItem_removeColumns_0<RetType> {
  fn removeColumns_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_removeColumns_0<(/*void*/)> for (i32,i32) {
  fn removeColumns_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem13removeColumnsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:225
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void insertRow(int, QStandardItem *)

/*
Inserts a row at row containing items. If necessary, the column count is increased to the size of items.

This function was introduced in  Qt 4.2.

See also takeRow(), appendRow(), and insertColumn().
*/
impl /*struct*/ QStandardItem {
  pub fn insertRow_0<RetType, T: QStandardItem_insertRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_0(self);
    // return 1;
  }
}
pub trait QStandardItem_insertRow_0<RetType> {
  fn insertRow_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_insertRow_0<(/*void*/)> for (i32,usize) {
  fn insertRow_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem9insertRowEiPS_", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:226
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void appendRow(QStandardItem *)

/*
Appends a row containing items. If necessary, the column count is increased to the size of items.

This function was introduced in  Qt 4.2.

See also insertRow() and appendColumn().
*/
impl /*struct*/ QStandardItem {
  pub fn appendRow_0<RetType, T: QStandardItem_appendRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.appendRow_0(self);
    // return 1;
  }
}
pub trait QStandardItem_appendRow_0<RetType> {
  fn appendRow_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_appendRow_0<(/*void*/)> for (usize) {
  fn appendRow_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem9appendRowEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:228
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * takeChild(int, int)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn takeChild_0<RetType, T: QStandardItem_takeChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeChild_0(self);
    // return 1;
  }
}
pub trait QStandardItem_takeChild_0<RetType> {
  fn takeChild_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_takeChild_0<usize> for (i32,i32) {
  fn takeChild_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStandardItem9takeChildEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:232
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sortChildren(int, Qt::SortOrder)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn sortChildren_0<RetType, T: QStandardItem_sortChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortChildren_0(self);
    // return 1;
  }
}
pub trait QStandardItem_sortChildren_0<RetType> {
  fn sortChildren_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_sortChildren_0<(/*void*/)> for (i32,i32) {
  fn sortChildren_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem12sortChildrenEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:234
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStandardItem * clone() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn clone_0<RetType, T: QStandardItem_clone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clone_0(self);
    // return 1;
  }
}
pub trait QStandardItem_clone_0<RetType> {
  fn clone_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_clone_0<usize> for () {
  fn clone_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem5cloneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:237
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int type() const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn type__0<RetType, T: QStandardItem_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QStandardItem_type__0<RetType> {
  fn type__0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_type__0<i32> for () {
  fn type__0(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItem4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:240
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void read(QDataStream &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn read_0<RetType, T: QStandardItem_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QStandardItem_read_0<RetType> {
  fn read_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_read_0<(/*void*/)> for (usize) {
  fn read_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStandardItem4readER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:241
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void write(QDataStream &) const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn write_0<RetType, T: QStandardItem_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QStandardItem_write_0<RetType> {
  fn write_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_write_0<(/*void*/)> for (usize) {
  fn write_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QStandardItem5writeER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:243
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool operator<(const QStandardItem &) const

/*

*/
impl /*struct*/ QStandardItem {
  pub fn operator_less_than_0<RetType, T: QStandardItem_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QStandardItem_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QStandardItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStandardItemltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:248
// index:0
// Protected Visibility=Default Availability=Available
// [16] QStandardItem & operator=(const QStandardItem &)

/*

*/
impl /*struct*/ QStandardItem {
  pub fn operator_equal_0<RetType, T: QStandardItem_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QStandardItem_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QStandardItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStandardItemaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:251
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void emitDataChanged()

/*

*/
impl /*struct*/ QStandardItem {
  pub fn emitDataChanged_0<RetType, T: QStandardItem_emitDataChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.emitDataChanged_0(self);
    // return 1;
  }
}
pub trait QStandardItem_emitDataChanged_0<RetType> {
  fn emitDataChanged_0(self , rsthis: & QStandardItem) -> RetType;
}
impl<'a> /*trait*/ QStandardItem_emitDataChanged_0<(/*void*/)> for () {
  fn emitDataChanged_0(self , rsthis: & QStandardItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QStandardItem15emitDataChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QStandardItem__ItemType = i32;
// 
pub const QStandardItem__Type :QStandardItem__ItemType = 0;
// 
pub const QStandardItem__UserType :QStandardItem__ItemType = 1000;
pub fn QStandardItem_ItemTypeItemName(val: i32) ->String {
  match val {
     QStandardItem__Type => // 0
     {return String::from("Type");}
     QStandardItem__UserType => // 1000
     {return String::from("UserType");}
  _ => {return format!("{}", val);}
}
}
pub fn QStandardItem_ItemTypeItemName_s(val: i32) ->String {
  //var nilthis *QStandardItem
  //return nilthis.ItemTypeItemName(val);
  return QStandardItem_ItemTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
