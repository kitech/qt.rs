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
use super::qmovie::QMovie;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QPicture * QLabel::picture();
  fn _ZNK6QLabel7pictureEv() -> i32;
  // proto: void QLabel::setNum(double );
  fn _ZN6QLabel6setNumEd(arg0: c_double) -> i32;
  // proto: void QLabel::setPicture(const QPicture & );
  fn _ZN6QLabel10setPictureERK8QPicture(arg0: *const c_void) -> i32;
  // proto: void QLabel::setText(const QString & );
  fn _ZN6QLabel7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: const QPixmap * QLabel::pixmap();
  fn _ZNK6QLabel6pixmapEv() -> i32;
  // proto: void QLabel::setIndent(int );
  fn _ZN6QLabel9setIndentEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QLabel::metaObject();
  fn _ZNK6QLabel10metaObjectEv() -> i32;
  // proto: void QLabel::FreeQLabel();
  fn _ZN6QLabelD0Ev() -> i32;
  // proto: void QLabel::setSelection(int , int );
  fn _ZN6QLabel12setSelectionEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QLabel::hasScaledContents();
  fn _ZNK6QLabel17hasScaledContentsEv() -> i32;
  // proto: QString QLabel::text();
  fn _ZNK6QLabel4textEv() -> i32;
  // proto: int QLabel::heightForWidth(int );
  fn _ZNK6QLabel14heightForWidthEi(arg0: c_int) -> i32;
  // proto: bool QLabel::openExternalLinks();
  fn _ZNK6QLabel17openExternalLinksEv() -> i32;
  // proto: void QLabel::setNum(int );
  fn _ZN6QLabel6setNumEi(arg0: c_int) -> i32;
  // proto: void QLabel::setPixmap(const QPixmap & );
  fn _ZN6QLabel9setPixmapERK7QPixmap(arg0: *const c_void) -> i32;
  // proto: void QLabel::setOpenExternalLinks(bool open);
  fn _ZN6QLabel20setOpenExternalLinksEb(arg0: int8_t) -> i32;
  // proto: QWidget * QLabel::buddy();
  fn _ZNK6QLabel5buddyEv() -> i32;
  // proto: bool QLabel::wordWrap();
  fn _ZNK6QLabel8wordWrapEv() -> i32;
  // proto: void QLabel::setWordWrap(bool on);
  fn _ZN6QLabel11setWordWrapEb(arg0: int8_t) -> i32;
  // proto: void QLabel::clear();
  fn _ZN6QLabel5clearEv() -> i32;
  // proto: void QLabel::setMargin(int );
  fn _ZN6QLabel9setMarginEi(arg0: c_int) -> i32;
  // proto: QSize QLabel::minimumSizeHint();
  fn _ZNK6QLabel15minimumSizeHintEv() -> i32;
  // proto: int QLabel::selectionStart();
  fn _ZNK6QLabel14selectionStartEv() -> i32;
  // proto: bool QLabel::hasSelectedText();
  fn _ZNK6QLabel15hasSelectedTextEv() -> i32;
  // proto: void QLabel::linkActivated(const QString & link);
  fn _ZN6QLabel13linkActivatedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QLabel::setBuddy(QWidget * );
  fn _ZN6QLabel8setBuddyEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QLabel::NewQLabel(const QLabel & );
  fn _ZN6QLabelC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QLabel::indent();
  fn _ZNK6QLabel6indentEv() -> i32;
  // proto: QSize QLabel::sizeHint();
  fn _ZNK6QLabel8sizeHintEv() -> i32;
  // proto: int QLabel::margin();
  fn _ZNK6QLabel6marginEv() -> i32;
  // proto: QMovie * QLabel::movie();
  fn _ZNK6QLabel5movieEv() -> i32;
  // proto: void QLabel::setScaledContents(bool );
  fn _ZN6QLabel17setScaledContentsEb(arg0: int8_t) -> i32;
  // proto: void QLabel::setMovie(QMovie * movie);
  fn _ZN6QLabel8setMovieEP6QMovie(arg0: *mut c_void) -> i32;
  // proto: QString QLabel::selectedText();
  fn _ZNK6QLabel12selectedTextEv() -> i32;
  // proto: void QLabel::linkHovered(const QString & link);
  fn _ZN6QLabel11linkHoveredERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QLabel)=1
pub struct QLabel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLabel {
  pub fn picture<T: QLabel_picture>(&mut self, value: T) -> i32 {
    value.picture(self);
    return 1;
  }
}

pub trait QLabel_picture {
  fn picture(self, this: &mut QLabel) -> i32;
}

