

// mod ::gui::QSyntaxHighlighter
// package qtgui
// /usr/include/qt/QtGui/qsyntaxhighlighter.h
// #include <qsyntaxhighlighter.h>
// #include <QtGui>

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
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void highlightBlock(const QString &)
// func (this *QSyntaxHighlighter) InheritHighlightBlock(f func(text string) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "highlightBlock", f)
// }

// void setFormat(int, int, const QTextCharFormat &)
// func (this *QSyntaxHighlighter) InheritSetFormat(f func(start int, count int, format *QTextCharFormat) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setFormat", f)
// }

// QTextCharFormat format(int)
// func (this *QSyntaxHighlighter) InheritFormat(f func(pos int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "format", f)
// }

// int previousBlockState()
// func (this *QSyntaxHighlighter) InheritPreviousBlockState(f func() int) {
//  qtrt.SetAllInheritCallback(this, "previousBlockState", f)
// }

// int currentBlockState()
// func (this *QSyntaxHighlighter) InheritCurrentBlockState(f func() int) {
//  qtrt.SetAllInheritCallback(this, "currentBlockState", f)
// }

// void setCurrentBlockState(int)
// func (this *QSyntaxHighlighter) InheritSetCurrentBlockState(f func(newState int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setCurrentBlockState", f)
// }

// void setCurrentBlockUserData(QTextBlockUserData *)
// func (this *QSyntaxHighlighter) InheritSetCurrentBlockUserData(f func(data *QTextBlockUserData/*777 QTextBlockUserData **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setCurrentBlockUserData", f)
// }

// QTextBlockUserData * currentBlockUserData()
// func (this *QSyntaxHighlighter) InheritCurrentBlockUserData(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "currentBlockUserData", f)
// }

