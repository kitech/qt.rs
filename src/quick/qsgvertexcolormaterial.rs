// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgvertexcolormaterial.h
// dst-file: /src/quick/qsgvertexcolormaterial.rs
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
use super::qsgmaterial::QSGMaterial; // 773
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGVertexColorMaterial_Class_Size() -> c_int;
  // proto:  int QSGVertexColorMaterial::compare(const QSGMaterial * other);
  fn _ZNK22QSGVertexColorMaterial7compareEPK11QSGMaterial(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QSGVertexColorMaterial::QSGVertexColorMaterial();
  fn _ZN22QSGVertexColorMaterialC2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSGVertexColorMaterial)=1
#[derive(Default)]
pub struct QSGVertexColorMaterial {
  qbase: QSGMaterial,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGVertexColorMaterial {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGVertexColorMaterial {
    return QSGVertexColorMaterial{qbase: QSGMaterial::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGVertexColorMaterial {
  type Target = QSGMaterial;

  fn deref(&self) -> &QSGMaterial {
    return & self.qbase;
  }
}
impl AsRef<QSGMaterial> for QSGVertexColorMaterial {
  fn as_ref(& self) -> & QSGMaterial {
    return & self.qbase;
  }
}
  // proto:  int QSGVertexColorMaterial::compare(const QSGMaterial * other);
impl /*struct*/ QSGVertexColorMaterial {
  pub fn compare<RetType, T: QSGVertexColorMaterial_compare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.compare(self);
    // return 1;
  }
}

pub trait QSGVertexColorMaterial_compare<RetType> {
  fn compare(self , rsthis: & QSGVertexColorMaterial) -> RetType;
}

  // proto:  int QSGVertexColorMaterial::compare(const QSGMaterial * other);
impl<'a> /*trait*/ QSGVertexColorMaterial_compare<i32> for (&'a QSGMaterial) {
  fn compare(self , rsthis: & QSGVertexColorMaterial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QSGVertexColorMaterial7compareEPK11QSGMaterial()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK22QSGVertexColorMaterial7compareEPK11QSGMaterial(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSGVertexColorMaterial::QSGVertexColorMaterial();
impl /*struct*/ QSGVertexColorMaterial {
  pub fn new<T: QSGVertexColorMaterial_new>(value: T) -> QSGVertexColorMaterial {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGVertexColorMaterial_new {
  fn new(self) -> QSGVertexColorMaterial;
}

  // proto:  void QSGVertexColorMaterial::QSGVertexColorMaterial();
impl<'a> /*trait*/ QSGVertexColorMaterial_new for () {
  fn new(self) -> QSGVertexColorMaterial {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QSGVertexColorMaterialC2Ev()};
    let ctysz: c_int = unsafe{QSGVertexColorMaterial_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN22QSGVertexColorMaterialC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGVertexColorMaterial{qbase: QSGMaterial::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

