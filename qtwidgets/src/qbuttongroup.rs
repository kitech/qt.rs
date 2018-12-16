

// mod ::widgets::QButtonGroup
// package qtwidgets
// /usr/include/qt/QtWidgets/qbuttongroup.h
// #include <qbuttongroup.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QButtonGroup)=16
pub struct QButtonGroup {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QButtonGroup_ITF interface {
//    qtcore.QObject_ITF
//    QButtonGroup_PTR() *QButtonGroup
//}
//func (ptr *QButtonGroup) QButtonGroup_PTR() *QButtonGroup { return ptr }

impl /*struct*/ QButtonGroup {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QButtonGroup {
    return QButtonGroup{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QButtonGroup {
//  type Target = QButtonGroupBASE;
//
//  fn deref(&self) -> &QButtonGroupBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QButtonGroupBASE> for QButtonGroup {
//  fn as_ref(& self) -> & QButtonGroupBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qbuttongroup.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QButtonGroup {
  pub fn metaObject_0<RetType, T: QButtonGroup_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QButtonGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QButtonGroup10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QButtonGroup(QObject *)

/*
Constructs a new, empty button group with the given parent.

See also addButton() and setExclusive().
*/
// QButtonGroup(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QButtonGroup {
  pub fn QButtonGroup_0<T: QButtonGroup_QButtonGroup_0>(value: T) -> QButtonGroup {
    let rsthis = value.QButtonGroup_0();
    return rsthis;
    // return 1;
  }
}

pub trait QButtonGroup_QButtonGroup_0 {
  fn QButtonGroup_0(self) -> QButtonGroup;
}
// QButtonGroup(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QButtonGroup_QButtonGroup_0 for (usize) {
  fn QButtonGroup_0(self) -> QButtonGroup {
    // unsafe{_ZN12QButtonGroupC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QButtonGroupC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QButtonGroup{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QButtonGroup()

/*

*/
pub fn DeleteQButtonGroup(this :*mut QButtonGroup) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QButtonGroupD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qbuttongroup.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExclusive(bool)

/*

*/
impl /*struct*/ QButtonGroup {
  pub fn setExclusive_0<RetType, T: QButtonGroup_setExclusive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExclusive_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_setExclusive_0<RetType> {
  fn setExclusive_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_setExclusive_0<(/*void*/)> for (bool) {
  fn setExclusive_0(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup12setExclusiveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:64
// index:0
// Public Visibility=Default Availability=Available
// [1] bool exclusive() const

/*

*/
impl /*struct*/ QButtonGroup {
  pub fn exclusive_0<RetType, T: QButtonGroup_exclusive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exclusive_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_exclusive_0<RetType> {
  fn exclusive_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_exclusive_0<bool> for () {
  fn exclusive_0(self , rsthis: & QButtonGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QButtonGroup9exclusiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addButton(QAbstractButton *, int)

/*
Adds the given button to the button group. If id is -1, an id will be assigned to the button. Automatically assigned ids are guaranteed to be negative, starting with -2. If you are assigning your own ids, use positive values to avoid conflicts.

See also removeButton() and buttons().
*/
impl /*struct*/ QButtonGroup {
  pub fn addButton_0<RetType, T: QButtonGroup_addButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addButton_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_addButton_0<RetType> {
  fn addButton_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_addButton_0<(/*void*/)> for (usize,i32) {
  fn addButton_0(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup9addButtonEP15QAbstractButtoni", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeButton(QAbstractButton *)

/*
Removes the given button from the button group.

See also addButton() and buttons().
*/
impl /*struct*/ QButtonGroup {
  pub fn removeButton_0<RetType, T: QButtonGroup_removeButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeButton_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_removeButton_0<RetType> {
  fn removeButton_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_removeButton_0<(/*void*/)> for (usize) {
  fn removeButton_0(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup12removeButtonEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractButton * checkedButton() const

/*
Returns the button group's checked button, or 0 if no buttons are checked.

See also buttonClicked().
*/
impl /*struct*/ QButtonGroup {
  pub fn checkedButton_0<RetType, T: QButtonGroup_checkedButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkedButton_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_checkedButton_0<RetType> {
  fn checkedButton_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_checkedButton_0<usize> for () {
  fn checkedButton_0(self , rsthis: & QButtonGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QButtonGroup13checkedButtonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractButton * button(int) const

/*
Returns the button with the specified id, or 0 if no such button exists.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QButtonGroup {
  pub fn button_0<RetType, T: QButtonGroup_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_button_0<RetType> {
  fn button_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_button_0<usize> for (i32) {
  fn button_0(self , rsthis: & QButtonGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QButtonGroup6buttonEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setId(QAbstractButton *, int)

/*
Sets the id for the specified button. Note that id cannot be -1.

This function was introduced in  Qt 4.1.

See also id().
*/
impl /*struct*/ QButtonGroup {
  pub fn setId_0<RetType, T: QButtonGroup_setId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setId_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_setId_0<RetType> {
  fn setId_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_setId_0<(/*void*/)> for (usize,i32) {
  fn setId_0(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup5setIdEP15QAbstractButtoni", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int id(QAbstractButton *) const

/*
Returns the id for the specified button, or -1 if no such button exists.

This function was introduced in  Qt 4.1.

See also setId().
*/
impl /*struct*/ QButtonGroup {
  pub fn id_0<RetType, T: QButtonGroup_id_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.id_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_id_0<RetType> {
  fn id_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_id_0<i32> for (usize) {
  fn id_0(self , rsthis: & QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QButtonGroup2idEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] int checkedId() const

/*
Returns the id of the checkedButton(), or -1 if no button is checked.

This function was introduced in  Qt 4.1.

See also setId().
*/
impl /*struct*/ QButtonGroup {
  pub fn checkedId_0<RetType, T: QButtonGroup_checkedId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkedId_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_checkedId_0<RetType> {
  fn checkedId_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_checkedId_0<i32> for () {
  fn checkedId_0(self , rsthis: & QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QButtonGroup9checkedIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void buttonClicked(QAbstractButton *)

/*
This signal is emitted when the given button is clicked. A button is clicked when it is first pressed and then released, when its shortcut key is typed, or when QAbstractButton::click() or QAbstractButton::animateClick() is programmatically called.

Note: Signal buttonClicked is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(buttonGroup, QOverload<QAbstractButton *>::of(&QButtonGroup::buttonClicked),
      [=](QAbstractButton *button){ /-* ... *-/ });



See also checkedButton() and QAbstractButton::clicked().
*/
impl /*struct*/ QButtonGroup {
  pub fn buttonClicked_0<RetType, T: QButtonGroup_buttonClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonClicked_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_buttonClicked_0<RetType> {
  fn buttonClicked_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_buttonClicked_0<(/*void*/)> for (usize) {
  fn buttonClicked_0(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup13buttonClickedEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:81
// index:1
// Public Visibility=Default Availability=Available
// [-2] void buttonClicked(int)

/*
This signal is emitted when the given button is clicked. A button is clicked when it is first pressed and then released, when its shortcut key is typed, or when QAbstractButton::click() or QAbstractButton::animateClick() is programmatically called.

Note: Signal buttonClicked is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(buttonGroup, QOverload<QAbstractButton *>::of(&QButtonGroup::buttonClicked),
      [=](QAbstractButton *button){ /-* ... *-/ });



See also checkedButton() and QAbstractButton::clicked().
*/
impl /*struct*/ QButtonGroup {
  pub fn buttonClicked_1<RetType, T: QButtonGroup_buttonClicked_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonClicked_1(self);
    // return 1;
  }
}
pub trait QButtonGroup_buttonClicked_1<RetType> {
  fn buttonClicked_1(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_buttonClicked_1<(/*void*/)> for (i32) {
  fn buttonClicked_1(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup13buttonClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void buttonPressed(QAbstractButton *)

/*
This signal is emitted when the given button is pressed down.

Note: Signal buttonPressed is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(buttonGroup, QOverload<QAbstractButton *>::of(&QButtonGroup::buttonPressed),
      [=](QAbstractButton *button){ /-* ... *-/ });



This function was introduced in  Qt 4.2.

See also QAbstractButton::pressed().
*/
impl /*struct*/ QButtonGroup {
  pub fn buttonPressed_0<RetType, T: QButtonGroup_buttonPressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonPressed_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_buttonPressed_0<RetType> {
  fn buttonPressed_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_buttonPressed_0<(/*void*/)> for (usize) {
  fn buttonPressed_0(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup13buttonPressedEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:83
// index:1
// Public Visibility=Default Availability=Available
// [-2] void buttonPressed(int)

/*
This signal is emitted when the given button is pressed down.

Note: Signal buttonPressed is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(buttonGroup, QOverload<QAbstractButton *>::of(&QButtonGroup::buttonPressed),
      [=](QAbstractButton *button){ /-* ... *-/ });



This function was introduced in  Qt 4.2.

See also QAbstractButton::pressed().
*/
impl /*struct*/ QButtonGroup {
  pub fn buttonPressed_1<RetType, T: QButtonGroup_buttonPressed_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonPressed_1(self);
    // return 1;
  }
}
pub trait QButtonGroup_buttonPressed_1<RetType> {
  fn buttonPressed_1(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_buttonPressed_1<(/*void*/)> for (i32) {
  fn buttonPressed_1(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup13buttonPressedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void buttonReleased(QAbstractButton *)

/*
This signal is emitted when the given button is released.

Note: Signal buttonReleased is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(buttonGroup, QOverload<QAbstractButton *>::of(&QButtonGroup::buttonReleased),
      [=](QAbstractButton *button){ /-* ... *-/ });



This function was introduced in  Qt 4.2.

See also QAbstractButton::released().
*/
impl /*struct*/ QButtonGroup {
  pub fn buttonReleased_0<RetType, T: QButtonGroup_buttonReleased_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonReleased_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_buttonReleased_0<RetType> {
  fn buttonReleased_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_buttonReleased_0<(/*void*/)> for (usize) {
  fn buttonReleased_0(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup14buttonReleasedEP15QAbstractButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:85
// index:1
// Public Visibility=Default Availability=Available
// [-2] void buttonReleased(int)

/*
This signal is emitted when the given button is released.

Note: Signal buttonReleased is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(buttonGroup, QOverload<QAbstractButton *>::of(&QButtonGroup::buttonReleased),
      [=](QAbstractButton *button){ /-* ... *-/ });



This function was introduced in  Qt 4.2.

See also QAbstractButton::released().
*/
impl /*struct*/ QButtonGroup {
  pub fn buttonReleased_1<RetType, T: QButtonGroup_buttonReleased_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonReleased_1(self);
    // return 1;
  }
}
pub trait QButtonGroup_buttonReleased_1<RetType> {
  fn buttonReleased_1(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_buttonReleased_1<(/*void*/)> for (i32) {
  fn buttonReleased_1(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup14buttonReleasedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void buttonToggled(QAbstractButton *, bool)

/*
This signal is emitted when the given button is toggled. checked is true if the button is checked, or false if the button is unchecked.

Note: Signal buttonToggled is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(buttonGroup, QOverload<QAbstractButton *, bool>::of(&QButtonGroup::buttonToggled),
      [=](QAbstractButton *button, bool checked){ /-* ... *-/ });



This function was introduced in  Qt 5.2.

See also QAbstractButton::toggled().
*/
impl /*struct*/ QButtonGroup {
  pub fn buttonToggled_0<RetType, T: QButtonGroup_buttonToggled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonToggled_0(self);
    // return 1;
  }
}
pub trait QButtonGroup_buttonToggled_0<RetType> {
  fn buttonToggled_0(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_buttonToggled_0<(/*void*/)> for (usize,bool) {
  fn buttonToggled_0(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup13buttonToggledEP15QAbstractButtonb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qbuttongroup.h:87
// index:1
// Public Visibility=Default Availability=Available
// [-2] void buttonToggled(int, bool)

/*
This signal is emitted when the given button is toggled. checked is true if the button is checked, or false if the button is unchecked.

Note: Signal buttonToggled is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(buttonGroup, QOverload<QAbstractButton *, bool>::of(&QButtonGroup::buttonToggled),
      [=](QAbstractButton *button, bool checked){ /-* ... *-/ });



This function was introduced in  Qt 5.2.

See also QAbstractButton::toggled().
*/
impl /*struct*/ QButtonGroup {
  pub fn buttonToggled_1<RetType, T: QButtonGroup_buttonToggled_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonToggled_1(self);
    // return 1;
  }
}
pub trait QButtonGroup_buttonToggled_1<RetType> {
  fn buttonToggled_1(self , rsthis: & QButtonGroup) -> RetType;
}
impl<'a> /*trait*/ QButtonGroup_buttonToggled_1<(/*void*/)> for (i32,bool) {
  fn buttonToggled_1(self , rsthis: & QButtonGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QButtonGroup13buttonToggledEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
