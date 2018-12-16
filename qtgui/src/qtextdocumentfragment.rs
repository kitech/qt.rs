

// mod ::gui::QTextDocumentFragment
// package qtgui
// /usr/include/qt/QtGui/qtextdocumentfragment.h
// #include <qtextdocumentfragment.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
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
#[derive(Default)] // class sizeof(QTextDocumentFragment)=8
pub struct QTextDocumentFragment {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextDocumentFragment_ITF interface {
//    QTextDocumentFragment_PTR() *QTextDocumentFragment
//}
//func (ptr *QTextDocumentFragment) QTextDocumentFragment_PTR() *QTextDocumentFragment { return ptr }

impl /*struct*/ QTextDocumentFragment {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextDocumentFragment {
    return QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextDocumentFragment {
//  type Target = QTextDocumentFragmentBASE;
//
//  fn deref(&self) -> &QTextDocumentFragmentBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextDocumentFragmentBASE> for QTextDocumentFragment {
//  fn as_ref(& self) -> & QTextDocumentFragmentBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextdocumentfragment.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextDocumentFragment()

/*
Constructs an empty QTextDocumentFragment.

See also isEmpty().
*/
// QTextDocumentFragment() ctx.fn_proto_cpp
impl /*struct*/ QTextDocumentFragment {
  pub fn QTextDocumentFragment_0<T: QTextDocumentFragment_QTextDocumentFragment_0>(value: T) -> QTextDocumentFragment {
    let rsthis = value.QTextDocumentFragment_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentFragment_QTextDocumentFragment_0 {
  fn QTextDocumentFragment_0(self) -> QTextDocumentFragment;
}
// QTextDocumentFragment() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDocumentFragment_QTextDocumentFragment_0 for () {
  fn QTextDocumentFragment_0(self) -> QTextDocumentFragment {
    // unsafe{_ZN21QTextDocumentFragmentC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QTextDocumentFragmentC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:58
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextDocumentFragment(const QTextDocument *)

/*
Constructs an empty QTextDocumentFragment.

See also isEmpty().
*/
// QTextDocumentFragment(const QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QTextDocumentFragment {
  pub fn QTextDocumentFragment_1<T: QTextDocumentFragment_QTextDocumentFragment_1>(value: T) -> QTextDocumentFragment {
    let rsthis = value.QTextDocumentFragment_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentFragment_QTextDocumentFragment_1 {
  fn QTextDocumentFragment_1(self) -> QTextDocumentFragment;
}
// QTextDocumentFragment(const QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDocumentFragment_QTextDocumentFragment_1 for (usize) {
  fn QTextDocumentFragment_1(self) -> QTextDocumentFragment {
    // unsafe{_ZN21QTextDocumentFragmentC2EPK13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QTextDocumentFragmentC2EPK13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:59
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTextDocumentFragment(const QTextCursor &)

/*
Constructs an empty QTextDocumentFragment.

See also isEmpty().
*/
// QTextDocumentFragment(const QTextCursor &) ctx.fn_proto_cpp
impl /*struct*/ QTextDocumentFragment {
  pub fn QTextDocumentFragment_2<T: QTextDocumentFragment_QTextDocumentFragment_2>(value: T) -> QTextDocumentFragment {
    let rsthis = value.QTextDocumentFragment_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentFragment_QTextDocumentFragment_2 {
  fn QTextDocumentFragment_2(self) -> QTextDocumentFragment;
}
// QTextDocumentFragment(const QTextCursor &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDocumentFragment_QTextDocumentFragment_2 for (usize) {
  fn QTextDocumentFragment_2(self) -> QTextDocumentFragment {
    // unsafe{_ZN21QTextDocumentFragmentC2ERK11QTextCursor()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QTextDocumentFragmentC2ERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:61
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocumentFragment & operator=(const QTextDocumentFragment &)

/*

*/
impl /*struct*/ QTextDocumentFragment {
  pub fn operator_equal_0<RetType, T: QTextDocumentFragment_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTextDocumentFragment_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTextDocumentFragment) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentFragment_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTextDocumentFragment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QTextDocumentFragmentaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextDocumentFragment()

/*

*/
pub fn DeleteQTextDocumentFragment(this :*mut QTextDocumentFragment) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QTextDocumentFragmentD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextdocumentfragment.h:64
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the fragment is empty; otherwise returns false.
*/
impl /*struct*/ QTextDocumentFragment {
  pub fn isEmpty_0<RetType, T: QTextDocumentFragment_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QTextDocumentFragment_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QTextDocumentFragment) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentFragment_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QTextDocumentFragment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QTextDocumentFragment7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toPlainText() const

/*
Returns the document fragment's text as plain text (i.e. with no formatting information).

See also toHtml().
*/
impl /*struct*/ QTextDocumentFragment {
  pub fn toPlainText_0<RetType, T: QTextDocumentFragment_toPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPlainText_0(self);
    // return 1;
  }
}
pub trait QTextDocumentFragment_toPlainText_0<RetType> {
  fn toPlainText_0(self , rsthis: & QTextDocumentFragment) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentFragment_toPlainText_0<usize> for () {
  fn toPlainText_0(self , rsthis: & QTextDocumentFragment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QTextDocumentFragment11toPlainTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toHtml(const QByteArray &) const

/*
Returns the contents of the document fragment as HTML, using the specified encoding (e.g., "UTF-8", "ISO 8859-1").

This function was introduced in  Qt 4.2.

See also toPlainText(), QTextDocument::toHtml(), and QTextCodec.
*/
impl /*struct*/ QTextDocumentFragment {
  pub fn toHtml_0<RetType, T: QTextDocumentFragment_toHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHtml_0(self);
    // return 1;
  }
}
pub trait QTextDocumentFragment_toHtml_0<RetType> {
  fn toHtml_0(self , rsthis: & QTextDocumentFragment) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentFragment_toHtml_0<usize> for (usize) {
  fn toHtml_0(self , rsthis: & QTextDocumentFragment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QTextDocumentFragment6toHtmlERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:71
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTextDocumentFragment fromPlainText(const QString &)

/*
Returns a document fragment that contains the given plainText.

When inserting such a fragment into a QTextDocument the current char format of the QTextCursor used for insertion is used as format for the text.
*/
impl /*struct*/ QTextDocumentFragment {
  pub fn fromPlainText_0<RetType, T: QTextDocumentFragment_fromPlainText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromPlainText_0();
    // return 1;
  }
}
pub trait QTextDocumentFragment_fromPlainText_0<RetType> {
  fn fromPlainText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentFragment_fromPlainText_0<usize> for (usize) {
  fn fromPlainText_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QTextDocumentFragment13fromPlainTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:73
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTextDocumentFragment fromHtml(const QString &)

/*
Returns a QTextDocumentFragment based on the arbitrary piece of HTML in the given text. The formatting is preserved as much as possible; for example, "<b>bold</b>" will become a document fragment with the text "bold" with a bold character format.
*/
impl /*struct*/ QTextDocumentFragment {
  pub fn fromHtml_0<RetType, T: QTextDocumentFragment_fromHtml_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHtml_0();
    // return 1;
  }
}
pub trait QTextDocumentFragment_fromHtml_0<RetType> {
  fn fromHtml_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentFragment_fromHtml_0<usize> for (usize) {
  fn fromHtml_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QTextDocumentFragment8fromHtmlERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentfragment.h:74
// index:1
// Public static Visibility=Default Availability=Available
// [8] QTextDocumentFragment fromHtml(const QString &, const QTextDocument *)

/*
Returns a QTextDocumentFragment based on the arbitrary piece of HTML in the given text. The formatting is preserved as much as possible; for example, "<b>bold</b>" will become a document fragment with the text "bold" with a bold character format.
*/
impl /*struct*/ QTextDocumentFragment {
  pub fn fromHtml_1<RetType, T: QTextDocumentFragment_fromHtml_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHtml_1();
    // return 1;
  }
}
pub trait QTextDocumentFragment_fromHtml_1<RetType> {
  fn fromHtml_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentFragment_fromHtml_1<usize> for (usize,usize) {
  fn fromHtml_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QTextDocumentFragment8fromHtmlERK7QStringPK13QTextDocument", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
