// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qtextdocument::QTextDocument;
use super::qbytearray::QByteArray;
use super::qtextcursor::QTextCursor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html, const QTextDocument * resourceProvider);
  fn _ZN21QTextDocumentFragment8fromHtmlERK7QStringPK13QTextDocument(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::NewQTextDocumentFragment(const QTextDocumentFragment & rhs);
  fn _ZN21QTextDocumentFragmentC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromPlainText(const QString & plainText);
  fn _ZN21QTextDocumentFragment13fromPlainTextERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QTextDocumentFragment::toHtml(const QByteArray & encoding);
  fn _ZNK21QTextDocumentFragment6toHtmlERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::FreeQTextDocumentFragment();
  fn _ZN21QTextDocumentFragmentD0Ev(qthis: *mut c_void) ;
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html);
  fn _ZN21QTextDocumentFragment8fromHtmlERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::NewQTextDocumentFragment();
  fn _ZN21QTextDocumentFragmentC1Ev(qthis: *mut c_void) ;
  // proto:  QString QTextDocumentFragment::toPlainText();
  fn _ZNK21QTextDocumentFragment11toPlainTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::NewQTextDocumentFragment(const QTextCursor & range);
  fn _ZN21QTextDocumentFragmentC1ERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextDocumentFragment::NewQTextDocumentFragment(const QTextDocument * document);
  fn _ZN21QTextDocumentFragmentC1EPK13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextDocumentFragment::isEmpty();
  fn _ZNK21QTextDocumentFragment7isEmptyEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QTextDocumentFragment)=8
