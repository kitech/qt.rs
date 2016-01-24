// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtGui/qtextdocumentfragment.h
// dst-file: /src/gui/qtextdocumentfragment.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qtextdocument::QTextDocument; // 773
use super::super::core::qbytearray::QByteArray; // 771
use super::qtextcursor::QTextCursor; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextDocumentFragment_Class_Size() -> c_int;
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html, const QTextDocument * resourceProvider);
  fn C_ZN21QTextDocumentFragment8fromHtmlERK7QStringPK13QTextDocument(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocumentFragment & rhs);
  fn C_ZN21QTextDocumentFragmentC2ERKS_(arg0: *mut c_void) -> u64;
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromPlainText(const QString & plainText);
  fn C_ZN21QTextDocumentFragment13fromPlainTextERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QTextDocumentFragment::toHtml(const QByteArray & encoding);
  fn C_ZNK21QTextDocumentFragment6toHtmlERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::~QTextDocumentFragment();
  fn C_ZN21QTextDocumentFragmentD2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html);
  fn C_ZN21QTextDocumentFragment8fromHtmlERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::QTextDocumentFragment();
  fn C_ZN21QTextDocumentFragmentC2Ev() -> u64;
  // proto:  QString QTextDocumentFragment::toPlainText();
  fn C_ZNK21QTextDocumentFragment11toPlainTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextCursor & range);
  fn C_ZN21QTextDocumentFragmentC2ERK11QTextCursor(arg0: *mut c_void) -> u64;
  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocument * document);
  fn C_ZN21QTextDocumentFragmentC2EPK13QTextDocument(arg0: *mut c_void) -> u64;
  // proto:  bool QTextDocumentFragment::isEmpty();
  fn C_ZNK21QTextDocumentFragment7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QTextDocumentFragment)=8
#[derive(Default)]
pub struct QTextDocumentFragment {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextDocumentFragment {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextDocumentFragment {
    return QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html, const QTextDocument * resourceProvider);
impl /*struct*/ QTextDocumentFragment {
  pub fn fromHtml_s<RetType, T: QTextDocumentFragment_fromHtml_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHtml_s();
    // return 1;
  }
}

pub trait QTextDocumentFragment_fromHtml_s<RetType> {
  fn fromHtml_s(self ) -> RetType;
}

  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html, const QTextDocument * resourceProvider);
impl<'a> /*trait*/ QTextDocumentFragment_fromHtml_s<QTextDocumentFragment> for (&'a QString, &'a QTextDocument) {
  fn fromHtml_s(self ) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragment8fromHtmlERK7QStringPK13QTextDocument()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN21QTextDocumentFragment8fromHtmlERK7QStringPK13QTextDocument(arg0, arg1)};
    let mut ret1 = QTextDocumentFragment::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocumentFragment & rhs);
impl /*struct*/ QTextDocumentFragment {
  pub fn new<T: QTextDocumentFragment_new>(value: T) -> QTextDocumentFragment {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentFragment_new {
  fn new(self) -> QTextDocumentFragment;
}

  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocumentFragment & rhs);
impl<'a> /*trait*/ QTextDocumentFragment_new for (&'a QTextDocumentFragment) {
  fn new(self) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentC2ERKS_()};
    let ctysz: c_int = unsafe{QTextDocumentFragment_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QTextDocumentFragmentC2ERKS_(arg0)};
    let rsthis = QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QTextDocumentFragment QTextDocumentFragment::fromPlainText(const QString & plainText);
impl /*struct*/ QTextDocumentFragment {
  pub fn fromPlainText_s<RetType, T: QTextDocumentFragment_fromPlainText_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromPlainText_s();
    // return 1;
  }
}

pub trait QTextDocumentFragment_fromPlainText_s<RetType> {
  fn fromPlainText_s(self ) -> RetType;
}

  // proto: static QTextDocumentFragment QTextDocumentFragment::fromPlainText(const QString & plainText);
