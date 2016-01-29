// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgtextureprovider.h
// dst-file: /src/quick/qsgtextureprovider.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::qsgtexture::QSGTexture; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGTextureProvider_Class_Size() -> c_int;
  // proto:  QSGTexture * QSGTextureProvider::texture();
  fn _ZNK18QSGTextureProvider7textureEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QSGTextureProvider::metaObject();
  fn _ZNK18QSGTextureProvider10metaObjectEv(qthis: u64 /* *mut c_void*/);
  fn QSGTextureProvider_SlotProxy_connect__ZN18QSGTextureProvider14textureChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSGTextureProvider)=1
#[derive(Default)]
pub struct QSGTextureProvider {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _textureChanged: QSGTextureProvider_textureChanged_signal,
}

impl /*struct*/ QSGTextureProvider {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGTextureProvider {
    return QSGTextureProvider{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGTextureProvider {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSGTextureProvider {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QSGTexture * QSGTextureProvider::texture();
impl /*struct*/ QSGTextureProvider {
  pub fn texture<RetType, T: QSGTextureProvider_texture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.texture(self);
    // return 1;
  }
}

pub trait QSGTextureProvider_texture<RetType> {
  fn texture(self , rsthis: & QSGTextureProvider) -> RetType;
}

  // proto:  QSGTexture * QSGTextureProvider::texture();
impl<'a> /*trait*/ QSGTextureProvider_texture<QSGTexture> for () {
  fn texture(self , rsthis: & QSGTextureProvider) -> QSGTexture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSGTextureProvider7textureEv()};
    let mut ret = unsafe {_ZNK18QSGTextureProvider7textureEv(rsthis.qclsinst)};
    let mut ret1 = QSGTexture::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSGTextureProvider::metaObject();
impl /*struct*/ QSGTextureProvider {
  pub fn metaObject<RetType, T: QSGTextureProvider_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSGTextureProvider_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSGTextureProvider) -> RetType;
}

  // proto:  const QMetaObject * QSGTextureProvider::metaObject();
impl<'a> /*trait*/ QSGTextureProvider_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSGTextureProvider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSGTextureProvider10metaObjectEv()};
     unsafe {_ZNK18QSGTextureProvider10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QSGTextureProvider_textureChanged
pub struct QSGTextureProvider_textureChanged_signal{poi:u64}
impl /* struct */ QSGTextureProvider {
  pub fn textureChanged(&self) -> QSGTextureProvider_textureChanged_signal {
     return QSGTextureProvider_textureChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSGTextureProvider_textureChanged_signal {
  pub fn connect<T: QSGTextureProvider_textureChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSGTextureProvider_textureChanged_signal_connect {
  fn connect(self, sigthis: QSGTextureProvider_textureChanged_signal);
}

// textureChanged()
extern fn QSGTextureProvider_textureChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QSGTextureProvider_textureChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QSGTextureProvider_textureChanged_signal_connect for fn() {
  fn connect(self, sigthis: QSGTextureProvider_textureChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSGTextureProvider_textureChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSGTextureProvider_SlotProxy_connect__ZN18QSGTextureProvider14textureChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSGTextureProvider_textureChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QSGTextureProvider_textureChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSGTextureProvider_textureChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSGTextureProvider_SlotProxy_connect__ZN18QSGTextureProvider14textureChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

