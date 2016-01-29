// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquickframebufferobject.h
// dst-file: /src/quick/qquickframebufferobject.rs
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
use super::qquickitem::QQuickItem; // 773
use std::ops::Deref;
use super::qsgtextureprovider::QSGTextureProvider; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickFramebufferObject_Class_Size() -> c_int;
  // proto:  bool QQuickFramebufferObject::textureFollowsItemSize();
  fn _ZNK23QQuickFramebufferObject22textureFollowsItemSizeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickFramebufferObject::setTextureFollowsItemSize(bool follows);
  fn _ZN23QQuickFramebufferObject25setTextureFollowsItemSizeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QQuickFramebufferObject::metaObject();
  fn _ZNK23QQuickFramebufferObject10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickFramebufferObject::QQuickFramebufferObject(QQuickItem * parent);
  fn _ZN23QQuickFramebufferObjectC2EP10QQuickItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QQuickFramebufferObject::isTextureProvider();
  fn _ZNK23QQuickFramebufferObject17isTextureProviderEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSGTextureProvider * QQuickFramebufferObject::textureProvider();
  fn _ZNK23QQuickFramebufferObject15textureProviderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickFramebufferObject::releaseResources();
  fn _ZN23QQuickFramebufferObject16releaseResourcesEv(qthis: u64 /* *mut c_void*/);
  fn QQuickFramebufferObject_SlotProxy_connect__ZN23QQuickFramebufferObject29textureFollowsItemSizeChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQuickFramebufferObject)=1
#[derive(Default)]
pub struct QQuickFramebufferObject {
  qbase: QQuickItem,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _textureFollowsItemSizeChanged: QQuickFramebufferObject_textureFollowsItemSizeChanged_signal,
}

impl /*struct*/ QQuickFramebufferObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickFramebufferObject {
    return QQuickFramebufferObject{qbase: QQuickItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickFramebufferObject {
  type Target = QQuickItem;

  fn deref(&self) -> &QQuickItem {
    return & self.qbase;
  }
}
impl AsRef<QQuickItem> for QQuickFramebufferObject {
  fn as_ref(& self) -> & QQuickItem {
    return & self.qbase;
  }
}
  // proto:  bool QQuickFramebufferObject::textureFollowsItemSize();
impl /*struct*/ QQuickFramebufferObject {
  pub fn textureFollowsItemSize<RetType, T: QQuickFramebufferObject_textureFollowsItemSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureFollowsItemSize(self);
    // return 1;
  }
}

pub trait QQuickFramebufferObject_textureFollowsItemSize<RetType> {
  fn textureFollowsItemSize(self , rsthis: & QQuickFramebufferObject) -> RetType;
}

  // proto:  bool QQuickFramebufferObject::textureFollowsItemSize();
impl<'a> /*trait*/ QQuickFramebufferObject_textureFollowsItemSize<i8> for () {
  fn textureFollowsItemSize(self , rsthis: & QQuickFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QQuickFramebufferObject22textureFollowsItemSizeEv()};
    let mut ret = unsafe {_ZNK23QQuickFramebufferObject22textureFollowsItemSizeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickFramebufferObject::setTextureFollowsItemSize(bool follows);
impl /*struct*/ QQuickFramebufferObject {
  pub fn setTextureFollowsItemSize<RetType, T: QQuickFramebufferObject_setTextureFollowsItemSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextureFollowsItemSize(self);
    // return 1;
  }
}

pub trait QQuickFramebufferObject_setTextureFollowsItemSize<RetType> {
  fn setTextureFollowsItemSize(self , rsthis: & QQuickFramebufferObject) -> RetType;
}

  // proto:  void QQuickFramebufferObject::setTextureFollowsItemSize(bool follows);
impl<'a> /*trait*/ QQuickFramebufferObject_setTextureFollowsItemSize<()> for (i8) {
  fn setTextureFollowsItemSize(self , rsthis: & QQuickFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QQuickFramebufferObject25setTextureFollowsItemSizeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN23QQuickFramebufferObject25setTextureFollowsItemSizeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QQuickFramebufferObject::metaObject();
impl /*struct*/ QQuickFramebufferObject {
  pub fn metaObject<RetType, T: QQuickFramebufferObject_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickFramebufferObject_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickFramebufferObject) -> RetType;
}

