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
use super::qmargins::QMargins;
use super::qpoint::QPoint;
use super::qcompleter::QCompleter;
use super::qvalidator::QValidator;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QLineEdit::cursorBackward(bool mark, int steps);
  fn _ZN9QLineEdit14cursorBackwardEbi(arg0: int8_t, arg1: c_int) -> i32;
  // proto: void QLineEdit::home(bool mark);
  fn _ZN9QLineEdit4homeEb(arg0: int8_t) -> i32;
  // proto: int QLineEdit::selectionStart();
  fn _ZNK9QLineEdit14selectionStartEv() -> i32;
  // proto: void QLineEdit::setCursorPosition(int );
  fn _ZN9QLineEdit17setCursorPositionEi(arg0: c_int) -> i32;
  // proto: bool QLineEdit::isRedoAvailable();
  fn _ZNK9QLineEdit15isRedoAvailableEv() -> i32;
  // proto: void QLineEdit::setModified(bool );
  fn _ZN9QLineEdit11setModifiedEb(arg0: int8_t) -> i32;
  // proto: void QLineEdit::NewQLineEdit(const QString & , QWidget * parent);
  fn _ZN9QLineEditC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: bool QLineEdit::event(QEvent * );
  fn _ZN9QLineEdit5eventEP6QEvent(arg0: *mut c_void) -> i32;
  // proto: int QLineEdit::maxLength();
  fn _ZNK9QLineEdit9maxLengthEv() -> i32;
  // proto: QMenu * QLineEdit::createStandardContextMenu();
  fn _ZN9QLineEdit25createStandardContextMenuEv() -> i32;
  // proto: void QLineEdit::setTextMargins(const QMargins & margins);
  fn _ZN9QLineEdit14setTextMarginsERK8QMargins(arg0: *const c_void) -> i32;
  // proto: int QLineEdit::cursorPositionAt(const QPoint & pos);
  fn _ZN9QLineEdit16cursorPositionAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: bool QLineEdit::hasSelectedText();
  fn _ZNK9QLineEdit15hasSelectedTextEv() -> i32;
  // proto: void QLineEdit::setPlaceholderText(const QString & );
  fn _ZN9QLineEdit18setPlaceholderTextERK7QString(arg0: *const c_void) -> i32;
  // proto: QSize QLineEdit::minimumSizeHint();
  fn _ZNK9QLineEdit15minimumSizeHintEv() -> i32;
  // proto: void QLineEdit::cursorForward(bool mark, int steps);
  fn _ZN9QLineEdit13cursorForwardEbi(arg0: int8_t, arg1: c_int) -> i32;
  // proto: void QLineEdit::insert(const QString & );
  fn _ZN9QLineEdit6insertERK7QString(arg0: *const c_void) -> i32;
  // proto: void QLineEdit::setText(const QString & );
  fn _ZN9QLineEdit7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QLineEdit::selectionChanged();
  fn _ZN9QLineEdit16selectionChangedEv() -> i32;
  // proto: const QValidator * QLineEdit::validator();
  fn _ZNK9QLineEdit9validatorEv() -> i32;
  // proto: void QLineEdit::deselect();
  fn _ZN9QLineEdit8deselectEv() -> i32;
  // proto: void QLineEdit::returnPressed();
  fn _ZN9QLineEdit13returnPressedEv() -> i32;
  // proto: QString QLineEdit::inputMask();
  fn _ZNK9QLineEdit9inputMaskEv() -> i32;
  // proto: QString QLineEdit::placeholderText();
  fn _ZNK9QLineEdit15placeholderTextEv() -> i32;
  // proto: void QLineEdit::cut();
  fn _ZN9QLineEdit3cutEv() -> i32;
  // proto: QString QLineEdit::text();
  fn _ZNK9QLineEdit4textEv() -> i32;
  // proto: const QMetaObject * QLineEdit::metaObject();
  fn _ZNK9QLineEdit10metaObjectEv() -> i32;
  // proto: void QLineEdit::del();
  fn _ZN9QLineEdit3delEv() -> i32;
  // proto: bool QLineEdit::isModified();
  fn _ZNK9QLineEdit10isModifiedEv() -> i32;
  // proto: void QLineEdit::editingFinished();
  fn _ZN9QLineEdit15editingFinishedEv() -> i32;
  // proto: void QLineEdit::cursorWordForward(bool mark);
  fn _ZN9QLineEdit17cursorWordForwardEb(arg0: int8_t) -> i32;
  // proto: void QLineEdit::selectAll();
  fn _ZN9QLineEdit9selectAllEv() -> i32;
  // proto: void QLineEdit::setSelection(int , int );
  fn _ZN9QLineEdit12setSelectionEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QLineEdit::setCompleter(QCompleter * completer);
  fn _ZN9QLineEdit12setCompleterEP10QCompleter(arg0: *mut c_void) -> i32;
  // proto: void QLineEdit::setMaxLength(int );
  fn _ZN9QLineEdit12setMaxLengthEi(arg0: c_int) -> i32;
  // proto: void QLineEdit::FreeQLineEdit();
  fn _ZN9QLineEditD0Ev() -> i32;
  // proto: void QLineEdit::textEdited(const QString & );
  fn _ZN9QLineEdit10textEditedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QLineEdit::setReadOnly(bool );
  fn _ZN9QLineEdit11setReadOnlyEb(arg0: int8_t) -> i32;
  // proto: QString QLineEdit::displayText();
  fn _ZNK9QLineEdit11displayTextEv() -> i32;
  // proto: void QLineEdit::setFrame(bool );
  fn _ZN9QLineEdit8setFrameEb(arg0: int8_t) -> i32;
  // proto: bool QLineEdit::hasAcceptableInput();
  fn _ZNK9QLineEdit18hasAcceptableInputEv() -> i32;
  // proto: bool QLineEdit::hasFrame();
  fn _ZNK9QLineEdit8hasFrameEv() -> i32;
  // proto: int QLineEdit::cursorPosition();
  fn _ZNK9QLineEdit14cursorPositionEv() -> i32;
  // proto: void QLineEdit::cursorWordBackward(bool mark);
  fn _ZN9QLineEdit18cursorWordBackwardEb(arg0: int8_t) -> i32;
  // proto: bool QLineEdit::dragEnabled();
  fn _ZNK9QLineEdit11dragEnabledEv() -> i32;
  // proto: void QLineEdit::textChanged(const QString & );
  fn _ZN9QLineEdit11textChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: QSize QLineEdit::sizeHint();
  fn _ZNK9QLineEdit8sizeHintEv() -> i32;
  // proto: void QLineEdit::paste();
  fn _ZN9QLineEdit5pasteEv() -> i32;
  // proto: void QLineEdit::setValidator(const QValidator * );
  fn _ZN9QLineEdit12setValidatorEPK10QValidator(arg0: *const c_void) -> i32;
  // proto: void QLineEdit::NewQLineEdit(QWidget * parent);
  fn _ZN9QLineEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QCompleter * QLineEdit::completer();
  fn _ZNK9QLineEdit9completerEv() -> i32;
  // proto: QMargins QLineEdit::textMargins();
  fn _ZNK9QLineEdit11textMarginsEv() -> i32;
  // proto: void QLineEdit::setClearButtonEnabled(bool enable);
  fn _ZN9QLineEdit21setClearButtonEnabledEb(arg0: int8_t) -> i32;
  // proto: QString QLineEdit::selectedText();
  fn _ZNK9QLineEdit12selectedTextEv() -> i32;
  // proto: void QLineEdit::clear();
  fn _ZN9QLineEdit5clearEv() -> i32;
  // proto: void QLineEdit::cursorPositionChanged(int , int );
  fn _ZN9QLineEdit21cursorPositionChangedEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QLineEdit::copy();
  fn _ZNK9QLineEdit4copyEv() -> i32;
  // proto: bool QLineEdit::isUndoAvailable();
  fn _ZNK9QLineEdit15isUndoAvailableEv() -> i32;
  // proto: void QLineEdit::undo();
  fn _ZN9QLineEdit4undoEv() -> i32;
  // proto: bool QLineEdit::isClearButtonEnabled();
  fn _ZNK9QLineEdit20isClearButtonEnabledEv() -> i32;
  // proto: void QLineEdit::NewQLineEdit(const QLineEdit & );
  fn _ZN9QLineEditC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QLineEdit::end(bool mark);
  fn _ZN9QLineEdit3endEb(arg0: int8_t) -> i32;
  // proto: void QLineEdit::setDragEnabled(bool b);
  fn _ZN9QLineEdit14setDragEnabledEb(arg0: int8_t) -> i32;
  // proto: void QLineEdit::backspace();
  fn _ZN9QLineEdit9backspaceEv() -> i32;
  // proto: void QLineEdit::redo();
  fn _ZN9QLineEdit4redoEv() -> i32;
  // proto: void QLineEdit::setTextMargins(int left, int top, int right, int bottom);
  fn _ZN9QLineEdit14setTextMarginsEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QLineEdit::setInputMask(const QString & inputMask);
  fn _ZN9QLineEdit12setInputMaskERK7QString(arg0: *const c_void) -> i32;
  // proto: void QLineEdit::getTextMargins(int * left, int * top, int * right, int * bottom);
  fn _ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: bool QLineEdit::isReadOnly();
  fn _ZNK9QLineEdit10isReadOnlyEv() -> i32;
}

