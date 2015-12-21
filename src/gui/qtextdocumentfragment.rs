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
  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocumentFragment & rhs);
  fn _ZN21QTextDocumentFragmentC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromPlainText(const QString & plainText);
  fn _ZN21QTextDocumentFragment13fromPlainTextERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QTextDocumentFragment::toHtml(const QByteArray & encoding);
  fn _ZNK21QTextDocumentFragment6toHtmlERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::~QTextDocumentFragment();
  fn _ZN21QTextDocumentFragmentD0Ev(qthis: *mut c_void);
  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html);
  fn _ZN21QTextDocumentFragment8fromHtmlERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::QTextDocumentFragment();
  fn _ZN21QTextDocumentFragmentC1Ev(qthis: *mut c_void);
  // proto:  QString QTextDocumentFragment::toPlainText();
  fn _ZNK21QTextDocumentFragment11toPlainTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextCursor & range);
  fn _ZN21QTextDocumentFragmentC1ERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocument * document);
  fn _ZN21QTextDocumentFragmentC1EPK13QTextDocument(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextDocumentFragment::isEmpty();
  fn _ZNK21QTextDocumentFragment7isEmptyEv(qthis: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QTextDocumentFragment)=8
pub struct QTextDocumentFragment {
  pub qclsinst: *mut c_void,
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
impl<'a> /*trait*/ QTextDocumentFragment_fromHtml_s<QTextDocumentFragment> for (QString, QTextDocument) {
  fn fromHtml_s(self ) -> QTextDocumentFragment {
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

  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocumentFragment & rhs);
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

  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocumentFragment & rhs);
impl<'a> /*trait*/ QTextDocumentFragment_NewQTextDocumentFragment for (QTextDocumentFragment) {
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
impl<'a> /*trait*/ QTextDocumentFragment_fromPlainText_s<QTextDocumentFragment> for (QString) {
  fn fromPlainText_s(self ) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragment13fromPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QTextDocumentFragment13fromPlainTextERK7QString(arg0)};
    let mut ret1 = QTextDocumentFragment{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextDocumentFragment::toHtml(const QByteArray & encoding);
impl /*struct*/ QTextDocumentFragment {
  pub fn toHtml<RetType, T: QTextDocumentFragment_toHtml<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toHtml(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_toHtml<RetType> {
  fn toHtml(self , rsthis: &mut QTextDocumentFragment) -> RetType;
}

  // proto:  QString QTextDocumentFragment::toHtml(const QByteArray & encoding);
impl<'a> /*trait*/ QTextDocumentFragment_toHtml<QString> for (QByteArray) {
  fn toHtml(self , rsthis: &mut QTextDocumentFragment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment6toHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QTextDocumentFragment6toHtmlERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentFragment::~QTextDocumentFragment();
impl /*struct*/ QTextDocumentFragment {
  pub fn FreeQTextDocumentFragment<RetType, T: QTextDocumentFragment_FreeQTextDocumentFragment<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextDocumentFragment(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_FreeQTextDocumentFragment<RetType> {
  fn FreeQTextDocumentFragment(self , rsthis: &mut QTextDocumentFragment) -> RetType;
}

  // proto:  void QTextDocumentFragment::~QTextDocumentFragment();
impl<'a> /*trait*/ QTextDocumentFragment_FreeQTextDocumentFragment<()> for () {
  fn FreeQTextDocumentFragment(self , rsthis: &mut QTextDocumentFragment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragmentD0Ev()};
     unsafe {_ZN21QTextDocumentFragmentD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString & html);
impl<'a> /*trait*/ QTextDocumentFragment_fromHtml_s<QTextDocumentFragment> for (QString) {
  fn fromHtml_s(self ) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QTextDocumentFragment8fromHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QTextDocumentFragment8fromHtmlERK7QString(arg0)};
    let mut ret1 = QTextDocumentFragment{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentFragment::QTextDocumentFragment();
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

  // proto:  QString QTextDocumentFragment::toPlainText();
impl /*struct*/ QTextDocumentFragment {
  pub fn toPlainText<RetType, T: QTextDocumentFragment_toPlainText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPlainText(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_toPlainText<RetType> {
  fn toPlainText(self , rsthis: &mut QTextDocumentFragment) -> RetType;
}

  // proto:  QString QTextDocumentFragment::toPlainText();
impl<'a> /*trait*/ QTextDocumentFragment_toPlainText<QString> for () {
  fn toPlainText(self , rsthis: &mut QTextDocumentFragment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment11toPlainTextEv()};
    let mut ret = unsafe {_ZNK21QTextDocumentFragment11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextCursor & range);
impl<'a> /*trait*/ QTextDocumentFragment_NewQTextDocumentFragment for (QTextCursor) {
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

  // proto:  void QTextDocumentFragment::QTextDocumentFragment(const QTextDocument * document);
impl<'a> /*trait*/ QTextDocumentFragment_NewQTextDocumentFragment for (QTextDocument) {
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

  // proto:  bool QTextDocumentFragment::isEmpty();
impl /*struct*/ QTextDocumentFragment {
  pub fn isEmpty<RetType, T: QTextDocumentFragment_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QTextDocumentFragment_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QTextDocumentFragment) -> RetType;
}

  // proto:  bool QTextDocumentFragment::isEmpty();
impl<'a> /*trait*/ QTextDocumentFragment_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QTextDocumentFragment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QTextDocumentFragment7isEmptyEv()};
    let mut ret = unsafe {_ZNK21QTextDocumentFragment7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

