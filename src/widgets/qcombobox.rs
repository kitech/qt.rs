// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtWidgets/qcombobox.h
// dst-file: /src/widgets/qcombobox.rs
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
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
use super::super::core::qsize::QSize; // 771
use super::qabstractitemview::QAbstractItemView; // 773
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::gui::qvalidator::QValidator; // 771
use super::qcompleter::QCompleter; // 773
use super::qabstractitemdelegate::QAbstractItemDelegate; // 773
use super::qlineedit::QLineEdit; // 773
use super::super::core::qcoreevent::QEvent; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QComboBox_Class_Size() -> c_int;
  // proto:  void QComboBox::setModel(QAbstractItemModel * model);
  fn _ZN9QComboBox8setModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::clearEditText();
  fn _ZN9QComboBox13clearEditTextEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QComboBox::setAutoCompletion(bool enable);
  fn _ZN9QComboBox17setAutoCompletionEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QComboBox::setFrame(bool );
  fn _ZN9QComboBox8setFrameEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QComboBox::setIconSize(const QSize & size);
  fn _ZN9QComboBox11setIconSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::setView(QAbstractItemView * itemView);
  fn _ZN9QComboBox7setViewEP17QAbstractItemView(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAbstractItemView * QComboBox::view();
  fn _ZNK9QComboBox4viewEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QComboBox::minimumSizeHint();
  fn _ZNK9QComboBox15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QComboBox::clear();
  fn _ZN9QComboBox5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QComboBox::maxCount();
  fn _ZNK9QComboBox8maxCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QComboBox::addItem(const QIcon & icon, const QString & text, const QVariant & userData);
  fn demth_ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QComboBox::insertItems(int index, const QStringList & texts);
  fn _ZN9QComboBox11insertItemsEiRK11QStringList(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QSize QComboBox::iconSize();
  fn _ZNK9QComboBox8iconSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QModelIndex QComboBox::rootModelIndex();
  fn _ZNK9QComboBox14rootModelIndexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QComboBox::setEditable(bool editable);
  fn _ZN9QComboBox11setEditableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QComboBox::setItemIcon(int index, const QIcon & icon);
  fn _ZN9QComboBox11setItemIconEiRK5QIcon(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QComboBox::autoCompletion();
  fn _ZNK9QComboBox14autoCompletionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QVariant QComboBox::currentData(int role);
  fn _ZNK9QComboBox11currentDataEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QComboBox::hasFrame();
  fn _ZNK9QComboBox8hasFrameEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QValidator * QComboBox::validator();
  fn _ZNK9QComboBox9validatorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QComboBox::itemText(int index);
  fn _ZNK9QComboBox8itemTextEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QComboBox::setItemData(int index, const QVariant & value, int role);
  fn _ZN9QComboBox11setItemDataEiRK8QVarianti(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  void QComboBox::hidePopup();
  fn _ZN9QComboBox9hidePopupEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QComboBox::insertItem(int index, const QIcon & icon, const QString & text, const QVariant & userData);
  fn _ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QComboBox::setCurrentText(const QString & text);
  fn _ZN9QComboBox14setCurrentTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QComboBox::modelColumn();
  fn _ZNK9QComboBox11modelColumnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QSize QComboBox::sizeHint();
  fn _ZNK9QComboBox8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QComboBox::itemData(int index, int role);
  fn _ZNK9QComboBox8itemDataEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QComboBox::setCompleter(QCompleter * c);
  fn _ZN9QComboBox12setCompleterEP10QCompleter(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QComboBox::maxVisibleItems();
  fn _ZNK9QComboBox15maxVisibleItemsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QComboBox::QComboBox(QWidget * parent);
  fn dector_ZN9QComboBoxC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QComboBoxC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::setCurrentIndex(int index);
  fn _ZN9QComboBox15setCurrentIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QComboBox::QComboBox(const QComboBox & );
  fn dector_ZN9QComboBoxC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QComboBoxC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::setRootModelIndex(const QModelIndex & index);
  fn _ZN9QComboBox17setRootModelIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::setEditText(const QString & text);
  fn _ZN9QComboBox11setEditTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::addItem(const QString & text, const QVariant & userData);
  fn demth_ZN9QComboBox7addItemERK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QCompleter * QComboBox::completer();
  fn _ZNK9QComboBox9completerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QComboBox::removeItem(int index);
  fn _ZN9QComboBox10removeItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QComboBox::count();
  fn _ZNK9QComboBox5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QComboBox::setItemDelegate(QAbstractItemDelegate * delegate);
  fn _ZN9QComboBox15setItemDelegateEP21QAbstractItemDelegate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::addItems(const QStringList & texts);
  fn demth_ZN9QComboBox8addItemsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::setMinimumContentsLength(int characters);
  fn _ZN9QComboBox24setMinimumContentsLengthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QComboBox::duplicatesEnabled();
  fn _ZNK9QComboBox17duplicatesEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QComboBox::~QComboBox();
  fn _ZN9QComboBoxD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractItemModel * QComboBox::model();
  fn _ZNK9QComboBox5modelEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QComboBox::minimumContentsLength();
  fn _ZNK9QComboBox21minimumContentsLengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QComboBox::isEditable();
  fn _ZNK9QComboBox10isEditableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QComboBox::setMaxCount(int max);
  fn _ZN9QComboBox11setMaxCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QComboBox::currentIndex();
  fn _ZNK9QComboBox12currentIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QComboBox::setDuplicatesEnabled(bool enable);
  fn _ZN9QComboBox20setDuplicatesEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QComboBox::currentText();
  fn _ZNK9QComboBox11currentTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QComboBox::showPopup();
  fn _ZN9QComboBox9showPopupEv(qthis: u64 /* *mut c_void*/);
  // proto:  QLineEdit * QComboBox::lineEdit();
  fn _ZNK9QComboBox8lineEditEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAbstractItemDelegate * QComboBox::itemDelegate();
  fn _ZNK9QComboBox12itemDelegateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QComboBox::setMaxVisibleItems(int maxItems);
  fn _ZN9QComboBox18setMaxVisibleItemsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QComboBox::event(QEvent * event);
  fn _ZN9QComboBox5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QComboBox::setModelColumn(int visibleColumn);
  fn _ZN9QComboBox14setModelColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QComboBox::setItemText(int index, const QString & text);
  fn _ZN9QComboBox11setItemTextEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QComboBox::setLineEdit(QLineEdit * edit);
  fn _ZN9QComboBox11setLineEditEP9QLineEdit(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QIcon QComboBox::itemIcon(int index);
  fn _ZNK9QComboBox8itemIconEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QComboBox::insertItem(int index, const QString & text, const QVariant & userData);
  fn demth_ZN9QComboBox10insertItemEiRK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QComboBox::setValidator(const QValidator * v);
  fn _ZN9QComboBox12setValidatorEPK10QValidator(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QComboBox::insertSeparator(int index);
  fn _ZN9QComboBox15insertSeparatorEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  const QMetaObject * QComboBox::metaObject();
  fn _ZNK9QComboBox10metaObjectEv(qthis: u64 /* *mut c_void*/);
  fn QComboBox_SlotProxy_connect__ZN9QComboBox9activatedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QComboBox_SlotProxy_connect__ZN9QComboBox19currentIndexChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QComboBox_SlotProxy_connect__ZN9QComboBox18currentTextChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QComboBox_SlotProxy_connect__ZN9QComboBox11highlightedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QComboBox_SlotProxy_connect__ZN9QComboBox15editTextChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QComboBox_SlotProxy_connect__ZN9QComboBox11highlightedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QComboBox_SlotProxy_connect__ZN9QComboBox9activatedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QComboBox_SlotProxy_connect__ZN9QComboBox19currentIndexChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QComboBox)=1
#[derive(Default)]
pub struct QComboBox {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _currentIndexChanged: QComboBox_currentIndexChanged_signal,
  pub _currentTextChanged: QComboBox_currentTextChanged_signal,
  pub _highlighted: QComboBox_highlighted_signal,
  pub _activated: QComboBox_activated_signal,
  pub _editTextChanged: QComboBox_editTextChanged_signal,
}

impl /*struct*/ QComboBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QComboBox {
    return QComboBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QComboBox {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QComboBox {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QComboBox::setModel(QAbstractItemModel * model);
impl /*struct*/ QComboBox {
  pub fn setModel<RetType, T: QComboBox_setModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModel(self);
    // return 1;
  }
}

pub trait QComboBox_setModel<RetType> {
  fn setModel(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setModel(QAbstractItemModel * model);
impl<'a> /*trait*/ QComboBox_setModel<()> for (&'a QAbstractItemModel) {
  fn setModel(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox8setModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox8setModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::clearEditText();
impl /*struct*/ QComboBox {
  pub fn clearEditText<RetType, T: QComboBox_clearEditText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearEditText(self);
    // return 1;
  }
}

pub trait QComboBox_clearEditText<RetType> {
  fn clearEditText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::clearEditText();
impl<'a> /*trait*/ QComboBox_clearEditText<()> for () {
  fn clearEditText(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox13clearEditTextEv()};
     unsafe {_ZN9QComboBox13clearEditTextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QComboBox::setAutoCompletion(bool enable);
impl /*struct*/ QComboBox {
  pub fn setAutoCompletion<RetType, T: QComboBox_setAutoCompletion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoCompletion(self);
    // return 1;
  }
}

pub trait QComboBox_setAutoCompletion<RetType> {
  fn setAutoCompletion(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setAutoCompletion(bool enable);
impl<'a> /*trait*/ QComboBox_setAutoCompletion<()> for (i8) {
  fn setAutoCompletion(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox17setAutoCompletionEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QComboBox17setAutoCompletionEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setFrame(bool );
impl /*struct*/ QComboBox {
  pub fn setFrame<RetType, T: QComboBox_setFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFrame(self);
    // return 1;
  }
}

pub trait QComboBox_setFrame<RetType> {
  fn setFrame(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setFrame(bool );
impl<'a> /*trait*/ QComboBox_setFrame<()> for (i8) {
  fn setFrame(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox8setFrameEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QComboBox8setFrameEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setIconSize(const QSize & size);
impl /*struct*/ QComboBox {
  pub fn setIconSize<RetType, T: QComboBox_setIconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QComboBox_setIconSize<RetType> {
  fn setIconSize(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setIconSize(const QSize & size);
impl<'a> /*trait*/ QComboBox_setIconSize<()> for (&'a QSize) {
  fn setIconSize(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setView(QAbstractItemView * itemView);
impl /*struct*/ QComboBox {
  pub fn setView<RetType, T: QComboBox_setView<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setView(self);
    // return 1;
  }
}

pub trait QComboBox_setView<RetType> {
  fn setView(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setView(QAbstractItemView * itemView);
impl<'a> /*trait*/ QComboBox_setView<()> for (&'a QAbstractItemView) {
  fn setView(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox7setViewEP17QAbstractItemView()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox7setViewEP17QAbstractItemView(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAbstractItemView * QComboBox::view();
impl /*struct*/ QComboBox {
  pub fn view<RetType, T: QComboBox_view<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.view(self);
    // return 1;
  }
}

pub trait QComboBox_view<RetType> {
  fn view(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QAbstractItemView * QComboBox::view();
impl<'a> /*trait*/ QComboBox_view<()> for () {
  fn view(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox4viewEv()};
     unsafe {_ZNK9QComboBox4viewEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QComboBox::minimumSizeHint();
impl /*struct*/ QComboBox {
  pub fn minimumSizeHint<RetType, T: QComboBox_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QComboBox_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QSize QComboBox::minimumSizeHint();
impl<'a> /*trait*/ QComboBox_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QComboBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::clear();
impl /*struct*/ QComboBox {
  pub fn clear<RetType, T: QComboBox_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QComboBox_clear<RetType> {
  fn clear(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::clear();
impl<'a> /*trait*/ QComboBox_clear<()> for () {
  fn clear(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox5clearEv()};
     unsafe {_ZN9QComboBox5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QComboBox::maxCount();
impl /*struct*/ QComboBox {
  pub fn maxCount<RetType, T: QComboBox_maxCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxCount(self);
    // return 1;
  }
}

pub trait QComboBox_maxCount<RetType> {
  fn maxCount(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::maxCount();
impl<'a> /*trait*/ QComboBox_maxCount<i32> for () {
  fn maxCount(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8maxCountEv()};
    let mut ret = unsafe {_ZNK9QComboBox8maxCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QComboBox::addItem(const QIcon & icon, const QString & text, const QVariant & userData);
impl /*struct*/ QComboBox {
  pub fn addItem<RetType, T: QComboBox_addItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QComboBox_addItem<RetType> {
  fn addItem(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::addItem(const QIcon & icon, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_addItem<()> for (&'a QIcon, &'a QString, &'a QVariant) {
  fn addItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {demth_ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QComboBox::insertItems(int index, const QStringList & texts);
impl /*struct*/ QComboBox {
  pub fn insertItems<RetType, T: QComboBox_insertItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertItems(self);
    // return 1;
  }
}

pub trait QComboBox_insertItems<RetType> {
  fn insertItems(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::insertItems(int index, const QStringList & texts);
impl<'a> /*trait*/ QComboBox_insertItems<()> for (i32, &'a QStringList) {
  fn insertItems(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11insertItemsEiRK11QStringList()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11insertItemsEiRK11QStringList(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QComboBox::iconSize();
impl /*struct*/ QComboBox {
  pub fn iconSize<RetType, T: QComboBox_iconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QComboBox_iconSize<RetType> {
  fn iconSize(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QSize QComboBox::iconSize();
impl<'a> /*trait*/ QComboBox_iconSize<QSize> for () {
  fn iconSize(self , rsthis: & QComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8iconSizeEv()};
    let mut ret = unsafe {_ZNK9QComboBox8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QComboBox::rootModelIndex();
impl /*struct*/ QComboBox {
  pub fn rootModelIndex<RetType, T: QComboBox_rootModelIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootModelIndex(self);
    // return 1;
  }
}

pub trait QComboBox_rootModelIndex<RetType> {
  fn rootModelIndex(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QModelIndex QComboBox::rootModelIndex();
impl<'a> /*trait*/ QComboBox_rootModelIndex<QModelIndex> for () {
  fn rootModelIndex(self , rsthis: & QComboBox) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox14rootModelIndexEv()};
    let mut ret = unsafe {_ZNK9QComboBox14rootModelIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::setEditable(bool editable);
impl /*struct*/ QComboBox {
  pub fn setEditable<RetType, T: QComboBox_setEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEditable(self);
    // return 1;
  }
}

pub trait QComboBox_setEditable<RetType> {
  fn setEditable(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setEditable(bool editable);
impl<'a> /*trait*/ QComboBox_setEditable<()> for (i8) {
  fn setEditable(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setEditableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QComboBox11setEditableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setItemIcon(int index, const QIcon & icon);
impl /*struct*/ QComboBox {
  pub fn setItemIcon<RetType, T: QComboBox_setItemIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemIcon(self);
    // return 1;
  }
}

pub trait QComboBox_setItemIcon<RetType> {
  fn setItemIcon(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setItemIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QComboBox_setItemIcon<()> for (i32, &'a QIcon) {
  fn setItemIcon(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setItemIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QComboBox::autoCompletion();
impl /*struct*/ QComboBox {
  pub fn autoCompletion<RetType, T: QComboBox_autoCompletion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoCompletion(self);
    // return 1;
  }
}

pub trait QComboBox_autoCompletion<RetType> {
  fn autoCompletion(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::autoCompletion();
impl<'a> /*trait*/ QComboBox_autoCompletion<i8> for () {
  fn autoCompletion(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox14autoCompletionEv()};
    let mut ret = unsafe {_ZNK9QComboBox14autoCompletionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVariant QComboBox::currentData(int role);
impl /*struct*/ QComboBox {
  pub fn currentData<RetType, T: QComboBox_currentData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentData(self);
    // return 1;
  }
}

pub trait QComboBox_currentData<RetType> {
  fn currentData(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QVariant QComboBox::currentData(int role);
impl<'a> /*trait*/ QComboBox_currentData<QVariant> for (i32) {
  fn currentData(self , rsthis: & QComboBox) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11currentDataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QComboBox11currentDataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QComboBox::hasFrame();
impl /*struct*/ QComboBox {
  pub fn hasFrame<RetType, T: QComboBox_hasFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFrame(self);
    // return 1;
  }
}

pub trait QComboBox_hasFrame<RetType> {
  fn hasFrame(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::hasFrame();
impl<'a> /*trait*/ QComboBox_hasFrame<i8> for () {
  fn hasFrame(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8hasFrameEv()};
    let mut ret = unsafe {_ZNK9QComboBox8hasFrameEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QValidator * QComboBox::validator();
impl /*struct*/ QComboBox {
  pub fn validator<RetType, T: QComboBox_validator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.validator(self);
    // return 1;
  }
}

pub trait QComboBox_validator<RetType> {
  fn validator(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  const QValidator * QComboBox::validator();
impl<'a> /*trait*/ QComboBox_validator<QValidator> for () {
  fn validator(self , rsthis: & QComboBox) -> QValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox9validatorEv()};
    let mut ret = unsafe {_ZNK9QComboBox9validatorEv(rsthis.qclsinst)};
    let mut ret1 = QValidator::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QComboBox::itemText(int index);
impl /*struct*/ QComboBox {
  pub fn itemText<RetType, T: QComboBox_itemText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemText(self);
    // return 1;
  }
}

pub trait QComboBox_itemText<RetType> {
  fn itemText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QString QComboBox::itemText(int index);
impl<'a> /*trait*/ QComboBox_itemText<QString> for (i32) {
  fn itemText(self , rsthis: & QComboBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QComboBox8itemTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::setItemData(int index, const QVariant & value, int role);
impl /*struct*/ QComboBox {
  pub fn setItemData<RetType, T: QComboBox_setItemData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemData(self);
    // return 1;
  }
}

pub trait QComboBox_setItemData<RetType> {
  fn setItemData(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setItemData(int index, const QVariant & value, int role);
impl<'a> /*trait*/ QComboBox_setItemData<()> for (i32, &'a QVariant, i32) {
  fn setItemData(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemDataEiRK8QVarianti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN9QComboBox11setItemDataEiRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QComboBox::hidePopup();
impl /*struct*/ QComboBox {
  pub fn hidePopup<RetType, T: QComboBox_hidePopup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hidePopup(self);
    // return 1;
  }
}

pub trait QComboBox_hidePopup<RetType> {
  fn hidePopup(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::hidePopup();
impl<'a> /*trait*/ QComboBox_hidePopup<()> for () {
  fn hidePopup(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9hidePopupEv()};
     unsafe {_ZN9QComboBox9hidePopupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QComboBox::insertItem(int index, const QIcon & icon, const QString & text, const QVariant & userData);
impl /*struct*/ QComboBox {
  pub fn insertItem<RetType, T: QComboBox_insertItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertItem(self);
    // return 1;
  }
}

pub trait QComboBox_insertItem<RetType> {
  fn insertItem(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::insertItem(int index, const QIcon & icon, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_insertItem<()> for (i32, &'a QIcon, &'a QString, &'a QVariant) {
  fn insertItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QComboBox::setCurrentText(const QString & text);
impl /*struct*/ QComboBox {
  pub fn setCurrentText<RetType, T: QComboBox_setCurrentText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentText(self);
    // return 1;
  }
}

pub trait QComboBox_setCurrentText<RetType> {
  fn setCurrentText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setCurrentText(const QString & text);
impl<'a> /*trait*/ QComboBox_setCurrentText<()> for (&'a QString) {
  fn setCurrentText(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox14setCurrentTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox14setCurrentTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QComboBox::modelColumn();
impl /*struct*/ QComboBox {
  pub fn modelColumn<RetType, T: QComboBox_modelColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.modelColumn(self);
    // return 1;
  }
}

pub trait QComboBox_modelColumn<RetType> {
  fn modelColumn(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::modelColumn();
impl<'a> /*trait*/ QComboBox_modelColumn<i32> for () {
  fn modelColumn(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11modelColumnEv()};
    let mut ret = unsafe {_ZNK9QComboBox11modelColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QComboBox::sizeHint();
impl /*struct*/ QComboBox {
  pub fn sizeHint<RetType, T: QComboBox_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QComboBox_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QSize QComboBox::sizeHint();
impl<'a> /*trait*/ QComboBox_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QComboBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QComboBox::itemData(int index, int role);
impl /*struct*/ QComboBox {
  pub fn itemData<RetType, T: QComboBox_itemData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemData(self);
    // return 1;
  }
}

pub trait QComboBox_itemData<RetType> {
  fn itemData(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QVariant QComboBox::itemData(int index, int role);
impl<'a> /*trait*/ QComboBox_itemData<QVariant> for (i32, i32) {
  fn itemData(self , rsthis: & QComboBox) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemDataEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK9QComboBox8itemDataEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::setCompleter(QCompleter * c);
impl /*struct*/ QComboBox {
  pub fn setCompleter<RetType, T: QComboBox_setCompleter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCompleter(self);
    // return 1;
  }
}

pub trait QComboBox_setCompleter<RetType> {
  fn setCompleter(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setCompleter(QCompleter * c);
impl<'a> /*trait*/ QComboBox_setCompleter<()> for (&'a QCompleter) {
  fn setCompleter(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox12setCompleterEP10QCompleter()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox12setCompleterEP10QCompleter(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QComboBox::maxVisibleItems();
impl /*struct*/ QComboBox {
  pub fn maxVisibleItems<RetType, T: QComboBox_maxVisibleItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxVisibleItems(self);
    // return 1;
  }
}

pub trait QComboBox_maxVisibleItems<RetType> {
  fn maxVisibleItems(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::maxVisibleItems();
impl<'a> /*trait*/ QComboBox_maxVisibleItems<i32> for () {
  fn maxVisibleItems(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox15maxVisibleItemsEv()};
    let mut ret = unsafe {_ZNK9QComboBox15maxVisibleItemsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QComboBox::QComboBox(QWidget * parent);
impl /*struct*/ QComboBox {
  pub fn new<T: QComboBox_new>(value: T) -> QComboBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QComboBox_new {
  fn new(self) -> QComboBox;
}

  // proto:  void QComboBox::QComboBox(QWidget * parent);
impl<'a> /*trait*/ QComboBox_new for (&'a QWidget) {
  fn new(self) -> QComboBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxC1EP7QWidget()};
    let ctysz: c_int = unsafe{QComboBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QComboBoxC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QComboBoxC1EP7QWidget(arg0)} as u64;
    let rsthis = QComboBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QComboBox::setCurrentIndex(int index);
impl /*struct*/ QComboBox {
  pub fn setCurrentIndex<RetType, T: QComboBox_setCurrentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QComboBox_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setCurrentIndex(int index);
impl<'a> /*trait*/ QComboBox_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::QComboBox(const QComboBox & );
impl<'a> /*trait*/ QComboBox_new for (&'a QComboBox) {
  fn new(self) -> QComboBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxC1ERKS_()};
    let ctysz: c_int = unsafe{QComboBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QComboBoxC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QComboBoxC1ERKS_(arg0)} as u64;
    let rsthis = QComboBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QComboBox::setRootModelIndex(const QModelIndex & index);
impl /*struct*/ QComboBox {
  pub fn setRootModelIndex<RetType, T: QComboBox_setRootModelIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootModelIndex(self);
    // return 1;
  }
}

pub trait QComboBox_setRootModelIndex<RetType> {
  fn setRootModelIndex(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setRootModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QComboBox_setRootModelIndex<()> for (&'a QModelIndex) {
  fn setRootModelIndex(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox17setRootModelIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox17setRootModelIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setEditText(const QString & text);
impl /*struct*/ QComboBox {
  pub fn setEditText<RetType, T: QComboBox_setEditText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEditText(self);
    // return 1;
  }
}

pub trait QComboBox_setEditText<RetType> {
  fn setEditText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setEditText(const QString & text);
impl<'a> /*trait*/ QComboBox_setEditText<()> for (&'a QString) {
  fn setEditText(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setEditTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setEditTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::addItem(const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_addItem<()> for (&'a QString, &'a QVariant) {
  fn addItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox7addItemERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {demth_ZN9QComboBox7addItemERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QCompleter * QComboBox::completer();
impl /*struct*/ QComboBox {
  pub fn completer<RetType, T: QComboBox_completer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completer(self);
    // return 1;
  }
}

pub trait QComboBox_completer<RetType> {
  fn completer(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QCompleter * QComboBox::completer();
impl<'a> /*trait*/ QComboBox_completer<QCompleter> for () {
  fn completer(self , rsthis: & QComboBox) -> QCompleter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox9completerEv()};
    let mut ret = unsafe {_ZNK9QComboBox9completerEv(rsthis.qclsinst)};
    let mut ret1 = QCompleter::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::removeItem(int index);
impl /*struct*/ QComboBox {
  pub fn removeItem<RetType, T: QComboBox_removeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeItem(self);
    // return 1;
  }
}

pub trait QComboBox_removeItem<RetType> {
  fn removeItem(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::removeItem(int index);
impl<'a> /*trait*/ QComboBox_removeItem<()> for (i32) {
  fn removeItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10removeItemEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox10removeItemEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QComboBox::count();
impl /*struct*/ QComboBox {
  pub fn count<RetType, T: QComboBox_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QComboBox_count<RetType> {
  fn count(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::count();
impl<'a> /*trait*/ QComboBox_count<i32> for () {
  fn count(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox5countEv()};
    let mut ret = unsafe {_ZNK9QComboBox5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QComboBox::setItemDelegate(QAbstractItemDelegate * delegate);
impl /*struct*/ QComboBox {
  pub fn setItemDelegate<RetType, T: QComboBox_setItemDelegate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegate(self);
    // return 1;
  }
}

pub trait QComboBox_setItemDelegate<RetType> {
  fn setItemDelegate(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setItemDelegate(QAbstractItemDelegate * delegate);
impl<'a> /*trait*/ QComboBox_setItemDelegate<()> for (&'a QAbstractItemDelegate) {
  fn setItemDelegate(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15setItemDelegateEP21QAbstractItemDelegate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox15setItemDelegateEP21QAbstractItemDelegate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::addItems(const QStringList & texts);
impl /*struct*/ QComboBox {
  pub fn addItems<RetType, T: QComboBox_addItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItems(self);
    // return 1;
  }
}

pub trait QComboBox_addItems<RetType> {
  fn addItems(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::addItems(const QStringList & texts);
impl<'a> /*trait*/ QComboBox_addItems<()> for (&'a QStringList) {
  fn addItems(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox8addItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN9QComboBox8addItemsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setMinimumContentsLength(int characters);
impl /*struct*/ QComboBox {
  pub fn setMinimumContentsLength<RetType, T: QComboBox_setMinimumContentsLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumContentsLength(self);
    // return 1;
  }
}

pub trait QComboBox_setMinimumContentsLength<RetType> {
  fn setMinimumContentsLength(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setMinimumContentsLength(int characters);
impl<'a> /*trait*/ QComboBox_setMinimumContentsLength<()> for (i32) {
  fn setMinimumContentsLength(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox24setMinimumContentsLengthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox24setMinimumContentsLengthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QComboBox::duplicatesEnabled();
impl /*struct*/ QComboBox {
  pub fn duplicatesEnabled<RetType, T: QComboBox_duplicatesEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.duplicatesEnabled(self);
    // return 1;
  }
}

pub trait QComboBox_duplicatesEnabled<RetType> {
  fn duplicatesEnabled(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::duplicatesEnabled();
impl<'a> /*trait*/ QComboBox_duplicatesEnabled<i8> for () {
  fn duplicatesEnabled(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox17duplicatesEnabledEv()};
    let mut ret = unsafe {_ZNK9QComboBox17duplicatesEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QComboBox::~QComboBox();
impl /*struct*/ QComboBox {
  pub fn free<RetType, T: QComboBox_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QComboBox_free<RetType> {
  fn free(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::~QComboBox();
impl<'a> /*trait*/ QComboBox_free<()> for () {
  fn free(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxD0Ev()};
     unsafe {_ZN9QComboBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QComboBox::model();
impl /*struct*/ QComboBox {
  pub fn model<RetType, T: QComboBox_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QComboBox_model<RetType> {
  fn model(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QAbstractItemModel * QComboBox::model();
impl<'a> /*trait*/ QComboBox_model<()> for () {
  fn model(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox5modelEv()};
     unsafe {_ZNK9QComboBox5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QComboBox::minimumContentsLength();
impl /*struct*/ QComboBox {
  pub fn minimumContentsLength<RetType, T: QComboBox_minimumContentsLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumContentsLength(self);
    // return 1;
  }
}

pub trait QComboBox_minimumContentsLength<RetType> {
  fn minimumContentsLength(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::minimumContentsLength();
impl<'a> /*trait*/ QComboBox_minimumContentsLength<i32> for () {
  fn minimumContentsLength(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox21minimumContentsLengthEv()};
    let mut ret = unsafe {_ZNK9QComboBox21minimumContentsLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QComboBox::isEditable();
impl /*struct*/ QComboBox {
  pub fn isEditable<RetType, T: QComboBox_isEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEditable(self);
    // return 1;
  }
}

pub trait QComboBox_isEditable<RetType> {
  fn isEditable(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::isEditable();
impl<'a> /*trait*/ QComboBox_isEditable<i8> for () {
  fn isEditable(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox10isEditableEv()};
    let mut ret = unsafe {_ZNK9QComboBox10isEditableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QComboBox::setMaxCount(int max);
impl /*struct*/ QComboBox {
  pub fn setMaxCount<RetType, T: QComboBox_setMaxCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxCount(self);
    // return 1;
  }
}

pub trait QComboBox_setMaxCount<RetType> {
  fn setMaxCount(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setMaxCount(int max);
impl<'a> /*trait*/ QComboBox_setMaxCount<()> for (i32) {
  fn setMaxCount(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setMaxCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox11setMaxCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QComboBox::currentIndex();
impl /*struct*/ QComboBox {
  pub fn currentIndex<RetType, T: QComboBox_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QComboBox_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::currentIndex();
impl<'a> /*trait*/ QComboBox_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox12currentIndexEv()};
    let mut ret = unsafe {_ZNK9QComboBox12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QComboBox::setDuplicatesEnabled(bool enable);
impl /*struct*/ QComboBox {
  pub fn setDuplicatesEnabled<RetType, T: QComboBox_setDuplicatesEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDuplicatesEnabled(self);
    // return 1;
  }
}

pub trait QComboBox_setDuplicatesEnabled<RetType> {
  fn setDuplicatesEnabled(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setDuplicatesEnabled(bool enable);
impl<'a> /*trait*/ QComboBox_setDuplicatesEnabled<()> for (i8) {
  fn setDuplicatesEnabled(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox20setDuplicatesEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QComboBox20setDuplicatesEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QComboBox::currentText();
impl /*struct*/ QComboBox {
  pub fn currentText<RetType, T: QComboBox_currentText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentText(self);
    // return 1;
  }
}

pub trait QComboBox_currentText<RetType> {
  fn currentText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QString QComboBox::currentText();
impl<'a> /*trait*/ QComboBox_currentText<QString> for () {
  fn currentText(self , rsthis: & QComboBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11currentTextEv()};
    let mut ret = unsafe {_ZNK9QComboBox11currentTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::showPopup();
impl /*struct*/ QComboBox {
  pub fn showPopup<RetType, T: QComboBox_showPopup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showPopup(self);
    // return 1;
  }
}

pub trait QComboBox_showPopup<RetType> {
  fn showPopup(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::showPopup();
impl<'a> /*trait*/ QComboBox_showPopup<()> for () {
  fn showPopup(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9showPopupEv()};
     unsafe {_ZN9QComboBox9showPopupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QLineEdit * QComboBox::lineEdit();
impl /*struct*/ QComboBox {
  pub fn lineEdit<RetType, T: QComboBox_lineEdit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineEdit(self);
    // return 1;
  }
}

pub trait QComboBox_lineEdit<RetType> {
  fn lineEdit(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QLineEdit * QComboBox::lineEdit();
impl<'a> /*trait*/ QComboBox_lineEdit<QLineEdit> for () {
  fn lineEdit(self , rsthis: & QComboBox) -> QLineEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8lineEditEv()};
    let mut ret = unsafe {_ZNK9QComboBox8lineEditEv(rsthis.qclsinst)};
    let mut ret1 = QLineEdit::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAbstractItemDelegate * QComboBox::itemDelegate();
impl /*struct*/ QComboBox {
  pub fn itemDelegate<RetType, T: QComboBox_itemDelegate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate(self);
    // return 1;
  }
}

pub trait QComboBox_itemDelegate<RetType> {
  fn itemDelegate(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QAbstractItemDelegate * QComboBox::itemDelegate();
impl<'a> /*trait*/ QComboBox_itemDelegate<()> for () {
  fn itemDelegate(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox12itemDelegateEv()};
     unsafe {_ZNK9QComboBox12itemDelegateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QComboBox::setMaxVisibleItems(int maxItems);
impl /*struct*/ QComboBox {
  pub fn setMaxVisibleItems<RetType, T: QComboBox_setMaxVisibleItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxVisibleItems(self);
    // return 1;
  }
}

pub trait QComboBox_setMaxVisibleItems<RetType> {
  fn setMaxVisibleItems(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setMaxVisibleItems(int maxItems);
impl<'a> /*trait*/ QComboBox_setMaxVisibleItems<()> for (i32) {
  fn setMaxVisibleItems(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox18setMaxVisibleItemsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox18setMaxVisibleItemsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QComboBox::event(QEvent * event);
impl /*struct*/ QComboBox {
  pub fn event<RetType, T: QComboBox_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QComboBox_event<RetType> {
  fn event(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::event(QEvent * event);
impl<'a> /*trait*/ QComboBox_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QComboBox5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QComboBox::setModelColumn(int visibleColumn);
impl /*struct*/ QComboBox {
  pub fn setModelColumn<RetType, T: QComboBox_setModelColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModelColumn(self);
    // return 1;
  }
}

pub trait QComboBox_setModelColumn<RetType> {
  fn setModelColumn(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setModelColumn(int visibleColumn);
impl<'a> /*trait*/ QComboBox_setModelColumn<()> for (i32) {
  fn setModelColumn(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox14setModelColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox14setModelColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setItemText(int index, const QString & text);
impl /*struct*/ QComboBox {
  pub fn setItemText<RetType, T: QComboBox_setItemText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemText(self);
    // return 1;
  }
}

pub trait QComboBox_setItemText<RetType> {
  fn setItemText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setItemText(int index, const QString & text);
impl<'a> /*trait*/ QComboBox_setItemText<()> for (i32, &'a QString) {
  fn setItemText(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setItemTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QComboBox::setLineEdit(QLineEdit * edit);
impl /*struct*/ QComboBox {
  pub fn setLineEdit<RetType, T: QComboBox_setLineEdit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLineEdit(self);
    // return 1;
  }
}

pub trait QComboBox_setLineEdit<RetType> {
  fn setLineEdit(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setLineEdit(QLineEdit * edit);
impl<'a> /*trait*/ QComboBox_setLineEdit<()> for (&'a QLineEdit) {
  fn setLineEdit(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setLineEditEP9QLineEdit()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setLineEditEP9QLineEdit(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QComboBox::itemIcon(int index);
impl /*struct*/ QComboBox {
  pub fn itemIcon<RetType, T: QComboBox_itemIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemIcon(self);
    // return 1;
  }
}

pub trait QComboBox_itemIcon<RetType> {
  fn itemIcon(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QIcon QComboBox::itemIcon(int index);
impl<'a> /*trait*/ QComboBox_itemIcon<QIcon> for (i32) {
  fn itemIcon(self , rsthis: & QComboBox) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemIconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QComboBox8itemIconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::insertItem(int index, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_insertItem<()> for (i32, &'a QString, &'a QVariant) {
  fn insertItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10insertItemEiRK7QStringRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {demth_ZN9QComboBox10insertItemEiRK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QComboBox::setValidator(const QValidator * v);
impl /*struct*/ QComboBox {
  pub fn setValidator<RetType, T: QComboBox_setValidator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValidator(self);
    // return 1;
  }
}

pub trait QComboBox_setValidator<RetType> {
  fn setValidator(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setValidator(const QValidator * v);
impl<'a> /*trait*/ QComboBox_setValidator<()> for (&'a QValidator) {
  fn setValidator(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox12setValidatorEPK10QValidator()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox12setValidatorEPK10QValidator(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::insertSeparator(int index);
impl /*struct*/ QComboBox {
  pub fn insertSeparator<RetType, T: QComboBox_insertSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertSeparator(self);
    // return 1;
  }
}

pub trait QComboBox_insertSeparator<RetType> {
  fn insertSeparator(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::insertSeparator(int index);
impl<'a> /*trait*/ QComboBox_insertSeparator<()> for (i32) {
  fn insertSeparator(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15insertSeparatorEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox15insertSeparatorEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QComboBox::metaObject();
impl /*struct*/ QComboBox {
  pub fn metaObject<RetType, T: QComboBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QComboBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  const QMetaObject * QComboBox::metaObject();
impl<'a> /*trait*/ QComboBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox10metaObjectEv()};
     unsafe {_ZNK9QComboBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QComboBox_currentIndexChanged
pub struct QComboBox_currentIndexChanged_signal{poi:u64}
impl /* struct */ QComboBox {
  pub fn currentIndexChanged(&self) -> QComboBox_currentIndexChanged_signal {
     return QComboBox_currentIndexChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QComboBox_currentIndexChanged_signal {
  pub fn connect<T: QComboBox_currentIndexChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QComboBox_currentIndexChanged_signal_connect {
  fn connect(self, sigthis: QComboBox_currentIndexChanged_signal);
}

#[derive(Default)] // for QComboBox_currentTextChanged
pub struct QComboBox_currentTextChanged_signal{poi:u64}
impl /* struct */ QComboBox {
  pub fn currentTextChanged(&self) -> QComboBox_currentTextChanged_signal {
     return QComboBox_currentTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QComboBox_currentTextChanged_signal {
  pub fn connect<T: QComboBox_currentTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QComboBox_currentTextChanged_signal_connect {
  fn connect(self, sigthis: QComboBox_currentTextChanged_signal);
}

#[derive(Default)] // for QComboBox_highlighted
pub struct QComboBox_highlighted_signal{poi:u64}
impl /* struct */ QComboBox {
  pub fn highlighted(&self) -> QComboBox_highlighted_signal {
     return QComboBox_highlighted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QComboBox_highlighted_signal {
  pub fn connect<T: QComboBox_highlighted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QComboBox_highlighted_signal_connect {
  fn connect(self, sigthis: QComboBox_highlighted_signal);
}

#[derive(Default)] // for QComboBox_activated
pub struct QComboBox_activated_signal{poi:u64}
impl /* struct */ QComboBox {
  pub fn activated(&self) -> QComboBox_activated_signal {
     return QComboBox_activated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QComboBox_activated_signal {
  pub fn connect<T: QComboBox_activated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QComboBox_activated_signal_connect {
  fn connect(self, sigthis: QComboBox_activated_signal);
}

#[derive(Default)] // for QComboBox_editTextChanged
pub struct QComboBox_editTextChanged_signal{poi:u64}
impl /* struct */ QComboBox {
  pub fn editTextChanged(&self) -> QComboBox_editTextChanged_signal {
     return QComboBox_editTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QComboBox_editTextChanged_signal {
  pub fn connect<T: QComboBox_editTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QComboBox_editTextChanged_signal_connect {
  fn connect(self, sigthis: QComboBox_editTextChanged_signal);
}

// activated(const class QString &)
extern fn QComboBox_activated_signal_connect_cb_0(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QComboBox_activated_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QComboBox_activated_signal_connect for fn(QString) {
  fn connect(self, sigthis: QComboBox_activated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_activated_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox9activatedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QComboBox_activated_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QComboBox_activated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_activated_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox9activatedERK7QString(arg0, arg1, arg2)};
  }
}
// currentIndexChanged(const class QString &)
extern fn QComboBox_currentIndexChanged_signal_connect_cb_1(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QComboBox_currentIndexChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QComboBox_currentIndexChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QComboBox_currentIndexChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_currentIndexChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox19currentIndexChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QComboBox_currentIndexChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QComboBox_currentIndexChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_currentIndexChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox19currentIndexChangedERK7QString(arg0, arg1, arg2)};
  }
}
// currentTextChanged(const class QString &)
extern fn QComboBox_currentTextChanged_signal_connect_cb_2(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QComboBox_currentTextChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QComboBox_currentTextChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QComboBox_currentTextChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_currentTextChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox18currentTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QComboBox_currentTextChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QComboBox_currentTextChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_currentTextChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox18currentTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
// highlighted(const class QString &)
extern fn QComboBox_highlighted_signal_connect_cb_3(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QComboBox_highlighted_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QComboBox_highlighted_signal_connect for fn(QString) {
  fn connect(self, sigthis: QComboBox_highlighted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_highlighted_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox11highlightedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QComboBox_highlighted_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QComboBox_highlighted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_highlighted_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox11highlightedERK7QString(arg0, arg1, arg2)};
  }
}
// editTextChanged(const class QString &)
extern fn QComboBox_editTextChanged_signal_connect_cb_4(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QComboBox_editTextChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QComboBox_editTextChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QComboBox_editTextChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_editTextChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox15editTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QComboBox_editTextChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QComboBox_editTextChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_editTextChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox15editTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
// highlighted(int)
extern fn QComboBox_highlighted_signal_connect_cb_5(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QComboBox_highlighted_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QComboBox_highlighted_signal_connect for fn(i32) {
  fn connect(self, sigthis: QComboBox_highlighted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_highlighted_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox11highlightedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QComboBox_highlighted_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QComboBox_highlighted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_highlighted_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox11highlightedEi(arg0, arg1, arg2)};
  }
}
// activated(int)
extern fn QComboBox_activated_signal_connect_cb_6(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QComboBox_activated_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QComboBox_activated_signal_connect for fn(i32) {
  fn connect(self, sigthis: QComboBox_activated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_activated_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox9activatedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QComboBox_activated_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QComboBox_activated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_activated_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox9activatedEi(arg0, arg1, arg2)};
  }
}
// currentIndexChanged(int)
extern fn QComboBox_currentIndexChanged_signal_connect_cb_7(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QComboBox_currentIndexChanged_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QComboBox_currentIndexChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QComboBox_currentIndexChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_currentIndexChanged_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox19currentIndexChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QComboBox_currentIndexChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QComboBox_currentIndexChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QComboBox_currentIndexChanged_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QComboBox_SlotProxy_connect__ZN9QComboBox19currentIndexChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

