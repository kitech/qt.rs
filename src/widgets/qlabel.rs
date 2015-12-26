// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtWidgets/qlabel.h
// dst-file: /src/widgets/qlabel.rs
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
use super::qframe::QFrame; // 773
use std::ops::Deref;
use super::super::gui::qpicture::QPicture; // 771
use super::super::core::qstring::QString; // 771
use super::qwidget::QWidget; // 773
use super::super::gui::qpixmap::QPixmap; // 771
use super::super::core::qsize::QSize; // 771
use super::super::gui::qmovie::QMovie; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLabel_Class_Size() -> c_int;
  // proto:  const QPicture * QLabel::picture();
  fn _ZNK6QLabel7pictureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLabel::setNum(double );
  fn _ZN6QLabel6setNumEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QLabel::setPicture(const QPicture & );
  fn _ZN6QLabel10setPictureERK8QPicture(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLabel::setText(const QString & );
  fn _ZN6QLabel7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QPixmap * QLabel::pixmap();
  fn _ZNK6QLabel6pixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLabel::setIndent(int );
  fn _ZN6QLabel9setIndentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const QMetaObject * QLabel::metaObject();
  fn _ZNK6QLabel10metaObjectEv(qthis: *mut c_void);
  // proto:  void QLabel::~QLabel();
  fn _ZN6QLabelD0Ev(qthis: *mut c_void);
  // proto:  void QLabel::setSelection(int , int );
  fn _ZN6QLabel12setSelectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  bool QLabel::hasScaledContents();
  fn _ZNK6QLabel17hasScaledContentsEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QLabel::text();
  fn _ZNK6QLabel4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLabel::heightForWidth(int );
  fn _ZNK6QLabel14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QLabel::openExternalLinks();
  fn _ZNK6QLabel17openExternalLinksEv(qthis: *mut c_void) -> c_char;
  // proto:  void QLabel::setNum(int );
  fn _ZN6QLabel6setNumEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QLabel::setPixmap(const QPixmap & );
  fn _ZN6QLabel9setPixmapERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLabel::setOpenExternalLinks(bool open);
  fn _ZN6QLabel20setOpenExternalLinksEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QWidget * QLabel::buddy();
  fn _ZNK6QLabel5buddyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QLabel::wordWrap();
  fn _ZNK6QLabel8wordWrapEv(qthis: *mut c_void) -> c_char;
  // proto:  void QLabel::setWordWrap(bool on);
  fn _ZN6QLabel11setWordWrapEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QLabel::clear();
  fn _ZN6QLabel5clearEv(qthis: *mut c_void);
  // proto:  void QLabel::setMargin(int );
  fn _ZN6QLabel9setMarginEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QSize QLabel::minimumSizeHint();
  fn _ZNK6QLabel15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLabel::selectionStart();
  fn _ZNK6QLabel14selectionStartEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QLabel::hasSelectedText();
  fn _ZNK6QLabel15hasSelectedTextEv(qthis: *mut c_void) -> c_char;
  // proto:  void QLabel::linkActivated(const QString & link);
  fn _ZN6QLabel13linkActivatedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLabel::setBuddy(QWidget * );
  fn _ZN6QLabel8setBuddyEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLabel::QLabel(const QLabel & );
  fn dector_ZN6QLabelC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QLabelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QLabel::indent();
  fn _ZNK6QLabel6indentEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QLabel::sizeHint();
  fn _ZNK6QLabel8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLabel::margin();
  fn _ZNK6QLabel6marginEv(qthis: *mut c_void) -> c_int;
  // proto:  QMovie * QLabel::movie();
  fn _ZNK6QLabel5movieEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLabel::setScaledContents(bool );
  fn _ZN6QLabel17setScaledContentsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QLabel::setMovie(QMovie * movie);
  fn _ZN6QLabel8setMovieEP6QMovie(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QLabel::selectedText();
  fn _ZNK6QLabel12selectedTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLabel::linkHovered(const QString & link);
  fn _ZN6QLabel11linkHoveredERK7QString(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLabel)=1
pub struct QLabel {
  qbase: QFrame,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLabel {
  pub fn inheritFrom(qthis: *mut c_void) -> QLabel {
    return QLabel{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QLabel {
  type Target = QFrame;

  fn deref(&self) -> &QFrame {
    return & self.qbase;
  }
}
impl AsRef<QFrame> for QLabel {
  fn as_ref(& self) -> & QFrame {
    return & self.qbase;
  }
}
  // proto:  const QPicture * QLabel::picture();
impl /*struct*/ QLabel {
  pub fn picture<RetType, T: QLabel_picture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.picture(self);
    // return 1;
  }
}

pub trait QLabel_picture<RetType> {
  fn picture(self , rsthis: & QLabel) -> RetType;
}

  // proto:  const QPicture * QLabel::picture();
impl<'a> /*trait*/ QLabel_picture<QPicture> for () {
  fn picture(self , rsthis: & QLabel) -> QPicture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel7pictureEv()};
    let mut ret = unsafe {_ZNK6QLabel7pictureEv(rsthis.qclsinst)};
    let mut ret1 = QPicture::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLabel::setNum(double );
impl /*struct*/ QLabel {
  pub fn setNum<RetType, T: QLabel_setNum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNum(self);
    // return 1;
  }
}

pub trait QLabel_setNum<RetType> {
  fn setNum(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setNum(double );
impl<'a> /*trait*/ QLabel_setNum<()> for (f64) {
  fn setNum(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel6setNumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QLabel6setNumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLabel::setPicture(const QPicture & );
impl /*struct*/ QLabel {
  pub fn setPicture<RetType, T: QLabel_setPicture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPicture(self);
    // return 1;
  }
}

pub trait QLabel_setPicture<RetType> {
  fn setPicture(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setPicture(const QPicture & );
impl<'a> /*trait*/ QLabel_setPicture<()> for (&'a QPicture) {
  fn setPicture(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel10setPictureERK8QPicture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel10setPictureERK8QPicture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLabel::setText(const QString & );
impl /*struct*/ QLabel {
  pub fn setText<RetType, T: QLabel_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QLabel_setText<RetType> {
  fn setText(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setText(const QString & );
impl<'a> /*trait*/ QLabel_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QPixmap * QLabel::pixmap();
impl /*struct*/ QLabel {
  pub fn pixmap<RetType, T: QLabel_pixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixmap(self);
    // return 1;
  }
}

pub trait QLabel_pixmap<RetType> {
  fn pixmap(self , rsthis: & QLabel) -> RetType;
}

  // proto:  const QPixmap * QLabel::pixmap();
impl<'a> /*trait*/ QLabel_pixmap<QPixmap> for () {
  fn pixmap(self , rsthis: & QLabel) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6pixmapEv()};
    let mut ret = unsafe {_ZNK6QLabel6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLabel::setIndent(int );
impl /*struct*/ QLabel {
  pub fn setIndent<RetType, T: QLabel_setIndent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIndent(self);
    // return 1;
  }
}

pub trait QLabel_setIndent<RetType> {
  fn setIndent(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setIndent(int );
impl<'a> /*trait*/ QLabel_setIndent<()> for (i32) {
  fn setIndent(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QLabel9setIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QLabel::metaObject();
impl /*struct*/ QLabel {
  pub fn metaObject<RetType, T: QLabel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QLabel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QLabel) -> RetType;
}

  // proto:  const QMetaObject * QLabel::metaObject();
impl<'a> /*trait*/ QLabel_metaObject<()> for () {
  fn metaObject(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel10metaObjectEv()};
     unsafe {_ZNK6QLabel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLabel::~QLabel();
impl /*struct*/ QLabel {
  pub fn Free<RetType, T: QLabel_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QLabel_Free<RetType> {
  fn Free(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::~QLabel();
impl<'a> /*trait*/ QLabel_Free<()> for () {
  fn Free(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabelD0Ev()};
     unsafe {_ZN6QLabelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLabel::setSelection(int , int );
impl /*struct*/ QLabel {
  pub fn setSelection<RetType, T: QLabel_setSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelection(self);
    // return 1;
  }
}

pub trait QLabel_setSelection<RetType> {
  fn setSelection(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setSelection(int , int );
impl<'a> /*trait*/ QLabel_setSelection<()> for (i32, i32) {
  fn setSelection(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN6QLabel12setSelectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QLabel::hasScaledContents();
impl /*struct*/ QLabel {
  pub fn hasScaledContents<RetType, T: QLabel_hasScaledContents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasScaledContents(self);
    // return 1;
  }
}

pub trait QLabel_hasScaledContents<RetType> {
  fn hasScaledContents(self , rsthis: & QLabel) -> RetType;
}

  // proto:  bool QLabel::hasScaledContents();
impl<'a> /*trait*/ QLabel_hasScaledContents<i8> for () {
  fn hasScaledContents(self , rsthis: & QLabel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel17hasScaledContentsEv()};
    let mut ret = unsafe {_ZNK6QLabel17hasScaledContentsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QLabel::text();
impl /*struct*/ QLabel {
  pub fn text<RetType, T: QLabel_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QLabel_text<RetType> {
  fn text(self , rsthis: & QLabel) -> RetType;
}

  // proto:  QString QLabel::text();
impl<'a> /*trait*/ QLabel_text<QString> for () {
  fn text(self , rsthis: & QLabel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel4textEv()};
    let mut ret = unsafe {_ZNK6QLabel4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QLabel::heightForWidth(int );
impl /*struct*/ QLabel {
  pub fn heightForWidth<RetType, T: QLabel_heightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QLabel_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: & QLabel) -> RetType;
}

  // proto:  int QLabel::heightForWidth(int );
impl<'a> /*trait*/ QLabel_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QLabel14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QLabel::openExternalLinks();
impl /*struct*/ QLabel {
  pub fn openExternalLinks<RetType, T: QLabel_openExternalLinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openExternalLinks(self);
    // return 1;
  }
}

pub trait QLabel_openExternalLinks<RetType> {
  fn openExternalLinks(self , rsthis: & QLabel) -> RetType;
}

  // proto:  bool QLabel::openExternalLinks();
impl<'a> /*trait*/ QLabel_openExternalLinks<i8> for () {
  fn openExternalLinks(self , rsthis: & QLabel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel17openExternalLinksEv()};
    let mut ret = unsafe {_ZNK6QLabel17openExternalLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLabel::setNum(int );
impl<'a> /*trait*/ QLabel_setNum<()> for (i32) {
  fn setNum(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel6setNumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QLabel6setNumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLabel::setPixmap(const QPixmap & );
impl /*struct*/ QLabel {
  pub fn setPixmap<RetType, T: QLabel_setPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPixmap(self);
    // return 1;
  }
}

pub trait QLabel_setPixmap<RetType> {
  fn setPixmap(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setPixmap(const QPixmap & );
impl<'a> /*trait*/ QLabel_setPixmap<()> for (&'a QPixmap) {
  fn setPixmap(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel9setPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLabel::setOpenExternalLinks(bool open);
impl /*struct*/ QLabel {
  pub fn setOpenExternalLinks<RetType, T: QLabel_setOpenExternalLinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpenExternalLinks(self);
    // return 1;
  }
}

pub trait QLabel_setOpenExternalLinks<RetType> {
  fn setOpenExternalLinks(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QLabel_setOpenExternalLinks<()> for (i8) {
  fn setOpenExternalLinks(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel20setOpenExternalLinksEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN6QLabel20setOpenExternalLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QLabel::buddy();
impl /*struct*/ QLabel {
  pub fn buddy<RetType, T: QLabel_buddy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buddy(self);
    // return 1;
  }
}

pub trait QLabel_buddy<RetType> {
  fn buddy(self , rsthis: & QLabel) -> RetType;
}

  // proto:  QWidget * QLabel::buddy();
impl<'a> /*trait*/ QLabel_buddy<QWidget> for () {
  fn buddy(self , rsthis: & QLabel) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel5buddyEv()};
    let mut ret = unsafe {_ZNK6QLabel5buddyEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QLabel::wordWrap();
impl /*struct*/ QLabel {
  pub fn wordWrap<RetType, T: QLabel_wordWrap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wordWrap(self);
    // return 1;
  }
}

pub trait QLabel_wordWrap<RetType> {
  fn wordWrap(self , rsthis: & QLabel) -> RetType;
}

  // proto:  bool QLabel::wordWrap();
impl<'a> /*trait*/ QLabel_wordWrap<i8> for () {
  fn wordWrap(self , rsthis: & QLabel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel8wordWrapEv()};
    let mut ret = unsafe {_ZNK6QLabel8wordWrapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLabel::setWordWrap(bool on);
impl /*struct*/ QLabel {
  pub fn setWordWrap<RetType, T: QLabel_setWordWrap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWordWrap(self);
    // return 1;
  }
}

pub trait QLabel_setWordWrap<RetType> {
  fn setWordWrap(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setWordWrap(bool on);
impl<'a> /*trait*/ QLabel_setWordWrap<()> for (i8) {
  fn setWordWrap(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel11setWordWrapEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN6QLabel11setWordWrapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLabel::clear();
impl /*struct*/ QLabel {
  pub fn clear<RetType, T: QLabel_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QLabel_clear<RetType> {
  fn clear(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::clear();
impl<'a> /*trait*/ QLabel_clear<()> for () {
  fn clear(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel5clearEv()};
     unsafe {_ZN6QLabel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLabel::setMargin(int );
impl /*struct*/ QLabel {
  pub fn setMargin<RetType, T: QLabel_setMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMargin(self);
    // return 1;
  }
}

pub trait QLabel_setMargin<RetType> {
  fn setMargin(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setMargin(int );
impl<'a> /*trait*/ QLabel_setMargin<()> for (i32) {
  fn setMargin(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setMarginEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QLabel9setMarginEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QLabel::minimumSizeHint();
impl /*struct*/ QLabel {
  pub fn minimumSizeHint<RetType, T: QLabel_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QLabel_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QLabel) -> RetType;
}

  // proto:  QSize QLabel::minimumSizeHint();
impl<'a> /*trait*/ QLabel_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QLabel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK6QLabel15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QLabel::selectionStart();
impl /*struct*/ QLabel {
  pub fn selectionStart<RetType, T: QLabel_selectionStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionStart(self);
    // return 1;
  }
}

pub trait QLabel_selectionStart<RetType> {
  fn selectionStart(self , rsthis: & QLabel) -> RetType;
}

  // proto:  int QLabel::selectionStart();
impl<'a> /*trait*/ QLabel_selectionStart<i32> for () {
  fn selectionStart(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel14selectionStartEv()};
    let mut ret = unsafe {_ZNK6QLabel14selectionStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QLabel::hasSelectedText();
impl /*struct*/ QLabel {
  pub fn hasSelectedText<RetType, T: QLabel_hasSelectedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasSelectedText(self);
    // return 1;
  }
}

pub trait QLabel_hasSelectedText<RetType> {
  fn hasSelectedText(self , rsthis: & QLabel) -> RetType;
}

  // proto:  bool QLabel::hasSelectedText();
impl<'a> /*trait*/ QLabel_hasSelectedText<i8> for () {
  fn hasSelectedText(self , rsthis: & QLabel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel15hasSelectedTextEv()};
    let mut ret = unsafe {_ZNK6QLabel15hasSelectedTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLabel::linkActivated(const QString & link);
impl /*struct*/ QLabel {
  pub fn linkActivated<RetType, T: QLabel_linkActivated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.linkActivated(self);
    // return 1;
  }
}

pub trait QLabel_linkActivated<RetType> {
  fn linkActivated(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::linkActivated(const QString & link);
impl<'a> /*trait*/ QLabel_linkActivated<()> for (&'a QString) {
  fn linkActivated(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel13linkActivatedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel13linkActivatedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLabel::setBuddy(QWidget * );
impl /*struct*/ QLabel {
  pub fn setBuddy<RetType, T: QLabel_setBuddy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBuddy(self);
    // return 1;
  }
}

pub trait QLabel_setBuddy<RetType> {
  fn setBuddy(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setBuddy(QWidget * );
impl<'a> /*trait*/ QLabel_setBuddy<()> for (&'a QWidget) {
  fn setBuddy(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel8setBuddyEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel8setBuddyEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLabel::QLabel(const QLabel & );
impl /*struct*/ QLabel {
  pub fn New<T: QLabel_New>(value: T) -> QLabel {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QLabel_New {
  fn New(self) -> QLabel;
}

  // proto:  void QLabel::QLabel(const QLabel & );
impl<'a> /*trait*/ QLabel_New for (&'a QLabel) {
  fn New(self) -> QLabel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabelC1ERKS_()};
    let ctysz: c_int = unsafe{QLabel_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QLabelC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QLabelC1ERKS_(arg0)};
    let rsthis = QLabel{/**/qbase: QFrame::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QLabel::indent();
impl /*struct*/ QLabel {
  pub fn indent<RetType, T: QLabel_indent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indent(self);
    // return 1;
  }
}

pub trait QLabel_indent<RetType> {
  fn indent(self , rsthis: & QLabel) -> RetType;
}

  // proto:  int QLabel::indent();
impl<'a> /*trait*/ QLabel_indent<i32> for () {
  fn indent(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6indentEv()};
    let mut ret = unsafe {_ZNK6QLabel6indentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QLabel::sizeHint();
impl /*struct*/ QLabel {
  pub fn sizeHint<RetType, T: QLabel_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QLabel_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QLabel) -> RetType;
}

  // proto:  QSize QLabel::sizeHint();
impl<'a> /*trait*/ QLabel_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QLabel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel8sizeHintEv()};
    let mut ret = unsafe {_ZNK6QLabel8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QLabel::margin();
impl /*struct*/ QLabel {
  pub fn margin<RetType, T: QLabel_margin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.margin(self);
    // return 1;
  }
}

pub trait QLabel_margin<RetType> {
  fn margin(self , rsthis: & QLabel) -> RetType;
}

  // proto:  int QLabel::margin();
impl<'a> /*trait*/ QLabel_margin<i32> for () {
  fn margin(self , rsthis: & QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6marginEv()};
    let mut ret = unsafe {_ZNK6QLabel6marginEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QMovie * QLabel::movie();
impl /*struct*/ QLabel {
  pub fn movie<RetType, T: QLabel_movie<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.movie(self);
    // return 1;
  }
}

pub trait QLabel_movie<RetType> {
  fn movie(self , rsthis: & QLabel) -> RetType;
}

  // proto:  QMovie * QLabel::movie();
impl<'a> /*trait*/ QLabel_movie<QMovie> for () {
  fn movie(self , rsthis: & QLabel) -> QMovie {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel5movieEv()};
    let mut ret = unsafe {_ZNK6QLabel5movieEv(rsthis.qclsinst)};
    let mut ret1 = QMovie::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLabel::setScaledContents(bool );
impl /*struct*/ QLabel {
  pub fn setScaledContents<RetType, T: QLabel_setScaledContents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScaledContents(self);
    // return 1;
  }
}

pub trait QLabel_setScaledContents<RetType> {
  fn setScaledContents(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setScaledContents(bool );
impl<'a> /*trait*/ QLabel_setScaledContents<()> for (i8) {
  fn setScaledContents(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel17setScaledContentsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN6QLabel17setScaledContentsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLabel::setMovie(QMovie * movie);
impl /*struct*/ QLabel {
  pub fn setMovie<RetType, T: QLabel_setMovie<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMovie(self);
    // return 1;
  }
}

pub trait QLabel_setMovie<RetType> {
  fn setMovie(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::setMovie(QMovie * movie);
impl<'a> /*trait*/ QLabel_setMovie<()> for (&'a QMovie) {
  fn setMovie(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel8setMovieEP6QMovie()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel8setMovieEP6QMovie(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QLabel::selectedText();
impl /*struct*/ QLabel {
  pub fn selectedText<RetType, T: QLabel_selectedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedText(self);
    // return 1;
  }
}

pub trait QLabel_selectedText<RetType> {
  fn selectedText(self , rsthis: & QLabel) -> RetType;
}

  // proto:  QString QLabel::selectedText();
impl<'a> /*trait*/ QLabel_selectedText<QString> for () {
  fn selectedText(self , rsthis: & QLabel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel12selectedTextEv()};
    let mut ret = unsafe {_ZNK6QLabel12selectedTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLabel::linkHovered(const QString & link);
impl /*struct*/ QLabel {
  pub fn linkHovered<RetType, T: QLabel_linkHovered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.linkHovered(self);
    // return 1;
  }
}

pub trait QLabel_linkHovered<RetType> {
  fn linkHovered(self , rsthis: & QLabel) -> RetType;
}

  // proto:  void QLabel::linkHovered(const QString & link);
impl<'a> /*trait*/ QLabel_linkHovered<()> for (&'a QString) {
  fn linkHovered(self , rsthis: & QLabel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel11linkHoveredERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel11linkHoveredERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

