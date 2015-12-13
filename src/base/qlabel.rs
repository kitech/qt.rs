// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpicture::QPicture;
use super::qstring::QString;
use super::qpixmap::QPixmap;
use super::qwidget::QWidget;
use super::qsize::QSize;
use super::qmovie::QMovie;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QPicture * QLabel::picture();
  fn _ZNK6QLabel7pictureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLabel::setNum(double );
  fn _ZN6QLabel6setNumEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QLabel::setPicture(const QPicture & );
  fn _ZN6QLabel10setPictureERK8QPicture(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLabel::setText(const QString & );
  fn _ZN6QLabel7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QPixmap * QLabel::pixmap();
  fn _ZNK6QLabel6pixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLabel::setIndent(int );
  fn _ZN6QLabel9setIndentEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QMetaObject * QLabel::metaObject();
  fn _ZNK6QLabel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QLabel::FreeQLabel();
  fn _ZN6QLabelD0Ev(qthis: *mut c_void) ;
  // proto:  void QLabel::setSelection(int , int );
  fn _ZN6QLabel12setSelectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QLabel::hasScaledContents();
  fn _ZNK6QLabel17hasScaledContentsEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QLabel::text();
  fn _ZNK6QLabel4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLabel::heightForWidth(int );
  fn _ZNK6QLabel14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QLabel::openExternalLinks();
  fn _ZNK6QLabel17openExternalLinksEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLabel::setNum(int );
  fn _ZN6QLabel6setNumEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QLabel::setPixmap(const QPixmap & );
  fn _ZN6QLabel9setPixmapERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLabel::setOpenExternalLinks(bool open);
  fn _ZN6QLabel20setOpenExternalLinksEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QWidget * QLabel::buddy();
  fn _ZNK6QLabel5buddyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QLabel::wordWrap();
  fn _ZNK6QLabel8wordWrapEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLabel::setWordWrap(bool on);
  fn _ZN6QLabel11setWordWrapEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QLabel::clear();
  fn _ZN6QLabel5clearEv(qthis: *mut c_void) ;
  // proto:  void QLabel::setMargin(int );
  fn _ZN6QLabel9setMarginEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QSize QLabel::minimumSizeHint();
  fn _ZNK6QLabel15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLabel::selectionStart();
  fn _ZNK6QLabel14selectionStartEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QLabel::hasSelectedText();
  fn _ZNK6QLabel15hasSelectedTextEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLabel::linkActivated(const QString & link);
  fn _ZN6QLabel13linkActivatedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLabel::setBuddy(QWidget * );
  fn _ZN6QLabel8setBuddyEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLabel::NewQLabel(const QLabel & );
  fn _ZN6QLabelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QLabel::indent();
  fn _ZNK6QLabel6indentEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QLabel::sizeHint();
  fn _ZNK6QLabel8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLabel::margin();
  fn _ZNK6QLabel6marginEv(qthis: *mut c_void) -> c_int;
  // proto:  QMovie * QLabel::movie();
  fn _ZNK6QLabel5movieEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLabel::setScaledContents(bool );
  fn _ZN6QLabel17setScaledContentsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QLabel::setMovie(QMovie * movie);
  fn _ZN6QLabel8setMovieEP6QMovie(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QLabel::selectedText();
  fn _ZNK6QLabel12selectedTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLabel::linkHovered(const QString & link);
  fn _ZN6QLabel11linkHoveredERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QLabel)=1
pub struct QLabel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLabel {
  pub fn picture<T: QLabel_picture>(&mut self, value: T) -> QPicture {
    return value.picture(self);
    // return 1;
  }
}

pub trait QLabel_picture {
  fn picture(self, rsthis: &mut QLabel) -> QPicture;
}

// proto:  const QPicture * QLabel::picture();
impl<'a> /*trait*/ QLabel_picture for () {
  fn picture(self, rsthis: &mut QLabel) -> QPicture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel7pictureEv()};
    let mut ret = unsafe {_ZNK6QLabel7pictureEv(rsthis.qclsinst)};
    let mut ret1 = QPicture{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setNum<T: QLabel_setNum>(&mut self, value: T)  {
     value.setNum(self);
    // return 1;
  }
}

