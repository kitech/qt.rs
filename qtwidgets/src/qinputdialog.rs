

// mod ::widgets::QInputDialog
// package qtwidgets
// /usr/include/qt/QtWidgets/qinputdialog.h
// #include <qinputdialog.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 96
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
#[derive(Default)] // class sizeof(QInputDialog)=48
pub struct QInputDialog {
  qbase: QDialog,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QInputDialog_ITF interface {
//    QDialog_ITF
//    QInputDialog_PTR() *QInputDialog
//}
//func (ptr *QInputDialog) QInputDialog_PTR() *QInputDialog { return ptr }

impl /*struct*/ QInputDialog {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QInputDialog {
    return QInputDialog{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QInputDialog {
//  type Target = QInputDialogBASE;
//
//  fn deref(&self) -> &QInputDialogBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QInputDialogBASE> for QInputDialog {
//  fn as_ref(& self) -> & QInputDialogBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qinputdialog.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn metaObject_0<RetType, T: QInputDialog_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QInputDialog_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QInputDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QInputDialog(QWidget *, Qt::WindowFlags)

/*
Constructs a new input dialog with the given parent and window flags.

This function was introduced in  Qt 4.5.
*/
// QInputDialog(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QInputDialog {
  pub fn QInputDialog_0<T: QInputDialog_QInputDialog_0>(value: T) -> QInputDialog {
    let rsthis = value.QInputDialog_0();
    return rsthis;
    // return 1;
  }
}

pub trait QInputDialog_QInputDialog_0 {
  fn QInputDialog_0(self) -> QInputDialog;
}
// QInputDialog(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QInputDialog_QInputDialog_0 for (usize,i32) {
  fn QInputDialog_0(self) -> QInputDialog {
    // unsafe{_ZN12QInputDialogC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QInputDialogC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QInputDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QInputDialog()

/*

*/
pub fn DeleteQInputDialog(this :*mut QInputDialog) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QInputDialogD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qinputdialog.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInputMode(QInputDialog::InputMode)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setInputMode_0<RetType, T: QInputDialog_setInputMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInputMode_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setInputMode_0<RetType> {
  fn setInputMode_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setInputMode_0<(/*void*/)> for (i32) {
  fn setInputMode_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog12setInputModeENS_9InputModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:98
// index:0
// Public Visibility=Default Availability=Available
// [4] QInputDialog::InputMode inputMode() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn inputMode_0<RetType, T: QInputDialog_inputMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMode_0(self);
    // return 1;
  }
}
pub trait QInputDialog_inputMode_0<RetType> {
  fn inputMode_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_inputMode_0<i32> for () {
  fn inputMode_0(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog9inputModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLabelText(const QString &)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setLabelText_0<RetType, T: QInputDialog_setLabelText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLabelText_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setLabelText_0<RetType> {
  fn setLabelText_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setLabelText_0<(/*void*/)> for (usize) {
  fn setLabelText_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog12setLabelTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QString labelText() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn labelText_0<RetType, T: QInputDialog_labelText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.labelText_0(self);
    // return 1;
  }
}
pub trait QInputDialog_labelText_0<RetType> {
  fn labelText_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_labelText_0<usize> for () {
  fn labelText_0(self , rsthis: & QInputDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog9labelTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOption(QInputDialog::InputDialogOption, bool)

/*
Sets the given option to be enabled if on is true; otherwise, clears the given option.

See also options and testOption().
*/
impl /*struct*/ QInputDialog {
  pub fn setOption_0<RetType, T: QInputDialog_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setOption_0<(/*void*/)> for (i32,bool) {
  fn setOption_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog9setOptionENS_17InputDialogOptionEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:104
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testOption(QInputDialog::InputDialogOption) const

/*
Returns true if the given option is enabled; otherwise, returns false.

See also options and setOption().
*/
impl /*struct*/ QInputDialog {
  pub fn testOption_0<RetType, T: QInputDialog_testOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_0(self);
    // return 1;
  }
}
pub trait QInputDialog_testOption_0<RetType> {
  fn testOption_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_testOption_0<bool> for (i32) {
  fn testOption_0(self , rsthis: & QInputDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog10testOptionENS_17InputDialogOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptions(QInputDialog::InputDialogOptions)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setOptions_0<RetType, T: QInputDialog_setOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptions_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setOptions_0<RetType> {
  fn setOptions_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setOptions_0<(/*void*/)> for (i32) {
  fn setOptions_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog10setOptionsE6QFlagsINS_17InputDialogOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:106
// index:0
// Public Visibility=Default Availability=Available
// [4] QInputDialog::InputDialogOptions options() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn options_0<RetType, T: QInputDialog_options_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.options_0(self);
    // return 1;
  }
}
pub trait QInputDialog_options_0<RetType> {
  fn options_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_options_0<i32> for () {
  fn options_0(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog7optionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextValue(const QString &)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setTextValue_0<RetType, T: QInputDialog_setTextValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextValue_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setTextValue_0<RetType> {
  fn setTextValue_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setTextValue_0<(/*void*/)> for (usize) {
  fn setTextValue_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog12setTextValueERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QString textValue() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn textValue_0<RetType, T: QInputDialog_textValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textValue_0(self);
    // return 1;
  }
}
pub trait QInputDialog_textValue_0<RetType> {
  fn textValue_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_textValue_0<usize> for () {
  fn textValue_0(self , rsthis: & QInputDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog9textValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextEchoMode(QLineEdit::EchoMode)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setTextEchoMode_0<RetType, T: QInputDialog_setTextEchoMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextEchoMode_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setTextEchoMode_0<RetType> {
  fn setTextEchoMode_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setTextEchoMode_0<(/*void*/)> for (i32) {
  fn setTextEchoMode_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog15setTextEchoModeEN9QLineEdit8EchoModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] QLineEdit::EchoMode textEchoMode() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn textEchoMode_0<RetType, T: QInputDialog_textEchoMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textEchoMode_0(self);
    // return 1;
  }
}
pub trait QInputDialog_textEchoMode_0<RetType> {
  fn textEchoMode_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_textEchoMode_0<i32> for () {
  fn textEchoMode_0(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog12textEchoModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setComboBoxEditable(bool)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setComboBoxEditable_0<RetType, T: QInputDialog_setComboBoxEditable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setComboBoxEditable_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setComboBoxEditable_0<RetType> {
  fn setComboBoxEditable_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setComboBoxEditable_0<(/*void*/)> for (bool) {
  fn setComboBoxEditable_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog19setComboBoxEditableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isComboBoxEditable() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn isComboBoxEditable_0<RetType, T: QInputDialog_isComboBoxEditable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isComboBoxEditable_0(self);
    // return 1;
  }
}
pub trait QInputDialog_isComboBoxEditable_0<RetType> {
  fn isComboBoxEditable_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_isComboBoxEditable_0<bool> for () {
  fn isComboBoxEditable_0(self , rsthis: & QInputDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog18isComboBoxEditableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setComboBoxItems(const QStringList &)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setComboBoxItems_0<RetType, T: QInputDialog_setComboBoxItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setComboBoxItems_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setComboBoxItems_0<RetType> {
  fn setComboBoxItems_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setComboBoxItems_0<(/*void*/)> for (usize) {
  fn setComboBoxItems_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog16setComboBoxItemsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:118
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList comboBoxItems() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn comboBoxItems_0<RetType, T: QInputDialog_comboBoxItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.comboBoxItems_0(self);
    // return 1;
  }
}
pub trait QInputDialog_comboBoxItems_0<RetType> {
  fn comboBoxItems_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_comboBoxItems_0<usize> for () {
  fn comboBoxItems_0(self , rsthis: & QInputDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog13comboBoxItemsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIntValue(int)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setIntValue_0<RetType, T: QInputDialog_setIntValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIntValue_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setIntValue_0<RetType> {
  fn setIntValue_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setIntValue_0<(/*void*/)> for (i32) {
  fn setIntValue_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog11setIntValueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:121
// index:0
// Public Visibility=Default Availability=Available
// [4] int intValue() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn intValue_0<RetType, T: QInputDialog_intValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intValue_0(self);
    // return 1;
  }
}
pub trait QInputDialog_intValue_0<RetType> {
  fn intValue_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_intValue_0<i32> for () {
  fn intValue_0(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog8intValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIntMinimum(int)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setIntMinimum_0<RetType, T: QInputDialog_setIntMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIntMinimum_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setIntMinimum_0<RetType> {
  fn setIntMinimum_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setIntMinimum_0<(/*void*/)> for (i32) {
  fn setIntMinimum_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog13setIntMinimumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:124
// index:0
// Public Visibility=Default Availability=Available
// [4] int intMinimum() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn intMinimum_0<RetType, T: QInputDialog_intMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intMinimum_0(self);
    // return 1;
  }
}
pub trait QInputDialog_intMinimum_0<RetType> {
  fn intMinimum_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_intMinimum_0<i32> for () {
  fn intMinimum_0(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog10intMinimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIntMaximum(int)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setIntMaximum_0<RetType, T: QInputDialog_setIntMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIntMaximum_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setIntMaximum_0<RetType> {
  fn setIntMaximum_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setIntMaximum_0<(/*void*/)> for (i32) {
  fn setIntMaximum_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog13setIntMaximumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:127
// index:0
// Public Visibility=Default Availability=Available
// [4] int intMaximum() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn intMaximum_0<RetType, T: QInputDialog_intMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intMaximum_0(self);
    // return 1;
  }
}
pub trait QInputDialog_intMaximum_0<RetType> {
  fn intMaximum_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_intMaximum_0<i32> for () {
  fn intMaximum_0(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog10intMaximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIntRange(int, int)

/*
Sets the range of integer values accepted by the dialog when used in IntInput mode, with minimum and maximum values specified by min and max respectively.
*/
impl /*struct*/ QInputDialog {
  pub fn setIntRange_0<RetType, T: QInputDialog_setIntRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIntRange_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setIntRange_0<RetType> {
  fn setIntRange_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setIntRange_0<(/*void*/)> for (i32,i32) {
  fn setIntRange_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog11setIntRangeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIntStep(int)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setIntStep_0<RetType, T: QInputDialog_setIntStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIntStep_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setIntStep_0<RetType> {
  fn setIntStep_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setIntStep_0<(/*void*/)> for (i32) {
  fn setIntStep_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog10setIntStepEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:132
// index:0
// Public Visibility=Default Availability=Available
// [4] int intStep() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn intStep_0<RetType, T: QInputDialog_intStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intStep_0(self);
    // return 1;
  }
}
pub trait QInputDialog_intStep_0<RetType> {
  fn intStep_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_intStep_0<i32> for () {
  fn intStep_0(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog7intStepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDoubleValue(double)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setDoubleValue_0<RetType, T: QInputDialog_setDoubleValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDoubleValue_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setDoubleValue_0<RetType> {
  fn setDoubleValue_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setDoubleValue_0<(/*void*/)> for (f64) {
  fn setDoubleValue_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog14setDoubleValueEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:135
// index:0
// Public Visibility=Default Availability=Available
// [8] double doubleValue() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn doubleValue_0<RetType, T: QInputDialog_doubleValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleValue_0(self);
    // return 1;
  }
}
pub trait QInputDialog_doubleValue_0<RetType> {
  fn doubleValue_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_doubleValue_0<f64> for () {
  fn doubleValue_0(self , rsthis: & QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog11doubleValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDoubleMinimum(double)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setDoubleMinimum_0<RetType, T: QInputDialog_setDoubleMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDoubleMinimum_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setDoubleMinimum_0<RetType> {
  fn setDoubleMinimum_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setDoubleMinimum_0<(/*void*/)> for (f64) {
  fn setDoubleMinimum_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog16setDoubleMinimumEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:138
// index:0
// Public Visibility=Default Availability=Available
// [8] double doubleMinimum() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn doubleMinimum_0<RetType, T: QInputDialog_doubleMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleMinimum_0(self);
    // return 1;
  }
}
pub trait QInputDialog_doubleMinimum_0<RetType> {
  fn doubleMinimum_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_doubleMinimum_0<f64> for () {
  fn doubleMinimum_0(self , rsthis: & QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog13doubleMinimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDoubleMaximum(double)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setDoubleMaximum_0<RetType, T: QInputDialog_setDoubleMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDoubleMaximum_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setDoubleMaximum_0<RetType> {
  fn setDoubleMaximum_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setDoubleMaximum_0<(/*void*/)> for (f64) {
  fn setDoubleMaximum_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog16setDoubleMaximumEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:141
// index:0
// Public Visibility=Default Availability=Available
// [8] double doubleMaximum() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn doubleMaximum_0<RetType, T: QInputDialog_doubleMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleMaximum_0(self);
    // return 1;
  }
}
pub trait QInputDialog_doubleMaximum_0<RetType> {
  fn doubleMaximum_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_doubleMaximum_0<f64> for () {
  fn doubleMaximum_0(self , rsthis: & QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog13doubleMaximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDoubleRange(double, double)

/*
Sets the range of double precision floating point values accepted by the dialog when used in DoubleInput mode, with minimum and maximum values specified by min and max respectively.
*/
impl /*struct*/ QInputDialog {
  pub fn setDoubleRange_0<RetType, T: QInputDialog_setDoubleRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDoubleRange_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setDoubleRange_0<RetType> {
  fn setDoubleRange_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setDoubleRange_0<(/*void*/)> for (f64,f64) {
  fn setDoubleRange_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog14setDoubleRangeEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDoubleDecimals(int)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setDoubleDecimals_0<RetType, T: QInputDialog_setDoubleDecimals_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDoubleDecimals_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setDoubleDecimals_0<RetType> {
  fn setDoubleDecimals_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setDoubleDecimals_0<(/*void*/)> for (i32) {
  fn setDoubleDecimals_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog17setDoubleDecimalsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:146
// index:0
// Public Visibility=Default Availability=Available
// [4] int doubleDecimals() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn doubleDecimals_0<RetType, T: QInputDialog_doubleDecimals_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleDecimals_0(self);
    // return 1;
  }
}
pub trait QInputDialog_doubleDecimals_0<RetType> {
  fn doubleDecimals_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_doubleDecimals_0<i32> for () {
  fn doubleDecimals_0(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog14doubleDecimalsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOkButtonText(const QString &)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setOkButtonText_0<RetType, T: QInputDialog_setOkButtonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOkButtonText_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setOkButtonText_0<RetType> {
  fn setOkButtonText_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setOkButtonText_0<(/*void*/)> for (usize) {
  fn setOkButtonText_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog15setOkButtonTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:149
// index:0
// Public Visibility=Default Availability=Available
// [8] QString okButtonText() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn okButtonText_0<RetType, T: QInputDialog_okButtonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.okButtonText_0(self);
    // return 1;
  }
}
pub trait QInputDialog_okButtonText_0<RetType> {
  fn okButtonText_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_okButtonText_0<usize> for () {
  fn okButtonText_0(self , rsthis: & QInputDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog12okButtonTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCancelButtonText(const QString &)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setCancelButtonText_0<RetType, T: QInputDialog_setCancelButtonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCancelButtonText_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setCancelButtonText_0<RetType> {
  fn setCancelButtonText_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setCancelButtonText_0<(/*void*/)> for (usize) {
  fn setCancelButtonText_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog19setCancelButtonTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:152
// index:0
// Public Visibility=Default Availability=Available
// [8] QString cancelButtonText() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn cancelButtonText_0<RetType, T: QInputDialog_cancelButtonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancelButtonText_0(self);
    // return 1;
  }
}
pub trait QInputDialog_cancelButtonText_0<RetType> {
  fn cancelButtonText_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_cancelButtonText_0<usize> for () {
  fn cancelButtonText_0(self , rsthis: & QInputDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog16cancelButtonTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void open(QObject *, const char *)

/*
This is an overloaded function.

This function connects one of its signals to the slot specified by receiver and member. The specific signal depends on the arguments that are specified in member. These are:


textValueSelected() if member has a QString for its first argument.
intValueSelected() if member has an int for its first argument.
doubleValueSelected() if member has a double for its first argument.
accepted() if member has NO arguments.


The signal will be disconnected from the slot when the dialog is closed.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QInputDialog {
  pub fn open_0<RetType, T: QInputDialog_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QInputDialog_open_0<RetType> {
  fn open_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_open_0<(/*void*/)> for (usize,usize) {
  fn open_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog4openEP7QObjectPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:157
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QInputDialog {
  pub fn minimumSizeHint_0<RetType, T: QInputDialog_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QInputDialog_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QInputDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:158
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QInputDialog {
  pub fn sizeHint_0<RetType, T: QInputDialog_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QInputDialog_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QInputDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:160
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().
*/
impl /*struct*/ QInputDialog {
  pub fn setVisible_0<RetType, T: QInputDialog_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:162
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString getText(QWidget *, const QString &, const QString &, QLineEdit::EchoMode, const QString &, bool *, Qt::WindowFlags, Qt::InputMethodHints)

/*
Static convenience function to get a string from the user.

title is the text which is displayed in the title bar of the dialog. label is the text which is shown to the user (it should say what should be entered). text is the default text which is placed in the line edit. mode is the echo mode the line edit will use. inputMethodHints is the input method hints that will be used in the edit widget if an input method is active.

If ok is nonnull *a ok will be set to true if the user pressed OK and to false if the user pressed Cancel. The dialog's parent is parent. The dialog will be modal and uses the specified widget flags.

If the dialog is accepted, this function returns the text in the dialog's line edit. If the dialog is rejected, a null QString is returned.

Use this static function like this:


      bool ok;
      QString text = QInputDialog::getText(this, tr("QInputDialog::getText()"),
                                           tr("User name:"), QLineEdit::Normal,
                                           QDir::home().dirName(), &ok);
      if (ok && !text.isEmpty())
          textLabel->setText(text);



See also getInt(), getDouble(), getItem(), and getMultiLineText().
*/
impl /*struct*/ QInputDialog {
  pub fn getText_0<RetType, T: QInputDialog_getText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getText_0();
    // return 1;
  }
}
pub trait QInputDialog_getText_0<RetType> {
  fn getText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_getText_0<usize> for (usize,usize,usize,i32,usize,usize,i32,i32) {
  fn getText_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const usize as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QInputDialog7getTextEP7QWidgetRK7QStringS4_N9QLineEdit8EchoModeES4_Pb6QFlagsIN2Qt10WindowTypeEES8_INS9_15InputMethodHintEE", 8,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:167
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString getMultiLineText(QWidget *, const QString &, const QString &, const QString &, bool *, Qt::WindowFlags, Qt::InputMethodHints)

/*
Static convenience function to get a multiline string from the user.

title is the text which is displayed in the title bar of the dialog. label is the text which is shown to the user (it should say what should be entered). text is the default text which is placed in the plain text edit. inputMethodHints is the input method hints that will be used in the edit widget if an input method is active.

If ok is nonnull *a ok will be set to true if the user pressed OK and to false if the user pressed Cancel. The dialog's parent is parent. The dialog will be modal and uses the specified widget flags.

If the dialog is accepted, this function returns the text in the dialog's plain text edit. If the dialog is rejected, a null QString is returned.

Use this static function like this:


      bool ok;
      QString text = QInputDialog::getMultiLineText(this, tr("QInputDialog::getMultiLineText()"),
                                                    tr("Address:"), "John Doe\nFreedom Street", &ok);
      if (ok && !text.isEmpty())
          multiLineTextLabel->setText(text);



This function was introduced in  Qt 5.2.

See also getInt(), getDouble(), getItem(), and getText().
*/
impl /*struct*/ QInputDialog {
  pub fn getMultiLineText_0<RetType, T: QInputDialog_getMultiLineText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getMultiLineText_0();
    // return 1;
  }
}
pub trait QInputDialog_getMultiLineText_0<RetType> {
  fn getMultiLineText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_getMultiLineText_0<usize> for (usize,usize,usize,usize,usize,i32,i32) {
  fn getMultiLineText_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QInputDialog16getMultiLineTextEP7QWidgetRK7QStringS4_S4_Pb6QFlagsIN2Qt10WindowTypeEES6_INS7_15InputMethodHintEE", 7,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:171
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString getItem(QWidget *, const QString &, const QString &, const QStringList &, int, bool, bool *, Qt::WindowFlags, Qt::InputMethodHints)

/*
Static convenience function to let the user select an item from a string list.

title is the text which is displayed in the title bar of the dialog. label is the text which is shown to the user (it should say what should be entered). items is the string list which is inserted into the combo box. current is the number of the item which should be the current item. inputMethodHints is the input method hints that will be used if the combo box is editable and an input method is active.

If editable is true the user can enter their own text; otherwise, the user may only select one of the existing items.

If ok is nonnull *a ok will be set to true if the user pressed OK and to false if the user pressed Cancel. The dialog's parent is parent. The dialog will be modal and uses the widget flags.

This function returns the text of the current item, or if editable is true, the current text of the combo box.

Use this static function like this:


      QStringList items;
      items << tr("Spring") << tr("Summer") << tr("Fall") << tr("Winter");

      bool ok;
      QString item = QInputDialog::getItem(this, tr("QInputDialog::getItem()"),
                                           tr("Season:"), items, 0, false, &ok);
      if (ok && !item.isEmpty())
          itemLabel->setText(item);



See also getText(), getInt(), getDouble(), and getMultiLineText().
*/
impl /*struct*/ QInputDialog {
  pub fn getItem_0<RetType, T: QInputDialog_getItem_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getItem_0();
    // return 1;
  }
}
pub trait QInputDialog_getItem_0<RetType> {
  fn getItem_0(self ) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_getItem_0<usize> for (usize,usize,usize,usize,i32,bool,usize,i32,i32) {
  fn getItem_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const bool as usize;
    let arg6 = (&self.6) as *const usize as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let arg8 = (&self.8) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QInputDialog7getItemEP7QWidgetRK7QStringS4_RK11QStringListibPb6QFlagsIN2Qt10WindowTypeEES9_INSA_15InputMethodHintEE", 9,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:176
// index:0
// Public static Visibility=Default Availability=Available
// [4] int getInt(QWidget *, const QString &, const QString &, int, int, int, int, bool *, Qt::WindowFlags)

/*
Static convenience function to get an integer input from the user.

title is the text which is displayed in the title bar of the dialog. label is the text which is shown to the user (it should say what should be entered). value is the default integer which the spinbox will be set to. min and max are the minimum and maximum values the user may choose. step is the amount by which the values change as the user presses the arrow buttons to increment or decrement the value.

If ok is nonnull *ok will be set to true if the user pressed OK and to false if the user pressed Cancel. The dialog's parent is parent. The dialog will be modal and uses the widget flags.

On success, this function returns the integer which has been entered by the user; on failure, it returns the initial value.

Use this static function like this:


      bool ok;
      int i = QInputDialog::getInt(this, tr("QInputDialog::getInteger()"),
                                   tr("Percentage:"), 25, 0, 100, 1, &ok);
      if (ok)
          integerLabel->setText(tr("%1%").arg(i));



This function was introduced in  Qt 4.5.

See also getText(), getDouble(), getItem(), and getMultiLineText().
*/
impl /*struct*/ QInputDialog {
  pub fn getInt_0<RetType, T: QInputDialog_getInt_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getInt_0();
    // return 1;
  }
}
pub trait QInputDialog_getInt_0<RetType> {
  fn getInt_0(self ) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_getInt_0<i32> for (usize,usize,usize,i32,i32,i32,i32,usize,i32) {
  fn getInt_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const usize as usize;
    let arg8 = (&self.8) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QInputDialog6getIntEP7QWidgetRK7QStringS4_iiiiPb6QFlagsIN2Qt10WindowTypeEE", 9,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:179
// index:0
// Public static Visibility=Default Availability=Available
// [8] double getDouble(QWidget *, const QString &, const QString &, double, double, double, int, bool *, Qt::WindowFlags)

/*
Static convenience function to get a floating point number from the user.

title is the text which is displayed in the title bar of the dialog. label is the text which is shown to the user (it should say what should be entered). value is the default floating point number that the line edit will be set to. min and max are the minimum and maximum values the user may choose. decimals is the maximum number of decimal places the number may have.

If ok is nonnull, *ok will be set to true if the user pressed OK and to false if the user pressed Cancel. The dialog's parent is parent. The dialog will be modal and uses the widget flags.

This function returns the floating point number which has been entered by the user.

Use this static function like this:


      bool ok;
      double d = QInputDialog::getDouble(this, tr("QInputDialog::getDouble()"),
                                         tr("Amount:"), 37.56, -10000, 10000, 2, &ok);
      if (ok)
          doubleLabel->setText(QString("$%1").arg(d));



See also getText(), getInt(), getItem(), and getMultiLineText().
*/
impl /*struct*/ QInputDialog {
  pub fn getDouble_0<RetType, T: QInputDialog_getDouble_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getDouble_0();
    // return 1;
  }
}
pub trait QInputDialog_getDouble_0<RetType> {
  fn getDouble_0(self ) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_getDouble_0<f64> for (usize,usize,usize,f64,f64,f64,i32,usize,i32) {
  fn getDouble_0(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const usize as usize;
    let arg8 = (&self.8) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QInputDialog9getDoubleEP7QWidgetRK7QStringS4_dddiPb6QFlagsIN2Qt10WindowTypeEE", 9,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:183
// index:1
// Public static Visibility=Default Availability=Available
// [8] double getDouble(QWidget *, const QString &, const QString &, double, double, double, int, bool *, Qt::WindowFlags, double)

/*
Static convenience function to get a floating point number from the user.

title is the text which is displayed in the title bar of the dialog. label is the text which is shown to the user (it should say what should be entered). value is the default floating point number that the line edit will be set to. min and max are the minimum and maximum values the user may choose. decimals is the maximum number of decimal places the number may have.

If ok is nonnull, *ok will be set to true if the user pressed OK and to false if the user pressed Cancel. The dialog's parent is parent. The dialog will be modal and uses the widget flags.

This function returns the floating point number which has been entered by the user.

Use this static function like this:


      bool ok;
      double d = QInputDialog::getDouble(this, tr("QInputDialog::getDouble()"),
                                         tr("Amount:"), 37.56, -10000, 10000, 2, &ok);
      if (ok)
          doubleLabel->setText(QString("$%1").arg(d));



See also getText(), getInt(), getItem(), and getMultiLineText().
*/
impl /*struct*/ QInputDialog {
  pub fn getDouble_1<RetType, T: QInputDialog_getDouble_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.getDouble_1();
    // return 1;
  }
}
pub trait QInputDialog_getDouble_1<RetType> {
  fn getDouble_1(self ) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_getDouble_1<f64> for (usize,usize,usize,f64,f64,f64,i32,usize,i32,f64) {
  fn getDouble_1(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const usize as usize;
    let arg8 = (&self.8) as *const i32 as usize;
    let arg9 = (&self.9) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QInputDialog9getDoubleEP7QWidgetRK7QStringS4_dddiPb6QFlagsIN2Qt10WindowTypeEEd", 10,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:196
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDoubleStep(double)

/*

*/
impl /*struct*/ QInputDialog {
  pub fn setDoubleStep_0<RetType, T: QInputDialog_setDoubleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDoubleStep_0(self);
    // return 1;
  }
}
pub trait QInputDialog_setDoubleStep_0<RetType> {
  fn setDoubleStep_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_setDoubleStep_0<(/*void*/)> for (f64) {
  fn setDoubleStep_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog13setDoubleStepEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:197
// index:0
// Public Visibility=Default Availability=Available
// [8] double doubleStep() const

/*

*/
impl /*struct*/ QInputDialog {
  pub fn doubleStep_0<RetType, T: QInputDialog_doubleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleStep_0(self);
    // return 1;
  }
}
pub trait QInputDialog_doubleStep_0<RetType> {
  fn doubleStep_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_doubleStep_0<f64> for () {
  fn doubleStep_0(self , rsthis: & QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputDialog10doubleStepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:201
// index:0
// Public Visibility=Default Availability=Available
// [-2] void textValueChanged(const QString &)

/*
This signal is emitted whenever the text string changes in the dialog. The current string is specified by text.

This signal is only relevant when the input dialog is used in TextInput mode.

Note: Notifier signal for property textValue.
*/
impl /*struct*/ QInputDialog {
  pub fn textValueChanged_0<RetType, T: QInputDialog_textValueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textValueChanged_0(self);
    // return 1;
  }
}
pub trait QInputDialog_textValueChanged_0<RetType> {
  fn textValueChanged_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_textValueChanged_0<(/*void*/)> for (usize) {
  fn textValueChanged_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog16textValueChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:202
// index:0
// Public Visibility=Default Availability=Available
// [-2] void textValueSelected(const QString &)

/*
This signal is emitted whenever the user selects a text string by accepting the dialog; for example, by clicking the OK button. The selected string is specified by text.

This signal is only relevant when the input dialog is used in TextInput mode.
*/
impl /*struct*/ QInputDialog {
  pub fn textValueSelected_0<RetType, T: QInputDialog_textValueSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textValueSelected_0(self);
    // return 1;
  }
}
pub trait QInputDialog_textValueSelected_0<RetType> {
  fn textValueSelected_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_textValueSelected_0<(/*void*/)> for (usize) {
  fn textValueSelected_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog17textValueSelectedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:203
// index:0
// Public Visibility=Default Availability=Available
// [-2] void intValueChanged(int)

/*
This signal is emitted whenever the integer value changes in the dialog. The current value is specified by value.

This signal is only relevant when the input dialog is used in IntInput mode.

Note: Notifier signal for property intValue.
*/
impl /*struct*/ QInputDialog {
  pub fn intValueChanged_0<RetType, T: QInputDialog_intValueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intValueChanged_0(self);
    // return 1;
  }
}
pub trait QInputDialog_intValueChanged_0<RetType> {
  fn intValueChanged_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_intValueChanged_0<(/*void*/)> for (i32) {
  fn intValueChanged_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog15intValueChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:204
// index:0
// Public Visibility=Default Availability=Available
// [-2] void intValueSelected(int)

/*
This signal is emitted whenever the user selects a integer value by accepting the dialog; for example, by clicking the OK button. The selected value is specified by value.

This signal is only relevant when the input dialog is used in IntInput mode.
*/
impl /*struct*/ QInputDialog {
  pub fn intValueSelected_0<RetType, T: QInputDialog_intValueSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intValueSelected_0(self);
    // return 1;
  }
}
pub trait QInputDialog_intValueSelected_0<RetType> {
  fn intValueSelected_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_intValueSelected_0<(/*void*/)> for (i32) {
  fn intValueSelected_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog16intValueSelectedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:205
// index:0
// Public Visibility=Default Availability=Available
// [-2] void doubleValueChanged(double)

/*
This signal is emitted whenever the double value changes in the dialog. The current value is specified by value.

This signal is only relevant when the input dialog is used in DoubleInput mode.

Note: Notifier signal for property doubleValue.
*/
impl /*struct*/ QInputDialog {
  pub fn doubleValueChanged_0<RetType, T: QInputDialog_doubleValueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleValueChanged_0(self);
    // return 1;
  }
}
pub trait QInputDialog_doubleValueChanged_0<RetType> {
  fn doubleValueChanged_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_doubleValueChanged_0<(/*void*/)> for (f64) {
  fn doubleValueChanged_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog18doubleValueChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:206
// index:0
// Public Visibility=Default Availability=Available
// [-2] void doubleValueSelected(double)

/*
This signal is emitted whenever the user selects a double value by accepting the dialog; for example, by clicking the OK button. The selected value is specified by value.

This signal is only relevant when the input dialog is used in DoubleInput mode.
*/
impl /*struct*/ QInputDialog {
  pub fn doubleValueSelected_0<RetType, T: QInputDialog_doubleValueSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleValueSelected_0(self);
    // return 1;
  }
}
pub trait QInputDialog_doubleValueSelected_0<RetType> {
  fn doubleValueSelected_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_doubleValueSelected_0<(/*void*/)> for (f64) {
  fn doubleValueSelected_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog19doubleValueSelectedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qinputdialog.h:209
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void done(int)

/*
Reimplemented from QDialog::done().

Closes the dialog and sets its result code to result. If this dialog is shown with exec(), done() causes the local event loop to finish, and exec() to return result.

See also QDialog::done().
*/
impl /*struct*/ QInputDialog {
  pub fn done_0<RetType, T: QInputDialog_done_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.done_0(self);
    // return 1;
  }
}
pub trait QInputDialog_done_0<RetType> {
  fn done_0(self , rsthis: & QInputDialog) -> RetType;
}
impl<'a> /*trait*/ QInputDialog_done_0<(/*void*/)> for (i32) {
  fn done_0(self , rsthis: & QInputDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputDialog4doneEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QInputDialog__InputDialogOption = i32;
// 
pub const QInputDialog__NoButtons :QInputDialog__InputDialogOption = 1;
// 
pub const QInputDialog__UseListViewForComboBoxItems :QInputDialog__InputDialogOption = 2;
// 
pub const QInputDialog__UsePlainTextEditForTextInput :QInputDialog__InputDialogOption = 4;
pub fn QInputDialog_InputDialogOptionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QInputDialog", val);
}
pub fn QInputDialog_InputDialogOptionItemName_s(val: i32) ->String {
  //var nilthis *QInputDialog
  //return nilthis.InputDialogOptionItemName(val);
  return QInputDialog_InputDialogOptionItemName(val);
}


/*
This enum describes the different modes of input that can be selected for the dialog.



This enum was introduced or modified in  Qt 4.5.

See also inputMode.

*/
pub type QInputDialog__InputMode = i32;
// Used to input text strings.
pub const QInputDialog__TextInput :QInputDialog__InputMode = 0;
// Used to input integers.
pub const QInputDialog__IntInput :QInputDialog__InputMode = 1;
// Used to input floating point numbers with double precision accuracy.
pub const QInputDialog__DoubleInput :QInputDialog__InputMode = 2;
pub fn QInputDialog_InputModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QInputDialog", val);
}
pub fn QInputDialog_InputModeItemName_s(val: i32) ->String {
  //var nilthis *QInputDialog
  //return nilthis.InputModeItemName(val);
  return QInputDialog_InputModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
