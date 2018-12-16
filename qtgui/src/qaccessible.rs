

// mod ::gui::QAccessible
// package qtgui
// /usr/include/qt/QtGui/qaccessible.h
// #include <qaccessible.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QAccessible)=1
pub struct QAccessible {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessible_ITF interface {
//    QAccessible_PTR() *QAccessible
//}
//func (ptr *QAccessible) QAccessible_PTR() *QAccessible { return ptr }

impl /*struct*/ QAccessible {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessible {
    return QAccessible{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessible {
//  type Target = QAccessibleBASE;
//
//  fn deref(&self) -> &QAccessibleBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleBASE> for QAccessible {
//  fn as_ref(& self) -> & QAccessibleBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:414
// index:0
// Public static Visibility=Default Availability=Available
// [8] QAccessibleInterface * queryAccessibleInterface(QObject *)

/*
If a QAccessibleInterface implementation exists for the given object, this function returns a pointer to the implementation; otherwise it returns 0.

The function calls all installed factory functions (from most recently installed to least recently installed) until one is found that provides an interface for the class of object. If no factory can provide an accessibility implementation for the class the function loads installed accessibility plugins, and tests if any of the plugins can provide the implementation.

If no implementation for the object's class is available, the function tries to find an implementation for the object's parent class, using the above strategy.

All interfaces are managed by an internal cache and should not be deleted.
*/
impl /*struct*/ QAccessible {
  pub fn queryAccessibleInterface_0<RetType, T: QAccessible_queryAccessibleInterface_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.queryAccessibleInterface_0();
    // return 1;
  }
}
pub trait QAccessible_queryAccessibleInterface_0<RetType> {
  fn queryAccessibleInterface_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_queryAccessibleInterface_0<usize> for (usize) {
  fn queryAccessibleInterface_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QAccessible24queryAccessibleInterfaceEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:415
// index:0
// Public static Visibility=Default Availability=Available
// [4] QAccessible::Id uniqueId(QAccessibleInterface *)

/*
Returns the unique ID for the QAccessibleInterface iface.
*/
impl /*struct*/ QAccessible {
  pub fn uniqueId_0<RetType, T: QAccessible_uniqueId_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.uniqueId_0();
    // return 1;
  }
}
pub trait QAccessible_uniqueId_0<RetType> {
  fn uniqueId_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_uniqueId_0<u32> for (usize) {
  fn uniqueId_0(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QAccessible8uniqueIdEP20QAccessibleInterface", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:416
// index:0
// Public static Visibility=Default Availability=Available
// [8] QAccessibleInterface * accessibleInterface(QAccessible::Id)

/*
Returns the QAccessibleInterface belonging to the id.

Returns 0 if the id is invalid.
*/
impl /*struct*/ QAccessible {
  pub fn accessibleInterface_0<RetType, T: QAccessible_accessibleInterface_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.accessibleInterface_0();
    // return 1;
  }
}
pub trait QAccessible_accessibleInterface_0<RetType> {
  fn accessibleInterface_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_accessibleInterface_0<usize> for (u32) {
  fn accessibleInterface_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QAccessible19accessibleInterfaceEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:417
// index:0
// Public static Visibility=Default Availability=Available
// [4] QAccessible::Id registerAccessibleInterface(QAccessibleInterface *)

/*
Call this function to ensure that manually created interfaces are properly memory managed.

Must only be called exactly once per interface iface. This is implicitly called when calling queryAccessibleInterface, calling this function is only required when QAccessibleInterfaces are instantiated with the "new" operator. This is not recommended, whenever possible use the default functions and let queryAccessibleInterface() take care of this.

When it is necessary to reimplement the QAccessibleInterface::child() function and returning the child after constructing it, this function needs to be called.
*/
impl /*struct*/ QAccessible {
  pub fn registerAccessibleInterface_0<RetType, T: QAccessible_registerAccessibleInterface_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerAccessibleInterface_0();
    // return 1;
  }
}
pub trait QAccessible_registerAccessibleInterface_0<RetType> {
  fn registerAccessibleInterface_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_registerAccessibleInterface_0<u32> for (usize) {
  fn registerAccessibleInterface_0(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:418
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void deleteAccessibleInterface(QAccessible::Id)

/*
Removes the interface belonging to this id from the cache and deletes it. The id becomes invalid an may be re-used by the cache.
*/
impl /*struct*/ QAccessible {
  pub fn deleteAccessibleInterface_0<RetType, T: QAccessible_deleteAccessibleInterface_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.deleteAccessibleInterface_0();
    // return 1;
  }
}
pub trait QAccessible_deleteAccessibleInterface_0<RetType> {
  fn deleteAccessibleInterface_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_deleteAccessibleInterface_0<(/*void*/)> for (u32) {
  fn deleteAccessibleInterface_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QAccessible25deleteAccessibleInterfaceEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:424
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void updateAccessibility(QAccessibleEvent *)

/*
Notifies about a change that might be relevant for accessibility clients.

event provides details about the change. These include the source of the change and the nature of the change. The event should contain enough information give meaningful notifications.

For example, the type ValueChange indicates that the position of a slider has been changed.

Call this function whenever the state of your accessible object or one of its sub-elements has been changed either programmatically (e.g. by calling QLabel::setText()) or by user interaction.

If there are no accessibility tools listening to this event, the performance penalty for calling this function is small, but if determining the parameters of the call is expensive you can test QAccessible::isActive() to avoid unnecessary computation.
*/
impl /*struct*/ QAccessible {
  pub fn updateAccessibility_0<RetType, T: QAccessible_updateAccessibility_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.updateAccessibility_0();
    // return 1;
  }
}
pub trait QAccessible_updateAccessibility_0<RetType> {
  fn updateAccessibility_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_updateAccessibility_0<(/*void*/)> for (usize) {
  fn updateAccessibility_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:426
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isActive()

/*
Returns true if the platform requested accessibility information.

This function will return false until a tool such as a screen reader accessed the accessibility framework. It is still possible to use QAccessible::queryAccessibleInterface() even if accessibility is not active. But there will be no notifications sent to the platform.

It is recommended to use this function to prevent expensive notifications via updateAccessibility() when they are not needed.
*/
impl /*struct*/ QAccessible {
  pub fn isActive_0<RetType, T: QAccessible_isActive_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isActive_0();
    // return 1;
  }
}
pub trait QAccessible_isActive_0<RetType> {
  fn isActive_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_isActive_0<bool> for () {
  fn isActive_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QAccessible8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:427
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setActive(bool)

/*

*/
impl /*struct*/ QAccessible {
  pub fn setActive_0<RetType, T: QAccessible_setActive_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setActive_0();
    // return 1;
  }
}
pub trait QAccessible_setActive_0<RetType> {
  fn setActive_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_setActive_0<(/*void*/)> for (bool) {
  fn setActive_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QAccessible9setActiveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:428
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setRootObject(QObject *)

/*
Sets the root object of the accessible objects of this application to object. All other accessible objects are reachable using object navigation from the root object.

Normally, it isn't necessary to call this function, because Qt sets the QApplication object as the root object immediately before the event loop is entered in QApplication::exec().

Use QAccessible::installRootObjectHandler() to redirect the function call to a customized handler function.

See also queryAccessibleInterface().
*/
impl /*struct*/ QAccessible {
  pub fn setRootObject_0<RetType, T: QAccessible_setRootObject_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setRootObject_0();
    // return 1;
  }
}
pub trait QAccessible_setRootObject_0<RetType> {
  fn setRootObject_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_setRootObject_0<(/*void*/)> for (usize) {
  fn setRootObject_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QAccessible13setRootObjectEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:430
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void cleanup()

/*

*/
impl /*struct*/ QAccessible {
  pub fn cleanup_0<RetType, T: QAccessible_cleanup_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanup_0();
    // return 1;
  }
}
pub trait QAccessible_cleanup_0<RetType> {
  fn cleanup_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAccessible_cleanup_0<(/*void*/)> for () {
  fn cleanup_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QAccessible7cleanupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQAccessible(this :*mut QAccessible) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QAccessibleD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum type defines accessible event types.

Internal: Used when creating subclasses of QAccessibleEvent.



The values for this enum are defined to be the same as those defined in the IAccessible2 and MSAA specifications.

*/
pub type QAccessible__Event = i32;
// 
pub const QAccessible__SoundPlayed :QAccessible__Event = 1;
// 
pub const QAccessible__Alert :QAccessible__Event = 2;
// 
pub const QAccessible__ForegroundChanged :QAccessible__Event = 3;
// 
pub const QAccessible__MenuStart :QAccessible__Event = 4;
// 
pub const QAccessible__MenuEnd :QAccessible__Event = 5;
// 
pub const QAccessible__PopupMenuStart :QAccessible__Event = 6;
// 
pub const QAccessible__PopupMenuEnd :QAccessible__Event = 7;
// 
pub const QAccessible__ContextHelpStart :QAccessible__Event = 12;
// 
pub const QAccessible__ContextHelpEnd :QAccessible__Event = 13;
// 
pub const QAccessible__DragDropStart :QAccessible__Event = 14;
// 
pub const QAccessible__DragDropEnd :QAccessible__Event = 15;
// 
pub const QAccessible__DialogStart :QAccessible__Event = 16;
// 
pub const QAccessible__DialogEnd :QAccessible__Event = 17;
// 
pub const QAccessible__ScrollingStart :QAccessible__Event = 18;
// 
pub const QAccessible__ScrollingEnd :QAccessible__Event = 19;
// 
pub const QAccessible__MenuCommand :QAccessible__Event = 24;
// 
pub const QAccessible__ActionChanged :QAccessible__Event = 257;
// 
pub const QAccessible__ActiveDescendantChanged :QAccessible__Event = 258;
// 
pub const QAccessible__AttributeChanged :QAccessible__Event = 259;
// 
pub const QAccessible__DocumentContentChanged :QAccessible__Event = 260;
// 
pub const QAccessible__DocumentLoadComplete :QAccessible__Event = 261;
// 
pub const QAccessible__DocumentLoadStopped :QAccessible__Event = 262;
// 
pub const QAccessible__DocumentReload :QAccessible__Event = 263;
// 
pub const QAccessible__HyperlinkEndIndexChanged :QAccessible__Event = 264;
// 
pub const QAccessible__HyperlinkNumberOfAnchorsChanged :QAccessible__Event = 265;
// 
pub const QAccessible__HyperlinkSelectedLinkChanged :QAccessible__Event = 266;
// 
pub const QAccessible__HypertextLinkActivated :QAccessible__Event = 267;
// 
pub const QAccessible__HypertextLinkSelected :QAccessible__Event = 268;
// 
pub const QAccessible__HyperlinkStartIndexChanged :QAccessible__Event = 269;
// 
pub const QAccessible__HypertextChanged :QAccessible__Event = 270;
// 
pub const QAccessible__HypertextNLinksChanged :QAccessible__Event = 271;
// 
pub const QAccessible__ObjectAttributeChanged :QAccessible__Event = 272;
// 
pub const QAccessible__PageChanged :QAccessible__Event = 273;
// 
pub const QAccessible__SectionChanged :QAccessible__Event = 274;
// 
pub const QAccessible__TableCaptionChanged :QAccessible__Event = 275;
// 
pub const QAccessible__TableColumnDescriptionChanged :QAccessible__Event = 276;
// 
pub const QAccessible__TableColumnHeaderChanged :QAccessible__Event = 277;
// 
pub const QAccessible__TableModelChanged :QAccessible__Event = 278;
// 
pub const QAccessible__TableRowDescriptionChanged :QAccessible__Event = 279;
// 
pub const QAccessible__TableRowHeaderChanged :QAccessible__Event = 280;
// 
pub const QAccessible__TableSummaryChanged :QAccessible__Event = 281;
// 
pub const QAccessible__TextAttributeChanged :QAccessible__Event = 282;
// 
pub const QAccessible__TextCaretMoved :QAccessible__Event = 283;
// 
pub const QAccessible__TextColumnChanged :QAccessible__Event = 285;
// 
pub const QAccessible__TextInserted :QAccessible__Event = 286;
// 
pub const QAccessible__TextRemoved :QAccessible__Event = 287;
// 
pub const QAccessible__TextUpdated :QAccessible__Event = 288;
// 
pub const QAccessible__TextSelectionChanged :QAccessible__Event = 289;
// 
pub const QAccessible__VisibleDataChanged :QAccessible__Event = 290;
// 
pub const QAccessible__ObjectCreated :QAccessible__Event = 32768;
// 
pub const QAccessible__ObjectDestroyed :QAccessible__Event = 32769;
// 
pub const QAccessible__ObjectShow :QAccessible__Event = 32770;
// 
pub const QAccessible__ObjectHide :QAccessible__Event = 32771;
// 
pub const QAccessible__ObjectReorder :QAccessible__Event = 32772;
// 
pub const QAccessible__Focus :QAccessible__Event = 32773;
// 
pub const QAccessible__Selection :QAccessible__Event = 32774;
// 
pub const QAccessible__SelectionAdd :QAccessible__Event = 32775;
// 
pub const QAccessible__SelectionRemove :QAccessible__Event = 32776;
// 
pub const QAccessible__SelectionWithin :QAccessible__Event = 32777;
// 
pub const QAccessible__StateChanged :QAccessible__Event = 32778;
// 
pub const QAccessible__LocationChanged :QAccessible__Event = 32779;
// 
pub const QAccessible__NameChanged :QAccessible__Event = 32780;
// 
pub const QAccessible__DescriptionChanged :QAccessible__Event = 32781;
// 
pub const QAccessible__ValueChanged :QAccessible__Event = 32782;
// 
pub const QAccessible__ParentChanged :QAccessible__Event = 32783;
// 
pub const QAccessible__HelpChanged :QAccessible__Event = 32928;
// 
pub const QAccessible__DefaultActionChanged :QAccessible__Event = 32944;
// 
pub const QAccessible__AcceleratorChanged :QAccessible__Event = 32960;
// 
pub const QAccessible__InvalidEvent :QAccessible__Event = 32961;
pub fn QAccessible_EventItemName(val: i32) ->String {
  match val {
     QAccessible__SoundPlayed => // 1
     {return String::from("SoundPlayed");}
     QAccessible__Alert => // 2
     {return String::from("Alert");}
     QAccessible__ForegroundChanged => // 3
     {return String::from("ForegroundChanged");}
     QAccessible__MenuStart => // 4
     {return String::from("MenuStart");}
     QAccessible__MenuEnd => // 5
     {return String::from("MenuEnd");}
     QAccessible__PopupMenuStart => // 6
     {return String::from("PopupMenuStart");}
     QAccessible__PopupMenuEnd => // 7
     {return String::from("PopupMenuEnd");}
     QAccessible__ContextHelpStart => // 12
     {return String::from("ContextHelpStart");}
     QAccessible__ContextHelpEnd => // 13
     {return String::from("ContextHelpEnd");}
     QAccessible__DragDropStart => // 14
     {return String::from("DragDropStart");}
     QAccessible__DragDropEnd => // 15
     {return String::from("DragDropEnd");}
     QAccessible__DialogStart => // 16
     {return String::from("DialogStart");}
     QAccessible__DialogEnd => // 17
     {return String::from("DialogEnd");}
     QAccessible__ScrollingStart => // 18
     {return String::from("ScrollingStart");}
     QAccessible__ScrollingEnd => // 19
     {return String::from("ScrollingEnd");}
     QAccessible__MenuCommand => // 24
     {return String::from("MenuCommand");}
     QAccessible__ActionChanged => // 257
     {return String::from("ActionChanged");}
     QAccessible__ActiveDescendantChanged => // 258
     {return String::from("ActiveDescendantChanged");}
     QAccessible__AttributeChanged => // 259
     {return String::from("AttributeChanged");}
     QAccessible__DocumentContentChanged => // 260
     {return String::from("DocumentContentChanged");}
     QAccessible__DocumentLoadComplete => // 261
     {return String::from("DocumentLoadComplete");}
     QAccessible__DocumentLoadStopped => // 262
     {return String::from("DocumentLoadStopped");}
     QAccessible__DocumentReload => // 263
     {return String::from("DocumentReload");}
     QAccessible__HyperlinkEndIndexChanged => // 264
     {return String::from("HyperlinkEndIndexChanged");}
     QAccessible__HyperlinkNumberOfAnchorsChanged => // 265
     {return String::from("HyperlinkNumberOfAnchorsChanged");}
     QAccessible__HyperlinkSelectedLinkChanged => // 266
     {return String::from("HyperlinkSelectedLinkChanged");}
     QAccessible__HypertextLinkActivated => // 267
     {return String::from("HypertextLinkActivated");}
     QAccessible__HypertextLinkSelected => // 268
     {return String::from("HypertextLinkSelected");}
     QAccessible__HyperlinkStartIndexChanged => // 269
     {return String::from("HyperlinkStartIndexChanged");}
     QAccessible__HypertextChanged => // 270
     {return String::from("HypertextChanged");}
     QAccessible__HypertextNLinksChanged => // 271
     {return String::from("HypertextNLinksChanged");}
     QAccessible__ObjectAttributeChanged => // 272
     {return String::from("ObjectAttributeChanged");}
     QAccessible__PageChanged => // 273
     {return String::from("PageChanged");}
     QAccessible__SectionChanged => // 274
     {return String::from("SectionChanged");}
     QAccessible__TableCaptionChanged => // 275
     {return String::from("TableCaptionChanged");}
     QAccessible__TableColumnDescriptionChanged => // 276
     {return String::from("TableColumnDescriptionChanged");}
     QAccessible__TableColumnHeaderChanged => // 277
     {return String::from("TableColumnHeaderChanged");}
     QAccessible__TableModelChanged => // 278
     {return String::from("TableModelChanged");}
     QAccessible__TableRowDescriptionChanged => // 279
     {return String::from("TableRowDescriptionChanged");}
     QAccessible__TableRowHeaderChanged => // 280
     {return String::from("TableRowHeaderChanged");}
     QAccessible__TableSummaryChanged => // 281
     {return String::from("TableSummaryChanged");}
     QAccessible__TextAttributeChanged => // 282
     {return String::from("TextAttributeChanged");}
     QAccessible__TextCaretMoved => // 283
     {return String::from("TextCaretMoved");}
     QAccessible__TextColumnChanged => // 285
     {return String::from("TextColumnChanged");}
     QAccessible__TextInserted => // 286
     {return String::from("TextInserted");}
     QAccessible__TextRemoved => // 287
     {return String::from("TextRemoved");}
     QAccessible__TextUpdated => // 288
     {return String::from("TextUpdated");}
     QAccessible__TextSelectionChanged => // 289
     {return String::from("TextSelectionChanged");}
     QAccessible__VisibleDataChanged => // 290
     {return String::from("VisibleDataChanged");}
     QAccessible__ObjectCreated => // 32768
     {return String::from("ObjectCreated");}
     QAccessible__ObjectDestroyed => // 32769
     {return String::from("ObjectDestroyed");}
     QAccessible__ObjectShow => // 32770
     {return String::from("ObjectShow");}
     QAccessible__ObjectHide => // 32771
     {return String::from("ObjectHide");}
     QAccessible__ObjectReorder => // 32772
     {return String::from("ObjectReorder");}
     QAccessible__Focus => // 32773
     {return String::from("Focus");}
     QAccessible__Selection => // 32774
     {return String::from("Selection");}
     QAccessible__SelectionAdd => // 32775
     {return String::from("SelectionAdd");}
     QAccessible__SelectionRemove => // 32776
     {return String::from("SelectionRemove");}
     QAccessible__SelectionWithin => // 32777
     {return String::from("SelectionWithin");}
     QAccessible__StateChanged => // 32778
     {return String::from("StateChanged");}
     QAccessible__LocationChanged => // 32779
     {return String::from("LocationChanged");}
     QAccessible__NameChanged => // 32780
     {return String::from("NameChanged");}
     QAccessible__DescriptionChanged => // 32781
     {return String::from("DescriptionChanged");}
     QAccessible__ValueChanged => // 32782
     {return String::from("ValueChanged");}
     QAccessible__ParentChanged => // 32783
     {return String::from("ParentChanged");}
     QAccessible__HelpChanged => // 32928
     {return String::from("HelpChanged");}
     QAccessible__DefaultActionChanged => // 32944
     {return String::from("DefaultActionChanged");}
     QAccessible__AcceleratorChanged => // 32960
     {return String::from("AcceleratorChanged");}
     QAccessible__InvalidEvent => // 32961
     {return String::from("InvalidEvent");}
  _ => {return format!("{}", val);}
}
}
pub fn QAccessible_EventItemName_s(val: i32) ->String {
  //var nilthis *QAccessible
  //return nilthis.EventItemName(val);
  return QAccessible_EventItemName(val);
}


/*
This enum defines the role of an accessible object. The roles are:


*/
pub type QAccessible__Role = i32;
// 
pub const QAccessible__NoRole :QAccessible__Role = 0;
// 
pub const QAccessible__TitleBar :QAccessible__Role = 1;
// 
pub const QAccessible__MenuBar :QAccessible__Role = 2;
// 
pub const QAccessible__ScrollBar :QAccessible__Role = 3;
// 
pub const QAccessible__Grip :QAccessible__Role = 4;
// 
pub const QAccessible__Sound :QAccessible__Role = 5;
// 
pub const QAccessible__Cursor :QAccessible__Role = 6;
// 
pub const QAccessible__Caret :QAccessible__Role = 7;
// 
pub const QAccessible__AlertMessage :QAccessible__Role = 8;
// 
pub const QAccessible__Window :QAccessible__Role = 9;
// 
pub const QAccessible__Client :QAccessible__Role = 10;
// 
pub const QAccessible__PopupMenu :QAccessible__Role = 11;
// 
pub const QAccessible__MenuItem :QAccessible__Role = 12;
// 
pub const QAccessible__ToolTip :QAccessible__Role = 13;
// 
pub const QAccessible__Application :QAccessible__Role = 14;
// 
pub const QAccessible__Document :QAccessible__Role = 15;
// 
pub const QAccessible__Pane :QAccessible__Role = 16;
// 
pub const QAccessible__Chart :QAccessible__Role = 17;
// 
pub const QAccessible__Dialog :QAccessible__Role = 18;
// 
pub const QAccessible__Border :QAccessible__Role = 19;
// 
pub const QAccessible__Grouping :QAccessible__Role = 20;
// 
pub const QAccessible__Separator :QAccessible__Role = 21;
// 
pub const QAccessible__ToolBar :QAccessible__Role = 22;
// 
pub const QAccessible__StatusBar :QAccessible__Role = 23;
// 
pub const QAccessible__Table :QAccessible__Role = 24;
// 
pub const QAccessible__ColumnHeader :QAccessible__Role = 25;
// 
pub const QAccessible__RowHeader :QAccessible__Role = 26;
// 
pub const QAccessible__Column :QAccessible__Role = 27;
// 
pub const QAccessible__Row :QAccessible__Role = 28;
// 
pub const QAccessible__Cell :QAccessible__Role = 29;
// 
pub const QAccessible__Link :QAccessible__Role = 30;
// 
pub const QAccessible__HelpBalloon :QAccessible__Role = 31;
// 
pub const QAccessible__Assistant :QAccessible__Role = 32;
// 
pub const QAccessible__List :QAccessible__Role = 33;
// 
pub const QAccessible__ListItem :QAccessible__Role = 34;
// 
pub const QAccessible__Tree :QAccessible__Role = 35;
// 
pub const QAccessible__TreeItem :QAccessible__Role = 36;
// 
pub const QAccessible__PageTab :QAccessible__Role = 37;
// 
pub const QAccessible__PropertyPage :QAccessible__Role = 38;
// 
pub const QAccessible__Indicator :QAccessible__Role = 39;
// 
pub const QAccessible__Graphic :QAccessible__Role = 40;
// 
pub const QAccessible__StaticText :QAccessible__Role = 41;
// 
pub const QAccessible__EditableText :QAccessible__Role = 42;
// 
pub const QAccessible__Button :QAccessible__Role = 43;
// 
pub const QAccessible__PushButton :QAccessible__Role = 43;
// 
pub const QAccessible__CheckBox :QAccessible__Role = 44;
// 
pub const QAccessible__RadioButton :QAccessible__Role = 45;
// 
pub const QAccessible__ComboBox :QAccessible__Role = 46;
// 
pub const QAccessible__ProgressBar :QAccessible__Role = 48;
// 
pub const QAccessible__Dial :QAccessible__Role = 49;
// 
pub const QAccessible__HotkeyField :QAccessible__Role = 50;
// 
pub const QAccessible__Slider :QAccessible__Role = 51;
// 
pub const QAccessible__SpinBox :QAccessible__Role = 52;
// 
pub const QAccessible__Canvas :QAccessible__Role = 53;
// 
pub const QAccessible__Animation :QAccessible__Role = 54;
// 
pub const QAccessible__Equation :QAccessible__Role = 55;
// 
pub const QAccessible__ButtonDropDown :QAccessible__Role = 56;
// 
pub const QAccessible__ButtonMenu :QAccessible__Role = 57;
// 
pub const QAccessible__ButtonDropGrid :QAccessible__Role = 58;
// 
pub const QAccessible__Whitespace :QAccessible__Role = 59;
// 
pub const QAccessible__PageTabList :QAccessible__Role = 60;
// 
pub const QAccessible__Clock :QAccessible__Role = 61;
// 
pub const QAccessible__Splitter :QAccessible__Role = 62;
// 
pub const QAccessible__LayeredPane :QAccessible__Role = 128;
// 
pub const QAccessible__Terminal :QAccessible__Role = 129;
// 
pub const QAccessible__Desktop :QAccessible__Role = 130;
// 
pub const QAccessible__Paragraph :QAccessible__Role = 131;
// 
pub const QAccessible__WebDocument :QAccessible__Role = 132;
// 
pub const QAccessible__Section :QAccessible__Role = 133;
// 
pub const QAccessible__ColorChooser :QAccessible__Role = 1028;
// 
pub const QAccessible__Footer :QAccessible__Role = 1038;
// 
pub const QAccessible__Form :QAccessible__Role = 1040;
// 
pub const QAccessible__Heading :QAccessible__Role = 1044;
// 
pub const QAccessible__Note :QAccessible__Role = 1051;
// 
pub const QAccessible__ComplementaryContent :QAccessible__Role = 1068;
// 
pub const QAccessible__UserRole :QAccessible__Role = 65535;
pub fn QAccessible_RoleItemName(val: i32) ->String {
  match val {
     QAccessible__NoRole => // 0
     {return String::from("NoRole");}
     QAccessible__TitleBar => // 1
     {return String::from("TitleBar");}
     QAccessible__MenuBar => // 2
     {return String::from("MenuBar");}
     QAccessible__ScrollBar => // 3
     {return String::from("ScrollBar");}
     QAccessible__Grip => // 4
     {return String::from("Grip");}
     QAccessible__Sound => // 5
     {return String::from("Sound");}
     QAccessible__Cursor => // 6
     {return String::from("Cursor");}
     QAccessible__Caret => // 7
     {return String::from("Caret");}
     QAccessible__AlertMessage => // 8
     {return String::from("AlertMessage");}
     QAccessible__Window => // 9
     {return String::from("Window");}
     QAccessible__Client => // 10
     {return String::from("Client");}
     QAccessible__PopupMenu => // 11
     {return String::from("PopupMenu");}
     QAccessible__MenuItem => // 12
     {return String::from("MenuItem");}
     QAccessible__ToolTip => // 13
     {return String::from("ToolTip");}
     QAccessible__Application => // 14
     {return String::from("Application");}
     QAccessible__Document => // 15
     {return String::from("Document");}
     QAccessible__Pane => // 16
     {return String::from("Pane");}
     QAccessible__Chart => // 17
     {return String::from("Chart");}
     QAccessible__Dialog => // 18
     {return String::from("Dialog");}
     QAccessible__Border => // 19
     {return String::from("Border");}
     QAccessible__Grouping => // 20
     {return String::from("Grouping");}
     QAccessible__Separator => // 21
     {return String::from("Separator");}
     QAccessible__ToolBar => // 22
     {return String::from("ToolBar");}
     QAccessible__StatusBar => // 23
     {return String::from("StatusBar");}
     QAccessible__Table => // 24
     {return String::from("Table");}
     QAccessible__ColumnHeader => // 25
     {return String::from("ColumnHeader");}
     QAccessible__RowHeader => // 26
     {return String::from("RowHeader");}
     QAccessible__Column => // 27
     {return String::from("Column");}
     QAccessible__Row => // 28
     {return String::from("Row");}
     QAccessible__Cell => // 29
     {return String::from("Cell");}
     QAccessible__Link => // 30
     {return String::from("Link");}
     QAccessible__HelpBalloon => // 31
     {return String::from("HelpBalloon");}
     QAccessible__Assistant => // 32
     {return String::from("Assistant");}
     QAccessible__List => // 33
     {return String::from("List");}
     QAccessible__ListItem => // 34
     {return String::from("ListItem");}
     QAccessible__Tree => // 35
     {return String::from("Tree");}
     QAccessible__TreeItem => // 36
     {return String::from("TreeItem");}
     QAccessible__PageTab => // 37
     {return String::from("PageTab");}
     QAccessible__PropertyPage => // 38
     {return String::from("PropertyPage");}
     QAccessible__Indicator => // 39
     {return String::from("Indicator");}
     QAccessible__Graphic => // 40
     {return String::from("Graphic");}
     QAccessible__StaticText => // 41
     {return String::from("StaticText");}
     QAccessible__EditableText => // 42
     {return String::from("EditableText");}
     QAccessible__Button => // 43
     {return String::from("Button,PushButton");}
    // QAccessible__PushButton => // 43
    // {return String::from("");}
     QAccessible__CheckBox => // 44
     {return String::from("CheckBox");}
     QAccessible__RadioButton => // 45
     {return String::from("RadioButton");}
     QAccessible__ComboBox => // 46
     {return String::from("ComboBox");}
     QAccessible__ProgressBar => // 48
     {return String::from("ProgressBar");}
     QAccessible__Dial => // 49
     {return String::from("Dial");}
     QAccessible__HotkeyField => // 50
     {return String::from("HotkeyField");}
     QAccessible__Slider => // 51
     {return String::from("Slider");}
     QAccessible__SpinBox => // 52
     {return String::from("SpinBox");}
     QAccessible__Canvas => // 53
     {return String::from("Canvas");}
     QAccessible__Animation => // 54
     {return String::from("Animation");}
     QAccessible__Equation => // 55
     {return String::from("Equation");}
     QAccessible__ButtonDropDown => // 56
     {return String::from("ButtonDropDown");}
     QAccessible__ButtonMenu => // 57
     {return String::from("ButtonMenu");}
     QAccessible__ButtonDropGrid => // 58
     {return String::from("ButtonDropGrid");}
     QAccessible__Whitespace => // 59
     {return String::from("Whitespace");}
     QAccessible__PageTabList => // 60
     {return String::from("PageTabList");}
     QAccessible__Clock => // 61
     {return String::from("Clock");}
     QAccessible__Splitter => // 62
     {return String::from("Splitter");}
     QAccessible__LayeredPane => // 128
     {return String::from("LayeredPane");}
     QAccessible__Terminal => // 129
     {return String::from("Terminal");}
     QAccessible__Desktop => // 130
     {return String::from("Desktop");}
     QAccessible__Paragraph => // 131
     {return String::from("Paragraph");}
     QAccessible__WebDocument => // 132
     {return String::from("WebDocument");}
     QAccessible__Section => // 133
     {return String::from("Section");}
     QAccessible__ColorChooser => // 1028
     {return String::from("ColorChooser");}
     QAccessible__Footer => // 1038
     {return String::from("Footer");}
     QAccessible__Form => // 1040
     {return String::from("Form");}
     QAccessible__Heading => // 1044
     {return String::from("Heading");}
     QAccessible__Note => // 1051
     {return String::from("Note");}
     QAccessible__ComplementaryContent => // 1068
     {return String::from("ComplementaryContent");}
     QAccessible__UserRole => // 65535
     {return String::from("UserRole");}
  _ => {return format!("{}", val);}
}
}
pub fn QAccessible_RoleItemName_s(val: i32) ->String {
  //var nilthis *QAccessible
  //return nilthis.RoleItemName(val);
  return QAccessible_RoleItemName(val);
}


/*
This enum specifies string information that an accessible object returns.


*/
pub type QAccessible__Text = i32;
// The name of the object. This can be used both as an identifier or a short description by accessible clients.
pub const QAccessible__Name :QAccessible__Text = 0;
// A short text describing the object.
pub const QAccessible__Description :QAccessible__Text = 1;
// The value of the object.
pub const QAccessible__Value :QAccessible__Text = 2;
// A longer text giving information about how to use the object.
pub const QAccessible__Help :QAccessible__Text = 3;
// The keyboard shortcut that executes the object's default action.
pub const QAccessible__Accelerator :QAccessible__Text = 4;
// 
pub const QAccessible__DebugDescription :QAccessible__Text = 5;
// 
pub const QAccessible__UserText :QAccessible__Text = 65535;
pub fn QAccessible_TextItemName(val: i32) ->String {
  match val {
     QAccessible__Name => // 0
     {return String::from("Name");}
     QAccessible__Description => // 1
     {return String::from("Description");}
     QAccessible__Value => // 2
     {return String::from("Value");}
     QAccessible__Help => // 3
     {return String::from("Help");}
     QAccessible__Accelerator => // 4
     {return String::from("Accelerator");}
     QAccessible__DebugDescription => // 5
     {return String::from("DebugDescription");}
     QAccessible__UserText => // 65535
     {return String::from("UserText");}
  _ => {return format!("{}", val);}
}
}
pub fn QAccessible_TextItemName_s(val: i32) ->String {
  //var nilthis *QAccessible
  //return nilthis.TextItemName(val);
  return QAccessible_TextItemName(val);
}


/*


*/
pub type QAccessible__RelationFlag = i32;
// 
pub const QAccessible__Label :QAccessible__RelationFlag = 1;
// 
pub const QAccessible__Labelled :QAccessible__RelationFlag = 2;
// 
pub const QAccessible__Controller :QAccessible__RelationFlag = 4;
// 
pub const QAccessible__Controlled :QAccessible__RelationFlag = 8;
// 
pub const QAccessible__AllRelations :QAccessible__RelationFlag = -1;
pub fn QAccessible_RelationFlagItemName(val: i32) ->String {
  match val {
     QAccessible__Label => // 1
     {return String::from("Label");}
     QAccessible__Labelled => // 2
     {return String::from("Labelled");}
     QAccessible__Controller => // 4
     {return String::from("Controller");}
     QAccessible__Controlled => // 8
     {return String::from("Controlled");}
     QAccessible__AllRelations => // -1
     {return String::from("AllRelations");}
  _ => {return format!("{}", val);}
}
}
pub fn QAccessible_RelationFlagItemName_s(val: i32) ->String {
  //var nilthis *QAccessible
  //return nilthis.RelationFlagItemName(val);
  return QAccessible_RelationFlagItemName(val);
}


/*
QAccessibleInterface supports several sub interfaces. In order to provide more information about some objects, their accessible representation should implement one or more of these interfaces.

Note: When subclassing one of these interfaces, QAccessibleInterface::interface_cast() needs to be implemented.



See also QAccessibleInterface::interface_cast(), QAccessibleTextInterface, QAccessibleValueInterface, QAccessibleActionInterface, QAccessibleTableInterface, and QAccessibleTableCellInterface.

*/
pub type QAccessible__InterfaceType = i32;
// For text that supports selections or is more than one line. Simple labels do not need to implement this interface. For text that can be edited by the user.
pub const QAccessible__TextInterface :QAccessible__InterfaceType = 0;
// 
pub const QAccessible__EditableTextInterface :QAccessible__InterfaceType = 1;
// For objects that are used to manipulate a value, for example slider or scroll bar.
pub const QAccessible__ValueInterface :QAccessible__InterfaceType = 2;
// For interactive objects that allow the user to trigger an action. Basically everything that allows for example mouse interaction. For objects that represent an image. This interface is generally less important.
pub const QAccessible__ActionInterface :QAccessible__InterfaceType = 3;
// 
pub const QAccessible__ImageInterface :QAccessible__InterfaceType = 4;
// For lists, tables and trees.
pub const QAccessible__TableInterface :QAccessible__InterfaceType = 5;
// For cells in a TableInterface object.
pub const QAccessible__TableCellInterface :QAccessible__InterfaceType = 6;
pub fn QAccessible_InterfaceTypeItemName(val: i32) ->String {
  match val {
     QAccessible__TextInterface => // 0
     {return String::from("TextInterface");}
     QAccessible__EditableTextInterface => // 1
     {return String::from("EditableTextInterface");}
     QAccessible__ValueInterface => // 2
     {return String::from("ValueInterface");}
     QAccessible__ActionInterface => // 3
     {return String::from("ActionInterface");}
     QAccessible__ImageInterface => // 4
     {return String::from("ImageInterface");}
     QAccessible__TableInterface => // 5
     {return String::from("TableInterface");}
     QAccessible__TableCellInterface => // 6
     {return String::from("TableCellInterface");}
  _ => {return format!("{}", val);}
}
}
pub fn QAccessible_InterfaceTypeItemName_s(val: i32) ->String {
  //var nilthis *QAccessible
  //return nilthis.InterfaceTypeItemName(val);
  return QAccessible_InterfaceTypeItemName(val);
}


/*
This enum describes different types of text boundaries. It follows the IAccessible2 API and is used in the QAccessibleTextInterface.



See also QAccessibleTextInterface.

*/
pub type QAccessible__TextBoundaryType = i32;
// Use individual characters as boundary.
pub const QAccessible__CharBoundary :QAccessible__TextBoundaryType = 0;
// Use words as boundaries.
pub const QAccessible__WordBoundary :QAccessible__TextBoundaryType = 1;
// Use sentences as boundary.
pub const QAccessible__SentenceBoundary :QAccessible__TextBoundaryType = 2;
// Use paragraphs as boundary.
pub const QAccessible__ParagraphBoundary :QAccessible__TextBoundaryType = 3;
// Use newlines as boundary.
pub const QAccessible__LineBoundary :QAccessible__TextBoundaryType = 4;
// No boundary (use the whole text).
pub const QAccessible__NoBoundary :QAccessible__TextBoundaryType = 5;
pub fn QAccessible_TextBoundaryTypeItemName(val: i32) ->String {
  match val {
     QAccessible__CharBoundary => // 0
     {return String::from("CharBoundary");}
     QAccessible__WordBoundary => // 1
     {return String::from("WordBoundary");}
     QAccessible__SentenceBoundary => // 2
     {return String::from("SentenceBoundary");}
     QAccessible__ParagraphBoundary => // 3
     {return String::from("ParagraphBoundary");}
     QAccessible__LineBoundary => // 4
     {return String::from("LineBoundary");}
     QAccessible__NoBoundary => // 5
     {return String::from("NoBoundary");}
  _ => {return format!("{}", val);}
}
}
pub fn QAccessible_TextBoundaryTypeItemName_s(val: i32) ->String {
  //var nilthis *QAccessible
  //return nilthis.TextBoundaryTypeItemName(val);
  return QAccessible_TextBoundaryTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