pub trait QLabel_setNum {
  fn setNum(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setNum(double );
impl<'a> /*trait*/ QLabel_setNum for (f64) {
  fn setNum(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel6setNumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QLabel6setNumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setPicture<T: QLabel_setPicture>(&mut self, value: T)  {
     value.setPicture(self);
    // return 1;
  }
}

pub trait QLabel_setPicture {
  fn setPicture(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setPicture(const QPicture & );
impl<'a> /*trait*/ QLabel_setPicture for (&'a  QPicture) {
  fn setPicture(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel10setPictureERK8QPicture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel10setPictureERK8QPicture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setText<T: QLabel_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QLabel_setText {
  fn setText(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setText(const QString & );
impl<'a> /*trait*/ QLabel_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn pixmap<T: QLabel_pixmap>(&mut self, value: T) -> QPixmap {
    return value.pixmap(self);
    // return 1;
  }
}

pub trait QLabel_pixmap {
  fn pixmap(self, rsthis: &mut QLabel) -> QPixmap;
}

// proto:  const QPixmap * QLabel::pixmap();
impl<'a> /*trait*/ QLabel_pixmap for () {
  fn pixmap(self, rsthis: &mut QLabel) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6pixmapEv()};
    let mut ret = unsafe {_ZNK6QLabel6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setIndent<T: QLabel_setIndent>(&mut self, value: T)  {
     value.setIndent(self);
    // return 1;
  }
}

pub trait QLabel_setIndent {
  fn setIndent(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setIndent(int );
impl<'a> /*trait*/ QLabel_setIndent for (i32) {
  fn setIndent(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QLabel9setIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn metaObject<T: QLabel_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QLabel_metaObject {
  fn metaObject(self, rsthis: &mut QLabel) ;
}

// proto:  const QMetaObject * QLabel::metaObject();
impl<'a> /*trait*/ QLabel_metaObject for () {
  fn metaObject(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel10metaObjectEv()};
     unsafe {_ZNK6QLabel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn FreeQLabel<T: QLabel_FreeQLabel>(&mut self, value: T)  {
     value.FreeQLabel(self);
    // return 1;
  }
}

pub trait QLabel_FreeQLabel {
  fn FreeQLabel(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::FreeQLabel();
impl<'a> /*trait*/ QLabel_FreeQLabel for () {
  fn FreeQLabel(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabelD0Ev()};
     unsafe {_ZN6QLabelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setSelection<T: QLabel_setSelection>(&mut self, value: T)  {
     value.setSelection(self);
    // return 1;
  }
}

pub trait QLabel_setSelection {
  fn setSelection(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setSelection(int , int );
impl<'a> /*trait*/ QLabel_setSelection for (i32, i32) {
  fn setSelection(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN6QLabel12setSelectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn hasScaledContents<T: QLabel_hasScaledContents>(&mut self, value: T) -> i8 {
    return value.hasScaledContents(self);
    // return 1;
  }
}

pub trait QLabel_hasScaledContents {
  fn hasScaledContents(self, rsthis: &mut QLabel) -> i8;
}

// proto:  bool QLabel::hasScaledContents();
impl<'a> /*trait*/ QLabel_hasScaledContents for () {
  fn hasScaledContents(self, rsthis: &mut QLabel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel17hasScaledContentsEv()};
    let mut ret = unsafe {_ZNK6QLabel17hasScaledContentsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn text<T: QLabel_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QLabel_text {
  fn text(self, rsthis: &mut QLabel) -> QString;
}

// proto:  QString QLabel::text();
impl<'a> /*trait*/ QLabel_text for () {
  fn text(self, rsthis: &mut QLabel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel4textEv()};
    let mut ret = unsafe {_ZNK6QLabel4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn heightForWidth<T: QLabel_heightForWidth>(&mut self, value: T) -> i32 {
    return value.heightForWidth(self);
    // return 1;
  }
}

pub trait QLabel_heightForWidth {
  fn heightForWidth(self, rsthis: &mut QLabel) -> i32;
}

// proto:  int QLabel::heightForWidth(int );
impl<'a> /*trait*/ QLabel_heightForWidth for (i32) {
  fn heightForWidth(self, rsthis: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QLabel14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn openExternalLinks<T: QLabel_openExternalLinks>(&mut self, value: T) -> i8 {
    return value.openExternalLinks(self);
    // return 1;
  }
}

pub trait QLabel_openExternalLinks {
  fn openExternalLinks(self, rsthis: &mut QLabel) -> i8;
}

// proto:  bool QLabel::openExternalLinks();
impl<'a> /*trait*/ QLabel_openExternalLinks for () {
  fn openExternalLinks(self, rsthis: &mut QLabel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel17openExternalLinksEv()};
    let mut ret = unsafe {_ZNK6QLabel17openExternalLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QLabel::setNum(int );
impl<'a> /*trait*/ QLabel_setNum for (i32) {
  fn setNum(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel6setNumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QLabel6setNumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setPixmap<T: QLabel_setPixmap>(&mut self, value: T)  {
     value.setPixmap(self);
    // return 1;
  }
}

pub trait QLabel_setPixmap {
  fn setPixmap(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setPixmap(const QPixmap & );
impl<'a> /*trait*/ QLabel_setPixmap for (&'a  QPixmap) {
  fn setPixmap(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel9setPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setOpenExternalLinks<T: QLabel_setOpenExternalLinks>(&mut self, value: T)  {
     value.setOpenExternalLinks(self);
    // return 1;
  }
}

pub trait QLabel_setOpenExternalLinks {
  fn setOpenExternalLinks(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QLabel_setOpenExternalLinks for (i8) {
  fn setOpenExternalLinks(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel20setOpenExternalLinksEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN6QLabel20setOpenExternalLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn buddy<T: QLabel_buddy>(&mut self, value: T) -> QWidget {
    return value.buddy(self);
    // return 1;
  }
}

pub trait QLabel_buddy {
  fn buddy(self, rsthis: &mut QLabel) -> QWidget;
}

// proto:  QWidget * QLabel::buddy();
impl<'a> /*trait*/ QLabel_buddy for () {
  fn buddy(self, rsthis: &mut QLabel) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel5buddyEv()};
    let mut ret = unsafe {_ZNK6QLabel5buddyEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn wordWrap<T: QLabel_wordWrap>(&mut self, value: T) -> i8 {
    return value.wordWrap(self);
    // return 1;
  }
}

pub trait QLabel_wordWrap {
  fn wordWrap(self, rsthis: &mut QLabel) -> i8;
}

// proto:  bool QLabel::wordWrap();
impl<'a> /*trait*/ QLabel_wordWrap for () {
  fn wordWrap(self, rsthis: &mut QLabel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel8wordWrapEv()};
    let mut ret = unsafe {_ZNK6QLabel8wordWrapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setWordWrap<T: QLabel_setWordWrap>(&mut self, value: T)  {
     value.setWordWrap(self);
    // return 1;
  }
}

pub trait QLabel_setWordWrap {
  fn setWordWrap(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setWordWrap(bool on);
impl<'a> /*trait*/ QLabel_setWordWrap for (i8) {
  fn setWordWrap(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel11setWordWrapEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN6QLabel11setWordWrapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn clear<T: QLabel_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QLabel_clear {
  fn clear(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::clear();
impl<'a> /*trait*/ QLabel_clear for () {
  fn clear(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel5clearEv()};
     unsafe {_ZN6QLabel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setMargin<T: QLabel_setMargin>(&mut self, value: T)  {
     value.setMargin(self);
    // return 1;
  }
}

pub trait QLabel_setMargin {
  fn setMargin(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setMargin(int );
impl<'a> /*trait*/ QLabel_setMargin for (i32) {
  fn setMargin(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setMarginEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QLabel9setMarginEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn minimumSizeHint<T: QLabel_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QLabel_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QLabel) -> QSize;
}

// proto:  QSize QLabel::minimumSizeHint();
impl<'a> /*trait*/ QLabel_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QLabel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK6QLabel15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn selectionStart<T: QLabel_selectionStart>(&mut self, value: T) -> i32 {
    return value.selectionStart(self);
    // return 1;
  }
}

pub trait QLabel_selectionStart {
  fn selectionStart(self, rsthis: &mut QLabel) -> i32;
}

// proto:  int QLabel::selectionStart();
impl<'a> /*trait*/ QLabel_selectionStart for () {
  fn selectionStart(self, rsthis: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel14selectionStartEv()};
    let mut ret = unsafe {_ZNK6QLabel14selectionStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn hasSelectedText<T: QLabel_hasSelectedText>(&mut self, value: T) -> i8 {
    return value.hasSelectedText(self);
    // return 1;
  }
}

pub trait QLabel_hasSelectedText {
  fn hasSelectedText(self, rsthis: &mut QLabel) -> i8;
}

// proto:  bool QLabel::hasSelectedText();
impl<'a> /*trait*/ QLabel_hasSelectedText for () {
  fn hasSelectedText(self, rsthis: &mut QLabel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel15hasSelectedTextEv()};
    let mut ret = unsafe {_ZNK6QLabel15hasSelectedTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn linkActivated<T: QLabel_linkActivated>(&mut self, value: T)  {
     value.linkActivated(self);
    // return 1;
  }
}

pub trait QLabel_linkActivated {
  fn linkActivated(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::linkActivated(const QString & link);
impl<'a> /*trait*/ QLabel_linkActivated for (&'a  QString) {
  fn linkActivated(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel13linkActivatedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel13linkActivatedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setBuddy<T: QLabel_setBuddy>(&mut self, value: T)  {
     value.setBuddy(self);
    // return 1;
  }
}

pub trait QLabel_setBuddy {
  fn setBuddy(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setBuddy(QWidget * );
impl<'a> /*trait*/ QLabel_setBuddy for (&'a mut QWidget) {
  fn setBuddy(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel8setBuddyEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel8setBuddyEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn NewQLabel<T: QLabel_NewQLabel>(value: T) -> QLabel {
    let rsthis = value.NewQLabel();
    return rsthis;
    // return 1;
  }
}

pub trait QLabel_NewQLabel {
  fn NewQLabel(self) -> QLabel;
}

// proto: void QLabel::NewQLabel(const QLabel & );
impl<'a> /*trait*/ QLabel_NewQLabel for (&'a  QLabel) {
  fn NewQLabel(self) -> QLabel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QLabelC1ERKS_(qthis, arg0)};
    let rsthis = QLabel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn indent<T: QLabel_indent>(&mut self, value: T) -> i32 {
    return value.indent(self);
    // return 1;
  }
}

pub trait QLabel_indent {
  fn indent(self, rsthis: &mut QLabel) -> i32;
}

// proto:  int QLabel::indent();
impl<'a> /*trait*/ QLabel_indent for () {
  fn indent(self, rsthis: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6indentEv()};
    let mut ret = unsafe {_ZNK6QLabel6indentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn sizeHint<T: QLabel_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QLabel_sizeHint {
  fn sizeHint(self, rsthis: &mut QLabel) -> QSize;
}

// proto:  QSize QLabel::sizeHint();
impl<'a> /*trait*/ QLabel_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QLabel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel8sizeHintEv()};
    let mut ret = unsafe {_ZNK6QLabel8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn margin<T: QLabel_margin>(&mut self, value: T) -> i32 {
    return value.margin(self);
    // return 1;
  }
}

pub trait QLabel_margin {
  fn margin(self, rsthis: &mut QLabel) -> i32;
}

// proto:  int QLabel::margin();
impl<'a> /*trait*/ QLabel_margin for () {
  fn margin(self, rsthis: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6marginEv()};
    let mut ret = unsafe {_ZNK6QLabel6marginEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn movie<T: QLabel_movie>(&mut self, value: T) -> QMovie {
    return value.movie(self);
    // return 1;
  }
}

pub trait QLabel_movie {
  fn movie(self, rsthis: &mut QLabel) -> QMovie;
}

// proto:  QMovie * QLabel::movie();
impl<'a> /*trait*/ QLabel_movie for () {
  fn movie(self, rsthis: &mut QLabel) -> QMovie {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel5movieEv()};
    let mut ret = unsafe {_ZNK6QLabel5movieEv(rsthis.qclsinst)};
    let mut ret1 = QMovie{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setScaledContents<T: QLabel_setScaledContents>(&mut self, value: T)  {
     value.setScaledContents(self);
    // return 1;
  }
}

pub trait QLabel_setScaledContents {
  fn setScaledContents(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setScaledContents(bool );
impl<'a> /*trait*/ QLabel_setScaledContents for (i8) {
  fn setScaledContents(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel17setScaledContentsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN6QLabel17setScaledContentsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setMovie<T: QLabel_setMovie>(&mut self, value: T)  {
     value.setMovie(self);
    // return 1;
  }
}

pub trait QLabel_setMovie {
  fn setMovie(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::setMovie(QMovie * movie);
impl<'a> /*trait*/ QLabel_setMovie for (&'a mut QMovie) {
  fn setMovie(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel8setMovieEP6QMovie()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel8setMovieEP6QMovie(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn selectedText<T: QLabel_selectedText>(&mut self, value: T) -> QString {
    return value.selectedText(self);
    // return 1;
  }
}

pub trait QLabel_selectedText {
  fn selectedText(self, rsthis: &mut QLabel) -> QString;
}

// proto:  QString QLabel::selectedText();
impl<'a> /*trait*/ QLabel_selectedText for () {
  fn selectedText(self, rsthis: &mut QLabel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel12selectedTextEv()};
    let mut ret = unsafe {_ZNK6QLabel12selectedTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn linkHovered<T: QLabel_linkHovered>(&mut self, value: T)  {
     value.linkHovered(self);
    // return 1;
  }
}

pub trait QLabel_linkHovered {
  fn linkHovered(self, rsthis: &mut QLabel) ;
}

// proto:  void QLabel::linkHovered(const QString & link);
impl<'a> /*trait*/ QLabel_linkHovered for (&'a  QString) {
  fn linkHovered(self, rsthis: &mut QLabel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel11linkHoveredERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLabel11linkHoveredERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

