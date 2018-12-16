

// mod ::gui::QStaticText
// package qtgui
// /usr/include/qt/QtGui/qstatictext.h
// #include <qstatictext.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 50
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QStaticText)=8
pub struct QStaticText {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStaticText_ITF interface {
//    QStaticText_PTR() *QStaticText
//}
//func (ptr *QStaticText) QStaticText_PTR() *QStaticText { return ptr }

impl /*struct*/ QStaticText {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStaticText {
    return QStaticText{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStaticText {
//  type Target = QStaticTextBASE;
//
//  fn deref(&self) -> &QStaticTextBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStaticTextBASE> for QStaticText {
//  fn as_ref(& self) -> & QStaticTextBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qstatictext.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStaticText()

/*
Constructs an empty QStaticText
*/
// QStaticText() ctx.fn_proto_cpp
impl /*struct*/ QStaticText {
  pub fn QStaticText_0<T: QStaticText_QStaticText_0>(value: T) -> QStaticText {
    let rsthis = value.QStaticText_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStaticText_QStaticText_0 {
  fn QStaticText_0(self) -> QStaticText;
}
// QStaticText() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStaticText_QStaticText_0 for () {
  fn QStaticText_0(self) -> QStaticText {
    // unsafe{_ZN11QStaticTextC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QStaticTextC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStaticText{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:65
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QStaticText(const QString &)

/*
Constructs an empty QStaticText
*/
// QStaticText(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QStaticText {
  pub fn QStaticText_1<T: QStaticText_QStaticText_1>(value: T) -> QStaticText {
    let rsthis = value.QStaticText_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStaticText_QStaticText_1 {
  fn QStaticText_1(self) -> QStaticText;
}
// QStaticText(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStaticText_QStaticText_1 for (usize) {
  fn QStaticText_1(self) -> QStaticText {
    // unsafe{_ZN11QStaticTextC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QStaticTextC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStaticText{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:68
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStaticText & operator=(QStaticText &&)

/*

*/
impl /*struct*/ QStaticText {
  pub fn operator_equal_0<RetType, T: QStaticText_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QStaticText_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QStaticText) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QStaticTextaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:70
// index:1
// Public Visibility=Default Availability=Available
// [8] QStaticText & operator=(const QStaticText &)

/*

*/
impl /*struct*/ QStaticText {
  pub fn operator_equal_1<RetType, T: QStaticText_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QStaticText_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QStaticText) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QStaticTextaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QStaticText()

/*

*/
pub fn DeleteQStaticText(this :*mut QStaticText) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QStaticTextD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qstatictext.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QStaticText &)

/*
Swaps this static text instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QStaticText {
  pub fn swap_0<RetType, T: QStaticText_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QStaticText_swap_0<RetType> {
  fn swap_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QStaticText) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QStaticText4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*
Sets the text of the QStaticText to text.

Note: This function will cause the layout of the text to require recalculation.

See also text().
*/
impl /*struct*/ QStaticText {
  pub fn setText_0<RetType, T: QStaticText_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QStaticText_setText_0<RetType> {
  fn setText_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QStaticText) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QStaticText7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*
Returns the text of the QStaticText.

See also setText().
*/
impl /*struct*/ QStaticText {
  pub fn text_0<RetType, T: QStaticText_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QStaticText_text_0<RetType> {
  fn text_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_text_0<usize> for () {
  fn text_0(self , rsthis: & QStaticText) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStaticText4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextFormat(Qt::TextFormat)

/*
Sets the text format of the QStaticText to textFormat. If textFormat is set to Qt::AutoText (the default), the format of the text will try to be determined using the function Qt::mightBeRichText(). If the text format is Qt::PlainText, then the text will be displayed as is, whereas it will be interpreted as HTML if the format is Qt::RichText. HTML tags that alter the font of the text, its color, or its layout are supported by QStaticText.

Note: This function will cause the layout of the text to require recalculation.

See also textFormat(), setText(), and text().
*/
impl /*struct*/ QStaticText {
  pub fn setTextFormat_0<RetType, T: QStaticText_setTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextFormat_0(self);
    // return 1;
  }
}
pub trait QStaticText_setTextFormat_0<RetType> {
  fn setTextFormat_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_setTextFormat_0<(/*void*/)> for (i32) {
  fn setTextFormat_0(self , rsthis: & QStaticText) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStaticText13setTextFormatEN2Qt10TextFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextFormat textFormat() const

/*
Returns the text format of the QStaticText.

See also setTextFormat(), setText(), and text().
*/
impl /*struct*/ QStaticText {
  pub fn textFormat_0<RetType, T: QStaticText_textFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textFormat_0(self);
    // return 1;
  }
}
pub trait QStaticText_textFormat_0<RetType> {
  fn textFormat_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_textFormat_0<i32> for () {
  fn textFormat_0(self , rsthis: & QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStaticText10textFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextWidth(qreal)

/*
Sets the preferred width for this QStaticText. If the text is wider than the specified width, it will be broken into multiple lines and grow vertically. If the text cannot be split into multiple lines, it will be larger than the specified textWidth.

Setting the preferred text width to a negative number will cause the text to be unbounded.

Use size() to get the actual size of the text.

Note: This function will cause the layout of the text to require recalculation.

See also textWidth() and size().
*/
impl /*struct*/ QStaticText {
  pub fn setTextWidth_0<RetType, T: QStaticText_setTextWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextWidth_0(self);
    // return 1;
  }
}
pub trait QStaticText_setTextWidth_0<RetType> {
  fn setTextWidth_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_setTextWidth_0<(/*void*/)> for (f64) {
  fn setTextWidth_0(self , rsthis: & QStaticText) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStaticText12setTextWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal textWidth() const

/*
Returns the preferred width for this QStaticText.

See also setTextWidth().
*/
impl /*struct*/ QStaticText {
  pub fn textWidth_0<RetType, T: QStaticText_textWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textWidth_0(self);
    // return 1;
  }
}
pub trait QStaticText_textWidth_0<RetType> {
  fn textWidth_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_textWidth_0<f64> for () {
  fn textWidth_0(self , rsthis: & QStaticText) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStaticText9textWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextOption(const QTextOption &)

/*
Sets the text option structure that controls the layout process to the given textOption.

See also textOption().
*/
impl /*struct*/ QStaticText {
  pub fn setTextOption_0<RetType, T: QStaticText_setTextOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextOption_0(self);
    // return 1;
  }
}
pub trait QStaticText_setTextOption_0<RetType> {
  fn setTextOption_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_setTextOption_0<(/*void*/)> for (usize) {
  fn setTextOption_0(self , rsthis: & QStaticText) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QStaticText13setTextOptionERK11QTextOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:85
// index:0
// Public Visibility=Default Availability=Available
// [32] QTextOption textOption() const

/*
Returns the current text option used to control the layout process.

See also setTextOption().
*/
impl /*struct*/ QStaticText {
  pub fn textOption_0<RetType, T: QStaticText_textOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textOption_0(self);
    // return 1;
  }
}
pub trait QStaticText_textOption_0<RetType> {
  fn textOption_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_textOption_0<usize> for () {
  fn textOption_0(self , rsthis: & QStaticText) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStaticText10textOptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:87
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF size() const

/*
Returns the size of the bounding rect for this QStaticText.

See also textWidth().
*/
impl /*struct*/ QStaticText {
  pub fn size_0<RetType, T: QStaticText_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QStaticText_size_0<RetType> {
  fn size_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_size_0<usize> for () {
  fn size_0(self , rsthis: & QStaticText) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStaticText4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void prepare(const QTransform &, const QFont &)

/*
Prepares the QStaticText object for being painted with the given matrix and the given font to avoid overhead when the actual drawStaticText() call is made.

When drawStaticText() is called, the layout of the QStaticText will be recalculated if any part of the QStaticText object has changed since the last time it was drawn. It will also be recalculated if the painter's font is not the same as when the QStaticText was last drawn, or, on any other paint engine than the OpenGL2 engine, if the painter's matrix has been altered since the static text was last drawn.

To avoid the overhead of creating the layout the first time you draw the QStaticText after making changes, you can use the prepare() function and pass in the matrix and font you expect to use when drawing the text.

See also QPainter::setFont() and QPainter::setMatrix().
*/
impl /*struct*/ QStaticText {
  pub fn prepare_0<RetType, T: QStaticText_prepare_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.prepare_0(self);
    // return 1;
  }
}
pub trait QStaticText_prepare_0<RetType> {
  fn prepare_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_prepare_0<(/*void*/)> for (usize,usize) {
  fn prepare_0(self , rsthis: & QStaticText) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QStaticText7prepareERK10QTransformRK5QFont", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPerformanceHint(QStaticText::PerformanceHint)

/*
Sets the performance hint of the QStaticText according to the performanceHint provided. The performanceHint is used to customize how much caching is done internally to improve performance.

The default is QStaticText::ModerateCaching.

Note: This function will cause the layout of the text to require recalculation.

See also performanceHint().
*/
impl /*struct*/ QStaticText {
  pub fn setPerformanceHint_0<RetType, T: QStaticText_setPerformanceHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPerformanceHint_0(self);
    // return 1;
  }
}
pub trait QStaticText_setPerformanceHint_0<RetType> {
  fn setPerformanceHint_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_setPerformanceHint_0<(/*void*/)> for (i32) {
  fn setPerformanceHint_0(self , rsthis: & QStaticText) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStaticText18setPerformanceHintENS_15PerformanceHintE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:92
// index:0
// Public Visibility=Default Availability=Available
// [4] QStaticText::PerformanceHint performanceHint() const

/*
Returns which performance hint is set for the QStaticText.

See also setPerformanceHint().
*/
impl /*struct*/ QStaticText {
  pub fn performanceHint_0<RetType, T: QStaticText_performanceHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.performanceHint_0(self);
    // return 1;
  }
}
pub trait QStaticText_performanceHint_0<RetType> {
  fn performanceHint_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_performanceHint_0<i32> for () {
  fn performanceHint_0(self , rsthis: & QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStaticText15performanceHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QStaticText &) const

/*

*/
impl /*struct*/ QStaticText {
  pub fn operator_equal_equal_0<RetType, T: QStaticText_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QStaticText_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QStaticText) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStaticTexteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstatictext.h:95
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QStaticText &) const

/*

*/
impl /*struct*/ QStaticText {
  pub fn operator_not_equal_0<RetType, T: QStaticText_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QStaticText_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QStaticText) -> RetType;
}
impl<'a> /*trait*/ QStaticText_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QStaticText) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStaticTextneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum the different performance hints that can be set on the QStaticText. These hints can be used to indicate that the QStaticText should use additional caches, if possible, to improve performance at the expense of memory. In particular, setting the performance hint AggressiveCaching on the QStaticText will improve performance when using the OpenGL graphics system or when drawing to a QOpenGLWidget.


*/
pub type QStaticText__PerformanceHint = i32;
// Do basic caching for high performance at a low memory cost.
pub const QStaticText__ModerateCaching :QStaticText__PerformanceHint = 0;
// Use additional caching when available. This may improve performance at a higher memory cost.
pub const QStaticText__AggressiveCaching :QStaticText__PerformanceHint = 1;
pub fn QStaticText_PerformanceHintItemName(val: i32) ->String {
  match val {
     QStaticText__ModerateCaching => // 0
     {return String::from("ModerateCaching");}
     QStaticText__AggressiveCaching => // 1
     {return String::from("AggressiveCaching");}
  _ => {return format!("{}", val);}
}
}
pub fn QStaticText_PerformanceHintItemName_s(val: i32) ->String {
  //var nilthis *QStaticText
  //return nilthis.PerformanceHintItemName(val);
  return QStaticText_PerformanceHintItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