// body block begin
// class sizeof(QLineEdit)=1
pub struct QLineEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLineEdit {
  pub fn cursorBackward<T: QLineEdit_cursorBackward>(&mut self, value: T) -> i32 {
    value.cursorBackward(self);
    return 1;
  }
}

pub trait QLineEdit_cursorBackward {
  fn cursorBackward(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::cursorBackward(bool mark, int steps);
impl<'a> /*trait*/ QLineEdit_cursorBackward for (i8, i32) {
  fn cursorBackward(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14cursorBackwardEbi()};
    let arg0 = self.0  as int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QLineEdit14cursorBackwardEbi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn home<T: QLineEdit_home>(&mut self, value: T) -> i32 {
    value.home(self);
    return 1;
  }
}

pub trait QLineEdit_home {
  fn home(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::home(bool mark);
impl<'a> /*trait*/ QLineEdit_home for (i8) {
  fn home(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4homeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit4homeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn selectionStart<T: QLineEdit_selectionStart>(&mut self, value: T) -> i32 {
    value.selectionStart(self);
    return 1;
  }
}

pub trait QLineEdit_selectionStart {
  fn selectionStart(self, this: &mut QLineEdit) -> i32;
}

// proto: int QLineEdit::selectionStart();
impl<'a> /*trait*/ QLineEdit_selectionStart for () {
  fn selectionStart(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14selectionStartEv()};
    unsafe {_ZNK9QLineEdit14selectionStartEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setCursorPosition<T: QLineEdit_setCursorPosition>(&mut self, value: T) -> i32 {
    value.setCursorPosition(self);
    return 1;
  }
}

pub trait QLineEdit_setCursorPosition {
  fn setCursorPosition(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setCursorPosition(int );
impl<'a> /*trait*/ QLineEdit_setCursorPosition for (i32) {
  fn setCursorPosition(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit17setCursorPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QLineEdit17setCursorPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isRedoAvailable<T: QLineEdit_isRedoAvailable>(&mut self, value: T) -> i32 {
    value.isRedoAvailable(self);
    return 1;
  }
}

pub trait QLineEdit_isRedoAvailable {
  fn isRedoAvailable(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::isRedoAvailable();
impl<'a> /*trait*/ QLineEdit_isRedoAvailable for () {
  fn isRedoAvailable(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15isRedoAvailableEv()};
    unsafe {_ZNK9QLineEdit15isRedoAvailableEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setModified<T: QLineEdit_setModified>(&mut self, value: T) -> i32 {
    value.setModified(self);
    return 1;
  }
}

pub trait QLineEdit_setModified {
  fn setModified(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setModified(bool );
impl<'a> /*trait*/ QLineEdit_setModified for (i8) {
  fn setModified(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11setModifiedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit11setModifiedEb(arg0)};
    return 1;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QLineEditC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QLineEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn event<T: QLineEdit_event>(&mut self, value: T) -> i32 {
    value.event(self);
    return 1;
  }
}

pub trait QLineEdit_event {
  fn event(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::event(QEvent * );
impl<'a> /*trait*/ QLineEdit_event for (&'a mut QEvent) {
  fn event(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QLineEdit5eventEP6QEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn maxLength<T: QLineEdit_maxLength>(&mut self, value: T) -> i32 {
    value.maxLength(self);
    return 1;
  }
}

pub trait QLineEdit_maxLength {
  fn maxLength(self, this: &mut QLineEdit) -> i32;
}

// proto: int QLineEdit::maxLength();
impl<'a> /*trait*/ QLineEdit_maxLength for () {
  fn maxLength(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9maxLengthEv()};
    unsafe {_ZNK9QLineEdit9maxLengthEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn createStandardContextMenu<T: QLineEdit_createStandardContextMenu>(&mut self, value: T) -> i32 {
    value.createStandardContextMenu(self);
    return 1;
  }
}

pub trait QLineEdit_createStandardContextMenu {
  fn createStandardContextMenu(self, this: &mut QLineEdit) -> i32;
}

// proto: QMenu * QLineEdit::createStandardContextMenu();
impl<'a> /*trait*/ QLineEdit_createStandardContextMenu for () {
  fn createStandardContextMenu(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit25createStandardContextMenuEv()};
    unsafe {_ZN9QLineEdit25createStandardContextMenuEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setTextMargins<T: QLineEdit_setTextMargins>(&mut self, value: T) -> i32 {
    value.setTextMargins(self);
    return 1;
  }
}

pub trait QLineEdit_setTextMargins {
  fn setTextMargins(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setTextMargins(const QMargins & margins);
impl<'a> /*trait*/ QLineEdit_setTextMargins for (&'a  QMargins) {
  fn setTextMargins(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14setTextMarginsERK8QMargins()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit14setTextMarginsERK8QMargins(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorPositionAt<T: QLineEdit_cursorPositionAt>(&mut self, value: T) -> i32 {
    value.cursorPositionAt(self);
    return 1;
  }
}

pub trait QLineEdit_cursorPositionAt {
  fn cursorPositionAt(self, this: &mut QLineEdit) -> i32;
}

// proto: int QLineEdit::cursorPositionAt(const QPoint & pos);
impl<'a> /*trait*/ QLineEdit_cursorPositionAt for (&'a  QPoint) {
  fn cursorPositionAt(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit16cursorPositionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit16cursorPositionAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn hasSelectedText<T: QLineEdit_hasSelectedText>(&mut self, value: T) -> i32 {
    value.hasSelectedText(self);
    return 1;
  }
}

pub trait QLineEdit_hasSelectedText {
  fn hasSelectedText(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::hasSelectedText();
impl<'a> /*trait*/ QLineEdit_hasSelectedText for () {
  fn hasSelectedText(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15hasSelectedTextEv()};
    unsafe {_ZNK9QLineEdit15hasSelectedTextEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setPlaceholderText<T: QLineEdit_setPlaceholderText>(&mut self, value: T) -> i32 {
    value.setPlaceholderText(self);
    return 1;
  }
}

pub trait QLineEdit_setPlaceholderText {
  fn setPlaceholderText(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setPlaceholderText(const QString & );
impl<'a> /*trait*/ QLineEdit_setPlaceholderText for (&'a  QString) {
  fn setPlaceholderText(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit18setPlaceholderTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn minimumSizeHint<T: QLineEdit_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QLineEdit_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QLineEdit) -> i32;
}

// proto: QSize QLineEdit::minimumSizeHint();
impl<'a> /*trait*/ QLineEdit_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15minimumSizeHintEv()};
    unsafe {_ZNK9QLineEdit15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorForward<T: QLineEdit_cursorForward>(&mut self, value: T) -> i32 {
    value.cursorForward(self);
    return 1;
  }
}

pub trait QLineEdit_cursorForward {
  fn cursorForward(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::cursorForward(bool mark, int steps);
impl<'a> /*trait*/ QLineEdit_cursorForward for (i8, i32) {
  fn cursorForward(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit13cursorForwardEbi()};
    let arg0 = self.0  as int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QLineEdit13cursorForwardEbi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn insert<T: QLineEdit_insert>(&mut self, value: T) -> i32 {
    value.insert(self);
    return 1;
  }
}

pub trait QLineEdit_insert {
  fn insert(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::insert(const QString & );
impl<'a> /*trait*/ QLineEdit_insert for (&'a  QString) {
  fn insert(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit6insertERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit6insertERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setText<T: QLineEdit_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QLineEdit_setText {
  fn setText(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setText(const QString & );
impl<'a> /*trait*/ QLineEdit_setText for (&'a  QString) {
  fn setText(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit7setTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn selectionChanged<T: QLineEdit_selectionChanged>(&mut self, value: T) -> i32 {
    value.selectionChanged(self);
    return 1;
  }
}

pub trait QLineEdit_selectionChanged {
  fn selectionChanged(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::selectionChanged();
impl<'a> /*trait*/ QLineEdit_selectionChanged for () {
  fn selectionChanged(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit16selectionChangedEv()};
    unsafe {_ZN9QLineEdit16selectionChangedEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn validator<T: QLineEdit_validator>(&mut self, value: T) -> i32 {
    value.validator(self);
    return 1;
  }
}

pub trait QLineEdit_validator {
  fn validator(self, this: &mut QLineEdit) -> i32;
}

// proto: const QValidator * QLineEdit::validator();
impl<'a> /*trait*/ QLineEdit_validator for () {
  fn validator(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9validatorEv()};
    unsafe {_ZNK9QLineEdit9validatorEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn deselect<T: QLineEdit_deselect>(&mut self, value: T) -> i32 {
    value.deselect(self);
    return 1;
  }
}

pub trait QLineEdit_deselect {
  fn deselect(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::deselect();
impl<'a> /*trait*/ QLineEdit_deselect for () {
  fn deselect(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit8deselectEv()};
    unsafe {_ZN9QLineEdit8deselectEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn returnPressed<T: QLineEdit_returnPressed>(&mut self, value: T) -> i32 {
    value.returnPressed(self);
    return 1;
  }
}

pub trait QLineEdit_returnPressed {
  fn returnPressed(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::returnPressed();
impl<'a> /*trait*/ QLineEdit_returnPressed for () {
  fn returnPressed(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit13returnPressedEv()};
    unsafe {_ZN9QLineEdit13returnPressedEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn inputMask<T: QLineEdit_inputMask>(&mut self, value: T) -> i32 {
    value.inputMask(self);
    return 1;
  }
}

pub trait QLineEdit_inputMask {
  fn inputMask(self, this: &mut QLineEdit) -> i32;
}

// proto: QString QLineEdit::inputMask();
impl<'a> /*trait*/ QLineEdit_inputMask for () {
  fn inputMask(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9inputMaskEv()};
    unsafe {_ZNK9QLineEdit9inputMaskEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn placeholderText<T: QLineEdit_placeholderText>(&mut self, value: T) -> i32 {
    value.placeholderText(self);
    return 1;
  }
}

pub trait QLineEdit_placeholderText {
  fn placeholderText(self, this: &mut QLineEdit) -> i32;
}

// proto: QString QLineEdit::placeholderText();
impl<'a> /*trait*/ QLineEdit_placeholderText for () {
  fn placeholderText(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15placeholderTextEv()};
    unsafe {_ZNK9QLineEdit15placeholderTextEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cut<T: QLineEdit_cut>(&mut self, value: T) -> i32 {
    value.cut(self);
    return 1;
  }
}

pub trait QLineEdit_cut {
  fn cut(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::cut();
impl<'a> /*trait*/ QLineEdit_cut for () {
  fn cut(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3cutEv()};
    unsafe {_ZN9QLineEdit3cutEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn text<T: QLineEdit_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QLineEdit_text {
  fn text(self, this: &mut QLineEdit) -> i32;
}

// proto: QString QLineEdit::text();
impl<'a> /*trait*/ QLineEdit_text for () {
  fn text(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit4textEv()};
    unsafe {_ZNK9QLineEdit4textEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn metaObject<T: QLineEdit_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QLineEdit_metaObject {
  fn metaObject(self, this: &mut QLineEdit) -> i32;
}

// proto: const QMetaObject * QLineEdit::metaObject();
impl<'a> /*trait*/ QLineEdit_metaObject for () {
  fn metaObject(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10metaObjectEv()};
    unsafe {_ZNK9QLineEdit10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn del<T: QLineEdit_del>(&mut self, value: T) -> i32 {
    value.del(self);
    return 1;
  }
}

pub trait QLineEdit_del {
  fn del(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::del();
impl<'a> /*trait*/ QLineEdit_del for () {
  fn del(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3delEv()};
    unsafe {_ZN9QLineEdit3delEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isModified<T: QLineEdit_isModified>(&mut self, value: T) -> i32 {
    value.isModified(self);
    return 1;
  }
}

pub trait QLineEdit_isModified {
  fn isModified(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::isModified();
impl<'a> /*trait*/ QLineEdit_isModified for () {
  fn isModified(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10isModifiedEv()};
    unsafe {_ZNK9QLineEdit10isModifiedEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn editingFinished<T: QLineEdit_editingFinished>(&mut self, value: T) -> i32 {
    value.editingFinished(self);
    return 1;
  }
}

pub trait QLineEdit_editingFinished {
  fn editingFinished(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::editingFinished();
impl<'a> /*trait*/ QLineEdit_editingFinished for () {
  fn editingFinished(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit15editingFinishedEv()};
    unsafe {_ZN9QLineEdit15editingFinishedEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorWordForward<T: QLineEdit_cursorWordForward>(&mut self, value: T) -> i32 {
    value.cursorWordForward(self);
    return 1;
  }
}

pub trait QLineEdit_cursorWordForward {
  fn cursorWordForward(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::cursorWordForward(bool mark);
impl<'a> /*trait*/ QLineEdit_cursorWordForward for (i8) {
  fn cursorWordForward(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit17cursorWordForwardEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit17cursorWordForwardEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn selectAll<T: QLineEdit_selectAll>(&mut self, value: T) -> i32 {
    value.selectAll(self);
    return 1;
  }
}

pub trait QLineEdit_selectAll {
  fn selectAll(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::selectAll();
impl<'a> /*trait*/ QLineEdit_selectAll for () {
  fn selectAll(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit9selectAllEv()};
    unsafe {_ZN9QLineEdit9selectAllEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setSelection<T: QLineEdit_setSelection>(&mut self, value: T) -> i32 {
    value.setSelection(self);
    return 1;
  }
}

pub trait QLineEdit_setSelection {
  fn setSelection(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setSelection(int , int );
impl<'a> /*trait*/ QLineEdit_setSelection for (i32, i32) {
  fn setSelection(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QLineEdit12setSelectionEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setCompleter<T: QLineEdit_setCompleter>(&mut self, value: T) -> i32 {
    value.setCompleter(self);
    return 1;
  }
}

pub trait QLineEdit_setCompleter {
  fn setCompleter(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setCompleter(QCompleter * completer);
impl<'a> /*trait*/ QLineEdit_setCompleter for (&'a mut QCompleter) {
  fn setCompleter(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setCompleterEP10QCompleter()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QLineEdit12setCompleterEP10QCompleter(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setMaxLength<T: QLineEdit_setMaxLength>(&mut self, value: T) -> i32 {
    value.setMaxLength(self);
    return 1;
  }
}

pub trait QLineEdit_setMaxLength {
  fn setMaxLength(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setMaxLength(int );
impl<'a> /*trait*/ QLineEdit_setMaxLength for (i32) {
  fn setMaxLength(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setMaxLengthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QLineEdit12setMaxLengthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn FreeQLineEdit<T: QLineEdit_FreeQLineEdit>(&mut self, value: T) -> i32 {
    value.FreeQLineEdit(self);
    return 1;
  }
}

pub trait QLineEdit_FreeQLineEdit {
  fn FreeQLineEdit(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::FreeQLineEdit();
impl<'a> /*trait*/ QLineEdit_FreeQLineEdit for () {
  fn FreeQLineEdit(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditD0Ev()};
    unsafe {_ZN9QLineEditD0Ev()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn textEdited<T: QLineEdit_textEdited>(&mut self, value: T) -> i32 {
    value.textEdited(self);
    return 1;
  }
}

pub trait QLineEdit_textEdited {
  fn textEdited(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::textEdited(const QString & );
impl<'a> /*trait*/ QLineEdit_textEdited for (&'a  QString) {
  fn textEdited(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit10textEditedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit10textEditedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setReadOnly<T: QLineEdit_setReadOnly>(&mut self, value: T) -> i32 {
    value.setReadOnly(self);
    return 1;
  }
}

pub trait QLineEdit_setReadOnly {
  fn setReadOnly(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setReadOnly(bool );
impl<'a> /*trait*/ QLineEdit_setReadOnly for (i8) {
  fn setReadOnly(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11setReadOnlyEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit11setReadOnlyEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn displayText<T: QLineEdit_displayText>(&mut self, value: T) -> i32 {
    value.displayText(self);
    return 1;
  }
}

pub trait QLineEdit_displayText {
  fn displayText(self, this: &mut QLineEdit) -> i32;
}

// proto: QString QLineEdit::displayText();
impl<'a> /*trait*/ QLineEdit_displayText for () {
  fn displayText(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11displayTextEv()};
    unsafe {_ZNK9QLineEdit11displayTextEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setFrame<T: QLineEdit_setFrame>(&mut self, value: T) -> i32 {
    value.setFrame(self);
    return 1;
  }
}

pub trait QLineEdit_setFrame {
  fn setFrame(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setFrame(bool );
impl<'a> /*trait*/ QLineEdit_setFrame for (i8) {
  fn setFrame(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit8setFrameEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit8setFrameEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn hasAcceptableInput<T: QLineEdit_hasAcceptableInput>(&mut self, value: T) -> i32 {
    value.hasAcceptableInput(self);
    return 1;
  }
}

pub trait QLineEdit_hasAcceptableInput {
  fn hasAcceptableInput(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::hasAcceptableInput();
impl<'a> /*trait*/ QLineEdit_hasAcceptableInput for () {
  fn hasAcceptableInput(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit18hasAcceptableInputEv()};
    unsafe {_ZNK9QLineEdit18hasAcceptableInputEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn hasFrame<T: QLineEdit_hasFrame>(&mut self, value: T) -> i32 {
    value.hasFrame(self);
    return 1;
  }
}

pub trait QLineEdit_hasFrame {
  fn hasFrame(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::hasFrame();
impl<'a> /*trait*/ QLineEdit_hasFrame for () {
  fn hasFrame(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit8hasFrameEv()};
    unsafe {_ZNK9QLineEdit8hasFrameEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorPosition<T: QLineEdit_cursorPosition>(&mut self, value: T) -> i32 {
    value.cursorPosition(self);
    return 1;
  }
}

pub trait QLineEdit_cursorPosition {
  fn cursorPosition(self, this: &mut QLineEdit) -> i32;
}

// proto: int QLineEdit::cursorPosition();
impl<'a> /*trait*/ QLineEdit_cursorPosition for () {
  fn cursorPosition(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14cursorPositionEv()};
    unsafe {_ZNK9QLineEdit14cursorPositionEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorWordBackward<T: QLineEdit_cursorWordBackward>(&mut self, value: T) -> i32 {
    value.cursorWordBackward(self);
    return 1;
  }
}

pub trait QLineEdit_cursorWordBackward {
  fn cursorWordBackward(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::cursorWordBackward(bool mark);
impl<'a> /*trait*/ QLineEdit_cursorWordBackward for (i8) {
  fn cursorWordBackward(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit18cursorWordBackwardEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit18cursorWordBackwardEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn dragEnabled<T: QLineEdit_dragEnabled>(&mut self, value: T) -> i32 {
    value.dragEnabled(self);
    return 1;
  }
}

pub trait QLineEdit_dragEnabled {
  fn dragEnabled(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::dragEnabled();
impl<'a> /*trait*/ QLineEdit_dragEnabled for () {
  fn dragEnabled(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11dragEnabledEv()};
    unsafe {_ZNK9QLineEdit11dragEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn textChanged<T: QLineEdit_textChanged>(&mut self, value: T) -> i32 {
    value.textChanged(self);
    return 1;
  }
}

pub trait QLineEdit_textChanged {
  fn textChanged(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::textChanged(const QString & );
impl<'a> /*trait*/ QLineEdit_textChanged for (&'a  QString) {
  fn textChanged(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11textChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit11textChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn sizeHint<T: QLineEdit_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QLineEdit_sizeHint {
  fn sizeHint(self, this: &mut QLineEdit) -> i32;
}

// proto: QSize QLineEdit::sizeHint();
impl<'a> /*trait*/ QLineEdit_sizeHint for () {
  fn sizeHint(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit8sizeHintEv()};
    unsafe {_ZNK9QLineEdit8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn paste<T: QLineEdit_paste>(&mut self, value: T) -> i32 {
    value.paste(self);
    return 1;
  }
}

pub trait QLineEdit_paste {
  fn paste(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::paste();
impl<'a> /*trait*/ QLineEdit_paste for () {
  fn paste(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5pasteEv()};
    unsafe {_ZN9QLineEdit5pasteEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setValidator<T: QLineEdit_setValidator>(&mut self, value: T) -> i32 {
    value.setValidator(self);
    return 1;
  }
}

pub trait QLineEdit_setValidator {
  fn setValidator(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setValidator(const QValidator * );
impl<'a> /*trait*/ QLineEdit_setValidator for (&'a  QValidator) {
  fn setValidator(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setValidatorEPK10QValidator()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit12setValidatorEPK10QValidator(arg0)};
    return 1;
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
  pub fn completer<T: QLineEdit_completer>(&mut self, value: T) -> i32 {
    value.completer(self);
    return 1;
  }
}

pub trait QLineEdit_completer {
  fn completer(self, this: &mut QLineEdit) -> i32;
}

// proto: QCompleter * QLineEdit::completer();
impl<'a> /*trait*/ QLineEdit_completer for () {
  fn completer(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9completerEv()};
    unsafe {_ZNK9QLineEdit9completerEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn textMargins<T: QLineEdit_textMargins>(&mut self, value: T) -> i32 {
    value.textMargins(self);
    return 1;
  }
}

pub trait QLineEdit_textMargins {
  fn textMargins(self, this: &mut QLineEdit) -> i32;
}

// proto: QMargins QLineEdit::textMargins();
impl<'a> /*trait*/ QLineEdit_textMargins for () {
  fn textMargins(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11textMarginsEv()};
    unsafe {_ZNK9QLineEdit11textMarginsEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setClearButtonEnabled<T: QLineEdit_setClearButtonEnabled>(&mut self, value: T) -> i32 {
    value.setClearButtonEnabled(self);
    return 1;
  }
}

pub trait QLineEdit_setClearButtonEnabled {
  fn setClearButtonEnabled(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setClearButtonEnabled(bool enable);
impl<'a> /*trait*/ QLineEdit_setClearButtonEnabled for (i8) {
  fn setClearButtonEnabled(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit21setClearButtonEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit21setClearButtonEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn selectedText<T: QLineEdit_selectedText>(&mut self, value: T) -> i32 {
    value.selectedText(self);
    return 1;
  }
}

pub trait QLineEdit_selectedText {
  fn selectedText(self, this: &mut QLineEdit) -> i32;
}

// proto: QString QLineEdit::selectedText();
impl<'a> /*trait*/ QLineEdit_selectedText for () {
  fn selectedText(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit12selectedTextEv()};
    unsafe {_ZNK9QLineEdit12selectedTextEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn clear<T: QLineEdit_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QLineEdit_clear {
  fn clear(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::clear();
impl<'a> /*trait*/ QLineEdit_clear for () {
  fn clear(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5clearEv()};
    unsafe {_ZN9QLineEdit5clearEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn cursorPositionChanged<T: QLineEdit_cursorPositionChanged>(&mut self, value: T) -> i32 {
    value.cursorPositionChanged(self);
    return 1;
  }
}

pub trait QLineEdit_cursorPositionChanged {
  fn cursorPositionChanged(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::cursorPositionChanged(int , int );
impl<'a> /*trait*/ QLineEdit_cursorPositionChanged for (i32, i32) {
  fn cursorPositionChanged(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit21cursorPositionChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QLineEdit21cursorPositionChangedEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn copy<T: QLineEdit_copy>(&mut self, value: T) -> i32 {
    value.copy(self);
    return 1;
  }
}

pub trait QLineEdit_copy {
  fn copy(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::copy();
impl<'a> /*trait*/ QLineEdit_copy for () {
  fn copy(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit4copyEv()};
    unsafe {_ZNK9QLineEdit4copyEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isUndoAvailable<T: QLineEdit_isUndoAvailable>(&mut self, value: T) -> i32 {
    value.isUndoAvailable(self);
    return 1;
  }
}

pub trait QLineEdit_isUndoAvailable {
  fn isUndoAvailable(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::isUndoAvailable();
impl<'a> /*trait*/ QLineEdit_isUndoAvailable for () {
  fn isUndoAvailable(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15isUndoAvailableEv()};
    unsafe {_ZNK9QLineEdit15isUndoAvailableEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn undo<T: QLineEdit_undo>(&mut self, value: T) -> i32 {
    value.undo(self);
    return 1;
  }
}

pub trait QLineEdit_undo {
  fn undo(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::undo();
impl<'a> /*trait*/ QLineEdit_undo for () {
  fn undo(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4undoEv()};
    unsafe {_ZN9QLineEdit4undoEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isClearButtonEnabled<T: QLineEdit_isClearButtonEnabled>(&mut self, value: T) -> i32 {
    value.isClearButtonEnabled(self);
    return 1;
  }
}

pub trait QLineEdit_isClearButtonEnabled {
  fn isClearButtonEnabled(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::isClearButtonEnabled();
impl<'a> /*trait*/ QLineEdit_isClearButtonEnabled for () {
  fn isClearButtonEnabled(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit20isClearButtonEnabledEv()};
    unsafe {_ZNK9QLineEdit20isClearButtonEnabledEv()};
    return 1;
  }
}

// proto: void QLineEdit::NewQLineEdit(const QLineEdit & );
impl<'a> /*trait*/ QLineEdit_NewQLineEdit for (&'a  QLineEdit) {
  fn NewQLineEdit(self) -> QLineEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEditC1ERKS_(qthis, arg0)};
    let rsthis = QLineEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn end<T: QLineEdit_end>(&mut self, value: T) -> i32 {
    value.end(self);
    return 1;
  }
}

pub trait QLineEdit_end {
  fn end(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::end(bool mark);
impl<'a> /*trait*/ QLineEdit_end for (i8) {
  fn end(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3endEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit3endEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setDragEnabled<T: QLineEdit_setDragEnabled>(&mut self, value: T) -> i32 {
    value.setDragEnabled(self);
    return 1;
  }
}

pub trait QLineEdit_setDragEnabled {
  fn setDragEnabled(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setDragEnabled(bool b);
impl<'a> /*trait*/ QLineEdit_setDragEnabled for (i8) {
  fn setDragEnabled(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14setDragEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QLineEdit14setDragEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn backspace<T: QLineEdit_backspace>(&mut self, value: T) -> i32 {
    value.backspace(self);
    return 1;
  }
}

pub trait QLineEdit_backspace {
  fn backspace(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::backspace();
impl<'a> /*trait*/ QLineEdit_backspace for () {
  fn backspace(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit9backspaceEv()};
    unsafe {_ZN9QLineEdit9backspaceEv()};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn redo<T: QLineEdit_redo>(&mut self, value: T) -> i32 {
    value.redo(self);
    return 1;
  }
}

pub trait QLineEdit_redo {
  fn redo(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::redo();
impl<'a> /*trait*/ QLineEdit_redo for () {
  fn redo(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4redoEv()};
    unsafe {_ZN9QLineEdit4redoEv()};
    return 1;
  }
}

// proto: void QLineEdit::setTextMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QLineEdit_setTextMargins for (i32, i32, i32, i32) {
  fn setTextMargins(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14setTextMarginsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN9QLineEdit14setTextMarginsEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn setInputMask<T: QLineEdit_setInputMask>(&mut self, value: T) -> i32 {
    value.setInputMask(self);
    return 1;
  }
}

pub trait QLineEdit_setInputMask {
  fn setInputMask(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::setInputMask(const QString & inputMask);
impl<'a> /*trait*/ QLineEdit_setInputMask for (&'a  QString) {
  fn setInputMask(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setInputMaskERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLineEdit12setInputMaskERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn getTextMargins<T: QLineEdit_getTextMargins>(&mut self, value: T) -> i32 {
    value.getTextMargins(self);
    return 1;
  }
}

pub trait QLineEdit_getTextMargins {
  fn getTextMargins(self, this: &mut QLineEdit) -> i32;
}

// proto: void QLineEdit::getTextMargins(int * left, int * top, int * right, int * bottom);
impl<'a> /*trait*/ QLineEdit_getTextMargins for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getTextMargins(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QLineEdit {
  pub fn isReadOnly<T: QLineEdit_isReadOnly>(&mut self, value: T) -> i32 {
    value.isReadOnly(self);
    return 1;
  }
}

pub trait QLineEdit_isReadOnly {
  fn isReadOnly(self, this: &mut QLineEdit) -> i32;
}

// proto: bool QLineEdit::isReadOnly();
impl<'a> /*trait*/ QLineEdit_isReadOnly for () {
  fn isReadOnly(self, this: &mut QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10isReadOnlyEv()};
    unsafe {_ZNK9QLineEdit10isReadOnlyEv()};
    return 1;
  }
}

