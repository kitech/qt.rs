

// mod ::widgets::QGroupBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qgroupbox.h
// #include <qgroupbox.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 111
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
// func (this *QGroupBox) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void childEvent(QChildEvent *)
// func (this *QGroupBox) InheritChildEvent(f func(event *qtcore.QChildEvent/*777 QChildEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "childEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QGroupBox) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QGroupBox) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QGroupBox) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QGroupBox) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QGroupBox) InheritMousePressEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QGroupBox) InheritMouseMoveEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QGroupBox) InheritMouseReleaseEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void initStyleOption(QStyleOptionGroupBox *)
// func (this *QGroupBox) InheritInitStyleOption(f func(option *QStyleOptionGroupBox/*777 QStyleOptionGroupBox **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGroupBox)=48
pub struct QGroupBox {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGroupBox_ITF interface {
//    QWidget_ITF
//    QGroupBox_PTR() *QGroupBox
//}
//func (ptr *QGroupBox) QGroupBox_PTR() *QGroupBox { return ptr }

impl /*struct*/ QGroupBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGroupBox {
    return QGroupBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGroupBox {
//  type Target = QGroupBoxBASE;
//
//  fn deref(&self) -> &QGroupBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGroupBoxBASE> for QGroupBox {
//  fn as_ref(& self) -> & QGroupBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgroupbox.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGroupBox {
  pub fn metaObject_0<RetType, T: QGroupBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGroupBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGroupBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGroupBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGroupBox(QWidget *)

/*
Constructs a group box widget with the given parent but with no title.
*/
// QGroupBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QGroupBox {
  pub fn QGroupBox_0<T: QGroupBox_QGroupBox_0>(value: T) -> QGroupBox {
    let rsthis = value.QGroupBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGroupBox_QGroupBox_0 {
  fn QGroupBox_0(self) -> QGroupBox;
}
// QGroupBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGroupBox_QGroupBox_0 for (usize) {
  fn QGroupBox_0(self) -> QGroupBox {
    // unsafe{_ZN9QGroupBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QGroupBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGroupBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:63
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QGroupBox(const QString &, QWidget *)

/*
Constructs a group box widget with the given parent but with no title.
*/
// QGroupBox(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QGroupBox {
  pub fn QGroupBox_1<T: QGroupBox_QGroupBox_1>(value: T) -> QGroupBox {
    let rsthis = value.QGroupBox_1();
    return rsthis;
    // return 1;
  }
}

pub trait QGroupBox_QGroupBox_1 {
  fn QGroupBox_1(self) -> QGroupBox;
}
// QGroupBox(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGroupBox_QGroupBox_1 for (usize,usize) {
  fn QGroupBox_1(self) -> QGroupBox {
    // unsafe{_ZN9QGroupBoxC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QGroupBoxC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGroupBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGroupBox()

/*

*/
pub fn DeleteQGroupBox(this :*mut QGroupBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QGroupBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgroupbox.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QString title() const

/*

*/
impl /*struct*/ QGroupBox {
  pub fn title_0<RetType, T: QGroupBox_title_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.title_0(self);
    // return 1;
  }
}
pub trait QGroupBox_title_0<RetType> {
  fn title_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_title_0<usize> for () {
  fn title_0(self , rsthis: & QGroupBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGroupBox5titleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTitle(const QString &)

/*

*/
impl /*struct*/ QGroupBox {
  pub fn setTitle_0<RetType, T: QGroupBox_setTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTitle_0(self);
    // return 1;
  }
}
pub trait QGroupBox_setTitle_0<RetType> {
  fn setTitle_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_setTitle_0<(/*void*/)> for (usize) {
  fn setTitle_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox8setTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:69
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QGroupBox {
  pub fn alignment_0<RetType, T: QGroupBox_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QGroupBox_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGroupBox9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(int)

/*

*/
impl /*struct*/ QGroupBox {
  pub fn setAlignment_0<RetType, T: QGroupBox_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QGroupBox_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox12setAlignmentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QGroupBox {
  pub fn minimumSizeHint_0<RetType, T: QGroupBox_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QGroupBox_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QGroupBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGroupBox15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFlat() const

/*

*/
impl /*struct*/ QGroupBox {
  pub fn isFlat_0<RetType, T: QGroupBox_isFlat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFlat_0(self);
    // return 1;
  }
}
pub trait QGroupBox_isFlat_0<RetType> {
  fn isFlat_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_isFlat_0<bool> for () {
  fn isFlat_0(self , rsthis: & QGroupBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGroupBox6isFlatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlat(bool)

/*

*/
impl /*struct*/ QGroupBox {
  pub fn setFlat_0<RetType, T: QGroupBox_setFlat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlat_0(self);
    // return 1;
  }
}
pub trait QGroupBox_setFlat_0<RetType> {
  fn setFlat_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_setFlat_0<(/*void*/)> for (bool) {
  fn setFlat_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox7setFlatEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:76
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCheckable() const

/*

*/
impl /*struct*/ QGroupBox {
  pub fn isCheckable_0<RetType, T: QGroupBox_isCheckable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCheckable_0(self);
    // return 1;
  }
}
pub trait QGroupBox_isCheckable_0<RetType> {
  fn isCheckable_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_isCheckable_0<bool> for () {
  fn isCheckable_0(self , rsthis: & QGroupBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGroupBox11isCheckableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCheckable(bool)

/*

*/
impl /*struct*/ QGroupBox {
  pub fn setCheckable_0<RetType, T: QGroupBox_setCheckable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckable_0(self);
    // return 1;
  }
}
pub trait QGroupBox_setCheckable_0<RetType> {
  fn setCheckable_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_setCheckable_0<(/*void*/)> for (bool) {
  fn setCheckable_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox12setCheckableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:78
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isChecked() const

/*

*/
impl /*struct*/ QGroupBox {
  pub fn isChecked_0<RetType, T: QGroupBox_isChecked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isChecked_0(self);
    // return 1;
  }
}
pub trait QGroupBox_isChecked_0<RetType> {
  fn isChecked_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_isChecked_0<bool> for () {
  fn isChecked_0(self , rsthis: & QGroupBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGroupBox9isCheckedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChecked(bool)

/*

*/
impl /*struct*/ QGroupBox {
  pub fn setChecked_0<RetType, T: QGroupBox_setChecked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChecked_0(self);
    // return 1;
  }
}
pub trait QGroupBox_setChecked_0<RetType> {
  fn setChecked_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_setChecked_0<(/*void*/)> for (bool) {
  fn setChecked_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox10setCheckedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clicked(bool)

/*
This signal is emitted when the check box is activated (i.e., pressed down then released while the mouse cursor is inside the button), or when the shortcut key is typed. Notably, this signal is not emitted if you call setChecked().

If the check box is checked, checked is true; it is false if the check box is unchecked.

This function was introduced in  Qt 4.2.

See also checkable, toggled(), and checked.
*/
impl /*struct*/ QGroupBox {
  pub fn clicked_0<RetType, T: QGroupBox_clicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clicked_0(self);
    // return 1;
  }
}
pub trait QGroupBox_clicked_0<RetType> {
  fn clicked_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_clicked_0<(/*void*/)> for (bool) {
  fn clicked_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox7clickedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toggled(bool)

/*
If the group box is checkable, this signal is emitted when the check box is toggled. on is true if the check box is checked; otherwise, it is false.

Note: Notifier signal for property checked. 

See also checkable.
*/
impl /*struct*/ QGroupBox {
  pub fn toggled_0<RetType, T: QGroupBox_toggled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggled_0(self);
    // return 1;
  }
}
pub trait QGroupBox_toggled_0<RetType> {
  fn toggled_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_toggled_0<(/*void*/)> for (bool) {
  fn toggled_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox7toggledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:88
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QGroupBox {
  pub fn event_0<RetType, T: QGroupBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QGroupBox_event_0<RetType> {
  fn event_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QGroupBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QGroupBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:89
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void childEvent(QChildEvent *)

/*
Reimplemented from QObject::childEvent().
*/
impl /*struct*/ QGroupBox {
  pub fn childEvent_0<RetType, T: QGroupBox_childEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childEvent_0(self);
    // return 1;
  }
}
pub trait QGroupBox_childEvent_0<RetType> {
  fn childEvent_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_childEvent_0<(/*void*/)> for (usize) {
  fn childEvent_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox10childEventEP11QChildEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:90
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QGroupBox {
  pub fn resizeEvent_0<RetType, T: QGroupBox_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QGroupBox_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:91
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QGroupBox {
  pub fn paintEvent_0<RetType, T: QGroupBox_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QGroupBox_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:92
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QGroupBox {
  pub fn focusInEvent_0<RetType, T: QGroupBox_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QGroupBox_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:93
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QGroupBox {
  pub fn changeEvent_0<RetType, T: QGroupBox_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QGroupBox_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:94
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QGroupBox {
  pub fn mousePressEvent_0<RetType, T: QGroupBox_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QGroupBox_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:95
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QGroupBox {
  pub fn mouseMoveEvent_0<RetType, T: QGroupBox_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGroupBox_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:96
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QGroupBox {
  pub fn mouseReleaseEvent_0<RetType, T: QGroupBox_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGroupBox_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGroupBox17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgroupbox.h:97
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionGroupBox *) const

/*
Initialize option with the values from this QGroupBox. This method is useful for subclasses when they need a QStyleOptionGroupBox, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QGroupBox {
  pub fn initStyleOption_0<RetType, T: QGroupBox_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QGroupBox_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QGroupBox) -> RetType;
}
impl<'a> /*trait*/ QGroupBox_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QGroupBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QGroupBox15initStyleOptionEP20QStyleOptionGroupBox", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
