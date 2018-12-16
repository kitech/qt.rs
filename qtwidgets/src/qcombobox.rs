

// mod ::widgets::QComboBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qcombobox.h
// #include <qcombobox.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 28
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

// void focusInEvent(QFocusEvent *)
// func (this *QComboBox) InheritFocusInEvent(f func(e *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QComboBox) InheritFocusOutEvent(f func(e *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QComboBox) InheritChangeEvent(f func(e *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QComboBox) InheritResizeEvent(f func(e *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QComboBox) InheritPaintEvent(f func(e *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QComboBox) InheritShowEvent(f func(e *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QComboBox) InheritHideEvent(f func(e *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QComboBox) InheritMousePressEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QComboBox) InheritMouseReleaseEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QComboBox) InheritKeyPressEvent(f func(e *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QComboBox) InheritKeyReleaseEvent(f func(e *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QComboBox) InheritWheelEvent(f func(e *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QComboBox) InheritContextMenuEvent(f func(e *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QComboBox) InheritInputMethodEvent(f func(arg0 *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// void initStyleOption(QStyleOptionComboBox *)
// func (this *QComboBox) InheritInitStyleOption(f func(option *QStyleOptionComboBox/*777 QStyleOptionComboBox **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QComboBox)=48
pub struct QComboBox {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QComboBox_ITF interface {
//    QWidget_ITF
//    QComboBox_PTR() *QComboBox
//}
//func (ptr *QComboBox) QComboBox_PTR() *QComboBox { return ptr }

impl /*struct*/ QComboBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QComboBox {
    return QComboBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QComboBox {
//  type Target = QComboBoxBASE;
//
//  fn deref(&self) -> &QComboBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QComboBoxBASE> for QComboBox {
//  fn as_ref(& self) -> & QComboBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcombobox.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn metaObject_0<RetType, T: QComboBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QComboBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QComboBox(QWidget *)

/*
Constructs a combobox with the given parent, using the default model QStandardItemModel.
*/
// QComboBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QComboBox {
  pub fn QComboBox_0<T: QComboBox_QComboBox_0>(value: T) -> QComboBox {
    let rsthis = value.QComboBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QComboBox_QComboBox_0 {
  fn QComboBox_0(self) -> QComboBox;
}
// QComboBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QComboBox_QComboBox_0 for (usize) {
  fn QComboBox_0(self) -> QComboBox {
    // unsafe{_ZN9QComboBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QComboBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QComboBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QComboBox()

/*

*/
pub fn DeleteQComboBox(this :*mut QComboBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QComboBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcombobox.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] int maxVisibleItems() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn maxVisibleItems_0<RetType, T: QComboBox_maxVisibleItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maxVisibleItems_0(self);
    // return 1;
  }
}
pub trait QComboBox_maxVisibleItems_0<RetType> {
  fn maxVisibleItems_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_maxVisibleItems_0<i32> for () {
  fn maxVisibleItems_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox15maxVisibleItemsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaxVisibleItems(int)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setMaxVisibleItems_0<RetType, T: QComboBox_setMaxVisibleItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaxVisibleItems_0(self);
    // return 1;
  }
}
pub trait QComboBox_setMaxVisibleItems_0<RetType> {
  fn setMaxVisibleItems_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setMaxVisibleItems_0<(/*void*/)> for (i32) {
  fn setMaxVisibleItems_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox18setMaxVisibleItemsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:91
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn count_0<RetType, T: QComboBox_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QComboBox_count_0<RetType> {
  fn count_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_count_0<i32> for () {
  fn count_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaxCount(int)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setMaxCount_0<RetType, T: QComboBox_setMaxCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaxCount_0(self);
    // return 1;
  }
}
pub trait QComboBox_setMaxCount_0<RetType> {
  fn setMaxCount_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setMaxCount_0<(/*void*/)> for (i32) {
  fn setMaxCount_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11setMaxCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] int maxCount() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn maxCount_0<RetType, T: QComboBox_maxCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maxCount_0(self);
    // return 1;
  }
}
pub trait QComboBox_maxCount_0<RetType> {
  fn maxCount_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_maxCount_0<i32> for () {
  fn maxCount_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8maxCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoCompletion() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn autoCompletion_0<RetType, T: QComboBox_autoCompletion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoCompletion_0(self);
    // return 1;
  }
}
pub trait QComboBox_autoCompletion_0<RetType> {
  fn autoCompletion_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_autoCompletion_0<bool> for () {
  fn autoCompletion_0(self , rsthis: & QComboBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox14autoCompletionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoCompletion(bool)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setAutoCompletion_0<RetType, T: QComboBox_setAutoCompletion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoCompletion_0(self);
    // return 1;
  }
}
pub trait QComboBox_setAutoCompletion_0<RetType> {
  fn setAutoCompletion_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setAutoCompletion_0<(/*void*/)> for (bool) {
  fn setAutoCompletion_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox17setAutoCompletionEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CaseSensitivity autoCompletionCaseSensitivity() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn autoCompletionCaseSensitivity_0<RetType, T: QComboBox_autoCompletionCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoCompletionCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QComboBox_autoCompletionCaseSensitivity_0<RetType> {
  fn autoCompletionCaseSensitivity_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_autoCompletionCaseSensitivity_0<i32> for () {
  fn autoCompletionCaseSensitivity_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox29autoCompletionCaseSensitivityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoCompletionCaseSensitivity(Qt::CaseSensitivity)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setAutoCompletionCaseSensitivity_0<RetType, T: QComboBox_setAutoCompletionCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoCompletionCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QComboBox_setAutoCompletionCaseSensitivity_0<RetType> {
  fn setAutoCompletionCaseSensitivity_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setAutoCompletionCaseSensitivity_0<(/*void*/)> for (i32) {
  fn setAutoCompletionCaseSensitivity_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox32setAutoCompletionCaseSensitivityEN2Qt15CaseSensitivityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:103
// index:0
// Public Visibility=Default Availability=Available
// [1] bool duplicatesEnabled() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn duplicatesEnabled_0<RetType, T: QComboBox_duplicatesEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.duplicatesEnabled_0(self);
    // return 1;
  }
}
pub trait QComboBox_duplicatesEnabled_0<RetType> {
  fn duplicatesEnabled_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_duplicatesEnabled_0<bool> for () {
  fn duplicatesEnabled_0(self , rsthis: & QComboBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox17duplicatesEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDuplicatesEnabled(bool)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setDuplicatesEnabled_0<RetType, T: QComboBox_setDuplicatesEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDuplicatesEnabled_0(self);
    // return 1;
  }
}
pub trait QComboBox_setDuplicatesEnabled_0<RetType> {
  fn setDuplicatesEnabled_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setDuplicatesEnabled_0<(/*void*/)> for (bool) {
  fn setDuplicatesEnabled_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox20setDuplicatesEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFrame(bool)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setFrame_0<RetType, T: QComboBox_setFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrame_0(self);
    // return 1;
  }
}
pub trait QComboBox_setFrame_0<RetType> {
  fn setFrame_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setFrame_0<(/*void*/)> for (bool) {
  fn setFrame_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox8setFrameEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:107
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFrame() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn hasFrame_0<RetType, T: QComboBox_hasFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFrame_0(self);
    // return 1;
  }
}
pub trait QComboBox_hasFrame_0<RetType> {
  fn hasFrame_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_hasFrame_0<bool> for () {
  fn hasFrame_0(self , rsthis: & QComboBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8hasFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:109
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int findText(const QString &, Qt::MatchFlags) const

/*
Returns the index of the item containing the given text; otherwise returns -1.

The flags specify how the items in the combobox are searched.
*/
impl /*struct*/ QComboBox {
  pub fn findText_0<RetType, T: QComboBox_findText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.findText_0(self);
    // return 1;
  }
}
pub trait QComboBox_findText_0<RetType> {
  fn findText_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_findText_0<i32> for (usize,i32) {
  fn findText_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8findTextERK7QString6QFlagsIN2Qt9MatchFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] int findData(const QVariant &, int, Qt::MatchFlags) const

/*
Returns the index of the item containing the given data for the given role; otherwise returns -1.

The flags specify how the items in the combobox are searched.
*/
impl /*struct*/ QComboBox {
  pub fn findData_0<RetType, T: QComboBox_findData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.findData_0(self);
    // return 1;
  }
}
pub trait QComboBox_findData_0<RetType> {
  fn findData_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_findData_0<i32> for (usize,i32,i32) {
  fn findData_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8findDataERK8QVarianti6QFlagsIN2Qt9MatchFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:126
// index:0
// Public Visibility=Default Availability=Available
// [4] QComboBox::InsertPolicy insertPolicy() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn insertPolicy_0<RetType, T: QComboBox_insertPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertPolicy_0(self);
    // return 1;
  }
}
pub trait QComboBox_insertPolicy_0<RetType> {
  fn insertPolicy_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_insertPolicy_0<i32> for () {
  fn insertPolicy_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox12insertPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInsertPolicy(QComboBox::InsertPolicy)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setInsertPolicy_0<RetType, T: QComboBox_setInsertPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInsertPolicy_0(self);
    // return 1;
  }
}
pub trait QComboBox_setInsertPolicy_0<RetType> {
  fn setInsertPolicy_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setInsertPolicy_0<(/*void*/)> for (i32) {
  fn setInsertPolicy_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox15setInsertPolicyENS_12InsertPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] QComboBox::SizeAdjustPolicy sizeAdjustPolicy() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn sizeAdjustPolicy_0<RetType, T: QComboBox_sizeAdjustPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeAdjustPolicy_0(self);
    // return 1;
  }
}
pub trait QComboBox_sizeAdjustPolicy_0<RetType> {
  fn sizeAdjustPolicy_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_sizeAdjustPolicy_0<i32> for () {
  fn sizeAdjustPolicy_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox16sizeAdjustPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizeAdjustPolicy(QComboBox::SizeAdjustPolicy)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setSizeAdjustPolicy_0<RetType, T: QComboBox_setSizeAdjustPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeAdjustPolicy_0(self);
    // return 1;
  }
}
pub trait QComboBox_setSizeAdjustPolicy_0<RetType> {
  fn setSizeAdjustPolicy_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setSizeAdjustPolicy_0<(/*void*/)> for (i32) {
  fn setSizeAdjustPolicy_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox19setSizeAdjustPolicyENS_16SizeAdjustPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:139
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimumContentsLength() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn minimumContentsLength_0<RetType, T: QComboBox_minimumContentsLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumContentsLength_0(self);
    // return 1;
  }
}
pub trait QComboBox_minimumContentsLength_0<RetType> {
  fn minimumContentsLength_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_minimumContentsLength_0<i32> for () {
  fn minimumContentsLength_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox21minimumContentsLengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumContentsLength(int)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setMinimumContentsLength_0<RetType, T: QComboBox_setMinimumContentsLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumContentsLength_0(self);
    // return 1;
  }
}
pub trait QComboBox_setMinimumContentsLength_0<RetType> {
  fn setMinimumContentsLength_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setMinimumContentsLength_0<(/*void*/)> for (i32) {
  fn setMinimumContentsLength_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox24setMinimumContentsLengthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:141
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize iconSize() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn iconSize_0<RetType, T: QComboBox_iconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSize_0(self);
    // return 1;
  }
}
pub trait QComboBox_iconSize_0<RetType> {
  fn iconSize_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_iconSize_0<usize> for () {
  fn iconSize_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8iconSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconSize(const QSize &)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setIconSize_0<RetType, T: QComboBox_setIconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconSize_0(self);
    // return 1;
  }
}
pub trait QComboBox_setIconSize_0<RetType> {
  fn setIconSize_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setIconSize_0<(/*void*/)> for (usize) {
  fn setIconSize_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11setIconSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:144
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEditable() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn isEditable_0<RetType, T: QComboBox_isEditable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEditable_0(self);
    // return 1;
  }
}
pub trait QComboBox_isEditable_0<RetType> {
  fn isEditable_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_isEditable_0<bool> for () {
  fn isEditable_0(self , rsthis: & QComboBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox10isEditableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEditable(bool)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setEditable_0<RetType, T: QComboBox_setEditable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEditable_0(self);
    // return 1;
  }
}
pub trait QComboBox_setEditable_0<RetType> {
  fn setEditable_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setEditable_0<(/*void*/)> for (bool) {
  fn setEditable_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11setEditableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLineEdit(QLineEdit *)

/*
Sets the line edit to use instead of the current line edit widget.

The combo box takes ownership of the line edit.

See also lineEdit().
*/
impl /*struct*/ QComboBox {
  pub fn setLineEdit_0<RetType, T: QComboBox_setLineEdit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineEdit_0(self);
    // return 1;
  }
}
pub trait QComboBox_setLineEdit_0<RetType> {
  fn setLineEdit_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setLineEdit_0<(/*void*/)> for (usize) {
  fn setLineEdit_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11setLineEditEP9QLineEdit", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:147
// index:0
// Public Visibility=Default Availability=Available
// [8] QLineEdit * lineEdit() const

/*
Returns the line edit used to edit items in the combobox, or 0 if there is no line edit.

Only editable combo boxes have a line edit.

See also setLineEdit().
*/
impl /*struct*/ QComboBox {
  pub fn lineEdit_0<RetType, T: QComboBox_lineEdit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineEdit_0(self);
    // return 1;
  }
}
pub trait QComboBox_lineEdit_0<RetType> {
  fn lineEdit_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_lineEdit_0<usize> for () {
  fn lineEdit_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8lineEditEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValidator(const QValidator *)

/*
Sets the validator to use instead of the current validator.

Note: The validator is removed when the editable property becomes false.

See also validator().
*/
impl /*struct*/ QComboBox {
  pub fn setValidator_0<RetType, T: QComboBox_setValidator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValidator_0(self);
    // return 1;
  }
}
pub trait QComboBox_setValidator_0<RetType> {
  fn setValidator_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setValidator_0<(/*void*/)> for (usize) {
  fn setValidator_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox12setValidatorEPK10QValidator", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:150
// index:0
// Public Visibility=Default Availability=Available
// [8] const QValidator * validator() const

/*
Returns the validator that is used to constrain text input for the combobox.

See also setValidator() and editable.
*/
impl /*struct*/ QComboBox {
  pub fn validator_0<RetType, T: QComboBox_validator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validator_0(self);
    // return 1;
  }
}
pub trait QComboBox_validator_0<RetType> {
  fn validator_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_validator_0<usize> for () {
  fn validator_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox9validatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCompleter(QCompleter *)

/*
Sets the completer to use instead of the current completer. If completer is 0, auto completion is disabled.

By default, for an editable combo box, a QCompleter that performs case insensitive inline completion is automatically created.

Note: The completer is removed when the editable property becomes false.

This function was introduced in  Qt 4.2.

See also completer().
*/
impl /*struct*/ QComboBox {
  pub fn setCompleter_0<RetType, T: QComboBox_setCompleter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCompleter_0(self);
    // return 1;
  }
}
pub trait QComboBox_setCompleter_0<RetType> {
  fn setCompleter_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setCompleter_0<(/*void*/)> for (usize) {
  fn setCompleter_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox12setCompleterEP10QCompleter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:155
// index:0
// Public Visibility=Default Availability=Available
// [8] QCompleter * completer() const

/*
Returns the completer that is used to auto complete text input for the combobox.

This function was introduced in  Qt 4.2.

See also setCompleter() and editable.
*/
impl /*struct*/ QComboBox {
  pub fn completer_0<RetType, T: QComboBox_completer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completer_0(self);
    // return 1;
  }
}
pub trait QComboBox_completer_0<RetType> {
  fn completer_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_completer_0<usize> for () {
  fn completer_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox9completerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:158
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemDelegate * itemDelegate() const

/*
Returns the item delegate used by the popup list view.

See also setItemDelegate().
*/
impl /*struct*/ QComboBox {
  pub fn itemDelegate_0<RetType, T: QComboBox_itemDelegate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate_0(self);
    // return 1;
  }
}
pub trait QComboBox_itemDelegate_0<RetType> {
  fn itemDelegate_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_itemDelegate_0<usize> for () {
  fn itemDelegate_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox12itemDelegateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemDelegate(QAbstractItemDelegate *)

/*
Sets the item delegate for the popup list view. The combobox takes ownership of the delegate.

Warning: You should not share the same instance of a delegate between comboboxes, widget mappers or views. Doing so can cause incorrect or unintuitive editing behavior since each view connected to a given delegate may receive the closeEditor() signal, and attempt to access, modify or close an editor that has already been closed.

See also itemDelegate().
*/
impl /*struct*/ QComboBox {
  pub fn setItemDelegate_0<RetType, T: QComboBox_setItemDelegate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegate_0(self);
    // return 1;
  }
}
pub trait QComboBox_setItemDelegate_0<RetType> {
  fn setItemDelegate_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setItemDelegate_0<(/*void*/)> for (usize) {
  fn setItemDelegate_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox15setItemDelegateEP21QAbstractItemDelegate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:161
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemModel * model() const

/*
Returns the model used by the combobox.

See also setModel().
*/
impl /*struct*/ QComboBox {
  pub fn model_0<RetType, T: QComboBox_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QComboBox_model_0<RetType> {
  fn model_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_model_0<usize> for () {
  fn model_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Sets the model to be model. model must not be 0. If you want to clear the contents of a model, call clear().

See also model() and clear().
*/
impl /*struct*/ QComboBox {
  pub fn setModel_0<RetType, T: QComboBox_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QComboBox_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:164
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex rootModelIndex() const

/*
Returns the root model item index for the items in the combobox.

See also setRootModelIndex().
*/
impl /*struct*/ QComboBox {
  pub fn rootModelIndex_0<RetType, T: QComboBox_rootModelIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootModelIndex_0(self);
    // return 1;
  }
}
pub trait QComboBox_rootModelIndex_0<RetType> {
  fn rootModelIndex_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_rootModelIndex_0<usize> for () {
  fn rootModelIndex_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox14rootModelIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:165
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRootModelIndex(const QModelIndex &)

/*
Sets the root model item index for the items in the combobox.

See also rootModelIndex().
*/
impl /*struct*/ QComboBox {
  pub fn setRootModelIndex_0<RetType, T: QComboBox_setRootModelIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootModelIndex_0(self);
    // return 1;
  }
}
pub trait QComboBox_setRootModelIndex_0<RetType> {
  fn setRootModelIndex_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setRootModelIndex_0<(/*void*/)> for (usize) {
  fn setRootModelIndex_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox17setRootModelIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:167
// index:0
// Public Visibility=Default Availability=Available
// [4] int modelColumn() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn modelColumn_0<RetType, T: QComboBox_modelColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modelColumn_0(self);
    // return 1;
  }
}
pub trait QComboBox_modelColumn_0<RetType> {
  fn modelColumn_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_modelColumn_0<i32> for () {
  fn modelColumn_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox11modelColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModelColumn(int)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setModelColumn_0<RetType, T: QComboBox_setModelColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModelColumn_0(self);
    // return 1;
  }
}
pub trait QComboBox_setModelColumn_0<RetType> {
  fn setModelColumn_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setModelColumn_0<(/*void*/)> for (i32) {
  fn setModelColumn_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox14setModelColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:170
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentIndex() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn currentIndex_0<RetType, T: QComboBox_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QComboBox_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_currentIndex_0<i32> for () {
  fn currentIndex_0(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:171
// index:0
// Public Visibility=Default Availability=Available
// [8] QString currentText() const

/*

*/
impl /*struct*/ QComboBox {
  pub fn currentText_0<RetType, T: QComboBox_currentText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentText_0(self);
    // return 1;
  }
}
pub trait QComboBox_currentText_0<RetType> {
  fn currentText_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_currentText_0<usize> for () {
  fn currentText_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox11currentTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:172
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant currentData(int) const

/*

*/
impl /*struct*/ QComboBox {
  pub fn currentData_0<RetType, T: QComboBox_currentData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentData_0(self);
    // return 1;
  }
}
pub trait QComboBox_currentData_0<RetType> {
  fn currentData_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_currentData_0<usize> for (i32) {
  fn currentData_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox11currentDataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:174
// index:0
// Public Visibility=Default Availability=Available
// [8] QString itemText(int) const

/*
Returns the text for the given index in the combobox.

See also setItemText().
*/
impl /*struct*/ QComboBox {
  pub fn itemText_0<RetType, T: QComboBox_itemText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemText_0(self);
    // return 1;
  }
}
pub trait QComboBox_itemText_0<RetType> {
  fn itemText_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_itemText_0<usize> for (i32) {
  fn itemText_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8itemTextEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:175
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon itemIcon(int) const

/*
Returns the icon for the given index in the combobox.

See also setItemIcon().
*/
impl /*struct*/ QComboBox {
  pub fn itemIcon_0<RetType, T: QComboBox_itemIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemIcon_0(self);
    // return 1;
  }
}
pub trait QComboBox_itemIcon_0<RetType> {
  fn itemIcon_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_itemIcon_0<usize> for (i32) {
  fn itemIcon_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8itemIconEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:176
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant itemData(int, int) const

/*
Returns the data for the given role in the given index in the combobox, or QVariant::Invalid if there is no data for this role.

See also setItemData().
*/
impl /*struct*/ QComboBox {
  pub fn itemData_0<RetType, T: QComboBox_itemData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemData_0(self);
    // return 1;
  }
}
pub trait QComboBox_itemData_0<RetType> {
  fn itemData_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_itemData_0<usize> for (i32,i32) {
  fn itemData_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8itemDataEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:178
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void addItem(const QString &, const QVariant &)

/*
Adds an item to the combobox with the given text, and containing the specified userData (stored in the Qt::UserRole). The item is appended to the list of existing items.
*/
impl /*struct*/ QComboBox {
  pub fn addItem_0<RetType, T: QComboBox_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QComboBox_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_addItem_0<(/*void*/)> for (usize,usize) {
  fn addItem_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox7addItemERK7QStringRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:179
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void addItem(const QIcon &, const QString &, const QVariant &)

/*
Adds an item to the combobox with the given text, and containing the specified userData (stored in the Qt::UserRole). The item is appended to the list of existing items.
*/
impl /*struct*/ QComboBox {
  pub fn addItem_1<RetType, T: QComboBox_addItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_1(self);
    // return 1;
  }
}
pub trait QComboBox_addItem_1<RetType> {
  fn addItem_1(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_addItem_1<(/*void*/)> for (usize,usize,usize) {
  fn addItem_1(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:181
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void addItems(const QStringList &)

/*
Adds each of the strings in the given texts to the combobox. Each item is appended to the list of existing items in turn.
*/
impl /*struct*/ QComboBox {
  pub fn addItems_0<RetType, T: QComboBox_addItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItems_0(self);
    // return 1;
  }
}
pub trait QComboBox_addItems_0<RetType> {
  fn addItems_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_addItems_0<(/*void*/)> for (usize) {
  fn addItems_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox8addItemsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:184
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void insertItem(int, const QString &, const QVariant &)

/*
Inserts the text and userData (stored in the Qt::UserRole) into the combobox at the given index.

If the index is equal to or higher than the total number of items, the new item is appended to the list of existing items. If the index is zero or negative, the new item is prepended to the list of existing items.

See also insertItems().
*/
impl /*struct*/ QComboBox {
  pub fn insertItem_0<RetType, T: QComboBox_insertItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItem_0(self);
    // return 1;
  }
}
pub trait QComboBox_insertItem_0<RetType> {
  fn insertItem_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_insertItem_0<(/*void*/)> for (i32,usize,usize) {
  fn insertItem_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox10insertItemEiRK7QStringRK8QVariant", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:185
// index:1
// Public Visibility=Default Availability=Available
// [-2] void insertItem(int, const QIcon &, const QString &, const QVariant &)

/*
Inserts the text and userData (stored in the Qt::UserRole) into the combobox at the given index.

If the index is equal to or higher than the total number of items, the new item is appended to the list of existing items. If the index is zero or negative, the new item is prepended to the list of existing items.

See also insertItems().
*/
impl /*struct*/ QComboBox {
  pub fn insertItem_1<RetType, T: QComboBox_insertItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItem_1(self);
    // return 1;
  }
}
pub trait QComboBox_insertItem_1<RetType> {
  fn insertItem_1(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_insertItem_1<(/*void*/)> for (i32,usize,usize,usize) {
  fn insertItem_1(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertItems(int, const QStringList &)

/*
Inserts the strings from the list into the combobox as separate items, starting at the index specified.

If the index is equal to or higher than the total number of items, the new items are appended to the list of existing items. If the index is zero or negative, the new items are prepended to the list of existing items.

See also insertItem().
*/
impl /*struct*/ QComboBox {
  pub fn insertItems_0<RetType, T: QComboBox_insertItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItems_0(self);
    // return 1;
  }
}
pub trait QComboBox_insertItems_0<RetType> {
  fn insertItems_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_insertItems_0<(/*void*/)> for (i32,usize) {
  fn insertItems_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11insertItemsEiRK11QStringList", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertSeparator(int)

/*
Inserts a separator item into the combobox at the given index.

If the index is equal to or higher than the total number of items, the new item is appended to the list of existing items. If the index is zero or negative, the new item is prepended to the list of existing items.

This function was introduced in  Qt 4.4.

See also insertItem().
*/
impl /*struct*/ QComboBox {
  pub fn insertSeparator_0<RetType, T: QComboBox_insertSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertSeparator_0(self);
    // return 1;
  }
}
pub trait QComboBox_insertSeparator_0<RetType> {
  fn insertSeparator_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_insertSeparator_0<(/*void*/)> for (i32) {
  fn insertSeparator_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox15insertSeparatorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeItem(int)

/*
Removes the item at the given index from the combobox. This will update the current index if the index is removed.

This function does nothing if index is out of range.
*/
impl /*struct*/ QComboBox {
  pub fn removeItem_0<RetType, T: QComboBox_removeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItem_0(self);
    // return 1;
  }
}
pub trait QComboBox_removeItem_0<RetType> {
  fn removeItem_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_removeItem_0<(/*void*/)> for (i32) {
  fn removeItem_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox10removeItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:192
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemText(int, const QString &)

/*
Sets the text for the item on the given index in the combobox.

See also itemText().
*/
impl /*struct*/ QComboBox {
  pub fn setItemText_0<RetType, T: QComboBox_setItemText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemText_0(self);
    // return 1;
  }
}
pub trait QComboBox_setItemText_0<RetType> {
  fn setItemText_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setItemText_0<(/*void*/)> for (i32,usize) {
  fn setItemText_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11setItemTextEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:193
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemIcon(int, const QIcon &)

/*
Sets the icon for the item on the given index in the combobox.

See also itemIcon().
*/
impl /*struct*/ QComboBox {
  pub fn setItemIcon_0<RetType, T: QComboBox_setItemIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemIcon_0(self);
    // return 1;
  }
}
pub trait QComboBox_setItemIcon_0<RetType> {
  fn setItemIcon_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setItemIcon_0<(/*void*/)> for (i32,usize) {
  fn setItemIcon_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11setItemIconEiRK5QIcon", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:194
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemData(int, const QVariant &, int)

/*
Sets the data role for the item on the given index in the combobox to the specified value.

See also itemData().
*/
impl /*struct*/ QComboBox {
  pub fn setItemData_0<RetType, T: QComboBox_setItemData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemData_0(self);
    // return 1;
  }
}
pub trait QComboBox_setItemData_0<RetType> {
  fn setItemData_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setItemData_0<(/*void*/)> for (i32,usize,i32) {
  fn setItemData_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11setItemDataEiRK8QVarianti", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:196
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemView * view() const

/*
Returns the list view used for the combobox popup.

See also setView().
*/
impl /*struct*/ QComboBox {
  pub fn view_0<RetType, T: QComboBox_view_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.view_0(self);
    // return 1;
  }
}
pub trait QComboBox_view_0<RetType> {
  fn view_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_view_0<usize> for () {
  fn view_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox4viewEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:197
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setView(QAbstractItemView *)

/*
Sets the view to be used in the combobox popup to the given itemView. The combobox takes ownership of the view.

Note: If you want to use the convenience views (like QListWidget, QTableWidget or QTreeWidget), make sure to call setModel() on the combobox with the convenience widgets model before calling this function.

See also view().
*/
impl /*struct*/ QComboBox {
  pub fn setView_0<RetType, T: QComboBox_setView_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setView_0(self);
    // return 1;
  }
}
pub trait QComboBox_setView_0<RetType> {
  fn setView_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setView_0<(/*void*/)> for (usize) {
  fn setView_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox7setViewEP17QAbstractItemView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:199
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().

This implementation caches the size hint to avoid resizing when the contents change dynamically. To invalidate the cached value change the sizeAdjustPolicy.
*/
impl /*struct*/ QComboBox {
  pub fn sizeHint_0<RetType, T: QComboBox_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QComboBox_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:200
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QComboBox {
  pub fn minimumSizeHint_0<RetType, T: QComboBox_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QComboBox_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:202
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void showPopup()

/*
Displays the list of items in the combobox. If the list is empty then the no items will be shown.

If you reimplement this function to show a custom pop-up, make sure you call hidePopup() to reset the internal state.

See also hidePopup().
*/
impl /*struct*/ QComboBox {
  pub fn showPopup_0<RetType, T: QComboBox_showPopup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showPopup_0(self);
    // return 1;
  }
}
pub trait QComboBox_showPopup_0<RetType> {
  fn showPopup_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_showPopup_0<(/*void*/)> for () {
  fn showPopup_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QComboBox9showPopupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:203
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void hidePopup()

/*
Hides the list of items in the combobox if it is currently visible and resets the internal state, so that if the custom pop-up was shown inside the reimplemented showPopup(), then you also need to reimplement the hidePopup() function to hide your custom pop-up and call the base class implementation to reset the internal state whenever your custom pop-up widget is hidden.

See also showPopup().
*/
impl /*struct*/ QComboBox {
  pub fn hidePopup_0<RetType, T: QComboBox_hidePopup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hidePopup_0(self);
    // return 1;
  }
}
pub trait QComboBox_hidePopup_0<RetType> {
  fn hidePopup_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_hidePopup_0<(/*void*/)> for () {
  fn hidePopup_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QComboBox9hidePopupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:205
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QComboBox {
  pub fn event_0<RetType, T: QComboBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QComboBox_event_0<RetType> {
  fn event_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QComboBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QComboBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:206
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QComboBox {
  pub fn inputMethodQuery_0<RetType, T: QComboBox_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QComboBox_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:207
// index:1
// Public Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery, const QVariant &) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QComboBox {
  pub fn inputMethodQuery_1<RetType, T: QComboBox_inputMethodQuery_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_1(self);
    // return 1;
  }
}
pub trait QComboBox_inputMethodQuery_1<RetType> {
  fn inputMethodQuery_1(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_inputMethodQuery_1<usize> for (i32,usize) {
  fn inputMethodQuery_1(self , rsthis: & QComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QComboBox16inputMethodQueryEN2Qt16InputMethodQueryERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:210
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the combobox, removing all items.

Note: If you have set an external model on the combobox this model will still be cleared when calling this function.
*/
impl /*struct*/ QComboBox {
  pub fn clear_0<RetType, T: QComboBox_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QComboBox_clear_0<RetType> {
  fn clear_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QComboBox5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:211
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearEditText()

/*
Clears the contents of the line edit used for editing in the combobox.
*/
impl /*struct*/ QComboBox {
  pub fn clearEditText_0<RetType, T: QComboBox_clearEditText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearEditText_0(self);
    // return 1;
  }
}
pub trait QComboBox_clearEditText_0<RetType> {
  fn clearEditText_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_clearEditText_0<(/*void*/)> for () {
  fn clearEditText_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QComboBox13clearEditTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:212
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEditText(const QString &)

/*
Sets the text in the combobox's text edit.
*/
impl /*struct*/ QComboBox {
  pub fn setEditText_0<RetType, T: QComboBox_setEditText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEditText_0(self);
    // return 1;
  }
}
pub trait QComboBox_setEditText_0<RetType> {
  fn setEditText_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setEditText_0<(/*void*/)> for (usize) {
  fn setEditText_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11setEditTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:213
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentIndex(int)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setCurrentIndex_0<RetType, T: QComboBox_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QComboBox_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setCurrentIndex_0<(/*void*/)> for (i32) {
  fn setCurrentIndex_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox15setCurrentIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:214
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentText(const QString &)

/*

*/
impl /*struct*/ QComboBox {
  pub fn setCurrentText_0<RetType, T: QComboBox_setCurrentText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentText_0(self);
    // return 1;
  }
}
pub trait QComboBox_setCurrentText_0<RetType> {
  fn setCurrentText_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_setCurrentText_0<(/*void*/)> for (usize) {
  fn setCurrentText_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox14setCurrentTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void editTextChanged(const QString &)

/*
This signal is emitted when the text in the combobox's line edit widget is changed. The new text is specified by text.
*/
impl /*struct*/ QComboBox {
  pub fn editTextChanged_0<RetType, T: QComboBox_editTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editTextChanged_0(self);
    // return 1;
  }
}
pub trait QComboBox_editTextChanged_0<RetType> {
  fn editTextChanged_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_editTextChanged_0<(/*void*/)> for (usize) {
  fn editTextChanged_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox15editTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:218
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activated(int)

/*
This signal is sent when the user chooses an item in the combobox. The item's index is passed. Note that this signal is sent even when the choice is not changed. If you need to know when the choice actually changes, use signal currentIndexChanged().

Note: Signal activated is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(comboBox, QOverload<int>::of(&QComboBox::activated),
      [=](int index){ /-* ... *-/ });
*/
impl /*struct*/ QComboBox {
  pub fn activated_0<RetType, T: QComboBox_activated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activated_0(self);
    // return 1;
  }
}
pub trait QComboBox_activated_0<RetType> {
  fn activated_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_activated_0<(/*void*/)> for (i32) {
  fn activated_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox9activatedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:219
// index:1
// Public Visibility=Default Availability=Available
// [-2] void activated(const QString &)

/*
This signal is sent when the user chooses an item in the combobox. The item's index is passed. Note that this signal is sent even when the choice is not changed. If you need to know when the choice actually changes, use signal currentIndexChanged().

Note: Signal activated is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(comboBox, QOverload<int>::of(&QComboBox::activated),
      [=](int index){ /-* ... *-/ });
*/
impl /*struct*/ QComboBox {
  pub fn activated_1<RetType, T: QComboBox_activated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activated_1(self);
    // return 1;
  }
}
pub trait QComboBox_activated_1<RetType> {
  fn activated_1(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_activated_1<(/*void*/)> for (usize) {
  fn activated_1(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox9activatedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void highlighted(int)

/*
This signal is sent when an item in the combobox popup list is highlighted by the user. The item's index is passed.

Note: Signal highlighted is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(comboBox, QOverload<int>::of(&QComboBox::highlighted),
      [=](int index){ /-* ... *-/ });
*/
impl /*struct*/ QComboBox {
  pub fn highlighted_0<RetType, T: QComboBox_highlighted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlighted_0(self);
    // return 1;
  }
}
pub trait QComboBox_highlighted_0<RetType> {
  fn highlighted_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_highlighted_0<(/*void*/)> for (i32) {
  fn highlighted_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11highlightedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:221
// index:1
// Public Visibility=Default Availability=Available
// [-2] void highlighted(const QString &)

/*
This signal is sent when an item in the combobox popup list is highlighted by the user. The item's index is passed.

Note: Signal highlighted is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(comboBox, QOverload<int>::of(&QComboBox::highlighted),
      [=](int index){ /-* ... *-/ });
*/
impl /*struct*/ QComboBox {
  pub fn highlighted_1<RetType, T: QComboBox_highlighted_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlighted_1(self);
    // return 1;
  }
}
pub trait QComboBox_highlighted_1<RetType> {
  fn highlighted_1(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_highlighted_1<(/*void*/)> for (usize) {
  fn highlighted_1(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11highlightedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:222
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentIndexChanged(int)

/*
This signal is sent whenever the currentIndex in the combobox changes either through user interaction or programmatically. The item's index is passed or -1 if the combobox becomes empty or the currentIndex was reset.

Note: Signal currentIndexChanged is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(comboBox, QOverload<int>::of(&QComboBox::currentIndexChanged),
      [=](int index){ /-* ... *-/ });



This function was introduced in  Qt 4.1.

Note: Notifier signal for property currentIndex.
*/
impl /*struct*/ QComboBox {
  pub fn currentIndexChanged_0<RetType, T: QComboBox_currentIndexChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndexChanged_0(self);
    // return 1;
  }
}
pub trait QComboBox_currentIndexChanged_0<RetType> {
  fn currentIndexChanged_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_currentIndexChanged_0<(/*void*/)> for (i32) {
  fn currentIndexChanged_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox19currentIndexChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:223
// index:1
// Public Visibility=Default Availability=Available
// [-2] void currentIndexChanged(const QString &)

/*
This signal is sent whenever the currentIndex in the combobox changes either through user interaction or programmatically. The item's index is passed or -1 if the combobox becomes empty or the currentIndex was reset.

Note: Signal currentIndexChanged is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(comboBox, QOverload<int>::of(&QComboBox::currentIndexChanged),
      [=](int index){ /-* ... *-/ });



This function was introduced in  Qt 4.1.

Note: Notifier signal for property currentIndex.
*/
impl /*struct*/ QComboBox {
  pub fn currentIndexChanged_1<RetType, T: QComboBox_currentIndexChanged_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndexChanged_1(self);
    // return 1;
  }
}
pub trait QComboBox_currentIndexChanged_1<RetType> {
  fn currentIndexChanged_1(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_currentIndexChanged_1<(/*void*/)> for (usize) {
  fn currentIndexChanged_1(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox19currentIndexChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:224
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentTextChanged(const QString &)

/*
This signal is sent whenever currentText changes. The new value is passed as text.

This function was introduced in  Qt 5.0.

Note: Notifier signal for property currentText.
*/
impl /*struct*/ QComboBox {
  pub fn currentTextChanged_0<RetType, T: QComboBox_currentTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentTextChanged_0(self);
    // return 1;
  }
}
pub trait QComboBox_currentTextChanged_0<RetType> {
  fn currentTextChanged_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_currentTextChanged_0<(/*void*/)> for (usize) {
  fn currentTextChanged_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox18currentTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:227
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QComboBox {
  pub fn focusInEvent_0<RetType, T: QComboBox_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:228
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QComboBox {
  pub fn focusOutEvent_0<RetType, T: QComboBox_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:229
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QComboBox {
  pub fn changeEvent_0<RetType, T: QComboBox_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:230
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QComboBox {
  pub fn resizeEvent_0<RetType, T: QComboBox_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:231
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QComboBox {
  pub fn paintEvent_0<RetType, T: QComboBox_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:232
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QComboBox {
  pub fn showEvent_0<RetType, T: QComboBox_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:233
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Reimplemented from QWidget::hideEvent().
*/
impl /*struct*/ QComboBox {
  pub fn hideEvent_0<RetType, T: QComboBox_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:234
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QComboBox {
  pub fn mousePressEvent_0<RetType, T: QComboBox_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:235
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QComboBox {
  pub fn mouseReleaseEvent_0<RetType, T: QComboBox_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:236
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QComboBox {
  pub fn keyPressEvent_0<RetType, T: QComboBox_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:237
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyReleaseEvent().
*/
impl /*struct*/ QComboBox {
  pub fn keyReleaseEvent_0<RetType, T: QComboBox_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:239
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QComboBox {
  pub fn wheelEvent_0<RetType, T: QComboBox_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:242
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().
*/
impl /*struct*/ QComboBox {
  pub fn contextMenuEvent_0<RetType, T: QComboBox_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:244
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
Reimplemented from QWidget::inputMethodEvent().
*/
impl /*struct*/ QComboBox {
  pub fn inputMethodEvent_0<RetType, T: QComboBox_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QComboBox_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QComboBox16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcombobox.h:245
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionComboBox *) const

/*
Initialize option with the values from this QComboBox. This method is useful for subclasses when they need a QStyleOptionComboBox, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QComboBox {
  pub fn initStyleOption_0<RetType, T: QComboBox_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QComboBox_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QComboBox) -> RetType;
}
impl<'a> /*trait*/ QComboBox_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QComboBox15initStyleOptionEP20QStyleOptionComboBox", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum specifies what the QComboBox should do when a new string is entered by the user.


*/
pub type QComboBox__InsertPolicy = i32;
// The string will not be inserted into the combobox.
pub const QComboBox__NoInsert :QComboBox__InsertPolicy = 0;
// The string will be inserted as the first item in the combobox.
pub const QComboBox__InsertAtTop :QComboBox__InsertPolicy = 1;
// The current item will be replaced by the string.
pub const QComboBox__InsertAtCurrent :QComboBox__InsertPolicy = 2;
// The string will be inserted after the last item in the combobox.
pub const QComboBox__InsertAtBottom :QComboBox__InsertPolicy = 3;
// The string is inserted after the current item in the combobox.
pub const QComboBox__InsertAfterCurrent :QComboBox__InsertPolicy = 4;
// The string is inserted before the current item in the combobox.
pub const QComboBox__InsertBeforeCurrent :QComboBox__InsertPolicy = 5;
// The string is inserted in the alphabetic order in the combobox.
pub const QComboBox__InsertAlphabetically :QComboBox__InsertPolicy = 6;
pub fn QComboBox_InsertPolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QComboBox", val);
}
pub fn QComboBox_InsertPolicyItemName_s(val: i32) ->String {
  //var nilthis *QComboBox
  //return nilthis.InsertPolicyItemName(val);
  return QComboBox_InsertPolicyItemName(val);
}


/*
This enum specifies how the size hint of the QComboBox should adjust when new content is added or content changes.


*/
pub type QComboBox__SizeAdjustPolicy = i32;
// The combobox will always adjust to the contents
pub const QComboBox__AdjustToContents :QComboBox__SizeAdjustPolicy = 0;
// The combobox will adjust to its contents the first time it is shown.
pub const QComboBox__AdjustToContentsOnFirstShow :QComboBox__SizeAdjustPolicy = 1;
// Use AdjustToContents or AdjustToContentsOnFirstShow instead.
pub const QComboBox__AdjustToMinimumContentsLength :QComboBox__SizeAdjustPolicy = 2;
// The combobox will adjust to minimumContentsLength plus space for an icon. For performance reasons use this policy on large models.
pub const QComboBox__AdjustToMinimumContentsLengthWithIcon :QComboBox__SizeAdjustPolicy = 3;
pub fn QComboBox_SizeAdjustPolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QComboBox", val);
}
pub fn QComboBox_SizeAdjustPolicyItemName_s(val: i32) ->String {
  //var nilthis *QComboBox
  //return nilthis.SizeAdjustPolicyItemName(val);
  return QComboBox_SizeAdjustPolicyItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