pub struct QTextDocumentFragment {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextDocumentFragment {
  pub fn fromHtml<T: QTextDocumentFragment_fromHtml>(&mut self, value: T) -> QTextDocumentFragment {
    return value.fromHtml(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_fromHtml {
  fn fromHtml(self, rsthis: &mut QTextDocumentFragment) -> QTextDocumentFragment;
}

// proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html, const QTextDocument * resourceProvider);
impl<'a> /*trait*/ QTextDocumentFragment_fromHtml for (&'a  QString, &'a  QTextDocument) {
  fn fromHtml(self, rsthis: &mut QTextDocumentFragment) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragment8fromHtmlERK7QStringPK13QTextDocument()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QTextDocumentFragment8fromHtmlERK7QStringPK13QTextDocument(arg0, arg1)};
    let mut ret1 = QTextDocumentFragment{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentFragment {
  pub fn NewQTextDocumentFragment<T: QTextDocumentFragment_NewQTextDocumentFragment>(value: T) -> QTextDocumentFragment {
    let rsthis = value.NewQTextDocumentFragment();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentFragment_NewQTextDocumentFragment {
  fn NewQTextDocumentFragment(self) -> QTextDocumentFragment;
}

// proto: void QTextDocumentFragment::NewQTextDocumentFragment(const QTextDocumentFragment & rhs);
impl<'a> /*trait*/ QTextDocumentFragment_NewQTextDocumentFragment for (&'a  QTextDocumentFragment) {
  fn NewQTextDocumentFragment(self) -> QTextDocumentFragment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QTextDocumentFragmentC1ERKS_(qthis, arg0)};
    let rsthis = QTextDocumentFragment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentFragment {
  pub fn fromPlainText<T: QTextDocumentFragment_fromPlainText>(&mut self, value: T) -> QTextDocumentFragment {
    return value.fromPlainText(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_fromPlainText {
  fn fromPlainText(self, rsthis: &mut QTextDocumentFragment) -> QTextDocumentFragment;
}

// proto: static QTextDocumentFragment QTextDocumentFragment::fromPlainText(const QString & plainText);
impl<'a> /*trait*/ QTextDocumentFragment_fromPlainText for (&'a  QString) {
  fn fromPlainText(self, rsthis: &mut QTextDocumentFragment) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragment13fromPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QTextDocumentFragment13fromPlainTextERK7QString(arg0)};
    let mut ret1 = QTextDocumentFragment{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentFragment {
  pub fn toHtml<T: QTextDocumentFragment_toHtml>(&mut self, value: T) -> QString {
    return value.toHtml(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_toHtml {
  fn toHtml(self, rsthis: &mut QTextDocumentFragment) -> QString;
}

// proto:  QString QTextDocumentFragment::toHtml(const QByteArray & encoding);
impl<'a> /*trait*/ QTextDocumentFragment_toHtml for (&'a  QByteArray) {
  fn toHtml(self, rsthis: &mut QTextDocumentFragment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment6toHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QTextDocumentFragment6toHtmlERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentFragment {
  pub fn FreeQTextDocumentFragment<T: QTextDocumentFragment_FreeQTextDocumentFragment>(&mut self, value: T)  {
     value.FreeQTextDocumentFragment(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_FreeQTextDocumentFragment {
  fn FreeQTextDocumentFragment(self, rsthis: &mut QTextDocumentFragment) ;
}

// proto:  void QTextDocumentFragment::FreeQTextDocumentFragment();
impl<'a> /*trait*/ QTextDocumentFragment_FreeQTextDocumentFragment for () {
  fn FreeQTextDocumentFragment(self, rsthis: &mut QTextDocumentFragment)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentD0Ev()};
     unsafe {_ZN21QTextDocumentFragmentD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html);
impl<'a> /*trait*/ QTextDocumentFragment_fromHtml for (&'a  QString) {
  fn fromHtml(self, rsthis: &mut QTextDocumentFragment) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragment8fromHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QTextDocumentFragment8fromHtmlERK7QString(arg0)};
    let mut ret1 = QTextDocumentFragment{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextDocumentFragment::NewQTextDocumentFragment();
impl<'a> /*trait*/ QTextDocumentFragment_NewQTextDocumentFragment for () {
  fn NewQTextDocumentFragment(self) -> QTextDocumentFragment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentC1Ev()};
    unsafe {_ZN21QTextDocumentFragmentC1Ev(qthis)};
    let rsthis = QTextDocumentFragment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentFragment {
  pub fn toPlainText<T: QTextDocumentFragment_toPlainText>(&mut self, value: T) -> QString {
    return value.toPlainText(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_toPlainText {
  fn toPlainText(self, rsthis: &mut QTextDocumentFragment) -> QString;
}

// proto:  QString QTextDocumentFragment::toPlainText();
impl<'a> /*trait*/ QTextDocumentFragment_toPlainText for () {
  fn toPlainText(self, rsthis: &mut QTextDocumentFragment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment11toPlainTextEv()};
    let mut ret = unsafe {_ZNK21QTextDocumentFragment11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextDocumentFragment::NewQTextDocumentFragment(const QTextCursor & range);
impl<'a> /*trait*/ QTextDocumentFragment_NewQTextDocumentFragment for (&'a  QTextCursor) {
  fn NewQTextDocumentFragment(self) -> QTextDocumentFragment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentC1ERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QTextDocumentFragmentC1ERK11QTextCursor(qthis, arg0)};
    let rsthis = QTextDocumentFragment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTextDocumentFragment::NewQTextDocumentFragment(const QTextDocument * document);
impl<'a> /*trait*/ QTextDocumentFragment_NewQTextDocumentFragment for (&'a  QTextDocument) {
  fn NewQTextDocumentFragment(self) -> QTextDocumentFragment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentC1EPK13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QTextDocumentFragmentC1EPK13QTextDocument(qthis, arg0)};
    let rsthis = QTextDocumentFragment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentFragment {
  pub fn isEmpty<T: QTextDocumentFragment_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_isEmpty {
  fn isEmpty(self, rsthis: &mut QTextDocumentFragment) -> i8;
}

// proto:  bool QTextDocumentFragment::isEmpty();
impl<'a> /*trait*/ QTextDocumentFragment_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QTextDocumentFragment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment7isEmptyEv()};
    let mut ret = unsafe {_ZNK21QTextDocumentFragment7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

