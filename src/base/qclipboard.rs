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
  // proto: void QClipboard::FreeQClipboard();
  fn _ZN10QClipboardD0Ev() -> i32;
  // proto: void QClipboard::NewQClipboard(QObject * parent);
  fn _ZN10QClipboardC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QClipboard::NewQClipboard(const QClipboard & );
  fn _ZN10QClipboardC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QClipboard::supportsFindBuffer();
  fn _ZNK10QClipboard18supportsFindBufferEv() -> i32;
  // proto: void QClipboard::selectionChanged();
  fn _ZN10QClipboard16selectionChangedEv() -> i32;
  // proto: bool QClipboard::ownsFindBuffer();
  fn _ZNK10QClipboard14ownsFindBufferEv() -> i32;
  // proto: void QClipboard::dataChanged();
  fn _ZN10QClipboard11dataChangedEv() -> i32;
  // proto: bool QClipboard::ownsClipboard();
  fn _ZNK10QClipboard13ownsClipboardEv() -> i32;
  // proto: const QMetaObject * QClipboard::metaObject();
  fn _ZNK10QClipboard10metaObjectEv() -> i32;
  // proto: bool QClipboard::supportsSelection();
  fn _ZNK10QClipboard17supportsSelectionEv() -> i32;
  // proto: bool QClipboard::ownsSelection();
  fn _ZNK10QClipboard13ownsSelectionEv() -> i32;
  // proto: void QClipboard::findBufferChanged();
  fn _ZN10QClipboard17findBufferChangedEv() -> i32;
}

// body block begin
// class sizeof(QClipboard)=1
pub struct QClipboard {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QClipboard {
  pub fn FreeQClipboard<T: QClipboard_FreeQClipboard>(&mut self, value: T) -> i32 {
    value.FreeQClipboard(self);
    return 1;
  }
}

pub trait QClipboard_FreeQClipboard {
  fn FreeQClipboard(self, this: &mut QClipboard) -> i32;
}

// proto: void QClipboard::FreeQClipboard();
impl<'a> /*trait*/ QClipboard_FreeQClipboard for () {
  fn FreeQClipboard(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboardD0Ev()};
    unsafe {_ZN10QClipboardD0Ev()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QClipboardC1ERKS_(qthis, arg0)};
    let rsthis = QClipboard{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn supportsFindBuffer<T: QClipboard_supportsFindBuffer>(&mut self, value: T) -> i32 {
    value.supportsFindBuffer(self);
    return 1;
  }
}

pub trait QClipboard_supportsFindBuffer {
  fn supportsFindBuffer(self, this: &mut QClipboard) -> i32;
}

// proto: bool QClipboard::supportsFindBuffer();
impl<'a> /*trait*/ QClipboard_supportsFindBuffer for () {
  fn supportsFindBuffer(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard18supportsFindBufferEv()};
    unsafe {_ZNK10QClipboard18supportsFindBufferEv()};
    return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn selectionChanged<T: QClipboard_selectionChanged>(&mut self, value: T) -> i32 {
    value.selectionChanged(self);
    return 1;
  }
}

pub trait QClipboard_selectionChanged {
  fn selectionChanged(self, this: &mut QClipboard) -> i32;
}

// proto: void QClipboard::selectionChanged();
impl<'a> /*trait*/ QClipboard_selectionChanged for () {
  fn selectionChanged(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard16selectionChangedEv()};
    unsafe {_ZN10QClipboard16selectionChangedEv()};
    return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn ownsFindBuffer<T: QClipboard_ownsFindBuffer>(&mut self, value: T) -> i32 {
    value.ownsFindBuffer(self);
    return 1;
  }
}

pub trait QClipboard_ownsFindBuffer {
  fn ownsFindBuffer(self, this: &mut QClipboard) -> i32;
}

// proto: bool QClipboard::ownsFindBuffer();
impl<'a> /*trait*/ QClipboard_ownsFindBuffer for () {
  fn ownsFindBuffer(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard14ownsFindBufferEv()};
    unsafe {_ZNK10QClipboard14ownsFindBufferEv()};
    return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn dataChanged<T: QClipboard_dataChanged>(&mut self, value: T) -> i32 {
    value.dataChanged(self);
    return 1;
  }
}

pub trait QClipboard_dataChanged {
  fn dataChanged(self, this: &mut QClipboard) -> i32;
}

// proto: void QClipboard::dataChanged();
impl<'a> /*trait*/ QClipboard_dataChanged for () {
  fn dataChanged(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard11dataChangedEv()};
    unsafe {_ZN10QClipboard11dataChangedEv()};
    return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn ownsClipboard<T: QClipboard_ownsClipboard>(&mut self, value: T) -> i32 {
    value.ownsClipboard(self);
    return 1;
  }
}

pub trait QClipboard_ownsClipboard {
  fn ownsClipboard(self, this: &mut QClipboard) -> i32;
}

// proto: bool QClipboard::ownsClipboard();
impl<'a> /*trait*/ QClipboard_ownsClipboard for () {
  fn ownsClipboard(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard13ownsClipboardEv()};
    unsafe {_ZNK10QClipboard13ownsClipboardEv()};
    return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn metaObject<T: QClipboard_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QClipboard_metaObject {
  fn metaObject(self, this: &mut QClipboard) -> i32;
}

// proto: const QMetaObject * QClipboard::metaObject();
impl<'a> /*trait*/ QClipboard_metaObject for () {
  fn metaObject(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard10metaObjectEv()};
    unsafe {_ZNK10QClipboard10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn supportsSelection<T: QClipboard_supportsSelection>(&mut self, value: T) -> i32 {
    value.supportsSelection(self);
    return 1;
  }
}

pub trait QClipboard_supportsSelection {
  fn supportsSelection(self, this: &mut QClipboard) -> i32;
}

// proto: bool QClipboard::supportsSelection();
impl<'a> /*trait*/ QClipboard_supportsSelection for () {
  fn supportsSelection(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard17supportsSelectionEv()};
    unsafe {_ZNK10QClipboard17supportsSelectionEv()};
    return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn ownsSelection<T: QClipboard_ownsSelection>(&mut self, value: T) -> i32 {
    value.ownsSelection(self);
    return 1;
  }
}

pub trait QClipboard_ownsSelection {
  fn ownsSelection(self, this: &mut QClipboard) -> i32;
}

// proto: bool QClipboard::ownsSelection();
impl<'a> /*trait*/ QClipboard_ownsSelection for () {
  fn ownsSelection(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard13ownsSelectionEv()};
    unsafe {_ZNK10QClipboard13ownsSelectionEv()};
    return 1;
  }
}

impl /*struct*/ QClipboard {
  pub fn findBufferChanged<T: QClipboard_findBufferChanged>(&mut self, value: T) -> i32 {
    value.findBufferChanged(self);
    return 1;
  }
}

pub trait QClipboard_findBufferChanged {
  fn findBufferChanged(self, this: &mut QClipboard) -> i32;
}

// proto: void QClipboard::findBufferChanged();
impl<'a> /*trait*/ QClipboard_findBufferChanged for () {
  fn findBufferChanged(self, this: &mut QClipboard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard17findBufferChangedEv()};
    unsafe {_ZN10QClipboard17findBufferChangedEv()};
    return 1;
  }
}

