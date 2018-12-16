

// mod ::widgets::QMessageBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qmessagebox.h
// #include <qmessagebox.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 43
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
// func (this *QMessageBox) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QMessageBox) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QMessageBox) InheritShowEvent(f func(event *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void closeEvent(QCloseEvent *)
// func (this *QMessageBox) InheritCloseEvent(f func(event *qtgui.QCloseEvent/*777 QCloseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QMessageBox) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QMessageBox) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QMessageBox)=48
pub struct QMessageBox {
  qbase: QDialog,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMessageBox_ITF interface {
//    QDialog_ITF
//    QMessageBox_PTR() *QMessageBox
//}
//func (ptr *QMessageBox) QMessageBox_PTR() *QMessageBox { return ptr }

impl /*struct*/ QMessageBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMessageBox {
    return QMessageBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMessageBox {
//  type Target = QMessageBoxBASE;
//
//  fn deref(&self) -> &QMessageBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMessageBoxBASE> for QMessageBox {
//  fn as_ref(& self) -> & QMessageBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qmessagebox.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn metaObject_0<RetType, T: QMessageBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMessageBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMessageBox(QWidget *)

/*
Constructs a message box with no text and no buttons. parent is passed to the QDialog constructor.

On macOS, if you want your message box to appear as a Qt::Sheet of its parent, set the message box's window modality to Qt::WindowModal or use open(). Otherwise, the message box will be a standard dialog.
*/
// QMessageBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QMessageBox {
  pub fn QMessageBox_0<T: QMessageBox_QMessageBox_0>(value: T) -> QMessageBox {
    let rsthis = value.QMessageBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageBox_QMessageBox_0 {
  fn QMessageBox_0(self) -> QMessageBox;
}
// QMessageBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageBox_QMessageBox_0 for (usize) {
  fn QMessageBox_0(self) -> QMessageBox {
    // unsafe{_ZN11QMessageBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMessageBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMessageBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:136
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QMessageBox(QMessageBox::Icon, const QString &, const QString &, QMessageBox::StandardButtons, QWidget *, Qt::WindowFlags)

/*
Constructs a message box with no text and no buttons. parent is passed to the QDialog constructor.

On macOS, if you want your message box to appear as a Qt::Sheet of its parent, set the message box's window modality to Qt::WindowModal or use open(). Otherwise, the message box will be a standard dialog.
*/
// QMessageBox(QMessageBox::Icon, const QString &, const QString &, QMessageBox::StandardButtons, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QMessageBox {
  pub fn QMessageBox_1<T: QMessageBox_QMessageBox_1>(value: T) -> QMessageBox {
    let rsthis = value.QMessageBox_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageBox_QMessageBox_1 {
  fn QMessageBox_1(self) -> QMessageBox;
}
// QMessageBox(QMessageBox::Icon, const QString &, const QString &, QMessageBox::StandardButtons, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageBox_QMessageBox_1 for (i32,usize,usize,i32,usize,i32) {
  fn QMessageBox_1(self) -> QMessageBox {
    // unsafe{_ZN11QMessageBoxC2ENS_4IconERK7QStringS3_6QFlagsINS_14StandardButtonEEP7QWidgetS4_IN2Qt10WindowTypeEE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMessageBoxC2ENS_4IconERK7QStringS3_6QFlagsINS_14StandardButtonEEP7QWidgetS4_IN2Qt10WindowTypeEE", 6,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMessageBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:202
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QMessageBox(const QString &, const QString &, QMessageBox::Icon, int, int, int, QWidget *, Qt::WindowFlags)

/*
Constructs a message box with no text and no buttons. parent is passed to the QDialog constructor.

On macOS, if you want your message box to appear as a Qt::Sheet of its parent, set the message box's window modality to Qt::WindowModal or use open(). Otherwise, the message box will be a standard dialog.
*/
// QMessageBox(const QString &, const QString &, QMessageBox::Icon, int, int, int, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QMessageBox {
  pub fn QMessageBox_2<T: QMessageBox_QMessageBox_2>(value: T) -> QMessageBox {
    let rsthis = value.QMessageBox_2();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageBox_QMessageBox_2 {
  fn QMessageBox_2(self) -> QMessageBox;
}
// QMessageBox(const QString &, const QString &, QMessageBox::Icon, int, int, int, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageBox_QMessageBox_2 for (usize,usize,i32,i32,i32,i32,usize,i32) {
  fn QMessageBox_2(self) -> QMessageBox {
    // unsafe{_ZN11QMessageBoxC2ERK7QStringS2_NS_4IconEiiiP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMessageBoxC2ERK7QStringS2_NS_4IconEiiiP7QWidget6QFlagsIN2Qt10WindowTypeEE", 8,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    let rsthis = QMessageBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:139
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMessageBox()

/*

*/
pub fn DeleteQMessageBox(this :*mut QMessageBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QMessageBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qmessagebox.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addButton(QAbstractButton *, QMessageBox::ButtonRole)

/*
Adds the given button to the message box with the specified role.

This function was introduced in  Qt 4.2.

See also removeButton(), button(), and setStandardButtons().
*/
impl /*struct*/ QMessageBox {
  pub fn addButton_0<RetType, T: QMessageBox_addButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addButton_0(self);
    // return 1;
  }
}
pub trait QMessageBox_addButton_0<RetType> {
  fn addButton_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_addButton_0<(/*void*/)> for (usize,i32) {
  fn addButton_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox9addButtonEP15QAbstractButtonNS_10ButtonRoleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:142
// index:1
// Public Visibility=Default Availability=Available
// [8] QPushButton * addButton(const QString &, QMessageBox::ButtonRole)

/*
Adds the given button to the message box with the specified role.

This function was introduced in  Qt 4.2.

See also removeButton(), button(), and setStandardButtons().
*/
impl /*struct*/ QMessageBox {
  pub fn addButton_1<RetType, T: QMessageBox_addButton_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addButton_1(self);
    // return 1;
  }
}
pub trait QMessageBox_addButton_1<RetType> {
  fn addButton_1(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_addButton_1<usize> for (usize,i32) {
  fn addButton_1(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox9addButtonERK7QStringNS_10ButtonRoleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:143
// index:2
// Public Visibility=Default Availability=Available
// [8] QPushButton * addButton(QMessageBox::StandardButton)

/*
Adds the given button to the message box with the specified role.

This function was introduced in  Qt 4.2.

See also removeButton(), button(), and setStandardButtons().
*/
impl /*struct*/ QMessageBox {
  pub fn addButton_2<RetType, T: QMessageBox_addButton_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addButton_2(self);
    // return 1;
  }
}
pub trait QMessageBox_addButton_2<RetType> {
  fn addButton_2(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_addButton_2<usize> for (i32) {
  fn addButton_2(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox9addButtonENS_14StandardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeButton(QAbstractButton *)

/*
Removes button from the button box without deleting it.

This function was introduced in  Qt 4.2.

See also addButton() and setStandardButtons().
*/
impl /*struct*/ QMessageBox {
  pub fn removeButton_0<RetType, T: QMessageBox_removeButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeButton_0(self);
    // return 1;
  }
}
pub trait QMessageBox_removeButton_0<RetType> {
  fn removeButton_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_removeButton_0<(/*void*/)> for (usize) {
  fn removeButton_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox12removeButtonEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:147
// index:0
// Public Visibility=Default Availability=Available
// [-2] void open(QObject *, const char *)

/*
This is an overloaded function.

Opens the dialog and connects its finished() or buttonClicked() signal to the slot specified by receiver and member. If the slot in member has a pointer for its first parameter the connection is to buttonClicked(), otherwise the connection is to finished().

The signal will be disconnected from the slot when the dialog is closed.
*/
impl /*struct*/ QMessageBox {
  pub fn open_0<RetType, T: QMessageBox_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QMessageBox_open_0<RetType> {
  fn open_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_open_0<(/*void*/)> for (usize,usize) {
  fn open_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox4openEP7QObjectPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:150
// index:0
// Public Visibility=Default Availability=Available
// [4] QMessageBox::ButtonRole buttonRole(QAbstractButton *) const

/*
Returns the button role for the specified button. This function returns InvalidRole if button is 0 or has not been added to the message box.

This function was introduced in  Qt 4.5.

See also buttons() and addButton().
*/
impl /*struct*/ QMessageBox {
  pub fn buttonRole_0<RetType, T: QMessageBox_buttonRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonRole_0(self);
    // return 1;
  }
}
pub trait QMessageBox_buttonRole_0<RetType> {
  fn buttonRole_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_buttonRole_0<i32> for (usize) {
  fn buttonRole_0(self , rsthis: & QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox10buttonRoleEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStandardButtons(QMessageBox::StandardButtons)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setStandardButtons_0<RetType, T: QMessageBox_setStandardButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStandardButtons_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setStandardButtons_0<RetType> {
  fn setStandardButtons_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setStandardButtons_0<(/*void*/)> for (i32) {
  fn setStandardButtons_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox18setStandardButtonsE6QFlagsINS_14StandardButtonEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:153
// index:0
// Public Visibility=Default Availability=Available
// [4] QMessageBox::StandardButtons standardButtons() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn standardButtons_0<RetType, T: QMessageBox_standardButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardButtons_0(self);
    // return 1;
  }
}
pub trait QMessageBox_standardButtons_0<RetType> {
  fn standardButtons_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_standardButtons_0<i32> for () {
  fn standardButtons_0(self , rsthis: & QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox15standardButtonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:154
// index:0
// Public Visibility=Default Availability=Available
// [4] QMessageBox::StandardButton standardButton(QAbstractButton *) const

/*
Returns the standard button enum value corresponding to the given button, or NoButton if the given button isn't a standard button.

This function was introduced in  Qt 4.2.

See also button() and standardButtons().
*/
impl /*struct*/ QMessageBox {
  pub fn standardButton_0<RetType, T: QMessageBox_standardButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardButton_0(self);
    // return 1;
  }
}
pub trait QMessageBox_standardButton_0<RetType> {
  fn standardButton_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_standardButton_0<i32> for (usize) {
  fn standardButton_0(self , rsthis: & QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox14standardButtonEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:155
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractButton * button(QMessageBox::StandardButton) const

/*
Returns a pointer corresponding to the standard button which, or 0 if the standard button doesn't exist in this message box.

This function was introduced in  Qt 4.2.

See also standardButtons and standardButton().
*/
impl /*struct*/ QMessageBox {
  pub fn button_0<RetType, T: QMessageBox_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QMessageBox_button_0<RetType> {
  fn button_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_button_0<usize> for (i32) {
  fn button_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox6buttonENS_14StandardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:157
// index:0
// Public Visibility=Default Availability=Available
// [8] QPushButton * defaultButton() const

/*
Returns the button that should be the message box's default button. Returns 0 if no default button was set.

This function was introduced in  Qt 4.2.

See also setDefaultButton(), addButton(), and QPushButton::setDefault().
*/
impl /*struct*/ QMessageBox {
  pub fn defaultButton_0<RetType, T: QMessageBox_defaultButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultButton_0(self);
    // return 1;
  }
}
pub trait QMessageBox_defaultButton_0<RetType> {
  fn defaultButton_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_defaultButton_0<usize> for () {
  fn defaultButton_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox13defaultButtonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultButton(QPushButton *)

/*
Sets the message box's default button to button.

This function was introduced in  Qt 4.2.

See also defaultButton(), addButton(), and QPushButton::setDefault().
*/
impl /*struct*/ QMessageBox {
  pub fn setDefaultButton_0<RetType, T: QMessageBox_setDefaultButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultButton_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setDefaultButton_0<RetType> {
  fn setDefaultButton_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setDefaultButton_0<(/*void*/)> for (usize) {
  fn setDefaultButton_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox16setDefaultButtonEP11QPushButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:159
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setDefaultButton(QMessageBox::StandardButton)

/*
Sets the message box's default button to button.

This function was introduced in  Qt 4.2.

See also defaultButton(), addButton(), and QPushButton::setDefault().
*/
impl /*struct*/ QMessageBox {
  pub fn setDefaultButton_1<RetType, T: QMessageBox_setDefaultButton_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultButton_1(self);
    // return 1;
  }
}
pub trait QMessageBox_setDefaultButton_1<RetType> {
  fn setDefaultButton_1(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setDefaultButton_1<(/*void*/)> for (i32) {
  fn setDefaultButton_1(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox16setDefaultButtonENS_14StandardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:161
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractButton * escapeButton() const

/*
Returns the button that is activated when escape is pressed.

By default, QMessageBox attempts to automatically detect an escape button as follows:
*/
impl /*struct*/ QMessageBox {
  pub fn escapeButton_0<RetType, T: QMessageBox_escapeButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.escapeButton_0(self);
    // return 1;
  }
}
pub trait QMessageBox_escapeButton_0<RetType> {
  fn escapeButton_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_escapeButton_0<usize> for () {
  fn escapeButton_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox12escapeButtonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEscapeButton(QAbstractButton *)

/*
Sets the button that gets activated when the Escape key is pressed to button.

This function was introduced in  Qt 4.2.

See also escapeButton(), addButton(), and clickedButton().
*/
impl /*struct*/ QMessageBox {
  pub fn setEscapeButton_0<RetType, T: QMessageBox_setEscapeButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEscapeButton_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setEscapeButton_0<RetType> {
  fn setEscapeButton_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setEscapeButton_0<(/*void*/)> for (usize) {
  fn setEscapeButton_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox15setEscapeButtonEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:163
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setEscapeButton(QMessageBox::StandardButton)

/*
Sets the button that gets activated when the Escape key is pressed to button.

This function was introduced in  Qt 4.2.

See also escapeButton(), addButton(), and clickedButton().
*/
impl /*struct*/ QMessageBox {
  pub fn setEscapeButton_1<RetType, T: QMessageBox_setEscapeButton_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEscapeButton_1(self);
    // return 1;
  }
}
pub trait QMessageBox_setEscapeButton_1<RetType> {
  fn setEscapeButton_1(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setEscapeButton_1<(/*void*/)> for (i32) {
  fn setEscapeButton_1(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox15setEscapeButtonENS_14StandardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:165
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractButton * clickedButton() const

/*
Returns the button that was clicked by the user, or 0 if the user hit the Esc key and no escape button was set.

If exec() hasn't been called yet, returns 0.

Example:


  QMessageBox messageBox(this);
  QAbstractButton *disconnectButton =
        messageBox.addButton(tr("Disconnect"), QMessageBox::ActionRole);
  ...
  messageBox.exec();
  if (messageBox.clickedButton() == disconnectButton) {
      ...
  }



This function was introduced in  Qt 4.2.

See also standardButton() and button().
*/
impl /*struct*/ QMessageBox {
  pub fn clickedButton_0<RetType, T: QMessageBox_clickedButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clickedButton_0(self);
    // return 1;
  }
}
pub trait QMessageBox_clickedButton_0<RetType> {
  fn clickedButton_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_clickedButton_0<usize> for () {
  fn clickedButton_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox13clickedButtonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:167
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn text_0<RetType, T: QMessageBox_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QMessageBox_text_0<RetType> {
  fn text_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_text_0<usize> for () {
  fn text_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setText_0<RetType, T: QMessageBox_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setText_0<RetType> {
  fn setText_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:170
// index:0
// Public Visibility=Default Availability=Available
// [4] QMessageBox::Icon icon() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn icon_0<RetType, T: QMessageBox_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QMessageBox_icon_0<RetType> {
  fn icon_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_icon_0<i32> for () {
  fn icon_0(self , rsthis: & QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:171
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIcon(QMessageBox::Icon)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setIcon_0<RetType, T: QMessageBox_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setIcon_0<(/*void*/)> for (i32) {
  fn setIcon_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox7setIconENS_4IconE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:173
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap iconPixmap() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn iconPixmap_0<RetType, T: QMessageBox_iconPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconPixmap_0(self);
    // return 1;
  }
}
pub trait QMessageBox_iconPixmap_0<RetType> {
  fn iconPixmap_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_iconPixmap_0<usize> for () {
  fn iconPixmap_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox10iconPixmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:174
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconPixmap(const QPixmap &)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setIconPixmap_0<RetType, T: QMessageBox_setIconPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconPixmap_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setIconPixmap_0<RetType> {
  fn setIconPixmap_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setIconPixmap_0<(/*void*/)> for (usize) {
  fn setIconPixmap_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox13setIconPixmapERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:176
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextFormat textFormat() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn textFormat_0<RetType, T: QMessageBox_textFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textFormat_0(self);
    // return 1;
  }
}
pub trait QMessageBox_textFormat_0<RetType> {
  fn textFormat_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_textFormat_0<i32> for () {
  fn textFormat_0(self , rsthis: & QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox10textFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextFormat(Qt::TextFormat)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setTextFormat_0<RetType, T: QMessageBox_setTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextFormat_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setTextFormat_0<RetType> {
  fn setTextFormat_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setTextFormat_0<(/*void*/)> for (i32) {
  fn setTextFormat_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox13setTextFormatEN2Qt10TextFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextInteractionFlags(Qt::TextInteractionFlags)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setTextInteractionFlags_0<RetType, T: QMessageBox_setTextInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setTextInteractionFlags_0<RetType> {
  fn setTextInteractionFlags_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setTextInteractionFlags_0<(/*void*/)> for (i32) {
  fn setTextInteractionFlags_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox23setTextInteractionFlagsE6QFlagsIN2Qt19TextInteractionFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:180
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextInteractionFlags textInteractionFlags() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn textInteractionFlags_0<RetType, T: QMessageBox_textInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QMessageBox_textInteractionFlags_0<RetType> {
  fn textInteractionFlags_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_textInteractionFlags_0<i32> for () {
  fn textInteractionFlags_0(self , rsthis: & QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox20textInteractionFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCheckBox(QCheckBox *)

/*
Sets the checkbox cb on the message dialog. The message box takes ownership of the checkbox. The argument cb can be 0 to remove an existing checkbox from the message box.

This function was introduced in  Qt 5.2.

See also checkBox().
*/
impl /*struct*/ QMessageBox {
  pub fn setCheckBox_0<RetType, T: QMessageBox_setCheckBox_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckBox_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setCheckBox_0<RetType> {
  fn setCheckBox_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setCheckBox_0<(/*void*/)> for (usize) {
  fn setCheckBox_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox11setCheckBoxEP9QCheckBox", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:183
// index:0
// Public Visibility=Default Availability=Available
// [8] QCheckBox * checkBox() const

/*
Returns the checkbox shown on the dialog. This is 0 if no checkbox is set.

This function was introduced in  Qt 5.2.

See also setCheckBox().
*/
impl /*struct*/ QMessageBox {
  pub fn checkBox_0<RetType, T: QMessageBox_checkBox_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkBox_0(self);
    // return 1;
  }
}
pub trait QMessageBox_checkBox_0<RetType> {
  fn checkBox_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_checkBox_0<usize> for () {
  fn checkBox_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox8checkBoxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:185
// index:0
// Public static Visibility=Default Availability=Available
// [4] QMessageBox::StandardButton information(QWidget *, const QString &, const QString &, QMessageBox::StandardButtons, QMessageBox::StandardButton)

/*
Opens an information message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), warning(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn information_0<RetType, T: QMessageBox_information_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.information_0();
    // return 1;
  }
}
pub trait QMessageBox_information_0<RetType> {
  fn information_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_information_0<i32> for (usize,usize,usize,i32,i32) {
  fn information_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_6QFlagsINS_14StandardButtonEES6_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:207
// index:1
// Public static Visibility=Default Availability=Available
// [4] int information(QWidget *, const QString &, const QString &, int, int, int)

/*
Opens an information message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), warning(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn information_1<RetType, T: QMessageBox_information_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.information_1();
    // return 1;
  }
}
pub trait QMessageBox_information_1<RetType> {
  fn information_1(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_information_1<i32> for (usize,usize,usize,i32,i32,i32) {
  fn information_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_iii", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:210
// index:2
// Public static Visibility=Default Availability=Available
// [4] int information(QWidget *, const QString &, const QString &, const QString &, const QString &, const QString &, int, int)

/*
Opens an information message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), warning(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn information_2<RetType, T: QMessageBox_information_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.information_2();
    // return 1;
  }
}
pub trait QMessageBox_information_2<RetType> {
  fn information_2(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_information_2<i32> for (usize,usize,usize,usize,usize,usize,i32,i32) {
  fn information_2(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_S4_S4_S4_ii", 8,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:217
// index:3
// Public static inline Visibility=Default Availability=Available
// [4] QMessageBox::StandardButton information(QWidget *, const QString &, const QString &, QMessageBox::StandardButton, QMessageBox::StandardButton)

/*
Opens an information message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), warning(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn information_3<RetType, T: QMessageBox_information_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.information_3();
    // return 1;
  }
}
pub trait QMessageBox_information_3<RetType> {
  fn information_3(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_information_3<i32> for (usize,usize,usize,i32,i32) {
  fn information_3(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_NS_14StandardButtonES5_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:188
// index:0
// Public static Visibility=Default Availability=Available
// [4] QMessageBox::StandardButton question(QWidget *, const QString &, const QString &, QMessageBox::StandardButtons, QMessageBox::StandardButton)

/*
Opens a question message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also information(), warning(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn question_0<RetType, T: QMessageBox_question_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.question_0();
    // return 1;
  }
}
pub trait QMessageBox_question_0<RetType> {
  fn question_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_question_0<i32> for (usize,usize,usize,i32,i32) {
  fn question_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_6QFlagsINS_14StandardButtonEES6_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:222
// index:1
// Public static Visibility=Default Availability=Available
// [4] int question(QWidget *, const QString &, const QString &, int, int, int)

/*
Opens a question message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also information(), warning(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn question_1<RetType, T: QMessageBox_question_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.question_1();
    // return 1;
  }
}
pub trait QMessageBox_question_1<RetType> {
  fn question_1(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_question_1<i32> for (usize,usize,usize,i32,i32,i32) {
  fn question_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_iii", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:225
// index:2
// Public static Visibility=Default Availability=Available
// [4] int question(QWidget *, const QString &, const QString &, const QString &, const QString &, const QString &, int, int)

/*
Opens a question message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also information(), warning(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn question_2<RetType, T: QMessageBox_question_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.question_2();
    // return 1;
  }
}
pub trait QMessageBox_question_2<RetType> {
  fn question_2(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_question_2<i32> for (usize,usize,usize,usize,usize,usize,i32,i32) {
  fn question_2(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_S4_S4_S4_ii", 8,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:232
// index:3
// Public static inline Visibility=Default Availability=Available
// [4] int question(QWidget *, const QString &, const QString &, QMessageBox::StandardButton, QMessageBox::StandardButton)

/*
Opens a question message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also information(), warning(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn question_3<RetType, T: QMessageBox_question_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.question_3();
    // return 1;
  }
}
pub trait QMessageBox_question_3<RetType> {
  fn question_3(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_question_3<i32> for (usize,usize,usize,i32,i32) {
  fn question_3(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_NS_14StandardButtonES5_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:191
// index:0
// Public static Visibility=Default Availability=Available
// [4] QMessageBox::StandardButton warning(QWidget *, const QString &, const QString &, QMessageBox::StandardButtons, QMessageBox::StandardButton)

/*
Opens a warning message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), information(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn warning_0<RetType, T: QMessageBox_warning_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.warning_0();
    // return 1;
  }
}
pub trait QMessageBox_warning_0<RetType> {
  fn warning_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_warning_0<i32> for (usize,usize,usize,i32,i32) {
  fn warning_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_6QFlagsINS_14StandardButtonEES6_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:237
// index:1
// Public static Visibility=Default Availability=Available
// [4] int warning(QWidget *, const QString &, const QString &, int, int, int)

/*
Opens a warning message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), information(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn warning_1<RetType, T: QMessageBox_warning_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.warning_1();
    // return 1;
  }
}
pub trait QMessageBox_warning_1<RetType> {
  fn warning_1(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_warning_1<i32> for (usize,usize,usize,i32,i32,i32) {
  fn warning_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_iii", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:240
// index:2
// Public static Visibility=Default Availability=Available
// [4] int warning(QWidget *, const QString &, const QString &, const QString &, const QString &, const QString &, int, int)

/*
Opens a warning message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), information(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn warning_2<RetType, T: QMessageBox_warning_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.warning_2();
    // return 1;
  }
}
pub trait QMessageBox_warning_2<RetType> {
  fn warning_2(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_warning_2<i32> for (usize,usize,usize,usize,usize,usize,i32,i32) {
  fn warning_2(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_S4_S4_S4_ii", 8,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:247
// index:3
// Public static inline Visibility=Default Availability=Available
// [4] int warning(QWidget *, const QString &, const QString &, QMessageBox::StandardButton, QMessageBox::StandardButton)

/*
Opens a warning message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), information(), and critical().
*/
impl /*struct*/ QMessageBox {
  pub fn warning_3<RetType, T: QMessageBox_warning_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.warning_3();
    // return 1;
  }
}
pub trait QMessageBox_warning_3<RetType> {
  fn warning_3(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_warning_3<i32> for (usize,usize,usize,i32,i32) {
  fn warning_3(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_NS_14StandardButtonES5_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:194
// index:0
// Public static Visibility=Default Availability=Available
// [4] QMessageBox::StandardButton critical(QWidget *, const QString &, const QString &, QMessageBox::StandardButtons, QMessageBox::StandardButton)

/*
Opens a critical message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), warning(), and information().
*/
impl /*struct*/ QMessageBox {
  pub fn critical_0<RetType, T: QMessageBox_critical_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.critical_0();
    // return 1;
  }
}
pub trait QMessageBox_critical_0<RetType> {
  fn critical_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_critical_0<i32> for (usize,usize,usize,i32,i32) {
  fn critical_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_6QFlagsINS_14StandardButtonEES6_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:252
// index:1
// Public static Visibility=Default Availability=Available
// [4] int critical(QWidget *, const QString &, const QString &, int, int, int)

/*
Opens a critical message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), warning(), and information().
*/
impl /*struct*/ QMessageBox {
  pub fn critical_1<RetType, T: QMessageBox_critical_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.critical_1();
    // return 1;
  }
}
pub trait QMessageBox_critical_1<RetType> {
  fn critical_1(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_critical_1<i32> for (usize,usize,usize,i32,i32,i32) {
  fn critical_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_iii", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:255
// index:2
// Public static Visibility=Default Availability=Available
// [4] int critical(QWidget *, const QString &, const QString &, const QString &, const QString &, const QString &, int, int)

/*
Opens a critical message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), warning(), and information().
*/
impl /*struct*/ QMessageBox {
  pub fn critical_2<RetType, T: QMessageBox_critical_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.critical_2();
    // return 1;
  }
}
pub trait QMessageBox_critical_2<RetType> {
  fn critical_2(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_critical_2<i32> for (usize,usize,usize,usize,usize,usize,i32,i32) {
  fn critical_2(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_S4_S4_S4_ii", 8,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:262
// index:3
// Public static inline Visibility=Default Availability=Available
// [4] int critical(QWidget *, const QString &, const QString &, QMessageBox::StandardButton, QMessageBox::StandardButton)

/*
Opens a critical message box with the given title and text in front of the specified parent widget.

The standard buttons are added to the message box. defaultButton specifies the button used when Enter is pressed. defaultButton must refer to a button that was given in buttons. If defaultButton is QMessageBox::NoButton, QMessageBox chooses a suitable default automatically.

Returns the identity of the standard button that was clicked. If Esc was pressed instead, the escape button is returned.

The message box is an application modal dialog box.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QMessageBox constructors.

This function was introduced in  Qt 4.2.

See also question(), warning(), and information().
*/
impl /*struct*/ QMessageBox {
  pub fn critical_3<RetType, T: QMessageBox_critical_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.critical_3();
    // return 1;
  }
}
pub trait QMessageBox_critical_3<RetType> {
  fn critical_3(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_critical_3<i32> for (usize,usize,usize,i32,i32) {
  fn critical_3(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_NS_14StandardButtonES5_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:197
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void about(QWidget *, const QString &, const QString &)

/*
Displays a simple about box with title title and text text. The about box's parent is parent.

about() looks for a suitable icon in four locations:
*/
impl /*struct*/ QMessageBox {
  pub fn about_0<RetType, T: QMessageBox_about_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.about_0();
    // return 1;
  }
}
pub trait QMessageBox_about_0<RetType> {
  fn about_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_about_0<(/*void*/)> for (usize,usize,usize) {
  fn about_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox5aboutEP7QWidgetRK7QStringS4_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:198
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void aboutQt(QWidget *, const QString &)

/*
Displays a simple message box about Qt, with the given title and centered over parent (if parent is not 0). The message includes the version number of Qt being used by the application.

This is useful for inclusion in the Help menu of an application, as shown in the Menus example.

QApplication provides this functionality as a slot.

On macOS, the about box is popped up as a modeless window; on other platforms, it is currently application modal.

See also QApplication::aboutQt().
*/
impl /*struct*/ QMessageBox {
  pub fn aboutQt_0<RetType, T: QMessageBox_aboutQt_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.aboutQt_0();
    // return 1;
  }
}
pub trait QMessageBox_aboutQt_0<RetType> {
  fn aboutQt_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_aboutQt_0<(/*void*/)> for (usize,usize) {
  fn aboutQt_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox7aboutQtEP7QWidgetRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:267
// index:0
// Public Visibility=Default Availability=Available
// [8] QString buttonText(int) const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn buttonText_0<RetType, T: QMessageBox_buttonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonText_0(self);
    // return 1;
  }
}
pub trait QMessageBox_buttonText_0<RetType> {
  fn buttonText_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_buttonText_0<usize> for (i32) {
  fn buttonText_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox10buttonTextEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtonText(int, const QString &)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setButtonText_0<RetType, T: QMessageBox_setButtonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtonText_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setButtonText_0<RetType> {
  fn setButtonText_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setButtonText_0<(/*void*/)> for (i32,usize) {
  fn setButtonText_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox13setButtonTextEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:270
// index:0
// Public Visibility=Default Availability=Available
// [8] QString informativeText() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn informativeText_0<RetType, T: QMessageBox_informativeText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.informativeText_0(self);
    // return 1;
  }
}
pub trait QMessageBox_informativeText_0<RetType> {
  fn informativeText_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_informativeText_0<usize> for () {
  fn informativeText_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox15informativeTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:271
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInformativeText(const QString &)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setInformativeText_0<RetType, T: QMessageBox_setInformativeText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInformativeText_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setInformativeText_0<RetType> {
  fn setInformativeText_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setInformativeText_0<(/*void*/)> for (usize) {
  fn setInformativeText_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox18setInformativeTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:274
// index:0
// Public Visibility=Default Availability=Available
// [8] QString detailedText() const

/*

*/
impl /*struct*/ QMessageBox {
  pub fn detailedText_0<RetType, T: QMessageBox_detailedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detailedText_0(self);
    // return 1;
  }
}
pub trait QMessageBox_detailedText_0<RetType> {
  fn detailedText_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_detailedText_0<usize> for () {
  fn detailedText_0(self , rsthis: & QMessageBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMessageBox12detailedTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:275
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDetailedText(const QString &)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn setDetailedText_0<RetType, T: QMessageBox_setDetailedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDetailedText_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setDetailedText_0<RetType> {
  fn setDetailedText_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setDetailedText_0<(/*void*/)> for (usize) {
  fn setDetailedText_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox15setDetailedTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:278
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowTitle(const QString &)

/*
This function shadows QWidget::setWindowTitle().

Sets the title of the message box to title. On macOS, the window title is ignored (as required by the macOS Guidelines).

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QMessageBox {
  pub fn setWindowTitle_0<RetType, T: QMessageBox_setWindowTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowTitle_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setWindowTitle_0<RetType> {
  fn setWindowTitle_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setWindowTitle_0<(/*void*/)> for (usize) {
  fn setWindowTitle_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox14setWindowTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:279
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowModality(Qt::WindowModality)

/*
This function shadows QWidget::setWindowModality().

Sets the modality of the message box to windowModality.

On macOS, if the modality is set to Qt::WindowModal and the message box has a parent, then the message box will be a Qt::Sheet, otherwise the message box will be a standard dialog.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QMessageBox {
  pub fn setWindowModality_0<RetType, T: QMessageBox_setWindowModality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowModality_0(self);
    // return 1;
  }
}
pub trait QMessageBox_setWindowModality_0<RetType> {
  fn setWindowModality_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_setWindowModality_0<(/*void*/)> for (i32) {
  fn setWindowModality_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox17setWindowModalityEN2Qt14WindowModalityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:282
// index:0
// Public static Visibility=Default Availability=Available
// [32] QPixmap standardIcon(QMessageBox::Icon)

/*

*/
impl /*struct*/ QMessageBox {
  pub fn standardIcon_0<RetType, T: QMessageBox_standardIcon_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.standardIcon_0();
    // return 1;
  }
}
pub trait QMessageBox_standardIcon_0<RetType> {
  fn standardIcon_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_standardIcon_0<usize> for (i32) {
  fn standardIcon_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox12standardIconENS_4IconE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:285
// index:0
// Public Visibility=Default Availability=Available
// [-2] void buttonClicked(QAbstractButton *)

/*
This signal is emitted whenever a button is clicked inside the QMessageBox. The button that was clicked in returned in button.
*/
impl /*struct*/ QMessageBox {
  pub fn buttonClicked_0<RetType, T: QMessageBox_buttonClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonClicked_0(self);
    // return 1;
  }
}
pub trait QMessageBox_buttonClicked_0<RetType> {
  fn buttonClicked_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_buttonClicked_0<(/*void*/)> for (usize) {
  fn buttonClicked_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox13buttonClickedEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:293
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QMessageBox {
  pub fn event_0<RetType, T: QMessageBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QMessageBox_event_0<RetType> {
  fn event_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QMessageBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMessageBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:294
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QMessageBox {
  pub fn resizeEvent_0<RetType, T: QMessageBox_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QMessageBox_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:295
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QMessageBox {
  pub fn showEvent_0<RetType, T: QMessageBox_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QMessageBox_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:296
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEvent(QCloseEvent *)

/*
Reimplemented from QWidget::closeEvent().
*/
impl /*struct*/ QMessageBox {
  pub fn closeEvent_0<RetType, T: QMessageBox_closeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEvent_0(self);
    // return 1;
  }
}
pub trait QMessageBox_closeEvent_0<RetType> {
  fn closeEvent_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_closeEvent_0<(/*void*/)> for (usize) {
  fn closeEvent_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox10closeEventEP11QCloseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:297
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QMessageBox {
  pub fn keyPressEvent_0<RetType, T: QMessageBox_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QMessageBox_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmessagebox.h:298
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QMessageBox {
  pub fn changeEvent_0<RetType, T: QMessageBox_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QMessageBox_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QMessageBox) -> RetType;
}
impl<'a> /*trait*/ QMessageBox_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QMessageBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMessageBox11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum has the following values:


*/
pub type QMessageBox__Icon = i32;
// the message box does not have any icon.
pub const QMessageBox__NoIcon :QMessageBox__Icon = 0;
// an icon indicating that the message is nothing out of the ordinary.
pub const QMessageBox__Information :QMessageBox__Icon = 1;
// an icon indicating that the message is a warning, but can be dealt with.
pub const QMessageBox__Warning :QMessageBox__Icon = 2;
// an icon indicating that the message represents a critical problem.
pub const QMessageBox__Critical :QMessageBox__Icon = 3;
// an icon indicating that the message is asking a question.
pub const QMessageBox__Question :QMessageBox__Icon = 4;
pub fn QMessageBox_IconItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMessageBox", val);
}
pub fn QMessageBox_IconItemName_s(val: i32) ->String {
  //var nilthis *QMessageBox
  //return nilthis.IconItemName(val);
  return QMessageBox_IconItemName(val);
}


/*
This enum describes the roles that can be used to describe buttons in the button box. Combinations of these roles are as flags used to describe different aspects of their behavior.



See also StandardButton.

*/
pub type QMessageBox__ButtonRole = i32;
// 
pub const QMessageBox__InvalidRole :QMessageBox__ButtonRole = -1;
// Clicking the button causes the dialog to be accepted (e.g. OK).
pub const QMessageBox__AcceptRole :QMessageBox__ButtonRole = 0;
// Clicking the button causes the dialog to be rejected (e.g. Cancel).
pub const QMessageBox__RejectRole :QMessageBox__ButtonRole = 1;
// Clicking the button causes a destructive change (e.g. for Discarding Changes) and closes the dialog.
pub const QMessageBox__DestructiveRole :QMessageBox__ButtonRole = 2;
// Clicking the button causes changes to the elements within the dialog.
pub const QMessageBox__ActionRole :QMessageBox__ButtonRole = 3;
// The button can be clicked to request help.
pub const QMessageBox__HelpRole :QMessageBox__ButtonRole = 4;
// The button is a "Yes"-like button.
pub const QMessageBox__YesRole :QMessageBox__ButtonRole = 5;
// The button is a "No"-like button.
pub const QMessageBox__NoRole :QMessageBox__ButtonRole = 6;
// The button resets the dialog's fields to default values.
pub const QMessageBox__ResetRole :QMessageBox__ButtonRole = 7;
// The button applies current changes.
pub const QMessageBox__ApplyRole :QMessageBox__ButtonRole = 8;
// 
pub const QMessageBox__NRoles :QMessageBox__ButtonRole = 9;
pub fn QMessageBox_ButtonRoleItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMessageBox", val);
}
pub fn QMessageBox_ButtonRoleItemName_s(val: i32) ->String {
  //var nilthis *QMessageBox
  //return nilthis.ButtonRoleItemName(val);
  return QMessageBox_ButtonRoleItemName(val);
}


/*


*/
pub type QMessageBox__StandardButton = i32;
// 
pub const QMessageBox__NoButton :QMessageBox__StandardButton = 0;
// 
pub const QMessageBox__Ok :QMessageBox__StandardButton = 1024;
// 
pub const QMessageBox__Save :QMessageBox__StandardButton = 2048;
// 
pub const QMessageBox__SaveAll :QMessageBox__StandardButton = 4096;
// 
pub const QMessageBox__Open :QMessageBox__StandardButton = 8192;
// 
pub const QMessageBox__Yes :QMessageBox__StandardButton = 16384;
// 
pub const QMessageBox__YesToAll :QMessageBox__StandardButton = 32768;
// 
pub const QMessageBox__No :QMessageBox__StandardButton = 65536;
// 
pub const QMessageBox__NoToAll :QMessageBox__StandardButton = 131072;
// 
pub const QMessageBox__Abort :QMessageBox__StandardButton = 262144;
// 
pub const QMessageBox__Retry :QMessageBox__StandardButton = 524288;
// 
pub const QMessageBox__Ignore :QMessageBox__StandardButton = 1048576;
// 
pub const QMessageBox__Close :QMessageBox__StandardButton = 2097152;
// 
pub const QMessageBox__Cancel :QMessageBox__StandardButton = 4194304;
// 
pub const QMessageBox__Discard :QMessageBox__StandardButton = 8388608;
// 
pub const QMessageBox__Help :QMessageBox__StandardButton = 16777216;
// 
pub const QMessageBox__Apply :QMessageBox__StandardButton = 33554432;
// 
pub const QMessageBox__Reset :QMessageBox__StandardButton = 67108864;
// 
pub const QMessageBox__RestoreDefaults :QMessageBox__StandardButton = 134217728;
// 
pub const QMessageBox__FirstButton :QMessageBox__StandardButton = 1024;
// 
pub const QMessageBox__LastButton :QMessageBox__StandardButton = 134217728;
// 
pub const QMessageBox__YesAll :QMessageBox__StandardButton = 32768;
// 
pub const QMessageBox__NoAll :QMessageBox__StandardButton = 131072;
// 
pub const QMessageBox__Default :QMessageBox__StandardButton = 256;
// 
pub const QMessageBox__Escape :QMessageBox__StandardButton = 512;
// 
pub const QMessageBox__FlagMask :QMessageBox__StandardButton = 768;
// 
pub const QMessageBox__ButtonMask :QMessageBox__StandardButton = -769;
pub fn QMessageBox_StandardButtonItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMessageBox", val);
}
pub fn QMessageBox_StandardButtonItemName_s(val: i32) ->String {
  //var nilthis *QMessageBox
  //return nilthis.StandardButtonItemName(val);
  return QMessageBox_StandardButtonItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
