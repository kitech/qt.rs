

// mod ::widgets::QColorDialog
// package qtwidgets
// /usr/include/qt/QtWidgets/qcolordialog.h
// #include <qcolordialog.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 31
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
// func (this *QColorDialog) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void done(int)
// func (this *QColorDialog) InheritDone(f func(result int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "done", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QColorDialog)=48
pub struct QColorDialog {
  qbase: QDialog,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QColorDialog_ITF interface {
//    QDialog_ITF
//    QColorDialog_PTR() *QColorDialog
//}
//func (ptr *QColorDialog) QColorDialog_PTR() *QColorDialog { return ptr }

impl /*struct*/ QColorDialog {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QColorDialog {
    return QColorDialog{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QColorDialog {
//  type Target = QColorDialogBASE;
//
//  fn deref(&self) -> &QColorDialogBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QColorDialogBASE> for QColorDialog {
//  fn as_ref(& self) -> & QColorDialogBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcolordialog.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QColorDialog {
  pub fn metaObject_0<RetType, T: QColorDialog_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QColorDialog_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QColorDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QColorDialog10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QColorDialog(QWidget *)

/*
Constructs a color dialog with the given parent.

This function was introduced in  Qt 4.5.
*/
// QColorDialog(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QColorDialog {
  pub fn QColorDialog_0<T: QColorDialog_QColorDialog_0>(value: T) -> QColorDialog {
    let rsthis = value.QColorDialog_0();
    return rsthis;
    // return 1;
  }
}

pub trait QColorDialog_QColorDialog_0 {
  fn QColorDialog_0(self) -> QColorDialog;
}
// QColorDialog(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColorDialog_QColorDialog_0 for (usize) {
  fn QColorDialog_0(self) -> QColorDialog {
    // unsafe{_ZN12QColorDialogC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QColorDialogC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColorDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:72
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QColorDialog(const QColor &, QWidget *)

/*
Constructs a color dialog with the given parent.

This function was introduced in  Qt 4.5.
*/
// QColorDialog(const QColor &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QColorDialog {
  pub fn QColorDialog_1<T: QColorDialog_QColorDialog_1>(value: T) -> QColorDialog {
    let rsthis = value.QColorDialog_1();
    return rsthis;
    // return 1;
  }
}

pub trait QColorDialog_QColorDialog_1 {
  fn QColorDialog_1(self) -> QColorDialog;
}
// QColorDialog(const QColor &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColorDialog_QColorDialog_1 for (usize,usize) {
  fn QColorDialog_1(self) -> QColorDialog {
    // unsafe{_ZN12QColorDialogC2ERK6QColorP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QColorDialogC2ERK6QColorP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColorDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QColorDialog()

/*

*/
pub fn DeleteQColorDialog(this :*mut QColorDialog) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QColorDialogD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcolordialog.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentColor(const QColor &)

/*

*/
impl /*struct*/ QColorDialog {
  pub fn setCurrentColor_0<RetType, T: QColorDialog_setCurrentColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentColor_0(self);
    // return 1;
  }
}
pub trait QColorDialog_setCurrentColor_0<RetType> {
  fn setCurrentColor_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_setCurrentColor_0<(/*void*/)> for (usize) {
  fn setCurrentColor_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog15setCurrentColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:76
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor currentColor() const

/*

*/
impl /*struct*/ QColorDialog {
  pub fn currentColor_0<RetType, T: QColorDialog_currentColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentColor_0(self);
    // return 1;
  }
}
pub trait QColorDialog_currentColor_0<RetType> {
  fn currentColor_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_currentColor_0<usize> for () {
  fn currentColor_0(self , rsthis: & QColorDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QColorDialog12currentColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:78
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor selectedColor() const

/*
Returns the color that the user selected by clicking the OK or equivalent button.

Note: This color is not always the same as the color held by the currentColor property since the user can choose different colors before finally selecting the one to use.
*/
impl /*struct*/ QColorDialog {
  pub fn selectedColor_0<RetType, T: QColorDialog_selectedColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedColor_0(self);
    // return 1;
  }
}
pub trait QColorDialog_selectedColor_0<RetType> {
  fn selectedColor_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_selectedColor_0<usize> for () {
  fn selectedColor_0(self , rsthis: & QColorDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QColorDialog13selectedColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOption(QColorDialog::ColorDialogOption, bool)

/*
Sets the given option to be enabled if on is true; otherwise, clears the given option.

See also options and testOption().
*/
impl /*struct*/ QColorDialog {
  pub fn setOption_0<RetType, T: QColorDialog_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QColorDialog_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_setOption_0<(/*void*/)> for (i32,bool) {
  fn setOption_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog9setOptionENS_17ColorDialogOptionEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testOption(QColorDialog::ColorDialogOption) const

/*
Returns true if the given option is enabled; otherwise, returns false.

This function was introduced in  Qt 4.5.

See also options and setOption().
*/
impl /*struct*/ QColorDialog {
  pub fn testOption_0<RetType, T: QColorDialog_testOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_0(self);
    // return 1;
  }
}
pub trait QColorDialog_testOption_0<RetType> {
  fn testOption_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_testOption_0<bool> for (i32) {
  fn testOption_0(self , rsthis: & QColorDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QColorDialog10testOptionENS_17ColorDialogOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptions(QColorDialog::ColorDialogOptions)

/*

*/
impl /*struct*/ QColorDialog {
  pub fn setOptions_0<RetType, T: QColorDialog_setOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptions_0(self);
    // return 1;
  }
}
pub trait QColorDialog_setOptions_0<RetType> {
  fn setOptions_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_setOptions_0<(/*void*/)> for (i32) {
  fn setOptions_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog10setOptionsE6QFlagsINS_17ColorDialogOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] QColorDialog::ColorDialogOptions options() const

/*

*/
impl /*struct*/ QColorDialog {
  pub fn options_0<RetType, T: QColorDialog_options_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.options_0(self);
    // return 1;
  }
}
pub trait QColorDialog_options_0<RetType> {
  fn options_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_options_0<i32> for () {
  fn options_0(self , rsthis: & QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QColorDialog7optionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void open(QObject *, const char *)

/*
This is an overloaded function.

Opens the dialog and connects its colorSelected() signal to the slot specified by receiver and member.

The signal will be disconnected from the slot when the dialog is closed.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QColorDialog {
  pub fn open_0<RetType, T: QColorDialog_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QColorDialog_open_0<RetType> {
  fn open_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_open_0<(/*void*/)> for (usize,usize) {
  fn open_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog4openEP7QObjectPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().

Changes the visibility of the dialog. If visible is true, the dialog is shown; otherwise, it is hidden.
*/
impl /*struct*/ QColorDialog {
  pub fn setVisible_0<RetType, T: QColorDialog_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QColorDialog_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:90
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor getColor(const QColor &, QWidget *, const QString &, QColorDialog::ColorDialogOptions)

/*
Pops up a modal color dialog with the given window title (or "Select Color" if none is specified), lets the user choose a color, and returns that color. The color is initially set to initial. The dialog is a child of parent. It returns an invalid (see QColor::isValid()) color if the user cancels the dialog.

The options argument allows you to customize the dialog.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QColorDialog {
  pub fn getColor_0<RetType, T: QColorDialog_getColor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getColor_0();
    // return 1;
  }
}
pub trait QColorDialog_getColor_0<RetType> {
  fn getColor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_getColor_0<usize> for (usize,usize,usize,i32) {
  fn getColor_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QColorDialog8getColorERK6QColorP7QWidgetRK7QString6QFlagsINS_17ColorDialogOptionEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:96
// index:0
// Public static Visibility=Default Availability=Available
// [4] QRgb getRgba(QRgb, bool *, QWidget *)

/*

*/
impl /*struct*/ QColorDialog {
  pub fn getRgba_0<RetType, T: QColorDialog_getRgba_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getRgba_0();
    // return 1;
  }
}
pub trait QColorDialog_getRgba_0<RetType> {
  fn getRgba_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_getRgba_0<u32> for (u32,usize,usize) {
  fn getRgba_0(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const u32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QColorDialog7getRgbaEjPbP7QWidget", 3,qtrt::FFITY_UINT32,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:98
// index:0
// Public static Visibility=Default Availability=Available
// [4] int customCount()

/*
Returns the number of custom colors supported by QColorDialog. All color dialogs share the same custom colors.
*/
impl /*struct*/ QColorDialog {
  pub fn customCount_0<RetType, T: QColorDialog_customCount_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.customCount_0();
    // return 1;
  }
}
pub trait QColorDialog_customCount_0<RetType> {
  fn customCount_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_customCount_0<i32> for () {
  fn customCount_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QColorDialog11customCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:99
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor customColor(int)

/*
Returns the custom color at the given index as a QColor value.

This function was introduced in  Qt 4.5.

See also setCustomColor().
*/
impl /*struct*/ QColorDialog {
  pub fn customColor_0<RetType, T: QColorDialog_customColor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.customColor_0();
    // return 1;
  }
}
pub trait QColorDialog_customColor_0<RetType> {
  fn customColor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_customColor_0<usize> for (i32) {
  fn customColor_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QColorDialog11customColorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:100
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setCustomColor(int, QColor)

/*
Sets the custom color at index to the QColor color value.

Note: This function does not apply to the Native Color Dialog on the macOS platform. If you still require this function, use the QColorDialog::DontUseNativeDialog option.

See also customColor().
*/
impl /*struct*/ QColorDialog {
  pub fn setCustomColor_0<RetType, T: QColorDialog_setCustomColor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCustomColor_0();
    // return 1;
  }
}
pub trait QColorDialog_setCustomColor_0<RetType> {
  fn setCustomColor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_setCustomColor_0<(/*void*/)> for (i32,usize) {
  fn setCustomColor_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog14setCustomColorEi6QColor", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:101
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor standardColor(int)

/*
Returns the standard color at the given index as a QColor value.

This function was introduced in  Qt 5.0.

See also setStandardColor().
*/
impl /*struct*/ QColorDialog {
  pub fn standardColor_0<RetType, T: QColorDialog_standardColor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.standardColor_0();
    // return 1;
  }
}
pub trait QColorDialog_standardColor_0<RetType> {
  fn standardColor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_standardColor_0<usize> for (i32) {
  fn standardColor_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QColorDialog13standardColorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:102
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setStandardColor(int, QColor)

/*
Sets the standard color at index to the QColor color value.

Note: This function does not apply to the Native Color Dialog on the macOS platform. If you still require this function, use the QColorDialog::DontUseNativeDialog option.

See also standardColor().
*/
impl /*struct*/ QColorDialog {
  pub fn setStandardColor_0<RetType, T: QColorDialog_setStandardColor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStandardColor_0();
    // return 1;
  }
}
pub trait QColorDialog_setStandardColor_0<RetType> {
  fn setStandardColor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_setStandardColor_0<(/*void*/)> for (i32,usize) {
  fn setStandardColor_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog16setStandardColorEi6QColor", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentColorChanged(const QColor &)

/*
This signal is emitted whenever the current color changes in the dialog. The current color is specified by color.

Note: Notifier signal for property currentColor. 

See also color and colorSelected().
*/
impl /*struct*/ QColorDialog {
  pub fn currentColorChanged_0<RetType, T: QColorDialog_currentColorChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentColorChanged_0(self);
    // return 1;
  }
}
pub trait QColorDialog_currentColorChanged_0<RetType> {
  fn currentColorChanged_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_currentColorChanged_0<(/*void*/)> for (usize) {
  fn currentColorChanged_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog19currentColorChangedERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void colorSelected(const QColor &)

/*
This signal is emitted just after the user has clicked OK to select a color to use. The chosen color is specified by color.

See also color and currentColorChanged().
*/
impl /*struct*/ QColorDialog {
  pub fn colorSelected_0<RetType, T: QColorDialog_colorSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorSelected_0(self);
    // return 1;
  }
}
pub trait QColorDialog_colorSelected_0<RetType> {
  fn colorSelected_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_colorSelected_0<(/*void*/)> for (usize) {
  fn colorSelected_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog13colorSelectedERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:109
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QColorDialog {
  pub fn changeEvent_0<RetType, T: QColorDialog_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QColorDialog_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolordialog.h:110
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void done(int)

/*
Reimplemented from QDialog::done().

Closes the dialog and sets its result code to result. If this dialog is shown with exec(), done() causes the local event loop to finish, and exec() to return result.

See also QDialog::done().
*/
impl /*struct*/ QColorDialog {
  pub fn done_0<RetType, T: QColorDialog_done_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.done_0(self);
    // return 1;
  }
}
pub trait QColorDialog_done_0<RetType> {
  fn done_0(self , rsthis: & QColorDialog) -> RetType;
}
impl<'a> /*trait*/ QColorDialog_done_0<(/*void*/)> for (i32) {
  fn done_0(self , rsthis: & QColorDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QColorDialog4doneEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QColorDialog__ColorDialogOption = i32;
// 
pub const QColorDialog__ShowAlphaChannel :QColorDialog__ColorDialogOption = 1;
// 
pub const QColorDialog__NoButtons :QColorDialog__ColorDialogOption = 2;
// 
pub const QColorDialog__DontUseNativeDialog :QColorDialog__ColorDialogOption = 4;
pub fn QColorDialog_ColorDialogOptionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QColorDialog", val);
}
pub fn QColorDialog_ColorDialogOptionItemName_s(val: i32) ->String {
  //var nilthis *QColorDialog
  //return nilthis.ColorDialogOptionItemName(val);
  return QColorDialog_ColorDialogOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
