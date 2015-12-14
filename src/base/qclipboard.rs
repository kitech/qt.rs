// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QClipboard::FreeQClipboard();
  fn _ZN10QClipboardD0Ev(qthis: *mut c_void) ;
  // proto:  void QClipboard::NewQClipboard(QObject * parent);
  fn _ZN10QClipboardC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QClipboard::NewQClipboard(const QClipboard & );
  fn _ZN10QClipboardC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QClipboard::supportsFindBuffer();
  fn _ZNK10QClipboard18supportsFindBufferEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QClipboard::selectionChanged();
  fn _ZN10QClipboard16selectionChangedEv(qthis: *mut c_void) ;
  // proto:  bool QClipboard::ownsFindBuffer();
  fn _ZNK10QClipboard14ownsFindBufferEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QClipboard::dataChanged();
  fn _ZN10QClipboard11dataChangedEv(qthis: *mut c_void) ;
  // proto:  bool QClipboard::ownsClipboard();
  fn _ZNK10QClipboard13ownsClipboardEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QClipboard::metaObject();
  fn _ZNK10QClipboard10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QClipboard::supportsSelection();
  fn _ZNK10QClipboard17supportsSelectionEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QClipboard::ownsSelection();
  fn _ZNK10QClipboard13ownsSelectionEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QClipboard::findBufferChanged();
  fn _ZN10QClipboard17findBufferChangedEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QClipboard)=1
pub struct QClipboard {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QClipboard {
  pub fn FreeQClipboard<T: QClipboard_FreeQClipboard>(&mut self, value: T)  {
     value.FreeQClipboard(self);
    // return 1;
  }
}

pub trait QClipboard_FreeQClipboard {
  fn FreeQClipboard(self, rsthis: &mut QClipboard) ;
}

// proto:  void QClipboard::FreeQClipboard();
impl<'a> /*trait*/ QClipboard_FreeQClipboard for () {
  fn FreeQClipboard(self, rsthis: &mut QClipboard)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboardD0Ev()};
     unsafe {_ZN10QClipboardD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn NewQClipboard<T: QClipboard_NewQClipboard>(value: T) -> QClipboard {
    let rsthis = value.NewQClipboard();
    return rsthis;
    // return 1;
  }
}

pub trait QClipboard_NewQClipboard {
  fn NewQClipboard(self) -> QClipboard;
}

// proto: void QClipboard::NewQClipboard(QObject * parent);
impl<'a> /*trait*/ QClipboard_NewQClipboard for (&'a mut QObject) {
  fn NewQClipboard(self) -> QClipboard {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboardC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QClipboardC1EP7QObject(qthis, arg0)};
    let rsthis = QClipboard{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QClipboard::NewQClipboard(const QClipboard & );
impl<'a> /*trait*/ QClipboard_NewQClipboard for (&'a  QClipboard) {
  fn NewQClipboard(self) -> QClipboard {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboardC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QClipboardC1ERKS_(qthis, arg0)};
    let rsthis = QClipboard{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn supportsFindBuffer<T: QClipboard_supportsFindBuffer>(&mut self, value: T) -> i8 {
    return value.supportsFindBuffer(self);
    // return 1;
  }
}

pub trait QClipboard_supportsFindBuffer {
  fn supportsFindBuffer(self, rsthis: &mut QClipboard) -> i8;
}

// proto:  bool QClipboard::supportsFindBuffer();
impl<'a> /*trait*/ QClipboard_supportsFindBuffer for () {
  fn supportsFindBuffer(self, rsthis: &mut QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard18supportsFindBufferEv()};
    let mut ret = unsafe {_ZNK10QClipboard18supportsFindBufferEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn selectionChanged<T: QClipboard_selectionChanged>(&mut self, value: T)  {
     value.selectionChanged(self);
    // return 1;
  }
}

pub trait QClipboard_selectionChanged {
  fn selectionChanged(self, rsthis: &mut QClipboard) ;
}

// proto:  void QClipboard::selectionChanged();
impl<'a> /*trait*/ QClipboard_selectionChanged for () {
  fn selectionChanged(self, rsthis: &mut QClipboard)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard16selectionChangedEv()};
     unsafe {_ZN10QClipboard16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn ownsFindBuffer<T: QClipboard_ownsFindBuffer>(&mut self, value: T) -> i8 {
    return value.ownsFindBuffer(self);
    // return 1;
  }
}

pub trait QClipboard_ownsFindBuffer {
  fn ownsFindBuffer(self, rsthis: &mut QClipboard) -> i8;
}

// proto:  bool QClipboard::ownsFindBuffer();
impl<'a> /*trait*/ QClipboard_ownsFindBuffer for () {
  fn ownsFindBuffer(self, rsthis: &mut QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard14ownsFindBufferEv()};
    let mut ret = unsafe {_ZNK10QClipboard14ownsFindBufferEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn dataChanged<T: QClipboard_dataChanged>(&mut self, value: T)  {
     value.dataChanged(self);
    // return 1;
  }
}

pub trait QClipboard_dataChanged {
  fn dataChanged(self, rsthis: &mut QClipboard) ;
}

// proto:  void QClipboard::dataChanged();
impl<'a> /*trait*/ QClipboard_dataChanged for () {
  fn dataChanged(self, rsthis: &mut QClipboard)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard11dataChangedEv()};
     unsafe {_ZN10QClipboard11dataChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn ownsClipboard<T: QClipboard_ownsClipboard>(&mut self, value: T) -> i8 {
    return value.ownsClipboard(self);
    // return 1;
  }
}

