

// mod ::widgets::QDialog
// package qtwidgets
// /usr/include/qt/QtWidgets/qdialog.h
// #include <qdialog.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 18
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

// void keyPressEvent(QKeyEvent *)
// func (this *QDialog) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void closeEvent(QCloseEvent *)
// func (this *QDialog) InheritCloseEvent(f func(arg0 *qtgui.QCloseEvent/*777 QCloseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QDialog) InheritShowEvent(f func(arg0 *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QDialog) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QDialog) InheritContextMenuEvent(f func(arg0 *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QDialog) InheritEventFilter(f func(arg0 *qtcore.QObject/*777 QObject **/, arg1 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// void adjustPosition(QWidget *)
// func (this *QDialog) InheritAdjustPosition(f func(arg0 *QWidget/*777 QWidget **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "adjustPosition", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QDialog)=48
pub struct QDialog {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDialog_ITF interface {
//    QWidget_ITF
//    QDialog_PTR() *QDialog
//}
//func (ptr *QDialog) QDialog_PTR() *QDialog { return ptr }

impl /*struct*/ QDialog {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDialog {
    return QDialog{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDialog {
//  type Target = QDialogBASE;
//
//  fn deref(&self) -> &QDialogBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDialogBASE> for QDialog {
//  fn as_ref(& self) -> & QDialogBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdialog.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDialog {
  pub fn metaObject_0<RetType, T: QDialog_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDialog_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QDialog10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDialog(QWidget *, Qt::WindowFlags)

/*
Constructs a dialog with parent parent.

A dialog is always a top-level widget, but if it has a parent, its default location is centered on top of the parent. It will also share the parent's taskbar entry.

The widget flags f are passed on to the QWidget constructor. If, for example, you don't want a What's This button in the title bar of the dialog, pass Qt::WindowTitleHint | Qt::WindowSystemMenuHint in f.

See also QWidget::setWindowFlags().
*/
// QDialog(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QDialog {
  pub fn QDialog_0<T: QDialog_QDialog_0>(value: T) -> QDialog {
    let rsthis = value.QDialog_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDialog_QDialog_0 {
  fn QDialog_0(self) -> QDialog;
}
// QDialog(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDialog_QDialog_0 for (usize,i32) {
  fn QDialog_0(self) -> QDialog {
    // unsafe{_ZN7QDialogC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QDialogC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDialog()

/*

*/
pub fn DeleteQDialog(this :*mut QDialog) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QDialogD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdialog.h:68
// index:0
// Public Visibility=Default Availability=Available
// [4] int result() const

/*
In general returns the modal dialog's result code, Accepted or Rejected.

Note: When called on a QMessageBox instance, the returned value is a value of the QMessageBox::StandardButton enum.

Do not call this function if the dialog was constructed with the Qt::WA_DeleteOnClose attribute.

See also setResult().
*/
impl /*struct*/ QDialog {
  pub fn result_0<RetType, T: QDialog_result_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.result_0(self);
    // return 1;
  }
}
pub trait QDialog_result_0<RetType> {
  fn result_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_result_0<i32> for () {
  fn result_0(self , rsthis: & QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QDialog6resultEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().
*/
impl /*struct*/ QDialog {
  pub fn setVisible_0<RetType, T: QDialog_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QDialog_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QDialog {
  pub fn setOrientation_0<RetType, T: QDialog_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QDialog_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:73
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QDialog {
  pub fn orientation_0<RetType, T: QDialog_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QDialog_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QDialog11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExtension(QWidget *)

/*

*/
impl /*struct*/ QDialog {
  pub fn setExtension_0<RetType, T: QDialog_setExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExtension_0(self);
    // return 1;
  }
}
pub trait QDialog_setExtension_0<RetType> {
  fn setExtension_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_setExtension_0<(/*void*/)> for (usize) {
  fn setExtension_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog12setExtensionEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * extension() const

/*

*/
impl /*struct*/ QDialog {
  pub fn extension_0<RetType, T: QDialog_extension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.extension_0(self);
    // return 1;
  }
}
pub trait QDialog_extension_0<RetType> {
  fn extension_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_extension_0<usize> for () {
  fn extension_0(self , rsthis: & QDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QDialog9extensionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QDialog {
  pub fn sizeHint_0<RetType, T: QDialog_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QDialog_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QDialog8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QDialog {
  pub fn minimumSizeHint_0<RetType, T: QDialog_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QDialog_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QDialog15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizeGripEnabled(bool)

/*

*/
impl /*struct*/ QDialog {
  pub fn setSizeGripEnabled_0<RetType, T: QDialog_setSizeGripEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeGripEnabled_0(self);
    // return 1;
  }
}
pub trait QDialog_setSizeGripEnabled_0<RetType> {
  fn setSizeGripEnabled_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_setSizeGripEnabled_0<(/*void*/)> for (bool) {
  fn setSizeGripEnabled_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog18setSizeGripEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:82
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSizeGripEnabled() const

/*

*/
impl /*struct*/ QDialog {
  pub fn isSizeGripEnabled_0<RetType, T: QDialog_isSizeGripEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSizeGripEnabled_0(self);
    // return 1;
  }
}
pub trait QDialog_isSizeGripEnabled_0<RetType> {
  fn isSizeGripEnabled_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_isSizeGripEnabled_0<bool> for () {
  fn isSizeGripEnabled_0(self , rsthis: & QDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QDialog17isSizeGripEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModal(bool)

/*

*/
impl /*struct*/ QDialog {
  pub fn setModal_0<RetType, T: QDialog_setModal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModal_0(self);
    // return 1;
  }
}
pub trait QDialog_setModal_0<RetType> {
  fn setModal_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_setModal_0<(/*void*/)> for (bool) {
  fn setModal_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog8setModalEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResult(int)

/*
Sets the modal dialog's result code to i.

Note: We recommend that you use one of the values defined by QDialog::DialogCode.

See also result().
*/
impl /*struct*/ QDialog {
  pub fn setResult_0<RetType, T: QDialog_setResult_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResult_0(self);
    // return 1;
  }
}
pub trait QDialog_setResult_0<RetType> {
  fn setResult_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_setResult_0<(/*void*/)> for (i32) {
  fn setResult_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog9setResultEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void finished(int)

/*
This signal is emitted when the dialog's result code has been set, either by the user or by calling done(), accept(), or reject().

Note that this signal is not emitted when hiding the dialog with hide() or setVisible(false). This includes deleting the dialog while it is visible.

This function was introduced in  Qt 4.1.

See also accepted() and rejected().
*/
impl /*struct*/ QDialog {
  pub fn finished_0<RetType, T: QDialog_finished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finished_0(self);
    // return 1;
  }
}
pub trait QDialog_finished_0<RetType> {
  fn finished_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_finished_0<(/*void*/)> for (i32) {
  fn finished_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog8finishedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void accepted()

/*
This signal is emitted when the dialog has been accepted either by the user or by calling accept() or done() with the QDialog::Accepted argument.

Note that this signal is not emitted when hiding the dialog with hide() or setVisible(false). This includes deleting the dialog while it is visible.

This function was introduced in  Qt 4.1.

See also finished() and rejected().
*/
impl /*struct*/ QDialog {
  pub fn accepted_0<RetType, T: QDialog_accepted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accepted_0(self);
    // return 1;
  }
}
pub trait QDialog_accepted_0<RetType> {
  fn accepted_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_accepted_0<(/*void*/)> for () {
  fn accepted_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QDialog8acceptedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rejected()

/*
This signal is emitted when the dialog has been rejected either by the user or by calling reject() or done() with the QDialog::Rejected argument.

Note that this signal is not emitted when hiding the dialog with hide() or setVisible(false). This includes deleting the dialog while it is visible.

This function was introduced in  Qt 4.1.

See also finished() and accepted().
*/
impl /*struct*/ QDialog {
  pub fn rejected_0<RetType, T: QDialog_rejected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rejected_0(self);
    // return 1;
  }
}
pub trait QDialog_rejected_0<RetType> {
  fn rejected_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_rejected_0<(/*void*/)> for () {
  fn rejected_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QDialog8rejectedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:93
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void open()

/*
Shows the dialog as a window modal dialog, returning immediately.

This function was introduced in  Qt 4.5.

See also exec(), show(), result(), and setWindowModality().
*/
impl /*struct*/ QDialog {
  pub fn open_0<RetType, T: QDialog_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QDialog_open_0<RetType> {
  fn open_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_open_0<(/*void*/)> for () {
  fn open_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QDialog4openEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:94
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int exec()

/*
Shows the dialog as a modal dialog, blocking until the user closes it. The function returns a DialogCode result.

If the dialog is application modal, users cannot interact with any other window in the same application until they close the dialog. If the dialog is window modal, only interaction with the parent window is blocked while the dialog is open. By default, the dialog is application modal.

See also open(), show(), result(), and setWindowModality().
*/
impl /*struct*/ QDialog {
  pub fn exec_0<RetType, T: QDialog_exec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exec_0(self);
    // return 1;
  }
}
pub trait QDialog_exec_0<RetType> {
  fn exec_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_exec_0<i32> for () {
  fn exec_0(self , rsthis: & QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QDialog4execEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void done(int)

/*
Closes the dialog and sets its result code to r. If this dialog is shown with exec(), done() causes the local event loop to finish, and exec() to return r.

As with QWidget::close(), done() deletes the dialog if the Qt::WA_DeleteOnClose flag is set. If the dialog is the application's main widget, the application terminates. If the dialog is the last window closed, the QApplication::lastWindowClosed() signal is emitted.

See also accept(), reject(), QApplication::activeWindow(), and QCoreApplication::quit().
*/
impl /*struct*/ QDialog {
  pub fn done_0<RetType, T: QDialog_done_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.done_0(self);
    // return 1;
  }
}
pub trait QDialog_done_0<RetType> {
  fn done_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_done_0<(/*void*/)> for (i32) {
  fn done_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog4doneEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void accept()

/*
Hides the modal dialog and sets the result code to Accepted.

See also reject() and done().
*/
impl /*struct*/ QDialog {
  pub fn accept_0<RetType, T: QDialog_accept_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accept_0(self);
    // return 1;
  }
}
pub trait QDialog_accept_0<RetType> {
  fn accept_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_accept_0<(/*void*/)> for () {
  fn accept_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QDialog6acceptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:97
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void reject()

/*
Hides the modal dialog and sets the result code to Rejected.

See also accept() and done().
*/
impl /*struct*/ QDialog {
  pub fn reject_0<RetType, T: QDialog_reject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reject_0(self);
    // return 1;
  }
}
pub trait QDialog_reject_0<RetType> {
  fn reject_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_reject_0<(/*void*/)> for () {
  fn reject_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QDialog6rejectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showExtension(bool)

/*

*/
impl /*struct*/ QDialog {
  pub fn showExtension_0<RetType, T: QDialog_showExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showExtension_0(self);
    // return 1;
  }
}
pub trait QDialog_showExtension_0<RetType> {
  fn showExtension_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_showExtension_0<(/*void*/)> for (bool) {
  fn showExtension_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog13showExtensionEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:104
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QDialog {
  pub fn keyPressEvent_0<RetType, T: QDialog_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QDialog_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:105
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEvent(QCloseEvent *)

/*
Reimplemented from QWidget::closeEvent().
*/
impl /*struct*/ QDialog {
  pub fn closeEvent_0<RetType, T: QDialog_closeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEvent_0(self);
    // return 1;
  }
}
pub trait QDialog_closeEvent_0<RetType> {
  fn closeEvent_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_closeEvent_0<(/*void*/)> for (usize) {
  fn closeEvent_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog10closeEventEP11QCloseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:106
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QDialog {
  pub fn showEvent_0<RetType, T: QDialog_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QDialog_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:107
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QDialog {
  pub fn resizeEvent_0<RetType, T: QDialog_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QDialog_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:109
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().
*/
impl /*struct*/ QDialog {
  pub fn contextMenuEvent_0<RetType, T: QDialog_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QDialog_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:111
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QDialog {
  pub fn eventFilter_0<RetType, T: QDialog_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QDialog_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QDialog11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdialog.h:112
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void adjustPosition(QWidget *)

/*

*/
impl /*struct*/ QDialog {
  pub fn adjustPosition_0<RetType, T: QDialog_adjustPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjustPosition_0(self);
    // return 1;
  }
}
pub trait QDialog_adjustPosition_0<RetType> {
  fn adjustPosition_0(self , rsthis: & QDialog) -> RetType;
}
impl<'a> /*trait*/ QDialog_adjustPosition_0<(/*void*/)> for (usize) {
  fn adjustPosition_0(self , rsthis: & QDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QDialog14adjustPositionEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
The value returned by a modal dialog.

ConstantValue
QDialog::Accepted1
QDialog::Rejected0

*/
pub type QDialog__DialogCode = i32;
// 
pub const QDialog__Rejected :QDialog__DialogCode = 0;
// 
pub const QDialog__Accepted :QDialog__DialogCode = 1;
pub fn QDialog_DialogCodeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDialog", val);
}
pub fn QDialog_DialogCodeItemName_s(val: i32) ->String {
  //var nilthis *QDialog
  //return nilthis.DialogCodeItemName(val);
  return QDialog_DialogCodeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
