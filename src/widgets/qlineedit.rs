// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qlineedit.h
// dst-file: /src/widgets/qlineedit.rs
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qcoreevent::QEvent; // 771
use super::qmenu::QMenu; // 773
use super::super::core::qmargins::QMargins; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qsize::QSize; // 771
use super::super::gui::qvalidator::QValidator; // 771
use super::qaction::QAction; // 773
use super::qcompleter::QCompleter; // 773
use super::super::gui::qicon::QIcon; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLineEdit_Class_Size() -> c_int;
  // proto:  void QLineEdit::cursorBackward(bool mark, int steps);
  fn _ZN9QLineEdit14cursorBackwardEbi(qthis: u64 /* *mut c_void*/, arg0: c_char, arg1: c_int);
  // proto:  void QLineEdit::home(bool mark);
  fn _ZN9QLineEdit4homeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QLineEdit::selectionStart();
  fn _ZNK9QLineEdit14selectionStartEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QLineEdit::setCursorPosition(int );
  fn _ZN9QLineEdit17setCursorPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QLineEdit::isRedoAvailable();
  fn _ZNK9QLineEdit15isRedoAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLineEdit::setModified(bool );
  fn _ZN9QLineEdit11setModifiedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QLineEdit::QLineEdit(const QString & , QWidget * parent);
  fn dector_ZN9QLineEditC1ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN9QLineEditC1ERK7QStringP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QLineEdit::event(QEvent * );
  fn _ZN9QLineEdit5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  int QLineEdit::maxLength();
  fn _ZNK9QLineEdit9maxLengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QMenu * QLineEdit::createStandardContextMenu();
  fn _ZN9QLineEdit25createStandardContextMenuEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLineEdit::setTextMargins(const QMargins & margins);
  fn _ZN9QLineEdit14setTextMarginsERK8QMargins(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QLineEdit::cursorPositionAt(const QPoint & pos);
  fn _ZN9QLineEdit16cursorPositionAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  bool QLineEdit::hasSelectedText();
  fn _ZNK9QLineEdit15hasSelectedTextEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLineEdit::setPlaceholderText(const QString & );
  fn _ZN9QLineEdit18setPlaceholderTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QLineEdit::minimumSizeHint();
  fn _ZNK9QLineEdit15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLineEdit::cursorForward(bool mark, int steps);
  fn _ZN9QLineEdit13cursorForwardEbi(qthis: u64 /* *mut c_void*/, arg0: c_char, arg1: c_int);
  // proto:  void QLineEdit::insert(const QString & );
  fn _ZN9QLineEdit6insertERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLineEdit::setText(const QString & );
  fn _ZN9QLineEdit7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLineEdit::selectionChanged();
  fn _ZN9QLineEdit16selectionChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QValidator * QLineEdit::validator();
  fn _ZNK9QLineEdit9validatorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLineEdit::deselect();
  fn _ZN9QLineEdit8deselectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::returnPressed();
  fn _ZN9QLineEdit13returnPressedEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QLineEdit::inputMask();
  fn _ZNK9QLineEdit9inputMaskEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLineEdit::placeholderText();
  fn _ZNK9QLineEdit15placeholderTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLineEdit::cut();
  fn _ZN9QLineEdit3cutEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QLineEdit::text();
  fn _ZNK9QLineEdit4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QLineEdit::metaObject();
  fn _ZNK9QLineEdit10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::del();
  fn _ZN9QLineEdit3delEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLineEdit::isModified();
  fn _ZNK9QLineEdit10isModifiedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLineEdit::editingFinished();
  fn _ZN9QLineEdit15editingFinishedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::cursorWordForward(bool mark);
  fn _ZN9QLineEdit17cursorWordForwardEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QLineEdit::selectAll();
  fn _ZN9QLineEdit9selectAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::setSelection(int , int );
  fn _ZN9QLineEdit12setSelectionEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QLineEdit::setCompleter(QCompleter * completer);
  fn _ZN9QLineEdit12setCompleterEP10QCompleter(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLineEdit::setMaxLength(int );
  fn _ZN9QLineEdit12setMaxLengthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QLineEdit::~QLineEdit();
  fn _ZN9QLineEditD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::textEdited(const QString & );
  fn _ZN9QLineEdit10textEditedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLineEdit::setReadOnly(bool );
  fn _ZN9QLineEdit11setReadOnlyEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QLineEdit::displayText();
  fn _ZNK9QLineEdit11displayTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLineEdit::setFrame(bool );
  fn _ZN9QLineEdit8setFrameEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QLineEdit::hasAcceptableInput();
  fn _ZNK9QLineEdit18hasAcceptableInputEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QLineEdit::hasFrame();
  fn _ZNK9QLineEdit8hasFrameEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QLineEdit::cursorPosition();
  fn _ZNK9QLineEdit14cursorPositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QLineEdit::cursorWordBackward(bool mark);
  fn _ZN9QLineEdit18cursorWordBackwardEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QLineEdit::dragEnabled();
  fn _ZNK9QLineEdit11dragEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLineEdit::textChanged(const QString & );
  fn _ZN9QLineEdit11textChangedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QLineEdit::sizeHint();
  fn _ZNK9QLineEdit8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLineEdit::paste();
  fn _ZN9QLineEdit5pasteEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::setValidator(const QValidator * );
  fn _ZN9QLineEdit12setValidatorEPK10QValidator(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLineEdit::QLineEdit(QWidget * parent);
  fn dector_ZN9QLineEditC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QLineEditC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QCompleter * QLineEdit::completer();
  fn _ZNK9QLineEdit9completerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QMargins QLineEdit::textMargins();
  fn _ZNK9QLineEdit11textMarginsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLineEdit::setClearButtonEnabled(bool enable);
  fn _ZN9QLineEdit21setClearButtonEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QLineEdit::selectedText();
  fn _ZNK9QLineEdit12selectedTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLineEdit::clear();
  fn _ZN9QLineEdit5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::cursorPositionChanged(int , int );
  fn _ZN9QLineEdit21cursorPositionChangedEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QLineEdit::copy();
  fn _ZNK9QLineEdit4copyEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLineEdit::isUndoAvailable();
  fn _ZNK9QLineEdit15isUndoAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLineEdit::undo();
  fn _ZN9QLineEdit4undoEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLineEdit::isClearButtonEnabled();
  fn _ZNK9QLineEdit20isClearButtonEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLineEdit::QLineEdit(const QLineEdit & );
  fn dector_ZN9QLineEditC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QLineEditC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLineEdit::end(bool mark);
  fn _ZN9QLineEdit3endEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QLineEdit::setDragEnabled(bool b);
  fn _ZN9QLineEdit14setDragEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QLineEdit::backspace();
  fn _ZN9QLineEdit9backspaceEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::redo();
  fn _ZN9QLineEdit4redoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLineEdit::setTextMargins(int left, int top, int right, int bottom);
  fn _ZN9QLineEdit14setTextMarginsEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QLineEdit::setInputMask(const QString & inputMask);
  fn _ZN9QLineEdit12setInputMaskERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLineEdit::getTextMargins(int * left, int * top, int * right, int * bottom);
  fn _ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  bool QLineEdit::isReadOnly();
  fn _ZNK9QLineEdit10isReadOnlyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QLineEdit_SlotProxy_connect__ZN9QLineEdit11textChangedERK7QString(qthis: *mut c_void, fptr: *mut c_void);
  fn QLineEdit_SlotProxy_connect__ZN9QLineEdit15editingFinishedEv(qthis: *mut c_void, fptr: *mut c_void);
  fn QLineEdit_SlotProxy_connect__ZN9QLineEdit16selectionChangedEv(qthis: *mut c_void, fptr: *mut c_void);
  fn QLineEdit_SlotProxy_connect__ZN9QLineEdit21cursorPositionChangedEii(qthis: *mut c_void, fptr: *mut c_void);
  fn QLineEdit_SlotProxy_connect__ZN9QLineEdit13returnPressedEv(qthis: *mut c_void, fptr: *mut c_void);
  fn QLineEdit_SlotProxy_connect__ZN9QLineEdit10textEditedERK7QString(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLineEdit)=1
#[derive(Default)]
pub struct QLineEdit {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _textEdited_1: QLineEdit_textEdited_signal,
  pub _returnPressed_1: QLineEdit_returnPressed_signal,
  pub _selectionChanged_1: QLineEdit_selectionChanged_signal,
  pub _cursorPositionChanged_1: QLineEdit_cursorPositionChanged_signal,
  pub _editingFinished_1: QLineEdit_editingFinished_signal,
  pub _textChanged_1: QLineEdit_textChanged_signal,
}

impl /*struct*/ QLineEdit {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLineEdit {
    return QLineEdit{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QLineEdit {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QLineEdit {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QLineEdit::cursorBackward(bool mark, int steps);
impl /*struct*/ QLineEdit {
  pub fn cursorBackward<RetType, T: QLineEdit_cursorBackward<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorBackward(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorBackward<RetType> {
  fn cursorBackward(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::cursorBackward(bool mark, int steps);
impl<'a> /*trait*/ QLineEdit_cursorBackward<()> for (i8, i32) {
  fn cursorBackward(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14cursorBackwardEbi()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QLineEdit14cursorBackwardEbi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLineEdit::home(bool mark);
impl /*struct*/ QLineEdit {
  pub fn home<RetType, T: QLineEdit_home<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.home(self);
    // return 1;
  }
}

pub trait QLineEdit_home<RetType> {
  fn home(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::home(bool mark);
impl<'a> /*trait*/ QLineEdit_home<()> for (i8) {
  fn home(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4homeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit4homeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QLineEdit::selectionStart();
impl /*struct*/ QLineEdit {
  pub fn selectionStart<RetType, T: QLineEdit_selectionStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionStart(self);
    // return 1;
  }
}

pub trait QLineEdit_selectionStart<RetType> {
  fn selectionStart(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  int QLineEdit::selectionStart();
impl<'a> /*trait*/ QLineEdit_selectionStart<i32> for () {
  fn selectionStart(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14selectionStartEv()};
    let mut ret = unsafe {_ZNK9QLineEdit14selectionStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLineEdit::setCursorPosition(int );
impl /*struct*/ QLineEdit {
  pub fn setCursorPosition<RetType, T: QLineEdit_setCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursorPosition(self);
    // return 1;
  }
}

pub trait QLineEdit_setCursorPosition<RetType> {
  fn setCursorPosition(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setCursorPosition(int );
impl<'a> /*trait*/ QLineEdit_setCursorPosition<()> for (i32) {
  fn setCursorPosition(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit17setCursorPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QLineEdit17setCursorPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QLineEdit::isRedoAvailable();
impl /*struct*/ QLineEdit {
  pub fn isRedoAvailable<RetType, T: QLineEdit_isRedoAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRedoAvailable(self);
    // return 1;
  }
}

pub trait QLineEdit_isRedoAvailable<RetType> {
  fn isRedoAvailable(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::isRedoAvailable();
impl<'a> /*trait*/ QLineEdit_isRedoAvailable<i8> for () {
  fn isRedoAvailable(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15isRedoAvailableEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15isRedoAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLineEdit::setModified(bool );
impl /*struct*/ QLineEdit {
  pub fn setModified<RetType, T: QLineEdit_setModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModified(self);
    // return 1;
  }
}

pub trait QLineEdit_setModified<RetType> {
  fn setModified(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setModified(bool );
impl<'a> /*trait*/ QLineEdit_setModified<()> for (i8) {
  fn setModified(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11setModifiedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit11setModifiedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::QLineEdit(const QString & , QWidget * parent);
impl /*struct*/ QLineEdit {
  pub fn New<T: QLineEdit_New>(value: T) -> QLineEdit {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QLineEdit_New {
  fn New(self) -> QLineEdit;
}

  // proto:  void QLineEdit::QLineEdit(const QString & , QWidget * parent);
impl<'a> /*trait*/ QLineEdit_New for (&'a QString, &'a QWidget) {
  fn New(self) -> QLineEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditC1ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QLineEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN9QLineEditC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN9QLineEditC1ERK7QStringP7QWidget(arg0, arg1)} as u64;
    let rsthis = QLineEdit{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QLineEdit::event(QEvent * );
impl /*struct*/ QLineEdit {
  pub fn event<RetType, T: QLineEdit_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QLineEdit_event<RetType> {
  fn event(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::event(QEvent * );
impl<'a> /*trait*/ QLineEdit_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QLineEdit5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QLineEdit::maxLength();
impl /*struct*/ QLineEdit {
  pub fn maxLength<RetType, T: QLineEdit_maxLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxLength(self);
    // return 1;
  }
}

pub trait QLineEdit_maxLength<RetType> {
  fn maxLength(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  int QLineEdit::maxLength();
impl<'a> /*trait*/ QLineEdit_maxLength<i32> for () {
  fn maxLength(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9maxLengthEv()};
    let mut ret = unsafe {_ZNK9QLineEdit9maxLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QMenu * QLineEdit::createStandardContextMenu();
impl /*struct*/ QLineEdit {
  pub fn createStandardContextMenu<RetType, T: QLineEdit_createStandardContextMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu(self);
    // return 1;
  }
}

pub trait QLineEdit_createStandardContextMenu<RetType> {
  fn createStandardContextMenu(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QMenu * QLineEdit::createStandardContextMenu();
impl<'a> /*trait*/ QLineEdit_createStandardContextMenu<QMenu> for () {
  fn createStandardContextMenu(self , rsthis: & QLineEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit25createStandardContextMenuEv()};
    let mut ret = unsafe {_ZN9QLineEdit25createStandardContextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineEdit::setTextMargins(const QMargins & margins);
impl /*struct*/ QLineEdit {
  pub fn setTextMargins<RetType, T: QLineEdit_setTextMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextMargins(self);
    // return 1;
  }
}

pub trait QLineEdit_setTextMargins<RetType> {
  fn setTextMargins(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setTextMargins(const QMargins & margins);
impl<'a> /*trait*/ QLineEdit_setTextMargins<()> for (&'a QMargins) {
  fn setTextMargins(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14setTextMarginsERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit14setTextMarginsERK8QMargins(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QLineEdit::cursorPositionAt(const QPoint & pos);
impl /*struct*/ QLineEdit {
  pub fn cursorPositionAt<RetType, T: QLineEdit_cursorPositionAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionAt(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorPositionAt<RetType> {
  fn cursorPositionAt(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  int QLineEdit::cursorPositionAt(const QPoint & pos);
impl<'a> /*trait*/ QLineEdit_cursorPositionAt<i32> for (&'a QPoint) {
  fn cursorPositionAt(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit16cursorPositionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QLineEdit16cursorPositionAtERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QLineEdit::hasSelectedText();
impl /*struct*/ QLineEdit {
  pub fn hasSelectedText<RetType, T: QLineEdit_hasSelectedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasSelectedText(self);
    // return 1;
  }
}

pub trait QLineEdit_hasSelectedText<RetType> {
  fn hasSelectedText(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::hasSelectedText();
impl<'a> /*trait*/ QLineEdit_hasSelectedText<i8> for () {
  fn hasSelectedText(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15hasSelectedTextEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15hasSelectedTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLineEdit::setPlaceholderText(const QString & );
impl /*struct*/ QLineEdit {
  pub fn setPlaceholderText<RetType, T: QLineEdit_setPlaceholderText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPlaceholderText(self);
    // return 1;
  }
}

pub trait QLineEdit_setPlaceholderText<RetType> {
  fn setPlaceholderText(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setPlaceholderText(const QString & );
impl<'a> /*trait*/ QLineEdit_setPlaceholderText<()> for (&'a QString) {
  fn setPlaceholderText(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit18setPlaceholderTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QLineEdit::minimumSizeHint();
impl /*struct*/ QLineEdit {
  pub fn minimumSizeHint<RetType, T: QLineEdit_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QLineEdit_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QSize QLineEdit::minimumSizeHint();
impl<'a> /*trait*/ QLineEdit_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QLineEdit) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineEdit::cursorForward(bool mark, int steps);
impl /*struct*/ QLineEdit {
  pub fn cursorForward<RetType, T: QLineEdit_cursorForward<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorForward(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorForward<RetType> {
  fn cursorForward(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::cursorForward(bool mark, int steps);
impl<'a> /*trait*/ QLineEdit_cursorForward<()> for (i8, i32) {
  fn cursorForward(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit13cursorForwardEbi()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QLineEdit13cursorForwardEbi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLineEdit::insert(const QString & );
impl /*struct*/ QLineEdit {
  pub fn insert<RetType, T: QLineEdit_insert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QLineEdit_insert<RetType> {
  fn insert(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::insert(const QString & );
impl<'a> /*trait*/ QLineEdit_insert<()> for (&'a QString) {
  fn insert(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit6insertERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit6insertERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::setText(const QString & );
impl /*struct*/ QLineEdit {
  pub fn setText<RetType, T: QLineEdit_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QLineEdit_setText<RetType> {
  fn setText(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setText(const QString & );
impl<'a> /*trait*/ QLineEdit_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::selectionChanged();
impl /*struct*/ QLineEdit {
  pub fn selectionChanged<RetType, T: QLineEdit_selectionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QLineEdit_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::selectionChanged();
impl<'a> /*trait*/ QLineEdit_selectionChanged<()> for () {
  fn selectionChanged(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit16selectionChangedEv()};
     unsafe {_ZN9QLineEdit16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QValidator * QLineEdit::validator();
impl /*struct*/ QLineEdit {
  pub fn validator<RetType, T: QLineEdit_validator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.validator(self);
    // return 1;
  }
}

pub trait QLineEdit_validator<RetType> {
  fn validator(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  const QValidator * QLineEdit::validator();
impl<'a> /*trait*/ QLineEdit_validator<QValidator> for () {
  fn validator(self , rsthis: & QLineEdit) -> QValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9validatorEv()};
    let mut ret = unsafe {_ZNK9QLineEdit9validatorEv(rsthis.qclsinst)};
    let mut ret1 = QValidator::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineEdit::deselect();
impl /*struct*/ QLineEdit {
  pub fn deselect<RetType, T: QLineEdit_deselect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deselect(self);
    // return 1;
  }
}

pub trait QLineEdit_deselect<RetType> {
  fn deselect(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::deselect();
impl<'a> /*trait*/ QLineEdit_deselect<()> for () {
  fn deselect(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit8deselectEv()};
     unsafe {_ZN9QLineEdit8deselectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::returnPressed();
impl /*struct*/ QLineEdit {
  pub fn returnPressed<RetType, T: QLineEdit_returnPressed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.returnPressed(self);
    // return 1;
  }
}

pub trait QLineEdit_returnPressed<RetType> {
  fn returnPressed(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::returnPressed();
impl<'a> /*trait*/ QLineEdit_returnPressed<()> for () {
  fn returnPressed(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit13returnPressedEv()};
     unsafe {_ZN9QLineEdit13returnPressedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QLineEdit::inputMask();
impl /*struct*/ QLineEdit {
  pub fn inputMask<RetType, T: QLineEdit_inputMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.inputMask(self);
    // return 1;
  }
}

pub trait QLineEdit_inputMask<RetType> {
  fn inputMask(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QString QLineEdit::inputMask();
impl<'a> /*trait*/ QLineEdit_inputMask<QString> for () {
  fn inputMask(self , rsthis: & QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9inputMaskEv()};
    let mut ret = unsafe {_ZNK9QLineEdit9inputMaskEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLineEdit::placeholderText();
impl /*struct*/ QLineEdit {
  pub fn placeholderText<RetType, T: QLineEdit_placeholderText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.placeholderText(self);
    // return 1;
  }
}

pub trait QLineEdit_placeholderText<RetType> {
  fn placeholderText(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QString QLineEdit::placeholderText();
impl<'a> /*trait*/ QLineEdit_placeholderText<QString> for () {
  fn placeholderText(self , rsthis: & QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15placeholderTextEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15placeholderTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineEdit::cut();
impl /*struct*/ QLineEdit {
  pub fn cut<RetType, T: QLineEdit_cut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cut(self);
    // return 1;
  }
}

pub trait QLineEdit_cut<RetType> {
  fn cut(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::cut();
impl<'a> /*trait*/ QLineEdit_cut<()> for () {
  fn cut(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3cutEv()};
     unsafe {_ZN9QLineEdit3cutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QLineEdit::text();
impl /*struct*/ QLineEdit {
  pub fn text<RetType, T: QLineEdit_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QLineEdit_text<RetType> {
  fn text(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QString QLineEdit::text();
impl<'a> /*trait*/ QLineEdit_text<QString> for () {
  fn text(self , rsthis: & QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit4textEv()};
    let mut ret = unsafe {_ZNK9QLineEdit4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QLineEdit::metaObject();
impl /*struct*/ QLineEdit {
  pub fn metaObject<RetType, T: QLineEdit_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QLineEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  const QMetaObject * QLineEdit::metaObject();
impl<'a> /*trait*/ QLineEdit_metaObject<()> for () {
  fn metaObject(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10metaObjectEv()};
     unsafe {_ZNK9QLineEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::del();
impl /*struct*/ QLineEdit {
  pub fn del<RetType, T: QLineEdit_del<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.del(self);
    // return 1;
  }
}

pub trait QLineEdit_del<RetType> {
  fn del(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::del();
impl<'a> /*trait*/ QLineEdit_del<()> for () {
  fn del(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3delEv()};
     unsafe {_ZN9QLineEdit3delEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLineEdit::isModified();
impl /*struct*/ QLineEdit {
  pub fn isModified<RetType, T: QLineEdit_isModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isModified(self);
    // return 1;
  }
}

pub trait QLineEdit_isModified<RetType> {
  fn isModified(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::isModified();
impl<'a> /*trait*/ QLineEdit_isModified<i8> for () {
  fn isModified(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10isModifiedEv()};
    let mut ret = unsafe {_ZNK9QLineEdit10isModifiedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLineEdit::editingFinished();
impl /*struct*/ QLineEdit {
  pub fn editingFinished<RetType, T: QLineEdit_editingFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.editingFinished(self);
    // return 1;
  }
}

pub trait QLineEdit_editingFinished<RetType> {
  fn editingFinished(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::editingFinished();
impl<'a> /*trait*/ QLineEdit_editingFinished<()> for () {
  fn editingFinished(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit15editingFinishedEv()};
     unsafe {_ZN9QLineEdit15editingFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::cursorWordForward(bool mark);
impl /*struct*/ QLineEdit {
  pub fn cursorWordForward<RetType, T: QLineEdit_cursorWordForward<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorWordForward(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorWordForward<RetType> {
  fn cursorWordForward(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::cursorWordForward(bool mark);
impl<'a> /*trait*/ QLineEdit_cursorWordForward<()> for (i8) {
  fn cursorWordForward(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit17cursorWordForwardEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit17cursorWordForwardEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::selectAll();
impl /*struct*/ QLineEdit {
  pub fn selectAll<RetType, T: QLineEdit_selectAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectAll(self);
    // return 1;
  }
}

pub trait QLineEdit_selectAll<RetType> {
  fn selectAll(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::selectAll();
impl<'a> /*trait*/ QLineEdit_selectAll<()> for () {
  fn selectAll(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit9selectAllEv()};
     unsafe {_ZN9QLineEdit9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::setSelection(int , int );
impl /*struct*/ QLineEdit {
  pub fn setSelection<RetType, T: QLineEdit_setSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelection(self);
    // return 1;
  }
}

pub trait QLineEdit_setSelection<RetType> {
  fn setSelection(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setSelection(int , int );
impl<'a> /*trait*/ QLineEdit_setSelection<()> for (i32, i32) {
  fn setSelection(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QLineEdit12setSelectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLineEdit::setCompleter(QCompleter * completer);
impl /*struct*/ QLineEdit {
  pub fn setCompleter<RetType, T: QLineEdit_setCompleter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCompleter(self);
    // return 1;
  }
}

pub trait QLineEdit_setCompleter<RetType> {
  fn setCompleter(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setCompleter(QCompleter * completer);
impl<'a> /*trait*/ QLineEdit_setCompleter<()> for (&'a QCompleter) {
  fn setCompleter(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setCompleterEP10QCompleter()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit12setCompleterEP10QCompleter(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::setMaxLength(int );
impl /*struct*/ QLineEdit {
  pub fn setMaxLength<RetType, T: QLineEdit_setMaxLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxLength(self);
    // return 1;
  }
}

pub trait QLineEdit_setMaxLength<RetType> {
  fn setMaxLength(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setMaxLength(int );
impl<'a> /*trait*/ QLineEdit_setMaxLength<()> for (i32) {
  fn setMaxLength(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setMaxLengthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QLineEdit12setMaxLengthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::~QLineEdit();
impl /*struct*/ QLineEdit {
  pub fn Free<RetType, T: QLineEdit_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QLineEdit_Free<RetType> {
  fn Free(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::~QLineEdit();
impl<'a> /*trait*/ QLineEdit_Free<()> for () {
  fn Free(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditD0Ev()};
     unsafe {_ZN9QLineEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::textEdited(const QString & );
impl /*struct*/ QLineEdit {
  pub fn textEdited<RetType, T: QLineEdit_textEdited<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textEdited(self);
    // return 1;
  }
}

pub trait QLineEdit_textEdited<RetType> {
  fn textEdited(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::textEdited(const QString & );
impl<'a> /*trait*/ QLineEdit_textEdited<()> for (&'a QString) {
  fn textEdited(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit10textEditedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit10textEditedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::setReadOnly(bool );
impl /*struct*/ QLineEdit {
  pub fn setReadOnly<RetType, T: QLineEdit_setReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly(self);
    // return 1;
  }
}

pub trait QLineEdit_setReadOnly<RetType> {
  fn setReadOnly(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setReadOnly(bool );
impl<'a> /*trait*/ QLineEdit_setReadOnly<()> for (i8) {
  fn setReadOnly(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11setReadOnlyEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QLineEdit::displayText();
impl /*struct*/ QLineEdit {
  pub fn displayText<RetType, T: QLineEdit_displayText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.displayText(self);
    // return 1;
  }
}

pub trait QLineEdit_displayText<RetType> {
  fn displayText(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QString QLineEdit::displayText();
impl<'a> /*trait*/ QLineEdit_displayText<QString> for () {
  fn displayText(self , rsthis: & QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11displayTextEv()};
    let mut ret = unsafe {_ZNK9QLineEdit11displayTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineEdit::setFrame(bool );
impl /*struct*/ QLineEdit {
  pub fn setFrame<RetType, T: QLineEdit_setFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFrame(self);
    // return 1;
  }
}

pub trait QLineEdit_setFrame<RetType> {
  fn setFrame(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setFrame(bool );
impl<'a> /*trait*/ QLineEdit_setFrame<()> for (i8) {
  fn setFrame(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit8setFrameEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit8setFrameEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QLineEdit::hasAcceptableInput();
impl /*struct*/ QLineEdit {
  pub fn hasAcceptableInput<RetType, T: QLineEdit_hasAcceptableInput<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasAcceptableInput(self);
    // return 1;
  }
}

pub trait QLineEdit_hasAcceptableInput<RetType> {
  fn hasAcceptableInput(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::hasAcceptableInput();
impl<'a> /*trait*/ QLineEdit_hasAcceptableInput<i8> for () {
  fn hasAcceptableInput(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit18hasAcceptableInputEv()};
    let mut ret = unsafe {_ZNK9QLineEdit18hasAcceptableInputEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLineEdit::hasFrame();
impl /*struct*/ QLineEdit {
  pub fn hasFrame<RetType, T: QLineEdit_hasFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFrame(self);
    // return 1;
  }
}

pub trait QLineEdit_hasFrame<RetType> {
  fn hasFrame(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::hasFrame();
impl<'a> /*trait*/ QLineEdit_hasFrame<i8> for () {
  fn hasFrame(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit8hasFrameEv()};
    let mut ret = unsafe {_ZNK9QLineEdit8hasFrameEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QLineEdit::cursorPosition();
impl /*struct*/ QLineEdit {
  pub fn cursorPosition<RetType, T: QLineEdit_cursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorPosition(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorPosition<RetType> {
  fn cursorPosition(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  int QLineEdit::cursorPosition();
impl<'a> /*trait*/ QLineEdit_cursorPosition<i32> for () {
  fn cursorPosition(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14cursorPositionEv()};
    let mut ret = unsafe {_ZNK9QLineEdit14cursorPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLineEdit::cursorWordBackward(bool mark);
impl /*struct*/ QLineEdit {
  pub fn cursorWordBackward<RetType, T: QLineEdit_cursorWordBackward<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorWordBackward(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorWordBackward<RetType> {
  fn cursorWordBackward(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::cursorWordBackward(bool mark);
impl<'a> /*trait*/ QLineEdit_cursorWordBackward<()> for (i8) {
  fn cursorWordBackward(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit18cursorWordBackwardEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit18cursorWordBackwardEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QLineEdit::dragEnabled();
impl /*struct*/ QLineEdit {
  pub fn dragEnabled<RetType, T: QLineEdit_dragEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dragEnabled(self);
    // return 1;
  }
}

pub trait QLineEdit_dragEnabled<RetType> {
  fn dragEnabled(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::dragEnabled();
impl<'a> /*trait*/ QLineEdit_dragEnabled<i8> for () {
  fn dragEnabled(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11dragEnabledEv()};
    let mut ret = unsafe {_ZNK9QLineEdit11dragEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLineEdit::textChanged(const QString & );
impl /*struct*/ QLineEdit {
  pub fn textChanged<RetType, T: QLineEdit_textChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textChanged(self);
    // return 1;
  }
}

pub trait QLineEdit_textChanged<RetType> {
  fn textChanged(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::textChanged(const QString & );
impl<'a> /*trait*/ QLineEdit_textChanged<()> for (&'a QString) {
  fn textChanged(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit11textChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit11textChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QLineEdit::sizeHint();
impl /*struct*/ QLineEdit {
  pub fn sizeHint<RetType, T: QLineEdit_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QLineEdit_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QSize QLineEdit::sizeHint();
impl<'a> /*trait*/ QLineEdit_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QLineEdit) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QLineEdit8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineEdit::paste();
impl /*struct*/ QLineEdit {
  pub fn paste<RetType, T: QLineEdit_paste<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paste(self);
    // return 1;
  }
}

pub trait QLineEdit_paste<RetType> {
  fn paste(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::paste();
impl<'a> /*trait*/ QLineEdit_paste<()> for () {
  fn paste(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5pasteEv()};
     unsafe {_ZN9QLineEdit5pasteEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::setValidator(const QValidator * );
impl /*struct*/ QLineEdit {
  pub fn setValidator<RetType, T: QLineEdit_setValidator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValidator(self);
    // return 1;
  }
}

pub trait QLineEdit_setValidator<RetType> {
  fn setValidator(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setValidator(const QValidator * );
impl<'a> /*trait*/ QLineEdit_setValidator<()> for (&'a QValidator) {
  fn setValidator(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setValidatorEPK10QValidator()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit12setValidatorEPK10QValidator(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::QLineEdit(QWidget * parent);
impl<'a> /*trait*/ QLineEdit_New for (&'a QWidget) {
  fn New(self) -> QLineEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditC1EP7QWidget()};
    let ctysz: c_int = unsafe{QLineEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QLineEditC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QLineEditC1EP7QWidget(arg0)} as u64;
    let rsthis = QLineEdit{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QCompleter * QLineEdit::completer();
impl /*struct*/ QLineEdit {
  pub fn completer<RetType, T: QLineEdit_completer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completer(self);
    // return 1;
  }
}

pub trait QLineEdit_completer<RetType> {
  fn completer(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QCompleter * QLineEdit::completer();
impl<'a> /*trait*/ QLineEdit_completer<QCompleter> for () {
  fn completer(self , rsthis: & QLineEdit) -> QCompleter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit9completerEv()};
    let mut ret = unsafe {_ZNK9QLineEdit9completerEv(rsthis.qclsinst)};
    let mut ret1 = QCompleter::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMargins QLineEdit::textMargins();
impl /*struct*/ QLineEdit {
  pub fn textMargins<RetType, T: QLineEdit_textMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textMargins(self);
    // return 1;
  }
}

pub trait QLineEdit_textMargins<RetType> {
  fn textMargins(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QMargins QLineEdit::textMargins();
impl<'a> /*trait*/ QLineEdit_textMargins<QMargins> for () {
  fn textMargins(self , rsthis: & QLineEdit) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit11textMarginsEv()};
    let mut ret = unsafe {_ZNK9QLineEdit11textMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineEdit::setClearButtonEnabled(bool enable);
impl /*struct*/ QLineEdit {
  pub fn setClearButtonEnabled<RetType, T: QLineEdit_setClearButtonEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClearButtonEnabled(self);
    // return 1;
  }
}

pub trait QLineEdit_setClearButtonEnabled<RetType> {
  fn setClearButtonEnabled(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setClearButtonEnabled(bool enable);
impl<'a> /*trait*/ QLineEdit_setClearButtonEnabled<()> for (i8) {
  fn setClearButtonEnabled(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit21setClearButtonEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit21setClearButtonEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QLineEdit::selectedText();
impl /*struct*/ QLineEdit {
  pub fn selectedText<RetType, T: QLineEdit_selectedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedText(self);
    // return 1;
  }
}

pub trait QLineEdit_selectedText<RetType> {
  fn selectedText(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  QString QLineEdit::selectedText();
impl<'a> /*trait*/ QLineEdit_selectedText<QString> for () {
  fn selectedText(self , rsthis: & QLineEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit12selectedTextEv()};
    let mut ret = unsafe {_ZNK9QLineEdit12selectedTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineEdit::clear();
impl /*struct*/ QLineEdit {
  pub fn clear<RetType, T: QLineEdit_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QLineEdit_clear<RetType> {
  fn clear(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::clear();
impl<'a> /*trait*/ QLineEdit_clear<()> for () {
  fn clear(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit5clearEv()};
     unsafe {_ZN9QLineEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::cursorPositionChanged(int , int );
impl /*struct*/ QLineEdit {
  pub fn cursorPositionChanged<RetType, T: QLineEdit_cursorPositionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionChanged(self);
    // return 1;
  }
}

pub trait QLineEdit_cursorPositionChanged<RetType> {
  fn cursorPositionChanged(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::cursorPositionChanged(int , int );
impl<'a> /*trait*/ QLineEdit_cursorPositionChanged<()> for (i32, i32) {
  fn cursorPositionChanged(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit21cursorPositionChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QLineEdit21cursorPositionChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLineEdit::copy();
impl /*struct*/ QLineEdit {
  pub fn copy<RetType, T: QLineEdit_copy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QLineEdit_copy<RetType> {
  fn copy(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::copy();
impl<'a> /*trait*/ QLineEdit_copy<()> for () {
  fn copy(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit4copyEv()};
     unsafe {_ZNK9QLineEdit4copyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLineEdit::isUndoAvailable();
impl /*struct*/ QLineEdit {
  pub fn isUndoAvailable<RetType, T: QLineEdit_isUndoAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndoAvailable(self);
    // return 1;
  }
}

pub trait QLineEdit_isUndoAvailable<RetType> {
  fn isUndoAvailable(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::isUndoAvailable();
impl<'a> /*trait*/ QLineEdit_isUndoAvailable<i8> for () {
  fn isUndoAvailable(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit15isUndoAvailableEv()};
    let mut ret = unsafe {_ZNK9QLineEdit15isUndoAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLineEdit::undo();
impl /*struct*/ QLineEdit {
  pub fn undo<RetType, T: QLineEdit_undo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QLineEdit_undo<RetType> {
  fn undo(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::undo();
impl<'a> /*trait*/ QLineEdit_undo<()> for () {
  fn undo(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4undoEv()};
     unsafe {_ZN9QLineEdit4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLineEdit::isClearButtonEnabled();
impl /*struct*/ QLineEdit {
  pub fn isClearButtonEnabled<RetType, T: QLineEdit_isClearButtonEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isClearButtonEnabled(self);
    // return 1;
  }
}

pub trait QLineEdit_isClearButtonEnabled<RetType> {
  fn isClearButtonEnabled(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::isClearButtonEnabled();
impl<'a> /*trait*/ QLineEdit_isClearButtonEnabled<i8> for () {
  fn isClearButtonEnabled(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit20isClearButtonEnabledEv()};
    let mut ret = unsafe {_ZNK9QLineEdit20isClearButtonEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLineEdit::QLineEdit(const QLineEdit & );
impl<'a> /*trait*/ QLineEdit_New for (&'a QLineEdit) {
  fn New(self) -> QLineEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEditC1ERKS_()};
    let ctysz: c_int = unsafe{QLineEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QLineEditC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QLineEditC1ERKS_(arg0)} as u64;
    let rsthis = QLineEdit{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLineEdit::end(bool mark);
impl /*struct*/ QLineEdit {
  pub fn end<RetType, T: QLineEdit_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QLineEdit_end<RetType> {
  fn end(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::end(bool mark);
impl<'a> /*trait*/ QLineEdit_end<()> for (i8) {
  fn end(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit3endEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit3endEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::setDragEnabled(bool b);
impl /*struct*/ QLineEdit {
  pub fn setDragEnabled<RetType, T: QLineEdit_setDragEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDragEnabled(self);
    // return 1;
  }
}

pub trait QLineEdit_setDragEnabled<RetType> {
  fn setDragEnabled(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setDragEnabled(bool b);
impl<'a> /*trait*/ QLineEdit_setDragEnabled<()> for (i8) {
  fn setDragEnabled(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit14setDragEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QLineEdit14setDragEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::backspace();
impl /*struct*/ QLineEdit {
  pub fn backspace<RetType, T: QLineEdit_backspace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backspace(self);
    // return 1;
  }
}

pub trait QLineEdit_backspace<RetType> {
  fn backspace(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::backspace();
impl<'a> /*trait*/ QLineEdit_backspace<()> for () {
  fn backspace(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit9backspaceEv()};
     unsafe {_ZN9QLineEdit9backspaceEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::redo();
impl /*struct*/ QLineEdit {
  pub fn redo<RetType, T: QLineEdit_redo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QLineEdit_redo<RetType> {
  fn redo(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::redo();
impl<'a> /*trait*/ QLineEdit_redo<()> for () {
  fn redo(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit4redoEv()};
     unsafe {_ZN9QLineEdit4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLineEdit::setTextMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QLineEdit_setTextMargins<()> for (i32, i32, i32, i32) {
  fn setTextMargins(self , rsthis: & QLineEdit) -> () {
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

  // proto:  void QLineEdit::setInputMask(const QString & inputMask);
impl /*struct*/ QLineEdit {
  pub fn setInputMask<RetType, T: QLineEdit_setInputMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInputMask(self);
    // return 1;
  }
}

pub trait QLineEdit_setInputMask<RetType> {
  fn setInputMask(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::setInputMask(const QString & inputMask);
impl<'a> /*trait*/ QLineEdit_setInputMask<()> for (&'a QString) {
  fn setInputMask(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLineEdit12setInputMaskERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QLineEdit12setInputMaskERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLineEdit::getTextMargins(int * left, int * top, int * right, int * bottom);
impl /*struct*/ QLineEdit {
  pub fn getTextMargins<RetType, T: QLineEdit_getTextMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getTextMargins(self);
    // return 1;
  }
}

pub trait QLineEdit_getTextMargins<RetType> {
  fn getTextMargins(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  void QLineEdit::getTextMargins(int * left, int * top, int * right, int * bottom);
impl<'a> /*trait*/ QLineEdit_getTextMargins<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn getTextMargins(self , rsthis: & QLineEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QLineEdit::isReadOnly();
impl /*struct*/ QLineEdit {
  pub fn isReadOnly<RetType, T: QLineEdit_isReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly(self);
    // return 1;
  }
}

pub trait QLineEdit_isReadOnly<RetType> {
  fn isReadOnly(self , rsthis: & QLineEdit) -> RetType;
}

  // proto:  bool QLineEdit::isReadOnly();
impl<'a> /*trait*/ QLineEdit_isReadOnly<i8> for () {
  fn isReadOnly(self , rsthis: & QLineEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLineEdit10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK9QLineEdit10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QLineEdit_textEdited
pub struct QLineEdit_textEdited_signal{poi:u64}
impl /* struct */ QLineEdit {
  pub fn textEdited_1(self) -> QLineEdit_textEdited_signal {
     return QLineEdit_textEdited_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLineEdit_textEdited_signal {
  pub fn connect<T: QLineEdit_textEdited_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLineEdit_textEdited_signal_connect {
  fn connect(self, sigthis: QLineEdit_textEdited_signal);
}

#[derive(Default)] // for QLineEdit_returnPressed
pub struct QLineEdit_returnPressed_signal{poi:u64}
impl /* struct */ QLineEdit {
  pub fn returnPressed_1(self) -> QLineEdit_returnPressed_signal {
     return QLineEdit_returnPressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLineEdit_returnPressed_signal {
  pub fn connect<T: QLineEdit_returnPressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLineEdit_returnPressed_signal_connect {
  fn connect(self, sigthis: QLineEdit_returnPressed_signal);
}

#[derive(Default)] // for QLineEdit_selectionChanged
pub struct QLineEdit_selectionChanged_signal{poi:u64}
impl /* struct */ QLineEdit {
  pub fn selectionChanged_1(self) -> QLineEdit_selectionChanged_signal {
     return QLineEdit_selectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLineEdit_selectionChanged_signal {
  pub fn connect<T: QLineEdit_selectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLineEdit_selectionChanged_signal_connect {
  fn connect(self, sigthis: QLineEdit_selectionChanged_signal);
}

#[derive(Default)] // for QLineEdit_cursorPositionChanged
pub struct QLineEdit_cursorPositionChanged_signal{poi:u64}
impl /* struct */ QLineEdit {
  pub fn cursorPositionChanged_1(self) -> QLineEdit_cursorPositionChanged_signal {
     return QLineEdit_cursorPositionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLineEdit_cursorPositionChanged_signal {
  pub fn connect<T: QLineEdit_cursorPositionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLineEdit_cursorPositionChanged_signal_connect {
  fn connect(self, sigthis: QLineEdit_cursorPositionChanged_signal);
}

#[derive(Default)] // for QLineEdit_editingFinished
pub struct QLineEdit_editingFinished_signal{poi:u64}
impl /* struct */ QLineEdit {
  pub fn editingFinished_1(self) -> QLineEdit_editingFinished_signal {
     return QLineEdit_editingFinished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLineEdit_editingFinished_signal {
  pub fn connect<T: QLineEdit_editingFinished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLineEdit_editingFinished_signal_connect {
  fn connect(self, sigthis: QLineEdit_editingFinished_signal);
}

#[derive(Default)] // for QLineEdit_textChanged
pub struct QLineEdit_textChanged_signal{poi:u64}
impl /* struct */ QLineEdit {
  pub fn textChanged_1(self) -> QLineEdit_textChanged_signal {
     return QLineEdit_textChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLineEdit_textChanged_signal {
  pub fn connect<T: QLineEdit_textChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLineEdit_textChanged_signal_connect {
  fn connect(self, sigthis: QLineEdit_textChanged_signal);
}

// textChanged(const class QString &)
extern fn QLineEdit_textChanged_signal_connect_cb_0(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QLineEdit_textChanged_signal_connect for (extern fn(QString)) {
  fn connect(self, sigthis: QLineEdit_textChanged_signal) {
    // do smth...
    unsafe {QLineEdit_SlotProxy_connect__ZN9QLineEdit11textChangedERK7QString(sigthis.poi as *mut c_void, QLineEdit_textChanged_signal_connect_cb_0 as *mut c_void)};
  }
}
// editingFinished()
extern fn QLineEdit_editingFinished_signal_connect_cb_1() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QLineEdit_editingFinished_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QLineEdit_editingFinished_signal) {
    // do smth...
    unsafe {QLineEdit_SlotProxy_connect__ZN9QLineEdit15editingFinishedEv(sigthis.poi as *mut c_void, QLineEdit_editingFinished_signal_connect_cb_1 as *mut c_void)};
  }
}
// selectionChanged()
extern fn QLineEdit_selectionChanged_signal_connect_cb_2() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QLineEdit_selectionChanged_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QLineEdit_selectionChanged_signal) {
    // do smth...
    unsafe {QLineEdit_SlotProxy_connect__ZN9QLineEdit16selectionChangedEv(sigthis.poi as *mut c_void, QLineEdit_selectionChanged_signal_connect_cb_2 as *mut c_void)};
  }
}
// cursorPositionChanged(int, int)
extern fn QLineEdit_cursorPositionChanged_signal_connect_cb_3(arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QLineEdit_cursorPositionChanged_signal_connect for (extern fn(i32, i32)) {
  fn connect(self, sigthis: QLineEdit_cursorPositionChanged_signal) {
    // do smth...
    unsafe {QLineEdit_SlotProxy_connect__ZN9QLineEdit21cursorPositionChangedEii(sigthis.poi as *mut c_void, QLineEdit_cursorPositionChanged_signal_connect_cb_3 as *mut c_void)};
  }
}
// returnPressed()
extern fn QLineEdit_returnPressed_signal_connect_cb_4() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QLineEdit_returnPressed_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QLineEdit_returnPressed_signal) {
    // do smth...
    unsafe {QLineEdit_SlotProxy_connect__ZN9QLineEdit13returnPressedEv(sigthis.poi as *mut c_void, QLineEdit_returnPressed_signal_connect_cb_4 as *mut c_void)};
  }
}
// textEdited(const class QString &)
extern fn QLineEdit_textEdited_signal_connect_cb_5(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QLineEdit_textEdited_signal_connect for (extern fn(QString)) {
  fn connect(self, sigthis: QLineEdit_textEdited_signal) {
    // do smth...
    unsafe {QLineEdit_SlotProxy_connect__ZN9QLineEdit10textEditedERK7QString(sigthis.poi as *mut c_void, QLineEdit_textEdited_signal_connect_cb_5 as *mut c_void)};
  }
}
// <= body block end