pub trait QClipboard_ownsClipboard {
  fn ownsClipboard(self, rsthis: &mut QClipboard) -> i8;
}

// proto:  bool QClipboard::ownsClipboard();
impl<'a> /*trait*/ QClipboard_ownsClipboard for () {
  fn ownsClipboard(self, rsthis: &mut QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard13ownsClipboardEv()};
    let mut ret = unsafe {_ZNK10QClipboard13ownsClipboardEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn metaObject<T: QClipboard_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QClipboard_metaObject {
  fn metaObject(self, rsthis: &mut QClipboard) ;
}

// proto:  const QMetaObject * QClipboard::metaObject();
impl<'a> /*trait*/ QClipboard_metaObject for () {
  fn metaObject(self, rsthis: &mut QClipboard)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard10metaObjectEv()};
     unsafe {_ZNK10QClipboard10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn supportsSelection<T: QClipboard_supportsSelection>(&mut self, value: T) -> i8 {
    return value.supportsSelection(self);
    // return 1;
  }
}

pub trait QClipboard_supportsSelection {
  fn supportsSelection(self, rsthis: &mut QClipboard) -> i8;
}

// proto:  bool QClipboard::supportsSelection();
impl<'a> /*trait*/ QClipboard_supportsSelection for () {
  fn supportsSelection(self, rsthis: &mut QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard17supportsSelectionEv()};
    let mut ret = unsafe {_ZNK10QClipboard17supportsSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn ownsSelection<T: QClipboard_ownsSelection>(&mut self, value: T) -> i8 {
    return value.ownsSelection(self);
    // return 1;
  }
}

pub trait QClipboard_ownsSelection {
  fn ownsSelection(self, rsthis: &mut QClipboard) -> i8;
}

// proto:  bool QClipboard::ownsSelection();
impl<'a> /*trait*/ QClipboard_ownsSelection for () {
  fn ownsSelection(self, rsthis: &mut QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard13ownsSelectionEv()};
    let mut ret = unsafe {_ZNK10QClipboard13ownsSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn findBufferChanged<T: QClipboard_findBufferChanged>(&mut self, value: T)  {
     value.findBufferChanged(self);
    // return 1;
  }
}

pub trait QClipboard_findBufferChanged {
  fn findBufferChanged(self, rsthis: &mut QClipboard) ;
}

// proto:  void QClipboard::findBufferChanged();
impl<'a> /*trait*/ QClipboard_findBufferChanged for () {
  fn findBufferChanged(self, rsthis: &mut QClipboard)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard17findBufferChangedEv()};
     unsafe {_ZN10QClipboard17findBufferChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

