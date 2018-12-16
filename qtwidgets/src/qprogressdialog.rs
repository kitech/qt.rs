

// mod ::widgets::QProgressDialog
// package qtwidgets
// /usr/include/qt/QtWidgets/qprogressdialog.h
// #include <qprogressdialog.h>
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

// void resizeEvent(QResizeEvent *)
// func (this *QProgressDialog) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void closeEvent(QCloseEvent *)
// func (this *QProgressDialog) InheritCloseEvent(f func(event *qtgui.QCloseEvent/*777 QCloseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QProgressDialog) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QProgressDialog) InheritShowEvent(f func(event *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void forceShow()
// func (this *QProgressDialog) InheritForceShow(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "forceShow", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QProgressDialog)=48
pub struct QProgressDialog {
  qbase: QDialog,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QProgressDialog_ITF interface {
//    QDialog_ITF
//    QProgressDialog_PTR() *QProgressDialog
//}
//func (ptr *QProgressDialog) QProgressDialog_PTR() *QProgressDialog { return ptr }

impl /*struct*/ QProgressDialog {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QProgressDialog {
    return QProgressDialog{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QProgressDialog {
//  type Target = QProgressDialogBASE;
//
//  fn deref(&self) -> &QProgressDialogBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QProgressDialogBASE> for QProgressDialog {
//  fn as_ref(& self) -> & QProgressDialogBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qprogressdialog.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn metaObject_0<RetType, T: QProgressDialog_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QProgressDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QProgressDialog(QWidget *, Qt::WindowFlags)

/*
Constructs a progress dialog.

Default settings:


The label text is empty.
The cancel button text is (translated) "Cancel".
minimum is 0;
maximum is 100


The parent argument is dialog's parent widget. The widget flags, f, are passed to the QDialog::QDialog() constructor.

See also setLabelText(), setCancelButtonText(), setCancelButton(), setMinimum(), and setMaximum().
*/
// QProgressDialog(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QProgressDialog {
  pub fn QProgressDialog_0<T: QProgressDialog_QProgressDialog_0>(value: T) -> QProgressDialog {
    let rsthis = value.QProgressDialog_0();
    return rsthis;
    // return 1;
  }
}

pub trait QProgressDialog_QProgressDialog_0 {
  fn QProgressDialog_0(self) -> QProgressDialog;
}
// QProgressDialog(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QProgressDialog_QProgressDialog_0 for (usize,i32) {
  fn QProgressDialog_0(self) -> QProgressDialog {
    // unsafe{_ZN15QProgressDialogC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QProgressDialogC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QProgressDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:72
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QProgressDialog(const QString &, const QString &, int, int, QWidget *, Qt::WindowFlags)

/*
Constructs a progress dialog.

Default settings:


The label text is empty.
The cancel button text is (translated) "Cancel".
minimum is 0;
maximum is 100


The parent argument is dialog's parent widget. The widget flags, f, are passed to the QDialog::QDialog() constructor.

See also setLabelText(), setCancelButtonText(), setCancelButton(), setMinimum(), and setMaximum().
*/
// QProgressDialog(const QString &, const QString &, int, int, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QProgressDialog {
  pub fn QProgressDialog_1<T: QProgressDialog_QProgressDialog_1>(value: T) -> QProgressDialog {
    let rsthis = value.QProgressDialog_1();
    return rsthis;
    // return 1;
  }
}

pub trait QProgressDialog_QProgressDialog_1 {
  fn QProgressDialog_1(self) -> QProgressDialog;
}
// QProgressDialog(const QString &, const QString &, int, int, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QProgressDialog_QProgressDialog_1 for (usize,usize,i32,i32,usize,i32) {
  fn QProgressDialog_1(self) -> QProgressDialog {
    // unsafe{_ZN15QProgressDialogC2ERK7QStringS2_iiP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QProgressDialogC2ERK7QStringS2_iiP7QWidget6QFlagsIN2Qt10WindowTypeEE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QProgressDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QProgressDialog()

/*

*/
pub fn DeleteQProgressDialog(this :*mut QProgressDialog) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QProgressDialogD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qprogressdialog.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLabel(QLabel *)

/*
Sets the label to label. The progress dialog resizes to fit. The label becomes owned by the progress dialog and will be deleted when necessary, so do not pass the address of an object on the stack.

See also setLabelText().
*/
impl /*struct*/ QProgressDialog {
  pub fn setLabel_0<RetType, T: QProgressDialog_setLabel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLabel_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setLabel_0<RetType> {
  fn setLabel_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setLabel_0<(/*void*/)> for (usize) {
  fn setLabel_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog8setLabelEP6QLabel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCancelButton(QPushButton *)

/*
Sets the cancel button to the push button, cancelButton. The progress dialog takes ownership of this button which will be deleted when necessary, so do not pass the address of an object that is on the stack, i.e. use new() to create the button. If 0 is passed then no cancel button will be shown.

See also setCancelButtonText().
*/
impl /*struct*/ QProgressDialog {
  pub fn setCancelButton_0<RetType, T: QProgressDialog_setCancelButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCancelButton_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setCancelButton_0<RetType> {
  fn setCancelButton_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setCancelButton_0<(/*void*/)> for (usize) {
  fn setCancelButton_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog15setCancelButtonEP11QPushButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBar(QProgressBar *)

/*
Sets the progress bar widget to bar. The progress dialog resizes to fit. The progress dialog takes ownership of the progress bar which will be deleted when necessary, so do not use a progress bar allocated on the stack.
*/
impl /*struct*/ QProgressDialog {
  pub fn setBar_0<RetType, T: QProgressDialog_setBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBar_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setBar_0<RetType> {
  fn setBar_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setBar_0<(/*void*/)> for (usize) {
  fn setBar_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog6setBarEP12QProgressBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wasCanceled() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn wasCanceled_0<RetType, T: QProgressDialog_wasCanceled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wasCanceled_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_wasCanceled_0<RetType> {
  fn wasCanceled_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_wasCanceled_0<bool> for () {
  fn wasCanceled_0(self , rsthis: & QProgressDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog11wasCanceledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimum() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn minimum_0<RetType, T: QProgressDialog_minimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimum_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_minimum_0<RetType> {
  fn minimum_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_minimum_0<i32> for () {
  fn minimum_0(self , rsthis: & QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog7minimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximum() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn maximum_0<RetType, T: QProgressDialog_maximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximum_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_maximum_0<RetType> {
  fn maximum_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_maximum_0<i32> for () {
  fn maximum_0(self , rsthis: & QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog7maximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:86
// index:0
// Public Visibility=Default Availability=Available
// [4] int value() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn value_0<RetType, T: QProgressDialog_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_value_0<RetType> {
  fn value_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_value_0<i32> for () {
  fn value_0(self , rsthis: & QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().

Returns a size that fits the contents of the progress dialog. The progress dialog resizes itself as required, so you should not need to call this yourself.
*/
impl /*struct*/ QProgressDialog {
  pub fn sizeHint_0<RetType, T: QProgressDialog_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QProgressDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QString labelText() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn labelText_0<RetType, T: QProgressDialog_labelText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.labelText_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_labelText_0<RetType> {
  fn labelText_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_labelText_0<usize> for () {
  fn labelText_0(self , rsthis: & QProgressDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog9labelTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:91
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimumDuration() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn minimumDuration_0<RetType, T: QProgressDialog_minimumDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumDuration_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_minimumDuration_0<RetType> {
  fn minimumDuration_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_minimumDuration_0<i32> for () {
  fn minimumDuration_0(self , rsthis: & QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog15minimumDurationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoReset(bool)

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn setAutoReset_0<RetType, T: QProgressDialog_setAutoReset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoReset_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setAutoReset_0<RetType> {
  fn setAutoReset_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setAutoReset_0<(/*void*/)> for (bool) {
  fn setAutoReset_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog12setAutoResetEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoReset() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn autoReset_0<RetType, T: QProgressDialog_autoReset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoReset_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_autoReset_0<RetType> {
  fn autoReset_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_autoReset_0<bool> for () {
  fn autoReset_0(self , rsthis: & QProgressDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog9autoResetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoClose(bool)

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn setAutoClose_0<RetType, T: QProgressDialog_setAutoClose_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoClose_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setAutoClose_0<RetType> {
  fn setAutoClose_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setAutoClose_0<(/*void*/)> for (bool) {
  fn setAutoClose_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog12setAutoCloseEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoClose() const

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn autoClose_0<RetType, T: QProgressDialog_autoClose_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoClose_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_autoClose_0<RetType> {
  fn autoClose_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_autoClose_0<bool> for () {
  fn autoClose_0(self , rsthis: & QProgressDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QProgressDialog9autoCloseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void open(QObject *, const char *)

/*
This is an overloaded function.

Opens the dialog and connects its canceled() signal to the slot specified by receiver and member.

The signal will be disconnected from the slot when the dialog is closed.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QProgressDialog {
  pub fn open_0<RetType, T: QProgressDialog_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_open_0<RetType> {
  fn open_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_open_0<(/*void*/)> for (usize,usize) {
  fn open_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog4openEP7QObjectPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cancel()

/*
Resets the progress dialog. wasCanceled() becomes true until the progress dialog is reset. The progress dialog becomes hidden.
*/
impl /*struct*/ QProgressDialog {
  pub fn cancel_0<RetType, T: QProgressDialog_cancel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancel_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_cancel_0<RetType> {
  fn cancel_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_cancel_0<(/*void*/)> for () {
  fn cancel_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog6cancelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*
Resets the progress dialog. The progress dialog becomes hidden if autoClose() is true.

See also setAutoClose() and setAutoReset().
*/
impl /*struct*/ QProgressDialog {
  pub fn reset_0<RetType, T: QProgressDialog_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_reset_0<RetType> {
  fn reset_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximum(int)

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn setMaximum_0<RetType, T: QProgressDialog_setMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximum_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setMaximum_0<RetType> {
  fn setMaximum_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setMaximum_0<(/*void*/)> for (i32) {
  fn setMaximum_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog10setMaximumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimum(int)

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn setMinimum_0<RetType, T: QProgressDialog_setMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimum_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setMinimum_0<RetType> {
  fn setMinimum_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setMinimum_0<(/*void*/)> for (i32) {
  fn setMinimum_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog10setMinimumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRange(int, int)

/*
Sets the progress dialog's minimum and maximum values to minimum and maximum, respectively.

If maximum is smaller than minimum, minimum becomes the only legal value.

If the current value falls outside the new range, the progress dialog is reset with reset().

See also minimum and maximum.
*/
impl /*struct*/ QProgressDialog {
  pub fn setRange_0<RetType, T: QProgressDialog_setRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRange_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setRange_0<RetType> {
  fn setRange_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setRange_0<(/*void*/)> for (i32,i32) {
  fn setRange_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog8setRangeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValue(int)

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn setValue_0<RetType, T: QProgressDialog_setValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValue_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setValue_0<RetType> {
  fn setValue_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setValue_0<(/*void*/)> for (i32) {
  fn setValue_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog8setValueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLabelText(const QString &)

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn setLabelText_0<RetType, T: QProgressDialog_setLabelText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLabelText_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setLabelText_0<RetType> {
  fn setLabelText_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setLabelText_0<(/*void*/)> for (usize) {
  fn setLabelText_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog12setLabelTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCancelButtonText(const QString &)

/*
Sets the cancel button's text to cancelButtonText. If the text is set to QString() then it will cause the cancel button to be hidden and deleted.

See also setCancelButton().
*/
impl /*struct*/ QProgressDialog {
  pub fn setCancelButtonText_0<RetType, T: QProgressDialog_setCancelButtonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCancelButtonText_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setCancelButtonText_0<RetType> {
  fn setCancelButtonText_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setCancelButtonText_0<(/*void*/)> for (usize) {
  fn setCancelButtonText_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog19setCancelButtonTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumDuration(int)

/*

*/
impl /*struct*/ QProgressDialog {
  pub fn setMinimumDuration_0<RetType, T: QProgressDialog_setMinimumDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumDuration_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_setMinimumDuration_0<RetType> {
  fn setMinimumDuration_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_setMinimumDuration_0<(/*void*/)> for (i32) {
  fn setMinimumDuration_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog18setMinimumDurationEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void canceled()

/*
This signal is emitted when the cancel button is clicked. It is connected to the cancel() slot by default.

See also wasCanceled().
*/
impl /*struct*/ QProgressDialog {
  pub fn canceled_0<RetType, T: QProgressDialog_canceled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canceled_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_canceled_0<RetType> {
  fn canceled_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_canceled_0<(/*void*/)> for () {
  fn canceled_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog8canceledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:116
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QProgressDialog {
  pub fn resizeEvent_0<RetType, T: QProgressDialog_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEvent(QCloseEvent *)

/*
Reimplemented from QWidget::closeEvent().
*/
impl /*struct*/ QProgressDialog {
  pub fn closeEvent_0<RetType, T: QProgressDialog_closeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEvent_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_closeEvent_0<RetType> {
  fn closeEvent_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_closeEvent_0<(/*void*/)> for (usize) {
  fn closeEvent_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog10closeEventEP11QCloseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:118
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QProgressDialog {
  pub fn changeEvent_0<RetType, T: QProgressDialog_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:119
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QProgressDialog {
  pub fn showEvent_0<RetType, T: QProgressDialog_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressdialog.h:122
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void forceShow()

/*
Shows the dialog if it is still hidden after the algorithm has been started and minimumDuration milliseconds have passed.

See also setMinimumDuration().
*/
impl /*struct*/ QProgressDialog {
  pub fn forceShow_0<RetType, T: QProgressDialog_forceShow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.forceShow_0(self);
    // return 1;
  }
}
pub trait QProgressDialog_forceShow_0<RetType> {
  fn forceShow_0(self , rsthis: & QProgressDialog) -> RetType;
}
impl<'a> /*trait*/ QProgressDialog_forceShow_0<(/*void*/)> for () {
  fn forceShow_0(self , rsthis: & QProgressDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QProgressDialog9forceShowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