impl<'a> /*trait*/ QTextDocumentFragment_fromPlainText_s<QTextDocumentFragment> for (&'a QString) {
  fn fromPlainText_s(self ) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragment13fromPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN21QTextDocumentFragment13fromPlainTextERK7QString(arg0)};
    let mut ret1 = QTextDocumentFragment::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextDocumentFragment::toHtml(const QByteArray & encoding);
impl /*struct*/ QTextDocumentFragment {
  pub fn toHtml<RetType, T: QTextDocumentFragment_toHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toHtml(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_toHtml<RetType> {
  fn toHtml(self , rsthis: & QTextDocumentFragment) -> RetType;
}

  // proto:  QString QTextDocumentFragment::toHtml(const QByteArray & encoding);
impl<'a> /*trait*/ QTextDocumentFragment_toHtml<QString> for (&'a QByteArray) {
  fn toHtml(self , rsthis: & QTextDocumentFragment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment6toHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QTextDocumentFragment6toHtmlERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentFragment::~QTextDocumentFragment();
impl /*struct*/ QTextDocumentFragment {
  pub fn free<RetType, T: QTextDocumentFragment_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_free<RetType> {
  fn free(self , rsthis: & QTextDocumentFragment) -> RetType;
}

  // proto:  void QTextDocumentFragment::~QTextDocumentFragment();
impl<'a> /*trait*/ QTextDocumentFragment_free<()> for () {
  fn free(self , rsthis: & QTextDocumentFragment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentD2Ev()};
     unsafe {C_ZN21QTextDocumentFragmentD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html);
impl<'a> /*trait*/ QTextDocumentFragment_fromHtml_s<QTextDocumentFragment> for (&'a QString) {
  fn fromHtml_s(self ) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragment8fromHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN21QTextDocumentFragment8fromHtmlERK7QString(arg0)};
    let mut ret1 = QTextDocumentFragment::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentFragment::QTextDocumentFragment();
impl<'a> /*trait*/ QTextDocumentFragment_new for () {
  fn new(self) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentC2Ev()};
    let ctysz: c_int = unsafe{QTextDocumentFragment_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN21QTextDocumentFragmentC2Ev()};
    let rsthis = QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QTextDocumentFragment::toPlainText();
impl /*struct*/ QTextDocumentFragment {
  pub fn toPlainText<RetType, T: QTextDocumentFragment_toPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPlainText(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_toPlainText<RetType> {
  fn toPlainText(self , rsthis: & QTextDocumentFragment) -> RetType;
}

  // proto:  QString QTextDocumentFragment::toPlainText();
impl<'a> /*trait*/ QTextDocumentFragment_toPlainText<QString> for () {
  fn toPlainText(self , rsthis: & QTextDocumentFragment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment11toPlainTextEv()};
    let mut ret = unsafe {C_ZNK21QTextDocumentFragment11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextCursor & range);
impl<'a> /*trait*/ QTextDocumentFragment_new for (&'a QTextCursor) {
  fn new(self) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentC2ERK11QTextCursor()};
    let ctysz: c_int = unsafe{QTextDocumentFragment_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QTextDocumentFragmentC2ERK11QTextCursor(arg0)};
    let rsthis = QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocument * document);
impl<'a> /*trait*/ QTextDocumentFragment_new for (&'a QTextDocument) {
  fn new(self) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentC2EPK13QTextDocument()};
    let ctysz: c_int = unsafe{QTextDocumentFragment_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QTextDocumentFragmentC2EPK13QTextDocument(arg0)};
    let rsthis = QTextDocumentFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextDocumentFragment::isEmpty();
impl /*struct*/ QTextDocumentFragment {
  pub fn isEmpty<RetType, T: QTextDocumentFragment_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QTextDocumentFragment) -> RetType;
}

  // proto:  bool QTextDocumentFragment::isEmpty();
impl<'a> /*trait*/ QTextDocumentFragment_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QTextDocumentFragment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment7isEmptyEv()};
    let mut ret = unsafe {C_ZNK21QTextDocumentFragment7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