// proto: const QPicture * QLabel::picture();
impl<'a> /*trait*/ QLabel_picture for () {
  fn picture(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel7pictureEv()};
    unsafe {_ZNK6QLabel7pictureEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setNum<T: QLabel_setNum>(&mut self, value: T) -> i32 {
    value.setNum(self);
    return 1;
  }
}

pub trait QLabel_setNum {
  fn setNum(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setNum(double );
impl<'a> /*trait*/ QLabel_setNum for (f64) {
  fn setNum(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel6setNumEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QLabel6setNumEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setPicture<T: QLabel_setPicture>(&mut self, value: T) -> i32 {
    value.setPicture(self);
    return 1;
  }
}

pub trait QLabel_setPicture {
  fn setPicture(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setPicture(const QPicture & );
impl<'a> /*trait*/ QLabel_setPicture for (&'a  QPicture) {
  fn setPicture(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel10setPictureERK8QPicture()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLabel10setPictureERK8QPicture(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setText<T: QLabel_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QLabel_setText {
  fn setText(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setText(const QString & );
impl<'a> /*trait*/ QLabel_setText for (&'a  QString) {
  fn setText(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLabel7setTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn pixmap<T: QLabel_pixmap>(&mut self, value: T) -> i32 {
    value.pixmap(self);
    return 1;
  }
}

pub trait QLabel_pixmap {
  fn pixmap(self, this: &mut QLabel) -> i32;
}

// proto: const QPixmap * QLabel::pixmap();
impl<'a> /*trait*/ QLabel_pixmap for () {
  fn pixmap(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6pixmapEv()};
    unsafe {_ZNK6QLabel6pixmapEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setIndent<T: QLabel_setIndent>(&mut self, value: T) -> i32 {
    value.setIndent(self);
    return 1;
  }
}

pub trait QLabel_setIndent {
  fn setIndent(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setIndent(int );
impl<'a> /*trait*/ QLabel_setIndent for (i32) {
  fn setIndent(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setIndentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QLabel9setIndentEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn metaObject<T: QLabel_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QLabel_metaObject {
  fn metaObject(self, this: &mut QLabel) -> i32;
}

// proto: const QMetaObject * QLabel::metaObject();
impl<'a> /*trait*/ QLabel_metaObject for () {
  fn metaObject(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel10metaObjectEv()};
    unsafe {_ZNK6QLabel10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn FreeQLabel<T: QLabel_FreeQLabel>(&mut self, value: T) -> i32 {
    value.FreeQLabel(self);
    return 1;
  }
}

pub trait QLabel_FreeQLabel {
  fn FreeQLabel(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::FreeQLabel();
impl<'a> /*trait*/ QLabel_FreeQLabel for () {
  fn FreeQLabel(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabelD0Ev()};
    unsafe {_ZN6QLabelD0Ev()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setSelection<T: QLabel_setSelection>(&mut self, value: T) -> i32 {
    value.setSelection(self);
    return 1;
  }
}

pub trait QLabel_setSelection {
  fn setSelection(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setSelection(int , int );
impl<'a> /*trait*/ QLabel_setSelection for (i32, i32) {
  fn setSelection(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN6QLabel12setSelectionEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn hasScaledContents<T: QLabel_hasScaledContents>(&mut self, value: T) -> i32 {
    value.hasScaledContents(self);
    return 1;
  }
}

pub trait QLabel_hasScaledContents {
  fn hasScaledContents(self, this: &mut QLabel) -> i32;
}

// proto: bool QLabel::hasScaledContents();
impl<'a> /*trait*/ QLabel_hasScaledContents for () {
  fn hasScaledContents(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel17hasScaledContentsEv()};
    unsafe {_ZNK6QLabel17hasScaledContentsEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn text<T: QLabel_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QLabel_text {
  fn text(self, this: &mut QLabel) -> i32;
}

// proto: QString QLabel::text();
impl<'a> /*trait*/ QLabel_text for () {
  fn text(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel4textEv()};
    unsafe {_ZNK6QLabel4textEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn heightForWidth<T: QLabel_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QLabel_heightForWidth {
  fn heightForWidth(self, this: &mut QLabel) -> i32;
}

// proto: int QLabel::heightForWidth(int );
impl<'a> /*trait*/ QLabel_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK6QLabel14heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn openExternalLinks<T: QLabel_openExternalLinks>(&mut self, value: T) -> i32 {
    value.openExternalLinks(self);
    return 1;
  }
}

pub trait QLabel_openExternalLinks {
  fn openExternalLinks(self, this: &mut QLabel) -> i32;
}

// proto: bool QLabel::openExternalLinks();
impl<'a> /*trait*/ QLabel_openExternalLinks for () {
  fn openExternalLinks(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel17openExternalLinksEv()};
    unsafe {_ZNK6QLabel17openExternalLinksEv()};
    return 1;
  }
}

// proto: void QLabel::setNum(int );
impl<'a> /*trait*/ QLabel_setNum for (i32) {
  fn setNum(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel6setNumEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QLabel6setNumEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setPixmap<T: QLabel_setPixmap>(&mut self, value: T) -> i32 {
    value.setPixmap(self);
    return 1;
  }
}

pub trait QLabel_setPixmap {
  fn setPixmap(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setPixmap(const QPixmap & );
impl<'a> /*trait*/ QLabel_setPixmap for (&'a  QPixmap) {
  fn setPixmap(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLabel9setPixmapERK7QPixmap(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setOpenExternalLinks<T: QLabel_setOpenExternalLinks>(&mut self, value: T) -> i32 {
    value.setOpenExternalLinks(self);
    return 1;
  }
}

pub trait QLabel_setOpenExternalLinks {
  fn setOpenExternalLinks(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QLabel_setOpenExternalLinks for (i8) {
  fn setOpenExternalLinks(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel20setOpenExternalLinksEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN6QLabel20setOpenExternalLinksEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn buddy<T: QLabel_buddy>(&mut self, value: T) -> i32 {
    value.buddy(self);
    return 1;
  }
}

pub trait QLabel_buddy {
  fn buddy(self, this: &mut QLabel) -> i32;
}

// proto: QWidget * QLabel::buddy();
impl<'a> /*trait*/ QLabel_buddy for () {
  fn buddy(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel5buddyEv()};
    unsafe {_ZNK6QLabel5buddyEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn wordWrap<T: QLabel_wordWrap>(&mut self, value: T) -> i32 {
    value.wordWrap(self);
    return 1;
  }
}

pub trait QLabel_wordWrap {
  fn wordWrap(self, this: &mut QLabel) -> i32;
}

// proto: bool QLabel::wordWrap();
impl<'a> /*trait*/ QLabel_wordWrap for () {
  fn wordWrap(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel8wordWrapEv()};
    unsafe {_ZNK6QLabel8wordWrapEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setWordWrap<T: QLabel_setWordWrap>(&mut self, value: T) -> i32 {
    value.setWordWrap(self);
    return 1;
  }
}

pub trait QLabel_setWordWrap {
  fn setWordWrap(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setWordWrap(bool on);
impl<'a> /*trait*/ QLabel_setWordWrap for (i8) {
  fn setWordWrap(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel11setWordWrapEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN6QLabel11setWordWrapEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn clear<T: QLabel_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QLabel_clear {
  fn clear(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::clear();
impl<'a> /*trait*/ QLabel_clear for () {
  fn clear(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel5clearEv()};
    unsafe {_ZN6QLabel5clearEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setMargin<T: QLabel_setMargin>(&mut self, value: T) -> i32 {
    value.setMargin(self);
    return 1;
  }
}

pub trait QLabel_setMargin {
  fn setMargin(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setMargin(int );
impl<'a> /*trait*/ QLabel_setMargin for (i32) {
  fn setMargin(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel9setMarginEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QLabel9setMarginEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn minimumSizeHint<T: QLabel_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QLabel_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QLabel) -> i32;
}

// proto: QSize QLabel::minimumSizeHint();
impl<'a> /*trait*/ QLabel_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel15minimumSizeHintEv()};
    unsafe {_ZNK6QLabel15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn selectionStart<T: QLabel_selectionStart>(&mut self, value: T) -> i32 {
    value.selectionStart(self);
    return 1;
  }
}

pub trait QLabel_selectionStart {
  fn selectionStart(self, this: &mut QLabel) -> i32;
}

// proto: int QLabel::selectionStart();
impl<'a> /*trait*/ QLabel_selectionStart for () {
  fn selectionStart(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel14selectionStartEv()};
    unsafe {_ZNK6QLabel14selectionStartEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn hasSelectedText<T: QLabel_hasSelectedText>(&mut self, value: T) -> i32 {
    value.hasSelectedText(self);
    return 1;
  }
}

pub trait QLabel_hasSelectedText {
  fn hasSelectedText(self, this: &mut QLabel) -> i32;
}

// proto: bool QLabel::hasSelectedText();
impl<'a> /*trait*/ QLabel_hasSelectedText for () {
  fn hasSelectedText(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel15hasSelectedTextEv()};
    unsafe {_ZNK6QLabel15hasSelectedTextEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn linkActivated<T: QLabel_linkActivated>(&mut self, value: T) -> i32 {
    value.linkActivated(self);
    return 1;
  }
}

pub trait QLabel_linkActivated {
  fn linkActivated(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::linkActivated(const QString & link);
impl<'a> /*trait*/ QLabel_linkActivated for (&'a  QString) {
  fn linkActivated(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel13linkActivatedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLabel13linkActivatedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setBuddy<T: QLabel_setBuddy>(&mut self, value: T) -> i32 {
    value.setBuddy(self);
    return 1;
  }
}

pub trait QLabel_setBuddy {
  fn setBuddy(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setBuddy(QWidget * );
impl<'a> /*trait*/ QLabel_setBuddy for (&'a mut QWidget) {
  fn setBuddy(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel8setBuddyEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QLabel8setBuddyEP7QWidget(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLabelC1ERKS_(qthis, arg0)};
    let rsthis = QLabel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn indent<T: QLabel_indent>(&mut self, value: T) -> i32 {
    value.indent(self);
    return 1;
  }
}

pub trait QLabel_indent {
  fn indent(self, this: &mut QLabel) -> i32;
}

// proto: int QLabel::indent();
impl<'a> /*trait*/ QLabel_indent for () {
  fn indent(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6indentEv()};
    unsafe {_ZNK6QLabel6indentEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn sizeHint<T: QLabel_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QLabel_sizeHint {
  fn sizeHint(self, this: &mut QLabel) -> i32;
}

// proto: QSize QLabel::sizeHint();
impl<'a> /*trait*/ QLabel_sizeHint for () {
  fn sizeHint(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel8sizeHintEv()};
    unsafe {_ZNK6QLabel8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn margin<T: QLabel_margin>(&mut self, value: T) -> i32 {
    value.margin(self);
    return 1;
  }
}

pub trait QLabel_margin {
  fn margin(self, this: &mut QLabel) -> i32;
}

// proto: int QLabel::margin();
impl<'a> /*trait*/ QLabel_margin for () {
  fn margin(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel6marginEv()};
    unsafe {_ZNK6QLabel6marginEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn movie<T: QLabel_movie>(&mut self, value: T) -> i32 {
    value.movie(self);
    return 1;
  }
}

pub trait QLabel_movie {
  fn movie(self, this: &mut QLabel) -> i32;
}

// proto: QMovie * QLabel::movie();
impl<'a> /*trait*/ QLabel_movie for () {
  fn movie(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel5movieEv()};
    unsafe {_ZNK6QLabel5movieEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setScaledContents<T: QLabel_setScaledContents>(&mut self, value: T) -> i32 {
    value.setScaledContents(self);
    return 1;
  }
}

pub trait QLabel_setScaledContents {
  fn setScaledContents(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setScaledContents(bool );
impl<'a> /*trait*/ QLabel_setScaledContents for (i8) {
  fn setScaledContents(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel17setScaledContentsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN6QLabel17setScaledContentsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn setMovie<T: QLabel_setMovie>(&mut self, value: T) -> i32 {
    value.setMovie(self);
    return 1;
  }
}

pub trait QLabel_setMovie {
  fn setMovie(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::setMovie(QMovie * movie);
impl<'a> /*trait*/ QLabel_setMovie for (&'a mut QMovie) {
  fn setMovie(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel8setMovieEP6QMovie()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QLabel8setMovieEP6QMovie(arg0)};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn selectedText<T: QLabel_selectedText>(&mut self, value: T) -> i32 {
    value.selectedText(self);
    return 1;
  }
}

pub trait QLabel_selectedText {
  fn selectedText(self, this: &mut QLabel) -> i32;
}

// proto: QString QLabel::selectedText();
impl<'a> /*trait*/ QLabel_selectedText for () {
  fn selectedText(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLabel12selectedTextEv()};
    unsafe {_ZNK6QLabel12selectedTextEv()};
    return 1;
  }
}

impl /*struct*/ QLabel {
  pub fn linkHovered<T: QLabel_linkHovered>(&mut self, value: T) -> i32 {
    value.linkHovered(self);
    return 1;
  }
}

pub trait QLabel_linkHovered {
  fn linkHovered(self, this: &mut QLabel) -> i32;
}

// proto: void QLabel::linkHovered(const QString & link);
impl<'a> /*trait*/ QLabel_linkHovered for (&'a  QString) {
  fn linkHovered(self, this: &mut QLabel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLabel11linkHoveredERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLabel11linkHoveredERK7QString(arg0)};
    return 1;
  }
}

