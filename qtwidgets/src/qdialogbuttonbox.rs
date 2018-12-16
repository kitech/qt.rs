

// mod ::widgets::QDialogButtonBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qdialogbuttonbox.h
// #include <qdialogbuttonbox.h>
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

// void changeEvent(QEvent *)
// func (this *QDialogButtonBox) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// bool event(QEvent *)
// func (this *QDialogButtonBox) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QDialogButtonBox)=48
pub struct QDialogButtonBox {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDialogButtonBox_ITF interface {
//    QWidget_ITF
//    QDialogButtonBox_PTR() *QDialogButtonBox
//}
//func (ptr *QDialogButtonBox) QDialogButtonBox_PTR() *QDialogButtonBox { return ptr }

impl /*struct*/ QDialogButtonBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDialogButtonBox {
    return QDialogButtonBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDialogButtonBox {
//  type Target = QDialogButtonBoxBASE;
//
//  fn deref(&self) -> &QDialogButtonBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDialogButtonBoxBASE> for QDialogButtonBox {
//  fn as_ref(& self) -> & QDialogButtonBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDialogButtonBox {
  pub fn metaObject_0<RetType, T: QDialogButtonBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDialogButtonBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDialogButtonBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDialogButtonBox(QWidget *)

/*
Constructs an empty, horizontal button box with the given parent.

See also orientation and addButton().
*/
// QDialogButtonBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDialogButtonBox {
  pub fn QDialogButtonBox_0<T: QDialogButtonBox_QDialogButtonBox_0>(value: T) -> QDialogButtonBox {
    let rsthis = value.QDialogButtonBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDialogButtonBox_QDialogButtonBox_0 {
  fn QDialogButtonBox_0(self) -> QDialogButtonBox;
}
// QDialogButtonBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDialogButtonBox_QDialogButtonBox_0 for (usize) {
  fn QDialogButtonBox_0(self) -> QDialogButtonBox {
    // unsafe{_ZN16QDialogButtonBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QDialogButtonBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDialogButtonBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:121
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDialogButtonBox(Qt::Orientation, QWidget *)

/*
Constructs an empty, horizontal button box with the given parent.

See also orientation and addButton().
*/
// QDialogButtonBox(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDialogButtonBox {
  pub fn QDialogButtonBox_1<T: QDialogButtonBox_QDialogButtonBox_1>(value: T) -> QDialogButtonBox {
    let rsthis = value.QDialogButtonBox_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDialogButtonBox_QDialogButtonBox_1 {
  fn QDialogButtonBox_1(self) -> QDialogButtonBox;
}
// QDialogButtonBox(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDialogButtonBox_QDialogButtonBox_1 for (i32,usize) {
  fn QDialogButtonBox_1(self) -> QDialogButtonBox {
    // unsafe{_ZN16QDialogButtonBoxC2EN2Qt11OrientationEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QDialogButtonBoxC2EN2Qt11OrientationEP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDialogButtonBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:122
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QDialogButtonBox(QDialogButtonBox::StandardButtons, QWidget *)

/*
Constructs an empty, horizontal button box with the given parent.

See also orientation and addButton().
*/
// QDialogButtonBox(QDialogButtonBox::StandardButtons, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDialogButtonBox {
  pub fn QDialogButtonBox_2<T: QDialogButtonBox_QDialogButtonBox_2>(value: T) -> QDialogButtonBox {
    let rsthis = value.QDialogButtonBox_2();
    return rsthis;
    // return 1;
  }
}

pub trait QDialogButtonBox_QDialogButtonBox_2 {
  fn QDialogButtonBox_2(self) -> QDialogButtonBox;
}
// QDialogButtonBox(QDialogButtonBox::StandardButtons, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDialogButtonBox_QDialogButtonBox_2 for (i32,usize) {
  fn QDialogButtonBox_2(self) -> QDialogButtonBox {
    // unsafe{_ZN16QDialogButtonBoxC2E6QFlagsINS_14StandardButtonEEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QDialogButtonBoxC2E6QFlagsINS_14StandardButtonEEP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDialogButtonBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:123
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QDialogButtonBox(QDialogButtonBox::StandardButtons, Qt::Orientation, QWidget *)

/*
Constructs an empty, horizontal button box with the given parent.

See also orientation and addButton().
*/
// QDialogButtonBox(QDialogButtonBox::StandardButtons, Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDialogButtonBox {
  pub fn QDialogButtonBox_3<T: QDialogButtonBox_QDialogButtonBox_3>(value: T) -> QDialogButtonBox {
    let rsthis = value.QDialogButtonBox_3();
    return rsthis;
    // return 1;
  }
}

pub trait QDialogButtonBox_QDialogButtonBox_3 {
  fn QDialogButtonBox_3(self) -> QDialogButtonBox;
}
// QDialogButtonBox(QDialogButtonBox::StandardButtons, Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDialogButtonBox_QDialogButtonBox_3 for (i32,i32,usize) {
  fn QDialogButtonBox_3(self) -> QDialogButtonBox {
    // unsafe{_ZN16QDialogButtonBoxC2E6QFlagsINS_14StandardButtonEEN2Qt11OrientationEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QDialogButtonBoxC2E6QFlagsINS_14StandardButtonEEN2Qt11OrientationEP7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDialogButtonBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:125
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDialogButtonBox()

/*

*/
pub fn DeleteQDialogButtonBox(this :*mut QDialogButtonBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QDialogButtonBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QDialogButtonBox {
  pub fn setOrientation_0<RetType, T: QDialogButtonBox_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:128
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QDialogButtonBox {
  pub fn orientation_0<RetType, T: QDialogButtonBox_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDialogButtonBox11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addButton(QAbstractButton *, QDialogButtonBox::ButtonRole)

/*
Adds the given button to the button box with the specified role. If the role is invalid, the button is not added.

If the button has already been added, it is removed and added again with the new role.

Note: The button box takes ownership of the button.

See also removeButton() and clear().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn addButton_0<RetType, T: QDialogButtonBox_addButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addButton_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_addButton_0<RetType> {
  fn addButton_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_addButton_0<(/*void*/)> for (usize,i32) {
  fn addButton_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox9addButtonEP15QAbstractButtonNS_10ButtonRoleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:131
// index:1
// Public Visibility=Default Availability=Available
// [8] QPushButton * addButton(const QString &, QDialogButtonBox::ButtonRole)

/*
Adds the given button to the button box with the specified role. If the role is invalid, the button is not added.

If the button has already been added, it is removed and added again with the new role.

Note: The button box takes ownership of the button.

See also removeButton() and clear().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn addButton_1<RetType, T: QDialogButtonBox_addButton_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addButton_1(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_addButton_1<RetType> {
  fn addButton_1(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_addButton_1<usize> for (usize,i32) {
  fn addButton_1(self , rsthis: & QDialogButtonBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox9addButtonERK7QStringNS_10ButtonRoleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:132
// index:2
// Public Visibility=Default Availability=Available
// [8] QPushButton * addButton(QDialogButtonBox::StandardButton)

/*
Adds the given button to the button box with the specified role. If the role is invalid, the button is not added.

If the button has already been added, it is removed and added again with the new role.

Note: The button box takes ownership of the button.

See also removeButton() and clear().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn addButton_2<RetType, T: QDialogButtonBox_addButton_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addButton_2(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_addButton_2<RetType> {
  fn addButton_2(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_addButton_2<usize> for (i32) {
  fn addButton_2(self , rsthis: & QDialogButtonBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox9addButtonENS_14StandardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeButton(QAbstractButton *)

/*
Removes button from the button box without deleting it and sets its parent to zero.

See also clear(), buttons(), and addButton().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn removeButton_0<RetType, T: QDialogButtonBox_removeButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeButton_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_removeButton_0<RetType> {
  fn removeButton_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_removeButton_0<(/*void*/)> for (usize) {
  fn removeButton_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox12removeButtonEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the button box, deleting all buttons within it.

See also removeButton() and addButton().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn clear_0<RetType, T: QDialogButtonBox_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_clear_0<RetType> {
  fn clear_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] QDialogButtonBox::ButtonRole buttonRole(QAbstractButton *) const

/*
Returns the button role for the specified button. This function returns InvalidRole if button is 0 or has not been added to the button box.

See also buttons() and addButton().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn buttonRole_0<RetType, T: QDialogButtonBox_buttonRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonRole_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_buttonRole_0<RetType> {
  fn buttonRole_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_buttonRole_0<i32> for (usize) {
  fn buttonRole_0(self , rsthis: & QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDialogButtonBox10buttonRoleEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStandardButtons(QDialogButtonBox::StandardButtons)

/*

*/
impl /*struct*/ QDialogButtonBox {
  pub fn setStandardButtons_0<RetType, T: QDialogButtonBox_setStandardButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStandardButtons_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_setStandardButtons_0<RetType> {
  fn setStandardButtons_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_setStandardButtons_0<(/*void*/)> for (i32) {
  fn setStandardButtons_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox18setStandardButtonsE6QFlagsINS_14StandardButtonEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:140
// index:0
// Public Visibility=Default Availability=Available
// [4] QDialogButtonBox::StandardButtons standardButtons() const

/*

*/
impl /*struct*/ QDialogButtonBox {
  pub fn standardButtons_0<RetType, T: QDialogButtonBox_standardButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardButtons_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_standardButtons_0<RetType> {
  fn standardButtons_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_standardButtons_0<i32> for () {
  fn standardButtons_0(self , rsthis: & QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDialogButtonBox15standardButtonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:141
// index:0
// Public Visibility=Default Availability=Available
// [4] QDialogButtonBox::StandardButton standardButton(QAbstractButton *) const

/*
Returns the standard button enum value corresponding to the given button, or NoButton if the given button isn't a standard button.

See also button(), buttons(), and standardButtons().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn standardButton_0<RetType, T: QDialogButtonBox_standardButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardButton_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_standardButton_0<RetType> {
  fn standardButton_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_standardButton_0<i32> for (usize) {
  fn standardButton_0(self , rsthis: & QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDialogButtonBox14standardButtonEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:142
// index:0
// Public Visibility=Default Availability=Available
// [8] QPushButton * button(QDialogButtonBox::StandardButton) const

/*
Returns the QPushButton corresponding to the standard button which, or 0 if the standard button doesn't exist in this button box.

See also standardButton(), standardButtons(), and buttons().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn button_0<RetType, T: QDialogButtonBox_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_button_0<RetType> {
  fn button_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_button_0<usize> for (i32) {
  fn button_0(self , rsthis: & QDialogButtonBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDialogButtonBox6buttonENS_14StandardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCenterButtons(bool)

/*

*/
impl /*struct*/ QDialogButtonBox {
  pub fn setCenterButtons_0<RetType, T: QDialogButtonBox_setCenterButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCenterButtons_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_setCenterButtons_0<RetType> {
  fn setCenterButtons_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_setCenterButtons_0<(/*void*/)> for (bool) {
  fn setCenterButtons_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox16setCenterButtonsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:145
// index:0
// Public Visibility=Default Availability=Available
// [1] bool centerButtons() const

/*

*/
impl /*struct*/ QDialogButtonBox {
  pub fn centerButtons_0<RetType, T: QDialogButtonBox_centerButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centerButtons_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_centerButtons_0<RetType> {
  fn centerButtons_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_centerButtons_0<bool> for () {
  fn centerButtons_0(self , rsthis: & QDialogButtonBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDialogButtonBox13centerButtonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clicked(QAbstractButton *)

/*
This signal is emitted when a button inside the button box is clicked. The specific button that was pressed is specified by button.

See also accepted(), rejected(), and helpRequested().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn clicked_0<RetType, T: QDialogButtonBox_clicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clicked_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_clicked_0<RetType> {
  fn clicked_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_clicked_0<(/*void*/)> for (usize) {
  fn clicked_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox7clickedEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void accepted()

/*
This signal is emitted when a button inside the button box is clicked, as long as it was defined with the AcceptRole or YesRole.

See also rejected(), clicked(), and helpRequested().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn accepted_0<RetType, T: QDialogButtonBox_accepted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accepted_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_accepted_0<RetType> {
  fn accepted_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_accepted_0<(/*void*/)> for () {
  fn accepted_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox8acceptedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:150
// index:0
// Public Visibility=Default Availability=Available
// [-2] void helpRequested()

/*
This signal is emitted when a button inside the button box is clicked, as long as it was defined with the HelpRole.

See also accepted(), rejected(), and clicked().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn helpRequested_0<RetType, T: QDialogButtonBox_helpRequested_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.helpRequested_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_helpRequested_0<RetType> {
  fn helpRequested_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_helpRequested_0<(/*void*/)> for () {
  fn helpRequested_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox13helpRequestedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rejected()

/*
This signal is emitted when a button inside the button box is clicked, as long as it was defined with the RejectRole or NoRole.

See also accepted(), helpRequested(), and clicked().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn rejected_0<RetType, T: QDialogButtonBox_rejected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rejected_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_rejected_0<RetType> {
  fn rejected_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_rejected_0<(/*void*/)> for () {
  fn rejected_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox8rejectedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:154
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn changeEvent_0<RetType, T: QDialogButtonBox_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QDialogButtonBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialogbuttonbox.h:155
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QDialogButtonBox {
  pub fn event_0<RetType, T: QDialogButtonBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QDialogButtonBox_event_0<RetType> {
  fn event_0(self , rsthis: & QDialogButtonBox) -> RetType;
}
impl<'a> /*trait*/ QDialogButtonBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QDialogButtonBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QDialogButtonBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum describes the roles that can be used to describe buttons in the button box. Combinations of these roles are as flags used to describe different aspects of their behavior.



See also StandardButton.

*/
pub type QDialogButtonBox__ButtonRole = i32;
// 
pub const QDialogButtonBox__InvalidRole :QDialogButtonBox__ButtonRole = -1;
// Clicking the button causes the dialog to be accepted (e.g. OK).
pub const QDialogButtonBox__AcceptRole :QDialogButtonBox__ButtonRole = 0;
// Clicking the button causes the dialog to be rejected (e.g. Cancel).
pub const QDialogButtonBox__RejectRole :QDialogButtonBox__ButtonRole = 1;
// Clicking the button causes a destructive change (e.g. for Discarding Changes) and closes the dialog.
pub const QDialogButtonBox__DestructiveRole :QDialogButtonBox__ButtonRole = 2;
// Clicking the button causes changes to the elements within the dialog.
pub const QDialogButtonBox__ActionRole :QDialogButtonBox__ButtonRole = 3;
// The button can be clicked to request help.
pub const QDialogButtonBox__HelpRole :QDialogButtonBox__ButtonRole = 4;
// The button is a "Yes"-like button.
pub const QDialogButtonBox__YesRole :QDialogButtonBox__ButtonRole = 5;
// The button is a "No"-like button.
pub const QDialogButtonBox__NoRole :QDialogButtonBox__ButtonRole = 6;
// The button resets the dialog's fields to default values.
pub const QDialogButtonBox__ResetRole :QDialogButtonBox__ButtonRole = 7;
// The button applies current changes.
pub const QDialogButtonBox__ApplyRole :QDialogButtonBox__ButtonRole = 8;
// 
pub const QDialogButtonBox__NRoles :QDialogButtonBox__ButtonRole = 9;
pub fn QDialogButtonBox_ButtonRoleItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDialogButtonBox", val);
}
pub fn QDialogButtonBox_ButtonRoleItemName_s(val: i32) ->String {
  //var nilthis *QDialogButtonBox
  //return nilthis.ButtonRoleItemName(val);
  return QDialogButtonBox_ButtonRoleItemName(val);
}


/*


*/
pub type QDialogButtonBox__StandardButton = i32;
// 
pub const QDialogButtonBox__NoButton :QDialogButtonBox__StandardButton = 0;
// 
pub const QDialogButtonBox__Ok :QDialogButtonBox__StandardButton = 1024;
// 
pub const QDialogButtonBox__Save :QDialogButtonBox__StandardButton = 2048;
// 
pub const QDialogButtonBox__SaveAll :QDialogButtonBox__StandardButton = 4096;
// 
pub const QDialogButtonBox__Open :QDialogButtonBox__StandardButton = 8192;
// 
pub const QDialogButtonBox__Yes :QDialogButtonBox__StandardButton = 16384;
// 
pub const QDialogButtonBox__YesToAll :QDialogButtonBox__StandardButton = 32768;
// 
pub const QDialogButtonBox__No :QDialogButtonBox__StandardButton = 65536;
// 
pub const QDialogButtonBox__NoToAll :QDialogButtonBox__StandardButton = 131072;
// 
pub const QDialogButtonBox__Abort :QDialogButtonBox__StandardButton = 262144;
// 
pub const QDialogButtonBox__Retry :QDialogButtonBox__StandardButton = 524288;
// 
pub const QDialogButtonBox__Ignore :QDialogButtonBox__StandardButton = 1048576;
// 
pub const QDialogButtonBox__Close :QDialogButtonBox__StandardButton = 2097152;
// 
pub const QDialogButtonBox__Cancel :QDialogButtonBox__StandardButton = 4194304;
// 
pub const QDialogButtonBox__Discard :QDialogButtonBox__StandardButton = 8388608;
// 
pub const QDialogButtonBox__Help :QDialogButtonBox__StandardButton = 16777216;
// 
pub const QDialogButtonBox__Apply :QDialogButtonBox__StandardButton = 33554432;
// 
pub const QDialogButtonBox__Reset :QDialogButtonBox__StandardButton = 67108864;
// 
pub const QDialogButtonBox__RestoreDefaults :QDialogButtonBox__StandardButton = 134217728;
// 
pub const QDialogButtonBox__FirstButton :QDialogButtonBox__StandardButton = 1024;
// 
pub const QDialogButtonBox__LastButton :QDialogButtonBox__StandardButton = 134217728;
pub fn QDialogButtonBox_StandardButtonItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDialogButtonBox", val);
}
pub fn QDialogButtonBox_StandardButtonItemName_s(val: i32) ->String {
  //var nilthis *QDialogButtonBox
  //return nilthis.StandardButtonItemName(val);
  return QDialogButtonBox_StandardButtonItemName(val);
}


/*
This enum describes the layout policy to be used when arranging the buttons contained in the button box.



The button layout is specified by the current style. However, on the X11 platform, it may be influenced by the desktop environment.

*/
pub type QDialogButtonBox__ButtonLayout = i32;
// Use a policy appropriate for applications on Windows.
pub const QDialogButtonBox__WinLayout :QDialogButtonBox__ButtonLayout = 0;
// Use a policy appropriate for applications on macOS.
pub const QDialogButtonBox__MacLayout :QDialogButtonBox__ButtonLayout = 1;
// Use a policy appropriate for applications on KDE.
pub const QDialogButtonBox__KdeLayout :QDialogButtonBox__ButtonLayout = 2;
// Use a policy appropriate for applications on GNOME.
pub const QDialogButtonBox__GnomeLayout :QDialogButtonBox__ButtonLayout = 3;
// 
pub const QDialogButtonBox__AndroidLayout :QDialogButtonBox__ButtonLayout = 5;
pub fn QDialogButtonBox_ButtonLayoutItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDialogButtonBox", val);
}
pub fn QDialogButtonBox_ButtonLayoutItemName_s(val: i32) ->String {
  //var nilthis *QDialogButtonBox
  //return nilthis.ButtonLayoutItemName(val);
  return QDialogButtonBox_ButtonLayoutItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
