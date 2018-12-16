

// mod ::widgets::QFontDialog
// package qtwidgets
// /usr/include/qt/QtWidgets/qfontdialog.h
// #include <qfontdialog.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 12
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
// func (this *QFontDialog) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void done(int)
// func (this *QFontDialog) InheritDone(f func(result int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "done", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QFontDialog) InheritEventFilter(f func(object *qtcore.QObject/*777 QObject **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFontDialog)=48
pub struct QFontDialog {
  qbase: QDialog,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFontDialog_ITF interface {
//    QDialog_ITF
//    QFontDialog_PTR() *QFontDialog
//}
//func (ptr *QFontDialog) QFontDialog_PTR() *QFontDialog { return ptr }

impl /*struct*/ QFontDialog {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFontDialog {
    return QFontDialog{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFontDialog {
//  type Target = QFontDialogBASE;
//
//  fn deref(&self) -> &QFontDialogBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFontDialogBASE> for QFontDialog {
//  fn as_ref(& self) -> & QFontDialogBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qfontdialog.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFontDialog {
  pub fn metaObject_0<RetType, T: QFontDialog_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFontDialog_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFontDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFontDialog10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFontDialog(QWidget *)

/*
Constructs a standard font dialog.

Use setCurrentFont() to set the initial font attributes.

The parent parameter is passed to the QDialog constructor.

This function was introduced in  Qt 4.5.

See also getFont().
*/
// QFontDialog(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QFontDialog {
  pub fn QFontDialog_0<T: QFontDialog_QFontDialog_0>(value: T) -> QFontDialog {
    let rsthis = value.QFontDialog_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFontDialog_QFontDialog_0 {
  fn QFontDialog_0(self) -> QFontDialog;
}
// QFontDialog(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontDialog_QFontDialog_0 for (usize) {
  fn QFontDialog_0(self) -> QFontDialog {
    // unsafe{_ZN11QFontDialogC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFontDialogC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:76
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFontDialog(const QFont &, QWidget *)

/*
Constructs a standard font dialog.

Use setCurrentFont() to set the initial font attributes.

The parent parameter is passed to the QDialog constructor.

This function was introduced in  Qt 4.5.

See also getFont().
*/
// QFontDialog(const QFont &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QFontDialog {
  pub fn QFontDialog_1<T: QFontDialog_QFontDialog_1>(value: T) -> QFontDialog {
    let rsthis = value.QFontDialog_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFontDialog_QFontDialog_1 {
  fn QFontDialog_1(self) -> QFontDialog;
}
// QFontDialog(const QFont &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontDialog_QFontDialog_1 for (usize,usize) {
  fn QFontDialog_1(self) -> QFontDialog {
    // unsafe{_ZN11QFontDialogC2ERK5QFontP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFontDialogC2ERK5QFontP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFontDialog()

/*

*/
pub fn DeleteQFontDialog(this :*mut QFontDialog) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QFontDialogD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qfontdialog.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentFont(const QFont &)

/*
Sets the font highlighted in the QFontDialog to the given font.

This function was introduced in  Qt 4.5.

Note: Setter function for property currentFont. 

See also currentFont() and selectedFont().
*/
impl /*struct*/ QFontDialog {
  pub fn setCurrentFont_0<RetType, T: QFontDialog_setCurrentFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentFont_0(self);
    // return 1;
  }
}
pub trait QFontDialog_setCurrentFont_0<RetType> {
  fn setCurrentFont_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_setCurrentFont_0<(/*void*/)> for (usize) {
  fn setCurrentFont_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog14setCurrentFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:80
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont currentFont() const

/*
Returns the current font.

This function was introduced in  Qt 4.5.

Note: Getter function for property currentFont. 

See also setCurrentFont() and selectedFont().
*/
impl /*struct*/ QFontDialog {
  pub fn currentFont_0<RetType, T: QFontDialog_currentFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentFont_0(self);
    // return 1;
  }
}
pub trait QFontDialog_currentFont_0<RetType> {
  fn currentFont_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_currentFont_0<usize> for () {
  fn currentFont_0(self , rsthis: & QFontDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFontDialog11currentFontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:82
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont selectedFont() const

/*
Returns the font that the user selected by clicking the OK or equivalent button.

Note: This font is not always the same as the font held by the currentFont property since the user can choose different fonts before finally selecting the one to use.
*/
impl /*struct*/ QFontDialog {
  pub fn selectedFont_0<RetType, T: QFontDialog_selectedFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedFont_0(self);
    // return 1;
  }
}
pub trait QFontDialog_selectedFont_0<RetType> {
  fn selectedFont_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_selectedFont_0<usize> for () {
  fn selectedFont_0(self , rsthis: & QFontDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFontDialog12selectedFontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOption(QFontDialog::FontDialogOption, bool)

/*
Sets the given option to be enabled if on is true; otherwise, clears the given option.

See also options and testOption().
*/
impl /*struct*/ QFontDialog {
  pub fn setOption_0<RetType, T: QFontDialog_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QFontDialog_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_setOption_0<(/*void*/)> for (i32,bool) {
  fn setOption_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog9setOptionENS_16FontDialogOptionEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:85
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testOption(QFontDialog::FontDialogOption) const

/*
Returns true if the given option is enabled; otherwise, returns false.

See also options and setOption().
*/
impl /*struct*/ QFontDialog {
  pub fn testOption_0<RetType, T: QFontDialog_testOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_0(self);
    // return 1;
  }
}
pub trait QFontDialog_testOption_0<RetType> {
  fn testOption_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_testOption_0<bool> for (i32) {
  fn testOption_0(self , rsthis: & QFontDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFontDialog10testOptionENS_16FontDialogOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptions(QFontDialog::FontDialogOptions)

/*

*/
impl /*struct*/ QFontDialog {
  pub fn setOptions_0<RetType, T: QFontDialog_setOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptions_0(self);
    // return 1;
  }
}
pub trait QFontDialog_setOptions_0<RetType> {
  fn setOptions_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_setOptions_0<(/*void*/)> for (i32) {
  fn setOptions_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog10setOptionsE6QFlagsINS_16FontDialogOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] QFontDialog::FontDialogOptions options() const

/*

*/
impl /*struct*/ QFontDialog {
  pub fn options_0<RetType, T: QFontDialog_options_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.options_0(self);
    // return 1;
  }
}
pub trait QFontDialog_options_0<RetType> {
  fn options_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_options_0<i32> for () {
  fn options_0(self , rsthis: & QFontDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFontDialog7optionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void open(QObject *, const char *)

/*
This is an overloaded function.

Opens the dialog and connects its fontSelected() signal to the slot specified by receiver and member.

The signal will be disconnected from the slot when the dialog is closed.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QFontDialog {
  pub fn open_0<RetType, T: QFontDialog_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QFontDialog_open_0<RetType> {
  fn open_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_open_0<(/*void*/)> for (usize,usize) {
  fn open_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog4openEP7QObjectPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:92
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().
*/
impl /*struct*/ QFontDialog {
  pub fn setVisible_0<RetType, T: QFontDialog_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QFontDialog_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:94
// index:0
// Public static Visibility=Default Availability=Available
// [16] QFont getFont(bool *, QWidget *)

/*
Executes a modal font dialog and returns a font.

If the user clicks OK, the selected font is returned. If the user clicks Cancel, the initial font is returned.

The dialog is constructed with the given parent and the options specified in options. title is shown as the window title of the dialog and initial is the initially selected font. If the ok parameter is not-null, the value it refers to is set to true if the user clicks OK, and set to false if the user clicks Cancel.

Examples:


  bool ok;
  QFont font = QFontDialog::getFont(&ok, QFont("Times", 12), this);
  if (ok) {
      // font is set to the font the user selected
  } else {
      // the user canceled the dialog; font is set to the initial
      // value, in this case Times, 12.
  }



The dialog can also be used to set a widget's font directly:


  myWidget.setFont(QFontDialog::getFont(0, myWidget.font()));



In this example, if the user clicks OK the font they chose will be used, and if they click Cancel the original font is used.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QFontDialog constructors.
*/
impl /*struct*/ QFontDialog {
  pub fn getFont_0<RetType, T: QFontDialog_getFont_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getFont_0();
    // return 1;
  }
}
pub trait QFontDialog_getFont_0<RetType> {
  fn getFont_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_getFont_0<usize> for (usize,usize) {
  fn getFont_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFontDialog7getFontEPbP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:95
// index:1
// Public static Visibility=Default Availability=Available
// [16] QFont getFont(bool *, const QFont &, QWidget *, const QString &, QFontDialog::FontDialogOptions)

/*
Executes a modal font dialog and returns a font.

If the user clicks OK, the selected font is returned. If the user clicks Cancel, the initial font is returned.

The dialog is constructed with the given parent and the options specified in options. title is shown as the window title of the dialog and initial is the initially selected font. If the ok parameter is not-null, the value it refers to is set to true if the user clicks OK, and set to false if the user clicks Cancel.

Examples:


  bool ok;
  QFont font = QFontDialog::getFont(&ok, QFont("Times", 12), this);
  if (ok) {
      // font is set to the font the user selected
  } else {
      // the user canceled the dialog; font is set to the initial
      // value, in this case Times, 12.
  }



The dialog can also be used to set a widget's font directly:


  myWidget.setFont(QFontDialog::getFont(0, myWidget.font()));



In this example, if the user clicks OK the font they chose will be used, and if they click Cancel the original font is used.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QFontDialog constructors.
*/
impl /*struct*/ QFontDialog {
  pub fn getFont_1<RetType, T: QFontDialog_getFont_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.getFont_1();
    // return 1;
  }
}
pub trait QFontDialog_getFont_1<RetType> {
  fn getFont_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_getFont_1<usize> for (usize,usize,usize,usize,i32) {
  fn getFont_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFontDialog7getFontEPbRK5QFontP7QWidgetRK7QString6QFlagsINS_16FontDialogOptionEE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentFontChanged(const QFont &)

/*
This signal is emitted when the current font is changed. The new font is specified in font.

The signal is emitted while a user is selecting a font. Ultimately, the chosen font may differ from the font currently selected.

This function was introduced in  Qt 4.5.

Note: Notifier signal for property currentFont. 

See also currentFont, fontSelected(), and selectedFont().
*/
impl /*struct*/ QFontDialog {
  pub fn currentFontChanged_0<RetType, T: QFontDialog_currentFontChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentFontChanged_0(self);
    // return 1;
  }
}
pub trait QFontDialog_currentFontChanged_0<RetType> {
  fn currentFontChanged_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_currentFontChanged_0<(/*void*/)> for (usize) {
  fn currentFontChanged_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog18currentFontChangedERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fontSelected(const QFont &)

/*
This signal is emitted when a font has been selected. The selected font is specified in font.

The signal is only emitted when a user has chosen the final font to be used. It is not emitted while the user is changing the current font in the font dialog.

This function was introduced in  Qt 4.5.

See also selectedFont(), currentFontChanged(), and currentFont.
*/
impl /*struct*/ QFontDialog {
  pub fn fontSelected_0<RetType, T: QFontDialog_fontSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontSelected_0(self);
    // return 1;
  }
}
pub trait QFontDialog_fontSelected_0<RetType> {
  fn fontSelected_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_fontSelected_0<(/*void*/)> for (usize) {
  fn fontSelected_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog12fontSelectedERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:103
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QFontDialog {
  pub fn changeEvent_0<RetType, T: QFontDialog_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QFontDialog_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:104
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void done(int)

/*
Reimplemented from QDialog::done().

Closes the dialog and sets its result code to result. If this dialog is shown with exec(), done() causes the local event loop to finish, and exec() to return result.

See also QDialog::done().
*/
impl /*struct*/ QFontDialog {
  pub fn done_0<RetType, T: QFontDialog_done_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.done_0(self);
    // return 1;
  }
}
pub trait QFontDialog_done_0<RetType> {
  fn done_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_done_0<(/*void*/)> for (i32) {
  fn done_0(self , rsthis: & QFontDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFontDialog4doneEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontdialog.h:105
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*

*/
impl /*struct*/ QFontDialog {
  pub fn eventFilter_0<RetType, T: QFontDialog_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QFontDialog_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QFontDialog) -> RetType;
}
impl<'a> /*trait*/ QFontDialog_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QFontDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFontDialog11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QFontDialog__FontDialogOption = i32;
// 
pub const QFontDialog__NoButtons :QFontDialog__FontDialogOption = 1;
// 
pub const QFontDialog__DontUseNativeDialog :QFontDialog__FontDialogOption = 2;
// 
pub const QFontDialog__ScalableFonts :QFontDialog__FontDialogOption = 4;
// 
pub const QFontDialog__NonScalableFonts :QFontDialog__FontDialogOption = 8;
// 
pub const QFontDialog__MonospacedFonts :QFontDialog__FontDialogOption = 16;
// 
pub const QFontDialog__ProportionalFonts :QFontDialog__FontDialogOption = 32;
pub fn QFontDialog_FontDialogOptionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFontDialog", val);
}
pub fn QFontDialog_FontDialogOptionItemName_s(val: i32) ->String {
  //var nilthis *QFontDialog
  //return nilthis.FontDialogOptionItemName(val);
  return QFontDialog_FontDialogOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
