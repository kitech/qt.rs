

// mod ::widgets::QLabel
// package qtwidgets
// /usr/include/qt/QtWidgets/qlabel.h
// #include <qlabel.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
// func (this *QLabel) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QLabel) InheritKeyPressEvent(f func(ev *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QLabel) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QLabel) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QLabel) InheritMousePressEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QLabel) InheritMouseMoveEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QLabel) InheritMouseReleaseEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QLabel) InheritContextMenuEvent(f func(ev *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QLabel) InheritFocusInEvent(f func(ev *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QLabel) InheritFocusOutEvent(f func(ev *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QLabel) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QLabel)=48
pub struct QLabel {
  qbase: QFrame,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLabel_ITF interface {
//    QFrame_ITF
//    QLabel_PTR() *QLabel
//}
//func (ptr *QLabel) QLabel_PTR() *QLabel { return ptr }

impl /*struct*/ QLabel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLabel {
    return QLabel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLabel {
//  type Target = QLabelBASE;
//
//  fn deref(&self) -> &QLabelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLabelBASE> for QLabel {
//  fn as_ref(& self) -> & QLabelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlabel.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QLabel {
  pub fn metaObject_0<RetType, T: QLabel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QLabel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLabel(QWidget *, Qt::WindowFlags)

/*
Constructs an empty label.

The parent and widget flag f, arguments are passed to the QFrame constructor.

See also setAlignment(), setFrameStyle(), and setIndent().
*/
// QLabel(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QLabel {
  pub fn QLabel_0<T: QLabel_QLabel_0>(value: T) -> QLabel {
    let rsthis = value.QLabel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLabel_QLabel_0 {
  fn QLabel_0(self) -> QLabel;
}
// QLabel(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLabel_QLabel_0 for (usize,i32) {
  fn QLabel_0(self) -> QLabel {
    // unsafe{_ZN6QLabelC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QLabelC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLabel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:71
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QLabel(const QString &, QWidget *, Qt::WindowFlags)

/*
Constructs an empty label.

The parent and widget flag f, arguments are passed to the QFrame constructor.

See also setAlignment(), setFrameStyle(), and setIndent().
*/
// QLabel(const QString &, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QLabel {
  pub fn QLabel_1<T: QLabel_QLabel_1>(value: T) -> QLabel {
    let rsthis = value.QLabel_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLabel_QLabel_1 {
  fn QLabel_1(self) -> QLabel;
}
// QLabel(const QString &, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLabel_QLabel_1 for (usize,usize,i32) {
  fn QLabel_1(self) -> QLabel {
    // unsafe{_ZN6QLabelC2ERK7QStringP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QLabelC2ERK7QStringP7QWidget6QFlagsIN2Qt10WindowTypeEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLabel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QLabel()

/*

*/
pub fn DeleteQLabel(this :*mut QLabel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QLabelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlabel.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QLabel {
  pub fn text_0<RetType, T: QLabel_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QLabel_text_0<RetType> {
  fn text_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_text_0<usize> for () {
  fn text_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] const QPixmap * pixmap() const

/*

*/
impl /*struct*/ QLabel {
  pub fn pixmap_0<RetType, T: QLabel_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QLabel_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_pixmap_0<usize> for () {
  fn pixmap_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel6pixmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] const QPicture * picture() const

/*
Returns the label's picture or 0 if the label doesn't have a picture.

See also setPicture().
*/
impl /*struct*/ QLabel {
  pub fn picture_0<RetType, T: QLabel_picture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.picture_0(self);
    // return 1;
  }
}
pub trait QLabel_picture_0<RetType> {
  fn picture_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_picture_0<usize> for () {
  fn picture_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel7pictureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:80
// index:0
// Public Visibility=Default Availability=Available
// [8] QMovie * movie() const

/*
Returns a pointer to the label's movie, or 0 if no movie has been set.

See also setMovie().
*/
impl /*struct*/ QLabel {
  pub fn movie_0<RetType, T: QLabel_movie_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.movie_0(self);
    // return 1;
  }
}
pub trait QLabel_movie_0<RetType> {
  fn movie_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_movie_0<usize> for () {
  fn movie_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel5movieEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextFormat textFormat() const

/*

*/
impl /*struct*/ QLabel {
  pub fn textFormat_0<RetType, T: QLabel_textFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textFormat_0(self);
    // return 1;
  }
}
pub trait QLabel_textFormat_0<RetType> {
  fn textFormat_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_textFormat_0<i32> for () {
  fn textFormat_0(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel10textFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextFormat(Qt::TextFormat)

/*

*/
impl /*struct*/ QLabel {
  pub fn setTextFormat_0<RetType, T: QLabel_setTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextFormat_0(self);
    // return 1;
  }
}
pub trait QLabel_setTextFormat_0<RetType> {
  fn setTextFormat_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setTextFormat_0<(/*void*/)> for (i32) {
  fn setTextFormat_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel13setTextFormatEN2Qt10TextFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:86
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QLabel {
  pub fn alignment_0<RetType, T: QLabel_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QLabel_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QLabel {
  pub fn setAlignment_0<RetType, T: QLabel_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QLabel_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWordWrap(bool)

/*

*/
impl /*struct*/ QLabel {
  pub fn setWordWrap_0<RetType, T: QLabel_setWordWrap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWordWrap_0(self);
    // return 1;
  }
}
pub trait QLabel_setWordWrap_0<RetType> {
  fn setWordWrap_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setWordWrap_0<(/*void*/)> for (bool) {
  fn setWordWrap_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel11setWordWrapEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:90
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wordWrap() const

/*

*/
impl /*struct*/ QLabel {
  pub fn wordWrap_0<RetType, T: QLabel_wordWrap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wordWrap_0(self);
    // return 1;
  }
}
pub trait QLabel_wordWrap_0<RetType> {
  fn wordWrap_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_wordWrap_0<bool> for () {
  fn wordWrap_0(self , rsthis: & QLabel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel8wordWrapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:92
// index:0
// Public Visibility=Default Availability=Available
// [4] int indent() const

/*

*/
impl /*struct*/ QLabel {
  pub fn indent_0<RetType, T: QLabel_indent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indent_0(self);
    // return 1;
  }
}
pub trait QLabel_indent_0<RetType> {
  fn indent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_indent_0<i32> for () {
  fn indent_0(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel6indentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIndent(int)

/*

*/
impl /*struct*/ QLabel {
  pub fn setIndent_0<RetType, T: QLabel_setIndent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIndent_0(self);
    // return 1;
  }
}
pub trait QLabel_setIndent_0<RetType> {
  fn setIndent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setIndent_0<(/*void*/)> for (i32) {
  fn setIndent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel9setIndentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:95
// index:0
// Public Visibility=Default Availability=Available
// [4] int margin() const

/*

*/
impl /*struct*/ QLabel {
  pub fn margin_0<RetType, T: QLabel_margin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.margin_0(self);
    // return 1;
  }
}
pub trait QLabel_margin_0<RetType> {
  fn margin_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_margin_0<i32> for () {
  fn margin_0(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel6marginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMargin(int)

/*

*/
impl /*struct*/ QLabel {
  pub fn setMargin_0<RetType, T: QLabel_setMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMargin_0(self);
    // return 1;
  }
}
pub trait QLabel_setMargin_0<RetType> {
  fn setMargin_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setMargin_0<(/*void*/)> for (i32) {
  fn setMargin_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel9setMarginEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:98
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasScaledContents() const

/*

*/
impl /*struct*/ QLabel {
  pub fn hasScaledContents_0<RetType, T: QLabel_hasScaledContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasScaledContents_0(self);
    // return 1;
  }
}
pub trait QLabel_hasScaledContents_0<RetType> {
  fn hasScaledContents_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_hasScaledContents_0<bool> for () {
  fn hasScaledContents_0(self , rsthis: & QLabel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel17hasScaledContentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScaledContents(bool)

/*

*/
impl /*struct*/ QLabel {
  pub fn setScaledContents_0<RetType, T: QLabel_setScaledContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScaledContents_0(self);
    // return 1;
  }
}
pub trait QLabel_setScaledContents_0<RetType> {
  fn setScaledContents_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setScaledContents_0<(/*void*/)> for (bool) {
  fn setScaledContents_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel17setScaledContentsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:100
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QLabel {
  pub fn sizeHint_0<RetType, T: QLabel_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QLabel_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:101
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QLabel {
  pub fn minimumSizeHint_0<RetType, T: QLabel_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QLabel_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBuddy(QWidget *)

/*
Sets this label's buddy to buddy.

When the user presses the shortcut key indicated by this label, the keyboard focus is transferred to the label's buddy widget.

The buddy mechanism is only available for QLabels that contain text in which one character is prefixed with an ampersand, '&'. This character is set as the shortcut key. See the QKeySequence::mnemonic() documentation for details (to display an actual ampersand, use '&&').

In a dialog, you might create two data entry widgets and a label for each, and set up the geometry layout so each label is just to the left of its data entry widget (its "buddy"), for example:


  QLineEdit *nameEdit  = new QLineEdit(this);
  QLabel    *nameLabel = new QLabel("&Name:", this);
  nameLabel->setBuddy(nameEdit);
  QLineEdit *phoneEdit  = new QLineEdit(this);
  QLabel    *phoneLabel = new QLabel("&Phone:", this);
  phoneLabel->setBuddy(phoneEdit);
  // (layout setup not shown)



With the code above, the focus jumps to the Name field when the user presses Alt+N, and to the Phone field when the user presses Alt+P.

To unset a previously set buddy, call this function with buddy set to 0.

See also buddy(), setText(), QShortcut, and setAlignment().
*/
impl /*struct*/ QLabel {
  pub fn setBuddy_0<RetType, T: QLabel_setBuddy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBuddy_0(self);
    // return 1;
  }
}
pub trait QLabel_setBuddy_0<RetType> {
  fn setBuddy_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setBuddy_0<(/*void*/)> for (usize) {
  fn setBuddy_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel8setBuddyEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * buddy() const

/*
Returns this label's buddy, or 0 if no buddy is currently set.

See also setBuddy().
*/
impl /*struct*/ QLabel {
  pub fn buddy_0<RetType, T: QLabel_buddy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buddy_0(self);
    // return 1;
  }
}
pub trait QLabel_buddy_0<RetType> {
  fn buddy_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_buddy_0<usize> for () {
  fn buddy_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel5buddyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:106
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Reimplemented from QWidget::heightForWidth().
*/
impl /*struct*/ QLabel {
  pub fn heightForWidth_0<RetType, T: QLabel_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QLabel_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:108
// index:0
// Public Visibility=Default Availability=Available
// [1] bool openExternalLinks() const

/*

*/
impl /*struct*/ QLabel {
  pub fn openExternalLinks_0<RetType, T: QLabel_openExternalLinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openExternalLinks_0(self);
    // return 1;
  }
}
pub trait QLabel_openExternalLinks_0<RetType> {
  fn openExternalLinks_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_openExternalLinks_0<bool> for () {
  fn openExternalLinks_0(self , rsthis: & QLabel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel17openExternalLinksEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpenExternalLinks(bool)

/*

*/
impl /*struct*/ QLabel {
  pub fn setOpenExternalLinks_0<RetType, T: QLabel_setOpenExternalLinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpenExternalLinks_0(self);
    // return 1;
  }
}
pub trait QLabel_setOpenExternalLinks_0<RetType> {
  fn setOpenExternalLinks_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setOpenExternalLinks_0<(/*void*/)> for (bool) {
  fn setOpenExternalLinks_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel20setOpenExternalLinksEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextInteractionFlags(Qt::TextInteractionFlags)

/*

*/
impl /*struct*/ QLabel {
  pub fn setTextInteractionFlags_0<RetType, T: QLabel_setTextInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QLabel_setTextInteractionFlags_0<RetType> {
  fn setTextInteractionFlags_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setTextInteractionFlags_0<(/*void*/)> for (i32) {
  fn setTextInteractionFlags_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel23setTextInteractionFlagsE6QFlagsIN2Qt19TextInteractionFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextInteractionFlags textInteractionFlags() const

/*

*/
impl /*struct*/ QLabel {
  pub fn textInteractionFlags_0<RetType, T: QLabel_textInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QLabel_textInteractionFlags_0<RetType> {
  fn textInteractionFlags_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_textInteractionFlags_0<i32> for () {
  fn textInteractionFlags_0(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel20textInteractionFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelection(int, int)

/*
Selects text from position start and for length characters.

Note: The textInteractionFlags set on the label need to include either TextSelectableByMouse or TextSelectableByKeyboard.

This function was introduced in  Qt 4.7.

See also selectedText().
*/
impl /*struct*/ QLabel {
  pub fn setSelection_0<RetType, T: QLabel_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QLabel_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setSelection_0<(/*void*/)> for (i32,i32) {
  fn setSelection_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel12setSelectionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasSelectedText() const

/*

*/
impl /*struct*/ QLabel {
  pub fn hasSelectedText_0<RetType, T: QLabel_hasSelectedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasSelectedText_0(self);
    // return 1;
  }
}
pub trait QLabel_hasSelectedText_0<RetType> {
  fn hasSelectedText_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_hasSelectedText_0<bool> for () {
  fn hasSelectedText_0(self , rsthis: & QLabel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel15hasSelectedTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:116
// index:0
// Public Visibility=Default Availability=Available
// [8] QString selectedText() const

/*

*/
impl /*struct*/ QLabel {
  pub fn selectedText_0<RetType, T: QLabel_selectedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedText_0(self);
    // return 1;
  }
}
pub trait QLabel_selectedText_0<RetType> {
  fn selectedText_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_selectedText_0<usize> for () {
  fn selectedText_0(self , rsthis: & QLabel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel12selectedTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:117
// index:0
// Public Visibility=Default Availability=Available
// [4] int selectionStart() const

/*
selectionStart() returns the index of the first selected character in the label or -1 if no text is selected.

Note: The textInteractionFlags set on the label need to include either TextSelectableByMouse or TextSelectableByKeyboard.

This function was introduced in  Qt 4.7.

See also selectedText().
*/
impl /*struct*/ QLabel {
  pub fn selectionStart_0<RetType, T: QLabel_selectionStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionStart_0(self);
    // return 1;
  }
}
pub trait QLabel_selectionStart_0<RetType> {
  fn selectionStart_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_selectionStart_0<i32> for () {
  fn selectionStart_0(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLabel14selectionStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QLabel {
  pub fn setText_0<RetType, T: QLabel_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QLabel_setText_0<RetType> {
  fn setText_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixmap(const QPixmap &)

/*

*/
impl /*struct*/ QLabel {
  pub fn setPixmap_0<RetType, T: QLabel_setPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixmap_0(self);
    // return 1;
  }
}
pub trait QLabel_setPixmap_0<RetType> {
  fn setPixmap_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setPixmap_0<(/*void*/)> for (usize) {
  fn setPixmap_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel9setPixmapERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPicture(const QPicture &)

/*
Sets the label contents to picture. Any previous content is cleared.

The buddy shortcut, if any, is disabled.

See also picture() and setBuddy().
*/
impl /*struct*/ QLabel {
  pub fn setPicture_0<RetType, T: QLabel_setPicture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPicture_0(self);
    // return 1;
  }
}
pub trait QLabel_setPicture_0<RetType> {
  fn setPicture_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setPicture_0<(/*void*/)> for (usize) {
  fn setPicture_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel10setPictureERK8QPicture", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMovie(QMovie *)

/*
Sets the label contents to movie. Any previous content is cleared. The label does NOT take ownership of the movie.

The buddy shortcut, if any, is disabled.

See also movie() and setBuddy().
*/
impl /*struct*/ QLabel {
  pub fn setMovie_0<RetType, T: QLabel_setMovie_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMovie_0(self);
    // return 1;
  }
}
pub trait QLabel_setMovie_0<RetType> {
  fn setMovie_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setMovie_0<(/*void*/)> for (usize) {
  fn setMovie_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel8setMovieEP6QMovie", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNum(int)

/*
Sets the label contents to plain text containing the textual representation of integer num. Any previous content is cleared. Does nothing if the integer's string representation is the same as the current contents of the label.

The buddy shortcut, if any, is disabled.

See also setText(), QString::setNum(), and setBuddy().
*/
impl /*struct*/ QLabel {
  pub fn setNum_0<RetType, T: QLabel_setNum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_0(self);
    // return 1;
  }
}
pub trait QLabel_setNum_0<RetType> {
  fn setNum_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setNum_0<(/*void*/)> for (i32) {
  fn setNum_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel6setNumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:129
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setNum(double)

/*
Sets the label contents to plain text containing the textual representation of integer num. Any previous content is cleared. Does nothing if the integer's string representation is the same as the current contents of the label.

The buddy shortcut, if any, is disabled.

See also setText(), QString::setNum(), and setBuddy().
*/
impl /*struct*/ QLabel {
  pub fn setNum_1<RetType, T: QLabel_setNum_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_1(self);
    // return 1;
  }
}
pub trait QLabel_setNum_1<RetType> {
  fn setNum_1(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_setNum_1<(/*void*/)> for (f64) {
  fn setNum_1(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel6setNumEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears any label contents.
*/
impl /*struct*/ QLabel {
  pub fn clear_0<RetType, T: QLabel_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QLabel_clear_0<RetType> {
  fn clear_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QLabel5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void linkActivated(const QString &)

/*
This signal is emitted when the user clicks a link. The URL referred to by the anchor is passed in link.

This function was introduced in  Qt 4.2.

See also linkHovered().
*/
impl /*struct*/ QLabel {
  pub fn linkActivated_0<RetType, T: QLabel_linkActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.linkActivated_0(self);
    // return 1;
  }
}
pub trait QLabel_linkActivated_0<RetType> {
  fn linkActivated_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_linkActivated_0<(/*void*/)> for (usize) {
  fn linkActivated_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel13linkActivatedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void linkHovered(const QString &)

/*
This signal is emitted when the user hovers over a link. The URL referred to by the anchor is passed in link.

This function was introduced in  Qt 4.2.

See also linkActivated().
*/
impl /*struct*/ QLabel {
  pub fn linkHovered_0<RetType, T: QLabel_linkHovered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.linkHovered_0(self);
    // return 1;
  }
}
pub trait QLabel_linkHovered_0<RetType> {
  fn linkHovered_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_linkHovered_0<(/*void*/)> for (usize) {
  fn linkHovered_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel11linkHoveredERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:137
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QLabel {
  pub fn event_0<RetType, T: QLabel_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QLabel_event_0<RetType> {
  fn event_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QLabel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QLabel5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:138
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QLabel {
  pub fn keyPressEvent_0<RetType, T: QLabel_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:139
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QLabel {
  pub fn paintEvent_0<RetType, T: QLabel_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:140
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QLabel {
  pub fn changeEvent_0<RetType, T: QLabel_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:141
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QLabel {
  pub fn mousePressEvent_0<RetType, T: QLabel_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:142
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QLabel {
  pub fn mouseMoveEvent_0<RetType, T: QLabel_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:143
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QLabel {
  pub fn mouseReleaseEvent_0<RetType, T: QLabel_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:145
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().
*/
impl /*struct*/ QLabel {
  pub fn contextMenuEvent_0<RetType, T: QLabel_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:147
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QLabel {
  pub fn focusInEvent_0<RetType, T: QLabel_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:148
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QLabel {
  pub fn focusOutEvent_0<RetType, T: QLabel_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QLabel_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QLabel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLabel13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlabel.h:149
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QLabel {
  pub fn focusNextPrevChild_0<RetType, T: QLabel_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QLabel_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QLabel) -> RetType;
}
impl<'a> /*trait*/ QLabel_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QLabel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QLabel18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