// QTextBlock currentBlock()
// func (this *QSyntaxHighlighter) InheritCurrentBlock(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "currentBlock", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSyntaxHighlighter)=16
pub struct QSyntaxHighlighter {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSyntaxHighlighter_ITF interface {
//    qtcore.QObject_ITF
//    QSyntaxHighlighter_PTR() *QSyntaxHighlighter
//}
//func (ptr *QSyntaxHighlighter) QSyntaxHighlighter_PTR() *QSyntaxHighlighter { return ptr }

impl /*struct*/ QSyntaxHighlighter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSyntaxHighlighter {
    return QSyntaxHighlighter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSyntaxHighlighter {
//  type Target = QSyntaxHighlighterBASE;
//
//  fn deref(&self) -> &QSyntaxHighlighterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSyntaxHighlighterBASE> for QSyntaxHighlighter {
//  fn as_ref(& self) -> & QSyntaxHighlighterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qsyntaxhighlighter.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn metaObject_0<RetType, T: QSyntaxHighlighter_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSyntaxHighlighter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QSyntaxHighlighter10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSyntaxHighlighter(QObject *)

/*
Constructs a QSyntaxHighlighter with the given parent.

If the parent is a QTextEdit, it installs the syntax highlighter on the parents document. The specified QTextEdit also becomes the owner of the QSyntaxHighlighter.
*/
// QSyntaxHighlighter(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSyntaxHighlighter {
  pub fn QSyntaxHighlighter_0<T: QSyntaxHighlighter_QSyntaxHighlighter_0>(value: T) -> QSyntaxHighlighter {
    let rsthis = value.QSyntaxHighlighter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSyntaxHighlighter_QSyntaxHighlighter_0 {
  fn QSyntaxHighlighter_0(self) -> QSyntaxHighlighter;
}
// QSyntaxHighlighter(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSyntaxHighlighter_QSyntaxHighlighter_0 for (usize) {
  fn QSyntaxHighlighter_0(self) -> QSyntaxHighlighter {
    // unsafe{_ZN18QSyntaxHighlighterC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighterC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSyntaxHighlighter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:66
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSyntaxHighlighter(QTextDocument *)

/*
Constructs a QSyntaxHighlighter with the given parent.

If the parent is a QTextEdit, it installs the syntax highlighter on the parents document. The specified QTextEdit also becomes the owner of the QSyntaxHighlighter.
*/
// QSyntaxHighlighter(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QSyntaxHighlighter {
  pub fn QSyntaxHighlighter_1<T: QSyntaxHighlighter_QSyntaxHighlighter_1>(value: T) -> QSyntaxHighlighter {
    let rsthis = value.QSyntaxHighlighter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSyntaxHighlighter_QSyntaxHighlighter_1 {
  fn QSyntaxHighlighter_1(self) -> QSyntaxHighlighter;
}
// QSyntaxHighlighter(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSyntaxHighlighter_QSyntaxHighlighter_1 for (usize) {
  fn QSyntaxHighlighter_1(self) -> QSyntaxHighlighter {
    // unsafe{_ZN18QSyntaxHighlighterC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighterC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSyntaxHighlighter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSyntaxHighlighter()

/*

*/
pub fn DeleteQSyntaxHighlighter(this :*mut QSyntaxHighlighter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qsyntaxhighlighter.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocument(QTextDocument *)

/*
Installs the syntax highlighter on the given QTextDocument doc. A QSyntaxHighlighter can only be used with one document at a time.

See also document().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn setDocument_0<RetType, T: QSyntaxHighlighter_setDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocument_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_setDocument_0<RetType> {
  fn setDocument_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_setDocument_0<(/*void*/)> for (usize) {
  fn setDocument_0(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocument * document() const

/*
Returns the QTextDocument on which this syntax highlighter is installed.

See also setDocument().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn document_0<RetType, T: QSyntaxHighlighter_document_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.document_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_document_0<RetType> {
  fn document_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_document_0<usize> for () {
  fn document_0(self , rsthis: & QSyntaxHighlighter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QSyntaxHighlighter8documentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rehighlight()

/*
Reapplies the highlighting to the whole document.

This function was introduced in  Qt 4.2.

See also rehighlightBlock().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn rehighlight_0<RetType, T: QSyntaxHighlighter_rehighlight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rehighlight_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_rehighlight_0<RetType> {
  fn rehighlight_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_rehighlight_0<(/*void*/)> for () {
  fn rehighlight_0(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter11rehighlightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rehighlightBlock(const QTextBlock &)

/*
Reapplies the highlighting to the given QTextBlock block.

This function was introduced in  Qt 4.6.

See also rehighlight().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn rehighlightBlock_0<RetType, T: QSyntaxHighlighter_rehighlightBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rehighlightBlock_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_rehighlightBlock_0<RetType> {
  fn rehighlightBlock_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_rehighlightBlock_0<(/*void*/)> for (usize) {
  fn rehighlightBlock_0(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:77
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [-2] void highlightBlock(const QString &)

/*
Highlights the given text block. This function is called when necessary by the rich text engine, i.e. on text blocks which have changed.

To provide your own syntax highlighting, you must subclass QSyntaxHighlighter and reimplement highlightBlock(). In your reimplementation you should parse the block's text and call setFormat() as often as necessary to apply any font and color changes that you require. For example:


  void MyHighlighter::highlightBlock(const QString &text)
  {
      QTextCharFormat myClassFormat;
      myClassFormat.setFontWeight(QFont::Bold);
      myClassFormat.setForeground(Qt::darkMagenta);

      QRegularExpression expression("\\bMy[A-Za-z]+\\b");
      QRegularExpressionMatchIterator i = expression.globalMatch(text);
      while (i.hasNext())
      {
        QRegularExpressionMatch match = i.next();
        setFormat(match.capturedStart(), match.capturedLength(), myClassFormat);
      }
  }



See the Detailed Description for examples of using setCurrentBlockState(), currentBlockState() and previousBlockState() to handle syntaxes with constructs that span several text blocks

See also previousBlockState(), setFormat(), and setCurrentBlockState().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn highlightBlock_0<RetType, T: QSyntaxHighlighter_highlightBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlightBlock_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_highlightBlock_0<RetType> {
  fn highlightBlock_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_highlightBlock_0<(/*void*/)> for (usize) {
  fn highlightBlock_0(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter14highlightBlockERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:79
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setFormat(int, int, const QTextCharFormat &)

/*
This function is applied to the syntax highlighter's current text block (i.e. the text that is passed to the highlightBlock() function).

The specified format is applied to the text from the start position for a length of count characters (if count is 0, nothing is done). The formatting properties set in format are merged at display time with the formatting information stored directly in the document, for example as previously set with QTextCursor's functions. Note that the document itself remains unmodified by the format set through this function.

See also format() and highlightBlock().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn setFormat_0<RetType, T: QSyntaxHighlighter_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_setFormat_0<(/*void*/)> for (i32,i32,usize) {
  fn setFormat_0(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter9setFormatEiiRK15QTextCharFormat", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:80
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void setFormat(int, int, const QColor &)

/*
This function is applied to the syntax highlighter's current text block (i.e. the text that is passed to the highlightBlock() function).

The specified format is applied to the text from the start position for a length of count characters (if count is 0, nothing is done). The formatting properties set in format are merged at display time with the formatting information stored directly in the document, for example as previously set with QTextCursor's functions. Note that the document itself remains unmodified by the format set through this function.

See also format() and highlightBlock().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn setFormat_1<RetType, T: QSyntaxHighlighter_setFormat_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_1(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_setFormat_1<RetType> {
  fn setFormat_1(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_setFormat_1<(/*void*/)> for (i32,i32,usize) {
  fn setFormat_1(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter9setFormatEiiRK6QColor", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:81
// index:2
// Protected Visibility=Default Availability=Available
// [-2] void setFormat(int, int, const QFont &)

/*
This function is applied to the syntax highlighter's current text block (i.e. the text that is passed to the highlightBlock() function).

The specified format is applied to the text from the start position for a length of count characters (if count is 0, nothing is done). The formatting properties set in format are merged at display time with the formatting information stored directly in the document, for example as previously set with QTextCursor's functions. Note that the document itself remains unmodified by the format set through this function.

See also format() and highlightBlock().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn setFormat_2<RetType, T: QSyntaxHighlighter_setFormat_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_2(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_setFormat_2<RetType> {
  fn setFormat_2(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_setFormat_2<(/*void*/)> for (i32,i32,usize) {
  fn setFormat_2(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter9setFormatEiiRK5QFont", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:82
// index:0
// Protected Visibility=Default Availability=Available
// [16] QTextCharFormat format(int) const

/*
Returns the format at position inside the syntax highlighter's current text block.

See also setFormat().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn format_0<RetType, T: QSyntaxHighlighter_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_format_0<RetType> {
  fn format_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_format_0<usize> for (i32) {
  fn format_0(self , rsthis: & QSyntaxHighlighter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QSyntaxHighlighter6formatEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:84
// index:0
// Protected Visibility=Default Availability=Available
// [4] int previousBlockState() const

/*
Returns the end state of the text block previous to the syntax highlighter's current block. If no value was previously set, the returned value is -1.

See also highlightBlock() and setCurrentBlockState().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn previousBlockState_0<RetType, T: QSyntaxHighlighter_previousBlockState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.previousBlockState_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_previousBlockState_0<RetType> {
  fn previousBlockState_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_previousBlockState_0<i32> for () {
  fn previousBlockState_0(self , rsthis: & QSyntaxHighlighter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QSyntaxHighlighter18previousBlockStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:85
// index:0
// Protected Visibility=Default Availability=Available
// [4] int currentBlockState() const

/*
Returns the state of the current text block. If no value is set, the returned value is -1.

See also setCurrentBlockState().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn currentBlockState_0<RetType, T: QSyntaxHighlighter_currentBlockState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentBlockState_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_currentBlockState_0<RetType> {
  fn currentBlockState_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_currentBlockState_0<i32> for () {
  fn currentBlockState_0(self , rsthis: & QSyntaxHighlighter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QSyntaxHighlighter17currentBlockStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:86
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setCurrentBlockState(int)

/*
Sets the state of the current text block to newState.

See also currentBlockState() and highlightBlock().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn setCurrentBlockState_0<RetType, T: QSyntaxHighlighter_setCurrentBlockState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentBlockState_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_setCurrentBlockState_0<RetType> {
  fn setCurrentBlockState_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_setCurrentBlockState_0<(/*void*/)> for (i32) {
  fn setCurrentBlockState_0(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter20setCurrentBlockStateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:88
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setCurrentBlockUserData(QTextBlockUserData *)

/*
Attaches the given data to the current text block. The ownership is passed to the underlying text document, i.e. the provided QTextBlockUserData object will be deleted if the corresponding text block gets deleted.

QTextBlockUserData can be used to store custom settings. In the case of syntax highlighting, it is in particular interesting as cache storage for information that you may figure out while parsing the paragraph's text.

For example while parsing the text, you can keep track of parenthesis characters that you encounter ('{[(' and the like), and store their relative position and the actual QChar in a simple class derived from QTextBlockUserData:


  struct ParenthesisInfo
  {
      QChar char;
      int position;
  };

  struct BlockData : public QTextBlockUserData
  {
      QVector<ParenthesisInfo> parentheses;
  };



During cursor navigation in the associated editor, you can ask the current QTextBlock (retrieved using the QTextCursor::block() function) if it has a user data object set and cast it to your BlockData object. Then you can check if the current cursor position matches with a previously recorded parenthesis position, and, depending on the type of parenthesis (opening or closing), find the next opening or closing parenthesis on the same level.

In this way you can do a visual parenthesis matching and highlight from the current cursor position to the matching parenthesis. That makes it easier to spot a missing parenthesis in your code and to find where a corresponding opening/closing parenthesis is when editing parenthesis intensive code.

See also currentBlockUserData() and QTextBlock::setUserData().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn setCurrentBlockUserData_0<RetType, T: QSyntaxHighlighter_setCurrentBlockUserData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentBlockUserData_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_setCurrentBlockUserData_0<RetType> {
  fn setCurrentBlockUserData_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_setCurrentBlockUserData_0<(/*void*/)> for (usize) {
  fn setCurrentBlockUserData_0(self , rsthis: & QSyntaxHighlighter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QSyntaxHighlighter23setCurrentBlockUserDataEP18QTextBlockUserData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:89
// index:0
// Protected Visibility=Default Availability=Available
// [8] QTextBlockUserData * currentBlockUserData() const

/*
Returns the QTextBlockUserData object previously attached to the current text block.

See also QTextBlock::userData() and setCurrentBlockUserData().
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn currentBlockUserData_0<RetType, T: QSyntaxHighlighter_currentBlockUserData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentBlockUserData_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_currentBlockUserData_0<RetType> {
  fn currentBlockUserData_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_currentBlockUserData_0<usize> for () {
  fn currentBlockUserData_0(self , rsthis: & QSyntaxHighlighter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QSyntaxHighlighter20currentBlockUserDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsyntaxhighlighter.h:91
// index:0
// Protected Visibility=Default Availability=Available
// [16] QTextBlock currentBlock() const

/*
Returns the current text block.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QSyntaxHighlighter {
  pub fn currentBlock_0<RetType, T: QSyntaxHighlighter_currentBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentBlock_0(self);
    // return 1;
  }
}
pub trait QSyntaxHighlighter_currentBlock_0<RetType> {
  fn currentBlock_0(self , rsthis: & QSyntaxHighlighter) -> RetType;
}
impl<'a> /*trait*/ QSyntaxHighlighter_currentBlock_0<usize> for () {
  fn currentBlock_0(self , rsthis: & QSyntaxHighlighter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QSyntaxHighlighter12currentBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