  // proto:  const QMetaObject * QQuickFramebufferObject::metaObject();
impl<'a> /*trait*/ QQuickFramebufferObject_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QQuickFramebufferObject10metaObjectEv()};
     unsafe {_ZNK23QQuickFramebufferObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickFramebufferObject::QQuickFramebufferObject(QQuickItem * parent);
impl /*struct*/ QQuickFramebufferObject {
  pub fn new<T: QQuickFramebufferObject_new>(value: T) -> QQuickFramebufferObject {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickFramebufferObject_new {
  fn new(self) -> QQuickFramebufferObject;
}

  // proto:  void QQuickFramebufferObject::QQuickFramebufferObject(QQuickItem * parent);
impl<'a> /*trait*/ QQuickFramebufferObject_new for (&'a QQuickItem) {
  fn new(self) -> QQuickFramebufferObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QQuickFramebufferObjectC2EP10QQuickItem()};
    let ctysz: c_int = unsafe{QQuickFramebufferObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QQuickFramebufferObjectC2EP10QQuickItem(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickFramebufferObject{qbase: QQuickItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQuickFramebufferObject::isTextureProvider();
impl /*struct*/ QQuickFramebufferObject {
  pub fn isTextureProvider<RetType, T: QQuickFramebufferObject_isTextureProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTextureProvider(self);
    // return 1;
  }
}

pub trait QQuickFramebufferObject_isTextureProvider<RetType> {
  fn isTextureProvider(self , rsthis: & QQuickFramebufferObject) -> RetType;
}

  // proto:  bool QQuickFramebufferObject::isTextureProvider();
impl<'a> /*trait*/ QQuickFramebufferObject_isTextureProvider<i8> for () {
  fn isTextureProvider(self , rsthis: & QQuickFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QQuickFramebufferObject17isTextureProviderEv()};
    let mut ret = unsafe {_ZNK23QQuickFramebufferObject17isTextureProviderEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSGTextureProvider * QQuickFramebufferObject::textureProvider();
impl /*struct*/ QQuickFramebufferObject {
  pub fn textureProvider<RetType, T: QQuickFramebufferObject_textureProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureProvider(self);
    // return 1;
  }
}

pub trait QQuickFramebufferObject_textureProvider<RetType> {
  fn textureProvider(self , rsthis: & QQuickFramebufferObject) -> RetType;
}

  // proto:  QSGTextureProvider * QQuickFramebufferObject::textureProvider();
impl<'a> /*trait*/ QQuickFramebufferObject_textureProvider<QSGTextureProvider> for () {
  fn textureProvider(self , rsthis: & QQuickFramebufferObject) -> QSGTextureProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QQuickFramebufferObject15textureProviderEv()};
    let mut ret = unsafe {_ZNK23QQuickFramebufferObject15textureProviderEv(rsthis.qclsinst)};
    let mut ret1 = QSGTextureProvider::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickFramebufferObject::releaseResources();
impl /*struct*/ QQuickFramebufferObject {
  pub fn releaseResources<RetType, T: QQuickFramebufferObject_releaseResources<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.releaseResources(self);
    // return 1;
  }
}

pub trait QQuickFramebufferObject_releaseResources<RetType> {
  fn releaseResources(self , rsthis: & QQuickFramebufferObject) -> RetType;
}

  // proto:  void QQuickFramebufferObject::releaseResources();
impl<'a> /*trait*/ QQuickFramebufferObject_releaseResources<()> for () {
  fn releaseResources(self , rsthis: & QQuickFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QQuickFramebufferObject16releaseResourcesEv()};
     unsafe {_ZN23QQuickFramebufferObject16releaseResourcesEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QQuickFramebufferObject_textureFollowsItemSizeChanged
pub struct QQuickFramebufferObject_textureFollowsItemSizeChanged_signal{poi:u64}
impl /* struct */ QQuickFramebufferObject {
  pub fn textureFollowsItemSizeChanged(&self) -> QQuickFramebufferObject_textureFollowsItemSizeChanged_signal {
     return QQuickFramebufferObject_textureFollowsItemSizeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickFramebufferObject_textureFollowsItemSizeChanged_signal {
  pub fn connect<T: QQuickFramebufferObject_textureFollowsItemSizeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickFramebufferObject_textureFollowsItemSizeChanged_signal_connect {
  fn connect(self, sigthis: QQuickFramebufferObject_textureFollowsItemSizeChanged_signal);
}

// textureFollowsItemSizeChanged(_Bool)
extern fn QQuickFramebufferObject_textureFollowsItemSizeChanged_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QQuickFramebufferObject_textureFollowsItemSizeChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickFramebufferObject_textureFollowsItemSizeChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QQuickFramebufferObject_textureFollowsItemSizeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickFramebufferObject_textureFollowsItemSizeChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickFramebufferObject_SlotProxy_connect__ZN23QQuickFramebufferObject29textureFollowsItemSizeChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickFramebufferObject_textureFollowsItemSizeChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QQuickFramebufferObject_textureFollowsItemSizeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickFramebufferObject_textureFollowsItemSizeChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickFramebufferObject_SlotProxy_connect__ZN23QQuickFramebufferObject29textureFollowsItemSizeChangedEb(arg0, arg1, arg2)};
  }
}
// <= body block end

