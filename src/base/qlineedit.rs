// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qevent::QEvent;
use super::qmenu::QMenu;
use super::qmargins::QMargins;
use super::qpoint::QPoint;
use super::qsize::QSize;
use super::qvalidator::QValidator;
use super::qcompleter::QCompleter;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QLineEdit::cursorBackward(bool mark, int steps);
  fn _ZN9QLineEdit14cursorBackwardEbi(qthis: *mut c_void, arg0: int8_t, arg1: c_int) ;
  // proto:  void QLineEdit::home(bool mark);
  fn _ZN9QLineEdit4homeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QLineEdit::selectionStart();
  fn _ZNK9QLineEdit14selectionStartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QLineEdit::setCursorPosition(int );
  fn _ZN9QLineEdit17setCursorPositionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QLineEdit::isRedoAvailable();
  fn _ZNK9QLineEdit15isRedoAvailableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLineEdit::setModified(bool );
  fn _ZN9QLineEdit11setModifiedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QLineEdit::NewQLineEdit(const QString & , QWidget * parent);
  fn _ZN9QLineEditC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QLineEdit::event(QEvent * );
  fn _ZN9QLineEdit5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  int QLineEdit::maxLength();
  fn _ZNK9QLineEdit9maxLengthEv(qthis: *mut c_void) -> c_int;
  // proto:  QMenu * QLineEdit::createStandardContextMenu();
  fn _ZN9QLineEdit25createStandardContextMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLineEdit::setTextMargins(const QMargins & margins);
  fn _ZN9QLineEdit14setTextMarginsERK8QMargins(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QLineEdit::cursorPositionAt(const QPoint & pos);
  fn _ZN9QLineEdit16cursorPositionAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  bool QLineEdit::hasSelectedText();
  fn _ZNK9QLineEdit15hasSelectedTextEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLineEdit::setPlaceholderText(const QString & );
  fn _ZN9QLineEdit18setPlaceholderTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QLineEdit::minimumSizeHint();
  fn _ZNK9QLineEdit15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLineEdit::cursorForward(bool mark, int steps);
  fn _ZN9QLineEdit13cursorForwardEbi(qthis: *mut c_void, arg0: int8_t, arg1: c_int) ;
  // proto:  void QLineEdit::insert(const QString & );
  fn _ZN9QLineEdit6insertERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLineEdit::setText(const QString & );
  fn _ZN9QLineEdit7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLineEdit::selectionChanged();
  fn _ZN9QLineEdit16selectionChangedEv(qthis: *mut c_void) ;
  // proto:  const QValidator * QLineEdit::validator();
  fn _ZNK9QLineEdit9validatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLineEdit::deselect();
  fn _ZN9QLineEdit8deselectEv(qthis: *mut c_void) ;
  // proto:  void QLineEdit::returnPressed();
  fn _ZN9QLineEdit13returnPressedEv(qthis: *mut c_void) ;
  // proto:  QString QLineEdit::inputMask();
  fn _ZNK9QLineEdit9inputMaskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QLineEdit::placeholderText();
  fn _ZNK9QLineEdit15placeholderTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLineEdit::cut();
  fn _ZN9QLineEdit3cutEv(qthis: *mut c_void) ;
  // proto:  QString QLineEdit::text();
  fn _ZNK9QLineEdit4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QLineEdit::metaObject();
  fn _ZNK9QLineEdit10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QLineEdit::del();
  fn _ZN9QLineEdit3delEv(qthis: *mut c_void) ;
  // proto:  bool QLineEdit::isModified();
  fn _ZNK9QLineEdit10isModifiedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLineEdit::editingFinished();
  fn _ZN9QLineEdit15editingFinishedEv(qthis: *mut c_void) ;
  // proto:  void QLineEdit::cursorWordForward(bool mark);
  fn _ZN9QLineEdit17cursorWordForwardEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QLineEdit::selectAll();
  fn _ZN9QLineEdit9selectAllEv(qthis: *mut c_void) ;
  // proto:  void QLineEdit::setSelection(int , int );
  fn _ZN9QLineEdit12setSelectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QLineEdit::setCompleter(QCompleter * completer);
  fn _ZN9QLineEdit12setCompleterEP10QCompleter(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLineEdit::setMaxLength(int );
  fn _ZN9QLineEdit12setMaxLengthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QLineEdit::FreeQLineEdit();
  fn _ZN9QLineEditD0Ev(qthis: *mut c_void) ;
  // proto:  void QLineEdit::textEdited(const QString & );
  fn _ZN9QLineEdit10textEditedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLineEdit::setReadOnly(bool );
  fn _ZN9QLineEdit11setReadOnlyEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QLineEdit::displayText();
  fn _ZNK9QLineEdit11displayTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLineEdit::setFrame(bool );
  fn _ZN9QLineEdit8setFrameEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QLineEdit::hasAcceptableInput();
  fn _ZNK9QLineEdit18hasAcceptableInputEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QLineEdit::hasFrame();
  fn _ZNK9QLineEdit8hasFrameEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QLineEdit::cursorPosition();
  fn _ZNK9QLineEdit14cursorPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QLineEdit::cursorWordBackward(bool mark);
  fn _ZN9QLineEdit18cursorWordBackwardEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QLineEdit::dragEnabled();
  fn _ZNK9QLineEdit11dragEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLineEdit::textChanged(const QString & );
  fn _ZN9QLineEdit11textChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QLineEdit::sizeHint();
  fn _ZNK9QLineEdit8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLineEdit::paste();
  fn _ZN9QLineEdit5pasteEv(qthis: *mut c_void) ;
  // proto:  void QLineEdit::setValidator(const QValidator * );
  fn _ZN9QLineEdit12setValidatorEPK10QValidator(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLineEdit::NewQLineEdit(QWidget * parent);
  fn _ZN9QLineEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QCompleter * QLineEdit::completer();
  fn _ZNK9QLineEdit9completerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMargins QLineEdit::textMargins();
  fn _ZNK9QLineEdit11textMarginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLineEdit::setClearButtonEnabled(bool enable);
  fn _ZN9QLineEdit21setClearButtonEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QLineEdit::selectedText();
  fn _ZNK9QLineEdit12selectedTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLineEdit::clear();
  fn _ZN9QLineEdit5clearEv(qthis: *mut c_void) ;
  // proto:  void QLineEdit::cursorPositionChanged(int , int );
  fn _ZN9QLineEdit21cursorPositionChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QLineEdit::copy();
  fn _ZNK9QLineEdit4copyEv(qthis: *mut c_void) ;
  // proto:  bool QLineEdit::isUndoAvailable();
  fn _ZNK9QLineEdit15isUndoAvailableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLineEdit::undo();
  fn _ZN9QLineEdit4undoEv(qthis: *mut c_void) ;
  // proto:  bool QLineEdit::isClearButtonEnabled();
  fn _ZNK9QLineEdit20isClearButtonEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLineEdit::NewQLineEdit(const QLineEdit & );
  fn _ZN9QLineEditC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLineEdit::end(bool mark);
  fn _ZN9QLineEdit3endEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QLineEdit::setDragEnabled(bool b);
  fn _ZN9QLineEdit14setDragEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QLineEdit::backspace();
  fn _ZN9QLineEdit9backspaceEv(qthis: *mut c_void) ;
  // proto:  void QLineEdit::redo();
  fn _ZN9QLineEdit4redoEv(qthis: *mut c_void) ;
  // proto:  void QLineEdit::setTextMargins(int left, int top, int right, int bottom);
  fn _ZN9QLineEdit14setTextMarginsEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QLineEdit::setInputMask(const QString & inputMask);
  fn _ZN9QLineEdit12setInputMaskERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLineEdit::getTextMargins(int * left, int * top, int * right, int * bottom);
  fn _ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  bool QLineEdit::isReadOnly();
  fn _ZNK9QLineEdit10isReadOnlyEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QLineEdit)=1
pub struct QLineEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLineEdit {
  pub fn cursorBackward<T: QLineEdit_cursorBackward>(&mut self, value: T)  {
     value.cursorBackward(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorBackward {
  fn cursorBackward(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::cursorBackward(bool mark, int steps);
impl<'a> /*trait*/ QLineEdit_cursorBackward for (i8, i32) {
  fn cursorBackward(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14cursorBackwardEbi()};
    let arg0 = self.0  as int8_t;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QLineEdit14cursorBackwardEbi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn home<T: QLineEdit_home>(&mut self, value: T)  {
     value.home(self);
    // return 1;
  }
}

pub trait QLineEdit_home {
  fn home(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::home(bool mark);
impl<'a> /*trait*/ QLineEdit_home for (i8) {
  fn home(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4homeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit4homeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn selectionStart<T: QLineEdit_selectionStart>(&mut self, value: T) -> i32 {
    return value.selectionStart(self);
    // return 1;
  }
}

pub trait QLineEdit_selectionStart {
  fn selectionStart(self, rsthis: &mut QLineEdit) -> i32;
}

// proto:  int QLineEdit::selectionStart();
impl<'a> /*trait*/ QLineEdit_selectionStart for () {
  fn selectionStart(self, rsthis: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14selectionStartEv()};
    let mut ret = unsafe {_ZNK9QLineEdit14selectionStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setCursorPosition<T: QLineEdit_setCursorPosition>(&mut self, value: T)  {
     value.setCursorPosition(self);
    // return 1;
  }
}

pub trait QLineEdit_setCursorPosition {
  fn setCursorPosition(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setCursorPosition(int );
impl<'a> /*trait*/ QLineEdit_setCursorPosition for (i32) {
  fn setCursorPosition(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit17setCursorPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QLineEdit17setCursorPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isRedoAvailable<T: QLineEdit_isRedoAvailable>(&mut self, value: T) -> i8 {
    return value.isRedoAvailable(self);
    // return 1;
  }
}

pub trait QLineEdit_isRedoAvailable {
  fn isRedoAvailable(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::isRedoAvailable();
impl<'a> /*trait*/ QLineEdit_isRedoAvailable for () {
  fn isRedoAvailable(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15isRedoAvailableEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15isRedoAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setModified<T: QLineEdit_setModified>(&mut self, value: T)  {
     value.setModified(self);
    // return 1;
  }
}

pub trait QLineEdit_setModified {
  fn setModified(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setModified(bool );
impl<'a> /*trait*/ QLineEdit_setModified for (i8) {
  fn setModified(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11setModifiedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit11setModifiedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn NewQLineEdit<T: QLineEdit_NewQLineEdit>(value: T) -> QLineEdit {
    let rsthis = value.NewQLineEdit();
    return rsthis;
    // return 1;
  }
}

pub trait QLineEdit_NewQLineEdit {
  fn NewQLineEdit(self) -> QLineEdit;
}

// proto: void QLineEdit::NewQLineEdit(const QString & , QWidget * parent);
impl<'a> /*trait*/ QLineEdit_NewQLineEdit for (&'a  QString, &'a mut QWidget) {
  fn NewQLineEdit(self) -> QLineEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QLineEditC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QLineEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn event<T: QLineEdit_event>(&mut self, value: T) -> i8 {
    return value.event(self);
    // return 1;
  }
}

pub trait QLineEdit_event {
  fn event(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::event(QEvent * );
impl<'a> /*trait*/ QLineEdit_event for (&'a mut QEvent) {
  fn event(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QLineEdit5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn maxLength<T: QLineEdit_maxLength>(&mut self, value: T) -> i32 {
    return value.maxLength(self);
    // return 1;
  }
}

pub trait QLineEdit_maxLength {
  fn maxLength(self, rsthis: &mut QLineEdit) -> i32;
}

// proto:  int QLineEdit::maxLength();
impl<'a> /*trait*/ QLineEdit_maxLength for () {
  fn maxLength(self, rsthis: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9maxLengthEv()};
    let mut ret = unsafe {_ZNK9QLineEdit9maxLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn createStandardContextMenu<T: QLineEdit_createStandardContextMenu>(&mut self, value: T) -> QMenu {
    return value.createStandardContextMenu(self);
    // return 1;
  }
}

pub trait QLineEdit_createStandardContextMenu {
  fn createStandardContextMenu(self, rsthis: &mut QLineEdit) -> QMenu;
}

// proto:  QMenu * QLineEdit::createStandardContextMenu();
impl<'a> /*trait*/ QLineEdit_createStandardContextMenu for () {
  fn createStandardContextMenu(self, rsthis: &mut QLineEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit25createStandardContextMenuEv()};
    let mut ret = unsafe {_ZN9QLineEdit25createStandardContextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setTextMargins<T: QLineEdit_setTextMargins>(&mut self, value: T)  {
     value.setTextMargins(self);
    // return 1;
  }
}

pub trait QLineEdit_setTextMargins {
  fn setTextMargins(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setTextMargins(const QMargins & margins);
impl<'a> /*trait*/ QLineEdit_setTextMargins for (&'a  QMargins) {
  fn setTextMargins(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14setTextMarginsERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit14setTextMarginsERK8QMargins(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorPositionAt<T: QLineEdit_cursorPositionAt>(&mut self, value: T) -> i32 {
    return value.cursorPositionAt(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorPositionAt {
  fn cursorPositionAt(self, rsthis: &mut QLineEdit) -> i32;
}

// proto:  int QLineEdit::cursorPositionAt(const QPoint & pos);
impl<'a> /*trait*/ QLineEdit_cursorPositionAt for (&'a  QPoint) {
  fn cursorPositionAt(self, rsthis: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit16cursorPositionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QLineEdit16cursorPositionAtERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn hasSelectedText<T: QLineEdit_hasSelectedText>(&mut self, value: T) -> i8 {
    return value.hasSelectedText(self);
    // return 1;
  }
}

pub trait QLineEdit_hasSelectedText {
  fn hasSelectedText(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::hasSelectedText();
impl<'a> /*trait*/ QLineEdit_hasSelectedText for () {
  fn hasSelectedText(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15hasSelectedTextEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15hasSelectedTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setPlaceholderText<T: QLineEdit_setPlaceholderText>(&mut self, value: T)  {
     value.setPlaceholderText(self);
    // return 1;
  }
}

pub trait QLineEdit_setPlaceholderText {
  fn setPlaceholderText(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setPlaceholderText(const QString & );
impl<'a> /*trait*/ QLineEdit_setPlaceholderText for (&'a  QString) {
  fn setPlaceholderText(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit18setPlaceholderTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn minimumSizeHint<T: QLineEdit_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QLineEdit_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QLineEdit) -> QSize;
}

// proto:  QSize QLineEdit::minimumSizeHint();
impl<'a> /*trait*/ QLineEdit_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QLineEdit) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorForward<T: QLineEdit_cursorForward>(&mut self, value: T)  {
     value.cursorForward(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorForward {
  fn cursorForward(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::cursorForward(bool mark, int steps);
impl<'a> /*trait*/ QLineEdit_cursorForward for (i8, i32) {
  fn cursorForward(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit13cursorForwardEbi()};
    let arg0 = self.0  as int8_t;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QLineEdit13cursorForwardEbi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn insert<T: QLineEdit_insert>(&mut self, value: T)  {
     value.insert(self);
    // return 1;
  }
}

pub trait QLineEdit_insert {
  fn insert(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::insert(const QString & );
impl<'a> /*trait*/ QLineEdit_insert for (&'a  QString) {
  fn insert(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit6insertERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit6insertERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setText<T: QLineEdit_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QLineEdit_setText {
  fn setText(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setText(const QString & );
impl<'a> /*trait*/ QLineEdit_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn selectionChanged<T: QLineEdit_selectionChanged>(&mut self, value: T)  {
     value.selectionChanged(self);
    // return 1;
  }
}

pub trait QLineEdit_selectionChanged {
  fn selectionChanged(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::selectionChanged();
impl<'a> /*trait*/ QLineEdit_selectionChanged for () {
  fn selectionChanged(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit16selectionChangedEv()};
     unsafe {_ZN9QLineEdit16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn validator<T: QLineEdit_validator>(&mut self, value: T) -> QValidator {
    return value.validator(self);
    // return 1;
  }
}

pub trait QLineEdit_validator {
  fn validator(self, rsthis: &mut QLineEdit) -> QValidator;
}

// proto:  const QValidator * QLineEdit::validator();
impl<'a> /*trait*/ QLineEdit_validator for () {
  fn validator(self, rsthis: &mut QLineEdit) -> QValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9validatorEv()};
    let mut ret = unsafe {_ZNK9QLineEdit9validatorEv(rsthis.qclsinst)};
    let mut ret1 = QValidator{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn deselect<T: QLineEdit_deselect>(&mut self, value: T)  {
     value.deselect(self);
    // return 1;
  }
}

pub trait QLineEdit_deselect {
  fn deselect(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::deselect();
impl<'a> /*trait*/ QLineEdit_deselect for () {
  fn deselect(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit8deselectEv()};
     unsafe {_ZN9QLineEdit8deselectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn returnPressed<T: QLineEdit_returnPressed>(&mut self, value: T)  {
     value.returnPressed(self);
    // return 1;
  }
}

pub trait QLineEdit_returnPressed {
  fn returnPressed(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::returnPressed();
impl<'a> /*trait*/ QLineEdit_returnPressed for () {
  fn returnPressed(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit13returnPressedEv()};
     unsafe {_ZN9QLineEdit13returnPressedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn inputMask<T: QLineEdit_inputMask>(&mut self, value: T) -> QString {
    return value.inputMask(self);
    // return 1;
  }
}

pub trait QLineEdit_inputMask {
  fn inputMask(self, rsthis: &mut QLineEdit) -> QString;
}

// proto:  QString QLineEdit::inputMask();
impl<'a> /*trait*/ QLineEdit_inputMask for () {
  fn inputMask(self, rsthis: &mut QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9inputMaskEv()};
    let mut ret = unsafe {_ZNK9QLineEdit9inputMaskEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn placeholderText<T: QLineEdit_placeholderText>(&mut self, value: T) -> QString {
    return value.placeholderText(self);
    // return 1;
  }
}

pub trait QLineEdit_placeholderText {
  fn placeholderText(self, rsthis: &mut QLineEdit) -> QString;
}

// proto:  QString QLineEdit::placeholderText();
impl<'a> /*trait*/ QLineEdit_placeholderText for () {
  fn placeholderText(self, rsthis: &mut QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15placeholderTextEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15placeholderTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cut<T: QLineEdit_cut>(&mut self, value: T)  {
     value.cut(self);
    // return 1;
  }
}

pub trait QLineEdit_cut {
  fn cut(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::cut();
impl<'a> /*trait*/ QLineEdit_cut for () {
  fn cut(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3cutEv()};
     unsafe {_ZN9QLineEdit3cutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn text<T: QLineEdit_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QLineEdit_text {
  fn text(self, rsthis: &mut QLineEdit) -> QString;
}

// proto:  QString QLineEdit::text();
impl<'a> /*trait*/ QLineEdit_text for () {
  fn text(self, rsthis: &mut QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit4textEv()};
    let mut ret = unsafe {_ZNK9QLineEdit4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn metaObject<T: QLineEdit_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QLineEdit_metaObject {
  fn metaObject(self, rsthis: &mut QLineEdit) ;
}

// proto:  const QMetaObject * QLineEdit::metaObject();
impl<'a> /*trait*/ QLineEdit_metaObject for () {
  fn metaObject(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10metaObjectEv()};
     unsafe {_ZNK9QLineEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn del<T: QLineEdit_del>(&mut self, value: T)  {
     value.del(self);
    // return 1;
  }
}

pub trait QLineEdit_del {
  fn del(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::del();
impl<'a> /*trait*/ QLineEdit_del for () {
  fn del(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3delEv()};
     unsafe {_ZN9QLineEdit3delEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isModified<T: QLineEdit_isModified>(&mut self, value: T) -> i8 {
    return value.isModified(self);
    // return 1;
  }
}

pub trait QLineEdit_isModified {
  fn isModified(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::isModified();
impl<'a> /*trait*/ QLineEdit_isModified for () {
  fn isModified(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10isModifiedEv()};
    let mut ret = unsafe {_ZNK9QLineEdit10isModifiedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn editingFinished<T: QLineEdit_editingFinished>(&mut self, value: T)  {
     value.editingFinished(self);
    // return 1;
  }
}

pub trait QLineEdit_editingFinished {
  fn editingFinished(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::editingFinished();
impl<'a> /*trait*/ QLineEdit_editingFinished for () {
  fn editingFinished(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit15editingFinishedEv()};
     unsafe {_ZN9QLineEdit15editingFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorWordForward<T: QLineEdit_cursorWordForward>(&mut self, value: T)  {
     value.cursorWordForward(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorWordForward {
  fn cursorWordForward(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::cursorWordForward(bool mark);
impl<'a> /*trait*/ QLineEdit_cursorWordForward for (i8) {
  fn cursorWordForward(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit17cursorWordForwardEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit17cursorWordForwardEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn selectAll<T: QLineEdit_selectAll>(&mut self, value: T)  {
     value.selectAll(self);
    // return 1;
  }
}

pub trait QLineEdit_selectAll {
  fn selectAll(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::selectAll();
impl<'a> /*trait*/ QLineEdit_selectAll for () {
  fn selectAll(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit9selectAllEv()};
     unsafe {_ZN9QLineEdit9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setSelection<T: QLineEdit_setSelection>(&mut self, value: T)  {
     value.setSelection(self);
    // return 1;
  }
}

pub trait QLineEdit_setSelection {
  fn setSelection(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setSelection(int , int );
impl<'a> /*trait*/ QLineEdit_setSelection for (i32, i32) {
  fn setSelection(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QLineEdit12setSelectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setCompleter<T: QLineEdit_setCompleter>(&mut self, value: T)  {
     value.setCompleter(self);
    // return 1;
  }
}

pub trait QLineEdit_setCompleter {
  fn setCompleter(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setCompleter(QCompleter * completer);
impl<'a> /*trait*/ QLineEdit_setCompleter for (&'a mut QCompleter) {
  fn setCompleter(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setCompleterEP10QCompleter()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit12setCompleterEP10QCompleter(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setMaxLength<T: QLineEdit_setMaxLength>(&mut self, value: T)  {
     value.setMaxLength(self);
    // return 1;
  }
}

pub trait QLineEdit_setMaxLength {
  fn setMaxLength(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setMaxLength(int );
impl<'a> /*trait*/ QLineEdit_setMaxLength for (i32) {
  fn setMaxLength(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setMaxLengthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QLineEdit12setMaxLengthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn FreeQLineEdit<T: QLineEdit_FreeQLineEdit>(&mut self, value: T)  {
     value.FreeQLineEdit(self);
    // return 1;
  }
}

pub trait QLineEdit_FreeQLineEdit {
  fn FreeQLineEdit(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::FreeQLineEdit();
impl<'a> /*trait*/ QLineEdit_FreeQLineEdit for () {
  fn FreeQLineEdit(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditD0Ev()};
     unsafe {_ZN9QLineEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn textEdited<T: QLineEdit_textEdited>(&mut self, value: T)  {
     value.textEdited(self);
    // return 1;
  }
}

pub trait QLineEdit_textEdited {
  fn textEdited(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::textEdited(const QString & );
impl<'a> /*trait*/ QLineEdit_textEdited for (&'a  QString) {
  fn textEdited(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit10textEditedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit10textEditedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setReadOnly<T: QLineEdit_setReadOnly>(&mut self, value: T)  {
     value.setReadOnly(self);
    // return 1;
  }
}

pub trait QLineEdit_setReadOnly {
  fn setReadOnly(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setReadOnly(bool );
impl<'a> /*trait*/ QLineEdit_setReadOnly for (i8) {
  fn setReadOnly(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11setReadOnlyEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn displayText<T: QLineEdit_displayText>(&mut self, value: T) -> QString {
    return value.displayText(self);
    // return 1;
  }
}

pub trait QLineEdit_displayText {
  fn displayText(self, rsthis: &mut QLineEdit) -> QString;
}

// proto:  QString QLineEdit::displayText();
impl<'a> /*trait*/ QLineEdit_displayText for () {
  fn displayText(self, rsthis: &mut QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11displayTextEv()};
    let mut ret = unsafe {_ZNK9QLineEdit11displayTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setFrame<T: QLineEdit_setFrame>(&mut self, value: T)  {
     value.setFrame(self);
    // return 1;
  }
}

pub trait QLineEdit_setFrame {
  fn setFrame(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setFrame(bool );
impl<'a> /*trait*/ QLineEdit_setFrame for (i8) {
  fn setFrame(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit8setFrameEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit8setFrameEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn hasAcceptableInput<T: QLineEdit_hasAcceptableInput>(&mut self, value: T) -> i8 {
    return value.hasAcceptableInput(self);
    // return 1;
  }
}

pub trait QLineEdit_hasAcceptableInput {
  fn hasAcceptableInput(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::hasAcceptableInput();
impl<'a> /*trait*/ QLineEdit_hasAcceptableInput for () {
  fn hasAcceptableInput(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit18hasAcceptableInputEv()};
    let mut ret = unsafe {_ZNK9QLineEdit18hasAcceptableInputEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn hasFrame<T: QLineEdit_hasFrame>(&mut self, value: T) -> i8 {
    return value.hasFrame(self);
    // return 1;
  }
}

pub trait QLineEdit_hasFrame {
  fn hasFrame(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::hasFrame();
impl<'a> /*trait*/ QLineEdit_hasFrame for () {
  fn hasFrame(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit8hasFrameEv()};
    let mut ret = unsafe {_ZNK9QLineEdit8hasFrameEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorPosition<T: QLineEdit_cursorPosition>(&mut self, value: T) -> i32 {
    return value.cursorPosition(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorPosition {
  fn cursorPosition(self, rsthis: &mut QLineEdit) -> i32;
}

// proto:  int QLineEdit::cursorPosition();
impl<'a> /*trait*/ QLineEdit_cursorPosition for () {
  fn cursorPosition(self, rsthis: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14cursorPositionEv()};
    let mut ret = unsafe {_ZNK9QLineEdit14cursorPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorWordBackward<T: QLineEdit_cursorWordBackward>(&mut self, value: T)  {
     value.cursorWordBackward(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorWordBackward {
  fn cursorWordBackward(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::cursorWordBackward(bool mark);
impl<'a> /*trait*/ QLineEdit_cursorWordBackward for (i8) {
  fn cursorWordBackward(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit18cursorWordBackwardEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit18cursorWordBackwardEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn dragEnabled<T: QLineEdit_dragEnabled>(&mut self, value: T) -> i8 {
    return value.dragEnabled(self);
    // return 1;
  }
}

pub trait QLineEdit_dragEnabled {
  fn dragEnabled(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::dragEnabled();
impl<'a> /*trait*/ QLineEdit_dragEnabled for () {
  fn dragEnabled(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11dragEnabledEv()};
    let mut ret = unsafe {_ZNK9QLineEdit11dragEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn textChanged<T: QLineEdit_textChanged>(&mut self, value: T)  {
     value.textChanged(self);
    // return 1;
  }
}

pub trait QLineEdit_textChanged {
  fn textChanged(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::textChanged(const QString & );
impl<'a> /*trait*/ QLineEdit_textChanged for (&'a  QString) {
  fn textChanged(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11textChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit11textChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn sizeHint<T: QLineEdit_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QLineEdit_sizeHint {
  fn sizeHint(self, rsthis: &mut QLineEdit) -> QSize;
}

// proto:  QSize QLineEdit::sizeHint();
impl<'a> /*trait*/ QLineEdit_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QLineEdit) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QLineEdit8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn paste<T: QLineEdit_paste>(&mut self, value: T)  {
     value.paste(self);
    // return 1;
  }
}

pub trait QLineEdit_paste {
  fn paste(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::paste();
impl<'a> /*trait*/ QLineEdit_paste for () {
  fn paste(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5pasteEv()};
     unsafe {_ZN9QLineEdit5pasteEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setValidator<T: QLineEdit_setValidator>(&mut self, value: T)  {
     value.setValidator(self);
    // return 1;
  }
}

pub trait QLineEdit_setValidator {
  fn setValidator(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setValidator(const QValidator * );
impl<'a> /*trait*/ QLineEdit_setValidator for (&'a  QValidator) {
  fn setValidator(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setValidatorEPK10QValidator()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit12setValidatorEPK10QValidator(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QLineEdit::NewQLineEdit(QWidget * parent);
impl<'a> /*trait*/ QLineEdit_NewQLineEdit for (&'a mut QWidget) {
  fn NewQLineEdit(self) -> QLineEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QLineEditC1EP7QWidget(qthis, arg0)};
    let rsthis = QLineEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn completer<T: QLineEdit_completer>(&mut self, value: T) -> QCompleter {
    return value.completer(self);
    // return 1;
  }
}

pub trait QLineEdit_completer {
  fn completer(self, rsthis: &mut QLineEdit) -> QCompleter;
}

// proto:  QCompleter * QLineEdit::completer();
impl<'a> /*trait*/ QLineEdit_completer for () {
  fn completer(self, rsthis: &mut QLineEdit) -> QCompleter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9completerEv()};
    let mut ret = unsafe {_ZNK9QLineEdit9completerEv(rsthis.qclsinst)};
    let mut ret1 = QCompleter{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn textMargins<T: QLineEdit_textMargins>(&mut self, value: T) -> QMargins {
    return value.textMargins(self);
    // return 1;
  }
}

pub trait QLineEdit_textMargins {
  fn textMargins(self, rsthis: &mut QLineEdit) -> QMargins;
}

// proto:  QMargins QLineEdit::textMargins();
impl<'a> /*trait*/ QLineEdit_textMargins for () {
  fn textMargins(self, rsthis: &mut QLineEdit) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11textMarginsEv()};
    let mut ret = unsafe {_ZNK9QLineEdit11textMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setClearButtonEnabled<T: QLineEdit_setClearButtonEnabled>(&mut self, value: T)  {
     value.setClearButtonEnabled(self);
    // return 1;
  }
}

pub trait QLineEdit_setClearButtonEnabled {
  fn setClearButtonEnabled(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setClearButtonEnabled(bool enable);
impl<'a> /*trait*/ QLineEdit_setClearButtonEnabled for (i8) {
  fn setClearButtonEnabled(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit21setClearButtonEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit21setClearButtonEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn selectedText<T: QLineEdit_selectedText>(&mut self, value: T) -> QString {
    return value.selectedText(self);
    // return 1;
  }
}

pub trait QLineEdit_selectedText {
  fn selectedText(self, rsthis: &mut QLineEdit) -> QString;
}

// proto:  QString QLineEdit::selectedText();
impl<'a> /*trait*/ QLineEdit_selectedText for () {
  fn selectedText(self, rsthis: &mut QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit12selectedTextEv()};
    let mut ret = unsafe {_ZNK9QLineEdit12selectedTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn clear<T: QLineEdit_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QLineEdit_clear {
  fn clear(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::clear();
impl<'a> /*trait*/ QLineEdit_clear for () {
  fn clear(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5clearEv()};
     unsafe {_ZN9QLineEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorPositionChanged<T: QLineEdit_cursorPositionChanged>(&mut self, value: T)  {
     value.cursorPositionChanged(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorPositionChanged {
  fn cursorPositionChanged(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::cursorPositionChanged(int , int );
impl<'a> /*trait*/ QLineEdit_cursorPositionChanged for (i32, i32) {
  fn cursorPositionChanged(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit21cursorPositionChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QLineEdit21cursorPositionChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn copy<T: QLineEdit_copy>(&mut self, value: T)  {
     value.copy(self);
    // return 1;
  }
}

pub trait QLineEdit_copy {
  fn copy(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::copy();
impl<'a> /*trait*/ QLineEdit_copy for () {
  fn copy(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit4copyEv()};
     unsafe {_ZNK9QLineEdit4copyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isUndoAvailable<T: QLineEdit_isUndoAvailable>(&mut self, value: T) -> i8 {
    return value.isUndoAvailable(self);
    // return 1;
  }
}

pub trait QLineEdit_isUndoAvailable {
  fn isUndoAvailable(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::isUndoAvailable();
impl<'a> /*trait*/ QLineEdit_isUndoAvailable for () {
  fn isUndoAvailable(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15isUndoAvailableEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15isUndoAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn undo<T: QLineEdit_undo>(&mut self, value: T)  {
     value.undo(self);
    // return 1;
  }
}

pub trait QLineEdit_undo {
  fn undo(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::undo();
impl<'a> /*trait*/ QLineEdit_undo for () {
  fn undo(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4undoEv()};
     unsafe {_ZN9QLineEdit4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isClearButtonEnabled<T: QLineEdit_isClearButtonEnabled>(&mut self, value: T) -> i8 {
    return value.isClearButtonEnabled(self);
    // return 1;
  }
}

pub trait QLineEdit_isClearButtonEnabled {
  fn isClearButtonEnabled(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::isClearButtonEnabled();
impl<'a> /*trait*/ QLineEdit_isClearButtonEnabled for () {
  fn isClearButtonEnabled(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit20isClearButtonEnabledEv()};
    let mut ret = unsafe {_ZNK9QLineEdit20isClearButtonEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QLineEdit::NewQLineEdit(const QLineEdit & );
impl<'a> /*trait*/ QLineEdit_NewQLineEdit for (&'a  QLineEdit) {
  fn NewQLineEdit(self) -> QLineEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QLineEditC1ERKS_(qthis, arg0)};
    let rsthis = QLineEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn end<T: QLineEdit_end>(&mut self, value: T)  {
     value.end(self);
    // return 1;
  }
}

pub trait QLineEdit_end {
  fn end(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::end(bool mark);
impl<'a> /*trait*/ QLineEdit_end for (i8) {
  fn end(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3endEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit3endEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setDragEnabled<T: QLineEdit_setDragEnabled>(&mut self, value: T)  {
     value.setDragEnabled(self);
    // return 1;
  }
}

pub trait QLineEdit_setDragEnabled {
  fn setDragEnabled(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setDragEnabled(bool b);
impl<'a> /*trait*/ QLineEdit_setDragEnabled for (i8) {
  fn setDragEnabled(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14setDragEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QLineEdit14setDragEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn backspace<T: QLineEdit_backspace>(&mut self, value: T)  {
     value.backspace(self);
    // return 1;
  }
}

pub trait QLineEdit_backspace {
  fn backspace(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::backspace();
impl<'a> /*trait*/ QLineEdit_backspace for () {
  fn backspace(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit9backspaceEv()};
     unsafe {_ZN9QLineEdit9backspaceEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn redo<T: QLineEdit_redo>(&mut self, value: T)  {
     value.redo(self);
    // return 1;
  }
}

pub trait QLineEdit_redo {
  fn redo(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::redo();
impl<'a> /*trait*/ QLineEdit_redo for () {
  fn redo(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4redoEv()};
     unsafe {_ZN9QLineEdit4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QLineEdit::setTextMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QLineEdit_setTextMargins for (i32, i32, i32, i32) {
  fn setTextMargins(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14setTextMarginsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN9QLineEdit14setTextMarginsEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setInputMask<T: QLineEdit_setInputMask>(&mut self, value: T)  {
     value.setInputMask(self);
    // return 1;
  }
}

pub trait QLineEdit_setInputMask {
  fn setInputMask(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::setInputMask(const QString & inputMask);
impl<'a> /*trait*/ QLineEdit_setInputMask for (&'a  QString) {
  fn setInputMask(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setInputMaskERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit12setInputMaskERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn getTextMargins<T: QLineEdit_getTextMargins>(&mut self, value: T)  {
     value.getTextMargins(self);
    // return 1;
  }
}

pub trait QLineEdit_getTextMargins {
  fn getTextMargins(self, rsthis: &mut QLineEdit) ;
}

// proto:  void QLineEdit::getTextMargins(int * left, int * top, int * right, int * bottom);
impl<'a> /*trait*/ QLineEdit_getTextMargins for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getTextMargins(self, rsthis: &mut QLineEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isReadOnly<T: QLineEdit_isReadOnly>(&mut self, value: T) -> i8 {
    return value.isReadOnly(self);
    // return 1;
  }
}

pub trait QLineEdit_isReadOnly {
  fn isReadOnly(self, rsthis: &mut QLineEdit) -> i8;
}

// proto:  bool QLineEdit::isReadOnly();
impl<'a> /*trait*/ QLineEdit_isReadOnly for () {
  fn isReadOnly(self, rsthis: &mut QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK9QLineEdit10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

